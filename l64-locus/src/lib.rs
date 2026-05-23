use l64_core::{
    CanonicalGraph, ChangeLedgerEntry, CnormReceipt, DnaHeaderReceipt, DnaValidationReport,
    GenomeArtifactClass, GenomeSurface, LocusCapabilityMask, LocusOpcode, LocusPacket,
    LocusPacketHeader, LocusPacketKind, LocusSection, NormalizedRna, RnaNormalizationReceipt,
    SemanticLoweringReceipt, SsrReceipt, decode_locus_packet, dna_header_receipt,
    encode_locus_packet, execute_lower_chain, locus_packet_summary, validate_dna_packet,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{fs, path::Path};
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
    pub canonical_graph: CanonicalGraph,
    pub lowering_receipt: SemanticLoweringReceipt,
    pub dna_header_receipt: DnaHeaderReceipt,
    pub dna_validation: DnaValidationReport,
    #[serde(default)]
    pub phase_ledger: Vec<ChangeLedgerEntry>,
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
            integrity_hash: format!("{:x}", l64_core::stable_hash_u64(schema_hash)),
            strand_manifest: vec!["core".into()],
            feature_flags: 0,
            root_subject_id: subject_id.into(),
        },
        sections: vec![LocusSection {
            opcode,
            flags: 0,
            subject_id: subject_id.into(),
            payload: bincode::serialize(payload)
                .map_err(|err| LocusIoError::Codec(err.to_string()))?,
        }],
    };
    encode_locus_packet(&packet).map_err(LocusIoError::Packet)
}

pub fn decode_section_payload<T: DeserializeOwned>(
    bytes: &[u8],
    opcode: LocusOpcode,
) -> Result<T, LocusIoError> {
    let packet = decode_locus_packet(bytes).map_err(LocusIoError::Packet)?;
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
    let packet = decode_locus_packet(bytes).map_err(LocusIoError::Packet)?;
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

pub fn read_section_packet_or_json<T: DeserializeOwned>(
    packet_path: &Path,
    legacy_json_path: &Path,
    opcode: LocusOpcode,
) -> Result<T, LocusIoError> {
    if packet_path.exists() {
        return decode_section_payload(&fs::read(packet_path)?, opcode);
    }
    Ok(serde_json::from_str(&fs::read_to_string(
        legacy_json_path,
    )?)?)
}

pub fn compile_rna_to_dna_packet(
    subject_id: &str,
    rna: &str,
    artifact_class: GenomeArtifactClass,
    strand_manifest: Vec<String>,
) -> Result<(Vec<u8>, RnaCompilationArtifact), LocusIoError> {
    let execution = execute_lower_chain(rna).map_err(|failure| {
        LocusIoError::Packet(
            serde_json::to_string(&failure).unwrap_or_else(|_| "lower chain failure".into()),
        )
    })?;
    let normalized = execution.normalized;
    let rn_receipt = execution.rn_receipt;
    let ssr_receipt = execution.ssr_receipt;
    let canonical_graph = execution.canonical_graph;
    let cnorm_receipt = execution.cnorm_receipt;
    let phase_ledger = execution.ledger_entries;
    let lowering_receipt = SemanticLoweringReceipt {
        id: format!("SLR_{:x}", l64_core::stable_hash_u64(subject_id)),
        source_surface: GenomeSurface::Rna,
        target: "canonical-graph".into(),
        canonical_hash: cnorm_receipt.canonical_hash.clone(),
        ephemeral_resolver_used: true,
    };
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
            schema_hash: "canonical_graph.v1".into(),
            integrity_hash: format!(
                "{:x}",
                l64_core::stable_hash_u64(&canonical_graph.canonical_text)
            ),
            strand_manifest,
            feature_flags: 0,
            root_subject_id: subject_id.into(),
        },
        sections: vec![LocusSection {
            opcode: LocusOpcode::CanonicalPayload,
            flags: 0,
            subject_id: subject_id.into(),
            payload: bincode::serialize(&canonical_graph)
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
    Ok((
        bytes,
        RnaCompilationArtifact {
            normalized,
            rn_receipt,
            ssr_receipt,
            cnorm_receipt,
            canonical_graph,
            lowering_receipt,
            dna_header_receipt,
            dna_validation,
            phase_ledger,
        },
    ))
}

pub fn sequence_dna_to_rna(bytes: &[u8]) -> Result<RnaCompilationArtifact, LocusIoError> {
    let packet = decode_locus_packet(bytes).map_err(LocusIoError::Packet)?;
    let section = packet
        .sections
        .iter()
        .find(|item| item.opcode == LocusOpcode::CanonicalPayload)
        .ok_or_else(|| LocusIoError::Packet("dna packet missing canonical payload".into()))?;
    let canonical_graph: CanonicalGraph = bincode::deserialize(&section.payload)
        .map_err(|err| LocusIoError::Codec(err.to_string()))?;
    let normalized = NormalizedRna {
        raw_text: canonical_graph.canonical_text.clone(),
        normalized_text: canonical_graph.canonical_text.clone(),
        state: l64_core::RnaState::Stabilized,
        splice_regions: Vec::new(),
    };
    let rn_receipt = RnaNormalizationReceipt {
        id: format!(
            "RNR_{:x}",
            l64_core::stable_hash_u64(&canonical_graph.canonical_text)
        ),
        token_stream_id: String::new(),
        state: l64_core::RnaState::Stabilized,
        normalized_text: canonical_graph.canonical_text.clone(),
        shorthand_eliminated: true,
        splice_regions: Vec::new(),
        issues: Vec::new(),
    };
    let ssr_receipt = SsrReceipt {
        id: format!(
            "SSR_RCP_{:x}",
            l64_core::stable_hash_u64(&canonical_graph.root_id)
        ),
        root_id: canonical_graph.root_id.clone(),
        node_count: canonical_graph.canonical_nodes.len(),
        transition_count: canonical_graph.canonical_nodes.len().saturating_sub(1),
        transition_table_hash: l64_core::hash_serialized(&l64_core::ssr_transition_specs()),
        ephemeral: true,
        normalized_rna_id: rn_receipt.id.clone(),
    };
    let cnorm_receipt = CnormReceipt {
        id: format!(
            "CNR_{:x}",
            l64_core::stable_hash_u64(&canonical_graph.root_id)
        ),
        root_id: canonical_graph.root_id.clone(),
        canonical_hash: format!(
            "{:x}",
            l64_core::stable_hash_u64(&canonical_graph.canonical_text)
        ),
        rule_table_hash: l64_core::hash_serialized(&l64_core::canonical_rule_specs()),
        idempotent: true,
        erased_variations: vec!["dna-sequenced".into()],
    };
    let lowering_receipt = SemanticLoweringReceipt {
        id: format!(
            "SLR_{:x}",
            l64_core::stable_hash_u64(&packet.header.root_subject_id)
        ),
        source_surface: GenomeSurface::Dna,
        target: "canonical-graph".into(),
        canonical_hash: cnorm_receipt.canonical_hash.clone(),
        ephemeral_resolver_used: false,
    };
    let dna_header_receipt = dna_header_receipt(&packet);
    let dna_validation = validate_dna_packet(&packet);
    Ok(RnaCompilationArtifact {
        normalized,
        rn_receipt,
        ssr_receipt,
        cnorm_receipt,
        canonical_graph,
        lowering_receipt,
        dna_header_receipt,
        dna_validation,
        phase_ledger: Vec::new(),
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
        assert!(artifact.dna_validation.failures.is_empty());

        let sequenced = sequence_dna_to_rna(&bytes).expect("sequence");
        assert_eq!(
            sequenced.canonical_graph.canonical_text,
            artifact.canonical_graph.canonical_text
        );
        assert_eq!(
            sequenced.lowering_receipt.source_surface,
            GenomeSurface::Dna
        );
        assert!(sequenced.phase_ledger.is_empty());
    }
}
