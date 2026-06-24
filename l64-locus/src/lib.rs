use l64_core::{
    BundleLock, CanonicalStructure, ChangeLedgerEntry, CnormReceipt, DnaHeaderReceipt,
    DnaValidationReport, ExecutionManifest, GenomeArtifactClass, GenomeSurface,
    LocusCapabilityMask, LocusDecodeMode, LocusOpcode, LocusPacket, LocusPacketHeader,
    LocusPacketKind, LocusSection, NormalizedRna, RnaNormalizationReceipt, SemanticLoweringReceipt,
    SsrReceipt, canonical_rna_from_structure, decode_locus_packet_with_mode, dna_header_receipt,
    encode_locus_packet, ensure_cache_subdir, execute_lower_chain, locus_packet_summary,
    validate_dna_packet,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LocusIoError {
    #[error("packet error: {0}")]
    Packet(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("codec error: {0}")]
    Codec(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RnaCompilationArtifact {
    pub normalized: NormalizedRna,
    pub rn_receipt: RnaNormalizationReceipt,
    pub ssr_receipt: SsrReceipt,
    pub cnorm_receipt: CnormReceipt,
    pub canonical_structure: CanonicalStructure,
    pub lowering_receipt: SemanticLoweringReceipt,
    pub dna_header_receipt: DnaHeaderReceipt,
    pub dna_validation: DnaValidationReport,
    #[serde(default)]
    pub phase_ledger: Vec<ChangeLedgerEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RnaDnaRoundtripReport {
    pub canonical_id: String,
    pub canonical_hash: String,
    pub dna_bytes_equal: bool,
    pub canonical_hash_equal: bool,
    pub canonical_rna: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenomeReleaseManifest {
    pub release_id: String,
    pub canonical_hash: String,
    pub canonical_id: String,
    pub source_sequence: String,
    pub canonical_genome: String,
    pub claim_pages: Vec<String>,
    pub dependency_spine: String,
    pub closure_map: String,
    pub closure_frontier: String,
    pub stress_map: String,
    pub lineage: String,
    pub replay_record: String,
    pub view_receipts: Vec<String>,
}

pub fn validate_rna_source_text(input: &str) -> Result<(), LocusIoError> {
    let trimmed = input.trim_start();
    if trimmed.is_empty() {
        return Err(LocusIoError::Packet("RNA source is empty".into()));
    }
    if matches!(trimmed.as_bytes().first(), Some(b'{') | Some(b'['))
        && serde_json::from_str::<serde_json::Value>(trimmed).is_ok()
    {
        return Err(LocusIoError::Packet(
            "compile-rna accepts source RNA only; JSON/report/projection artifacts must use an explicit import or inspection command".into(),
        ));
    }
    let report_markers = [
        "l64_release_artifact:",
        "artifact_role:",
        "surface_kind:",
        "canonical_genome:",
        "dependency_spine:",
        "closure_map:",
        "closure_frontier:",
        "stress_map:",
        "replay_record:",
        "view_receipt:",
        "\"artifact\"",
        "\"canonical_structure\"",
        "\"packet_summary\"",
        "\"lineage\"",
        "\"dna_validation\"",
        "\"phase_ledger\"",
        "\"view_receipt\"",
        "\"claim_page\"",
        "\"closure_map\"",
        "\"stress_map\"",
        "\"replay_record\"",
    ];
    if report_markers.iter().any(|marker| trimmed.contains(marker)) {
        return Err(LocusIoError::Packet(
            "compile-rna rejected a projection/report/receipt artifact; use source RNA or canonical RNA".into(),
        ));
    }
    Ok(())
}

pub fn encode_section_packet<T: Serialize>(
    kind: LocusPacketKind,
    opcode: LocusOpcode,
    subject_id: &str,
    schema_hash: &str,
    payload: &T,
    capabilities: LocusCapabilityMask,
    authority_tier: u8,
) -> Result<Vec<u8>, LocusIoError> {
    let payload_bytes =
        bincode::serialize(payload).map_err(|err| LocusIoError::Codec(err.to_string()))?;
    let packet = LocusPacket {
        header: LocusPacketHeader {
            artifact_class: GenomeArtifactClass::Gene,
            surface: GenomeSurface::Dna,
            kind,
            version_major: 1,
            version_minor: 0,
            authority_tier,
            capabilities,
            grammar_id: "rna.v1".into(),
            schema_hash: schema_hash.into(),
            integrity_hash: l64_core::dna_digest_from_bytes(&payload_bytes).0,
            strand_manifest: vec!["core".into()],
            feature_flags: 0,
            root_subject_id: subject_id.into(),
        },
        sections: vec![LocusSection {
            opcode,
            flags: 0,
            subject_id: subject_id.into(),
            payload: payload_bytes,
        }],
    };
    encode_locus_packet(&packet).map_err(LocusIoError::Packet)
}

pub fn decode_section_payload<T: DeserializeOwned>(
    bytes: &[u8],
    opcode: LocusOpcode,
) -> Result<T, LocusIoError> {
    let packet = decode_locus_packet_with_mode(bytes, LocusDecodeMode::CurrentAuthority)
        .map_err(LocusIoError::Packet)?;
    let section = packet
        .sections
        .iter()
        .find(|item| item.opcode == opcode)
        .ok_or_else(|| {
            LocusIoError::Packet(format!("locus packet missing section {:?}", opcode))
        })?;
    bincode::deserialize(&section.payload).map_err(|err| LocusIoError::Codec(err.to_string()))
}

pub fn decode_summary(
    bytes: &[u8],
) -> Result<std::collections::BTreeMap<String, String>, LocusIoError> {
    let packet = decode_locus_packet_with_mode(bytes, LocusDecodeMode::CurrentAuthority)
        .map_err(LocusIoError::Packet)?;
    Ok(locus_packet_summary(&packet))
}

pub fn write_section_packet<T: Serialize>(
    path: &Path,
    kind: LocusPacketKind,
    opcode: LocusOpcode,
    subject_id: &str,
    schema_hash: &str,
    payload: &T,
    capabilities: LocusCapabilityMask,
    authority_tier: u8,
) -> Result<(), LocusIoError> {
    let bytes = encode_section_packet(
        kind,
        opcode,
        subject_id,
        schema_hash,
        payload,
        capabilities,
        authority_tier,
    )?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, bytes)?;
    Ok(())
}

pub fn manifest_cache_root() -> Result<PathBuf, LocusIoError> {
    Ok(ensure_cache_subdir("manifests").map_err(LocusIoError::Packet)?)
}

fn manifest_packet_path(id: &str) -> Result<PathBuf, LocusIoError> {
    Ok(manifest_cache_root()?.join(format!("{id}.dna")))
}

fn bundle_lock_packet_path(id: &str) -> Result<PathBuf, LocusIoError> {
    Ok(manifest_cache_root()?.join(format!("{id}.lock.dna")))
}

pub fn load_execution_manifest(id: &str) -> Result<ExecutionManifest, LocusIoError> {
    let packet_path = manifest_packet_path(id)?;
    decode_section_payload(&fs::read(packet_path)?, LocusOpcode::CanonicalPayload)
}

pub fn persist_execution_manifest(manifest: &ExecutionManifest) -> Result<(), LocusIoError> {
    let path = manifest_packet_path(&manifest.id)?;
    write_section_packet(
        &path,
        LocusPacketKind::CanonicalTransfer,
        LocusOpcode::CanonicalPayload,
        &manifest.id,
        "execution_manifest.v1",
        manifest,
        LocusCapabilityMask::default(),
        1,
    )
}

pub fn load_bundle_lock(id: &str) -> Result<BundleLock, LocusIoError> {
    let packet_path = bundle_lock_packet_path(id)?;
    decode_section_payload(&fs::read(packet_path)?, LocusOpcode::CanonicalPayload)
}

pub fn persist_bundle_lock(lock: &BundleLock) -> Result<(), LocusIoError> {
    let path = bundle_lock_packet_path(&lock.id)?;
    write_section_packet(
        &path,
        LocusPacketKind::CanonicalTransfer,
        LocusOpcode::CanonicalPayload,
        &lock.id,
        "bundle_lock.v1",
        lock,
        LocusCapabilityMask::default(),
        1,
    )
}

pub fn encode_canonical_structure_to_dna_packet(
    subject_id: &str,
    artifact_class: GenomeArtifactClass,
    strand_manifest: Vec<String>,
    canonical_structure: &CanonicalStructure,
) -> Result<(Vec<u8>, DnaHeaderReceipt, DnaValidationReport), LocusIoError> {
    let packet = LocusPacket {
        header: LocusPacketHeader {
            artifact_class,
            surface: GenomeSurface::Dna,
            kind: LocusPacketKind::CanonicalTransfer,
            version_major: 1,
            version_minor: 0,
            authority_tier: 1,
            capabilities: LocusCapabilityMask::default(),
            grammar_id: "rna.v1".into(),
            schema_hash: "canonical_structure.v1".into(),
            integrity_hash: canonical_structure.canonical_hash.clone(),
            strand_manifest,
            feature_flags: 0,
            root_subject_id: subject_id.into(),
        },
        sections: vec![LocusSection {
            opcode: LocusOpcode::CanonicalPayload,
            flags: 0,
            subject_id: subject_id.into(),
            payload: bincode::serialize(canonical_structure)
                .map_err(|err| LocusIoError::Codec(err.to_string()))?,
        }],
    };
    let dna_validation = validate_dna_packet(&packet);
    if !dna_validation.failures.is_empty() {
        return Err(LocusIoError::Packet(format!(
            "DNA validation failed: {}",
            dna_validation.failures.join("; ")
        )));
    }
    let dna_header_receipt = dna_header_receipt(&packet);
    let bytes = encode_locus_packet(&packet).map_err(LocusIoError::Packet)?;
    Ok((bytes, dna_header_receipt, dna_validation))
}

pub fn decode_canonical_structure_from_dna_packet(
    bytes: &[u8],
) -> Result<(LocusPacket, CanonicalStructure, DnaValidationReport), LocusIoError> {
    let packet = decode_locus_packet_with_mode(bytes, LocusDecodeMode::CurrentAuthority)
        .map_err(LocusIoError::Packet)?;
    let dna_validation = validate_dna_packet(&packet);
    if !dna_validation.failures.is_empty() {
        return Err(LocusIoError::Packet(format!(
            "DNA validation failed: {}",
            dna_validation.failures.join("; ")
        )));
    }
    let section = packet
        .sections
        .iter()
        .find(|item| item.opcode == LocusOpcode::CanonicalPayload)
        .ok_or_else(|| LocusIoError::Packet("dna packet missing canonical payload".into()))?;
    let canonical_structure: CanonicalStructure = bincode::deserialize(&section.payload)
        .map_err(|err| LocusIoError::Codec(err.to_string()))?;
    Ok((packet, canonical_structure, dna_validation))
}

pub fn compile_rna_to_dna_packet(
    subject_id: &str,
    rna: &str,
    artifact_class: GenomeArtifactClass,
    strand_manifest: Vec<String>,
) -> Result<(Vec<u8>, RnaCompilationArtifact), LocusIoError> {
    validate_rna_source_text(rna)?;
    let execution = execute_lower_chain(rna).map_err(|failure| {
        LocusIoError::Packet(
            serde_json::to_string(&failure).unwrap_or_else(|_| "lower chain failure".into()),
        )
    })?;
    let normalized = execution.normalized;
    let rn_receipt = execution.rn_receipt;
    let ssr_receipt = execution.ssr_receipt;
    let canonical_structure = execution.canonical_structure;
    let cnorm_receipt = execution.cnorm_receipt;
    let phase_ledger = execution.ledger_entries;
    let lowering_receipt = SemanticLoweringReceipt {
        id: format!("SLR_{:x}", l64_core::stable_hash_u64(subject_id)),
        source_surface: GenomeSurface::Rna,
        target: "canonical-structure".into(),
        canonical_hash: cnorm_receipt.canonical_hash.clone(),
        ephemeral_resolver_used: true,
    };
    let (bytes, dna_header_receipt, dna_validation) = encode_canonical_structure_to_dna_packet(
        subject_id,
        artifact_class,
        strand_manifest,
        &canonical_structure,
    )?;
    Ok((
        bytes,
        RnaCompilationArtifact {
            normalized,
            rn_receipt,
            ssr_receipt,
            cnorm_receipt,
            canonical_structure,
            lowering_receipt,
            dna_header_receipt,
            dna_validation,
            phase_ledger,
        },
    ))
}

pub fn sequence_dna_to_canonical_rna(bytes: &[u8]) -> Result<String, LocusIoError> {
    let (_, canonical_structure, _) = decode_canonical_structure_from_dna_packet(bytes)?;
    Ok(canonical_rna_from_structure(&canonical_structure))
}

pub fn sequence_dna_to_rna(bytes: &[u8]) -> Result<RnaCompilationArtifact, LocusIoError> {
    let (packet, canonical_structure, dna_validation) =
        decode_canonical_structure_from_dna_packet(bytes)?;
    let canonical_rna = canonical_rna_from_structure(&canonical_structure);
    let normalized = NormalizedRna {
        raw_text: canonical_rna.clone(),
        normalized_text: canonical_rna.clone(),
        state: l64_core::RnaState::Stabilized,
        splice_regions: Vec::new(),
    };
    let rn_receipt = RnaNormalizationReceipt {
        id: format!(
            "RNR_{:x}",
            l64_core::stable_hash_u64(&canonical_structure.canonical_hash)
        ),
        token_stream_id: String::new(),
        state: l64_core::RnaState::Stabilized,
        normalized_text: canonical_rna,
        shorthand_eliminated: true,
        splice_regions: Vec::new(),
        issues: Vec::new(),
    };
    let ssr_receipt = SsrReceipt {
        id: format!(
            "SSR_RCP_{:x}",
            l64_core::stable_hash_u64(&canonical_structure.root_id)
        ),
        root_id: canonical_structure.root_id.clone(),
        node_count: canonical_structure.items.len(),
        transition_count: canonical_structure.items.len().saturating_sub(1),
        transition_table_hash: l64_core::hash_serialized(&l64_core::ssr_transition_specs()),
        ephemeral: true,
        normalized_rna_id: rn_receipt.id.clone(),
    };
    let cnorm_receipt = CnormReceipt {
        id: format!(
            "CNR_{:x}",
            l64_core::stable_hash_u64(&canonical_structure.root_id)
        ),
        root_id: canonical_structure.root_id.clone(),
        canonical_hash: canonical_structure.canonical_hash.clone(),
        rule_table_hash: l64_core::hash_serialized(&l64_core::structural_equivalence_law_specs()),
        idempotent: true,
        erased_variations: vec!["dna-sequenced".into()],
    };
    let lowering_receipt = SemanticLoweringReceipt {
        id: format!(
            "SLR_{:x}",
            l64_core::stable_hash_u64(&packet.header.root_subject_id)
        ),
        source_surface: GenomeSurface::Dna,
        target: "canonical-structure".into(),
        canonical_hash: cnorm_receipt.canonical_hash.clone(),
        ephemeral_resolver_used: false,
    };
    let dna_header_receipt = dna_header_receipt(&packet);
    Ok(RnaCompilationArtifact {
        normalized,
        rn_receipt,
        ssr_receipt,
        cnorm_receipt,
        canonical_structure,
        lowering_receipt,
        dna_header_receipt,
        dna_validation,
        phase_ledger: Vec::new(),
    })
}

pub fn verify_rna_dna_roundtrip(
    subject_id: &str,
    rna: &str,
    artifact_class: GenomeArtifactClass,
    strand_manifest: Vec<String>,
) -> Result<RnaDnaRoundtripReport, LocusIoError> {
    let (bytes, artifact) =
        compile_rna_to_dna_packet(subject_id, rna, artifact_class, strand_manifest.clone())?;
    let canonical_rna = sequence_dna_to_canonical_rna(&bytes)?;
    let (recompiled, reartifact) =
        compile_rna_to_dna_packet(subject_id, &canonical_rna, artifact_class, strand_manifest)?;
    Ok(RnaDnaRoundtripReport {
        canonical_id: artifact.canonical_structure.canonical_id.0.clone(),
        canonical_hash: artifact.canonical_structure.canonical_hash.clone(),
        dna_bytes_equal: bytes == recompiled,
        canonical_hash_equal: artifact.canonical_structure.canonical_hash
            == reartifact.canonical_structure.canonical_hash,
        canonical_rna,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_and_sequence_rna_roundtrip() {
        let (bytes, artifact) = compile_rna_to_dna_packet(
            "CHAIN_RULE",
            "ι ≔ σ ‖ κ",
            GenomeArtifactClass::Gene,
            vec!["core".into()],
        )
        .expect("compile");
        assert!(!bytes.is_empty());
        assert_eq!(artifact.lowering_receipt.source_surface, GenomeSurface::Rna);
        assert_eq!(artifact.phase_ledger.len(), 4);
        assert_eq!(
            artifact.phase_ledger[0].phase_id,
            l64_core::PhaseId::Tokenization
        );
        assert!(artifact.dna_header_receipt.header_truth_complete);
        assert!(artifact.dna_validation.reversible);
        assert!(artifact.dna_validation.canonical_payload_digest_valid);
        assert!(artifact.dna_validation.failures.is_empty());

        let sequenced = sequence_dna_to_rna(&bytes).expect("sequence");
        assert_eq!(
            sequenced.canonical_structure.canonical_id,
            artifact.canonical_structure.canonical_id
        );
        assert_eq!(
            sequenced.canonical_structure.canonical_hash,
            artifact.canonical_structure.canonical_hash
        );
        assert_eq!(
            sequenced.lowering_receipt.source_surface,
            GenomeSurface::Dna
        );
        assert!(sequenced.phase_ledger.is_empty());

        let (reencoded, _, revalidation) = encode_canonical_structure_to_dna_packet(
            "CHAIN_RULE",
            GenomeArtifactClass::Gene,
            vec!["core".into()],
            &sequenced.canonical_structure,
        )
        .expect("reencode canonical structure");
        assert!(revalidation.failures.is_empty());
        assert!(revalidation.canonical_payload_digest_valid);
        assert_eq!(reencoded, bytes);

        let canonical_rna = sequence_dna_to_canonical_rna(&bytes).expect("canonical rna");
        let (recompiled, recompiled_artifact) = compile_rna_to_dna_packet(
            "CHAIN_RULE",
            &canonical_rna,
            GenomeArtifactClass::Gene,
            vec!["core".into()],
        )
        .expect("recompile canonical rna");
        assert_eq!(recompiled, bytes);
        assert_eq!(
            recompiled_artifact.canonical_structure.canonical_hash,
            artifact.canonical_structure.canonical_hash
        );

        let report = verify_rna_dna_roundtrip(
            "CHAIN_RULE",
            "ι ≔ σ ‖ κ",
            GenomeArtifactClass::Gene,
            vec!["core".into()],
        )
        .expect("verify roundtrip");
        assert!(report.dna_bytes_equal);
        assert!(report.canonical_hash_equal);
    }

    #[test]
    fn sequence_rejects_dna_with_invalid_canonical_digest() {
        let (bytes, _) = compile_rna_to_dna_packet(
            "CHAIN_RULE",
            "ι ≔ σ ‖ κ",
            GenomeArtifactClass::Gene,
            vec!["core".into()],
        )
        .expect("compile");
        let mut packet = l64_core::decode_locus_packet(&bytes).expect("decode packet");
        packet.header.integrity_hash = "stale-digest".into();
        let corrupted = encode_locus_packet(&packet).expect("encode corrupted packet");
        let err = sequence_dna_to_rna(&corrupted).expect_err("stale digest must fail");
        assert!(err.to_string().contains("DNA validation failed"));
    }

    #[test]
    fn generic_section_packet_integrity_is_payload_bound() {
        let left = encode_section_packet(
            LocusPacketKind::CertificationEnvelope,
            LocusOpcode::CanonicalPayload,
            "SUBJ",
            "test-schema.v1",
            &vec!["left"],
            LocusCapabilityMask::default(),
            1,
        )
        .expect("left packet");
        let right = encode_section_packet(
            LocusPacketKind::CertificationEnvelope,
            LocusOpcode::CanonicalPayload,
            "SUBJ",
            "test-schema.v1",
            &vec!["right"],
            LocusCapabilityMask::default(),
            1,
        )
        .expect("right packet");

        let left_packet = l64_core::decode_locus_packet(&left).expect("left decode");
        let right_packet = l64_core::decode_locus_packet(&right).expect("right decode");
        assert_eq!(
            left_packet.header.schema_hash,
            right_packet.header.schema_hash
        );
        assert_ne!(
            left_packet.header.integrity_hash,
            right_packet.header.integrity_hash
        );
        assert_eq!(
            left_packet.header.integrity_hash,
            l64_core::dna_digest_from_bytes(&left_packet.sections[0].payload).0
        );
    }

    #[test]
    fn compile_rejects_projection_report_text() {
        let err = compile_rna_to_dna_packet(
            "REPORT",
            "{\"artifact\":{\"canonical_structure\":{}}}",
            GenomeArtifactClass::Gene,
            vec!["core".into()],
        )
        .expect_err("report text must not compile as RNA");
        assert!(err.to_string().contains("source RNA only"));
    }
}
