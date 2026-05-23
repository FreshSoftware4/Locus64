use anyhow::{Result, anyhow};
use l64_cert::{decode_locus_packet_report, encode_locus_packet_for_report};
use l64_core::{
    AtlasDeficiency, Budget, BundleLock, CapabilityMatrix, Certificate, CertificationReport,
    ExecutionManifest, FormatTransformReceipt, HeaderEnvelope, LocusCapabilityMask, LocusOpcode,
    LocusPacketKind, PolicyVerdict, QaDocument, QaEntry, RegistryLookup, ReplayLockManifest,
    ReplayStatus, RoundTripReport, RouteLedger, SurfaceDeficiency, SurfaceKind, SurfacePolicy,
    TransformKind, TransformVerdict, ensure_cache_subdir, resolve_cache_root,
};
use l64_locus::{read_section_packet_or_json, write_section_packet};
use l64_qc0::{parse_qc0, render_qc0};
use l64_qk0::{document_to_qk0, expand_qk0, qk0_to_document};
use l64_qm0::{parse_qm0, render_qm0};
use l64_registry::SeedRegistry;
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceArtifact {
    pub header: HeaderEnvelope,
    pub document: QaDocument,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceReceiptStore {
    pub receipts: Vec<FormatTransformReceipt>,
}

fn cache_root() -> Result<PathBuf> {
    let root = PathBuf::from(
        resolve_cache_root()
            .map_err(anyhow::Error::msg)?
            .absolute_path,
    );
    fs::create_dir_all(root.join("exports"))?;
    Ok(root)
}

fn receipt_store_path() -> Result<PathBuf> {
    Ok(cache_root()?.join("transform_receipts.json"))
}

fn receipt_store_packet_path() -> Result<PathBuf> {
    Ok(cache_root()?.join("transform_receipts.locus"))
}

fn manifest_packet_path(id: &str) -> Result<PathBuf> {
    Ok(manifest_cache_root()?.join(format!("{id}.locus")))
}

fn legacy_manifest_json_path(id: &str) -> Result<PathBuf> {
    Ok(manifest_cache_root()?.join(format!("{id}.json")))
}

fn bundle_lock_packet_path(id: &str) -> Result<PathBuf> {
    Ok(manifest_cache_root()?.join(format!("{id}.lock.locus")))
}

fn legacy_bundle_lock_json_path(id: &str) -> Result<PathBuf> {
    Ok(manifest_cache_root()?.join(format!("{id}.lock.json")))
}

pub fn report_cache_root() -> Result<PathBuf> {
    Ok(ensure_cache_subdir("reports").map_err(anyhow::Error::msg)?)
}

pub fn report_cache_path(id: &str) -> Result<PathBuf> {
    Ok(report_cache_root()?.join(format!("{id}.locus")))
}

fn legacy_report_cache_path(id: &str) -> Result<PathBuf> {
    Ok(report_cache_root()?.join(format!("{id}.json")))
}

pub fn manifest_cache_root() -> Result<PathBuf> {
    Ok(ensure_cache_subdir("manifests").map_err(anyhow::Error::msg)?)
}

pub fn report_id(report: &CertificationReport) -> String {
    format!(
        "REPORT_{}_{}",
        report.theorem_id,
        report
            .campaign_id
            .clone()
            .unwrap_or_else(|| "THEOREM".into())
    )
}

pub fn surface_extension(surface: &SurfaceKind) -> &'static str {
    match surface {
        SurfaceKind::Qc0 => "qc0",
        SurfaceKind::Qm0 => "qm0",
        SurfaceKind::Qk0 => "qk0",
        SurfaceKind::Qa0 => "qa0",
        _ => "txt",
    }
}

pub fn load_execution_manifest(id: &str) -> Result<ExecutionManifest> {
    let packet_path = manifest_packet_path(id)?;
    let legacy_path = legacy_manifest_json_path(id)?;
    read_section_packet_or_json(&packet_path, &legacy_path, LocusOpcode::CanonicalPayload)
        .map_err(anyhow::Error::msg)
}

pub fn persist_execution_manifest(manifest: &ExecutionManifest) -> Result<()> {
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
    .map_err(anyhow::Error::msg)?;
    Ok(())
}

pub fn load_bundle_lock(id: &str) -> Result<BundleLock> {
    let packet_path = bundle_lock_packet_path(id)?;
    let legacy_path = legacy_bundle_lock_json_path(id)?;
    read_section_packet_or_json(&packet_path, &legacy_path, LocusOpcode::CanonicalPayload)
        .map_err(anyhow::Error::msg)
}

pub fn persist_bundle_lock(lock: &BundleLock) -> Result<()> {
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
    .map_err(anyhow::Error::msg)?;
    Ok(())
}

pub fn persist_report_document(report: &CertificationReport) -> Result<()> {
    let path = report_cache_path(&report_id(report))?;
    let bytes = encode_locus_packet_for_report(report).map_err(anyhow::Error::msg)?;
    fs::write(path, bytes)?;
    Ok(())
}

pub fn document_for_registry_id(registry: &dyn RegistryLookup, id: &str) -> Option<QaDocument> {
    let entry = if let Some(item) = registry.get_object(id) {
        QaEntry::Object(item)
    } else if let Some(item) = registry.get_regime(id) {
        QaEntry::Regime(item)
    } else if let Some(item) = registry.get_bridge(id) {
        QaEntry::Bridge(item)
    } else if let Some(item) = registry.get_proof_shape(id) {
        QaEntry::ProofShape(item)
    } else if let Some(item) = registry.get_atlas_cell(id) {
        QaEntry::AtlasCell(item)
    } else if let Some(item) = registry.get_mechanization_package(id) {
        QaEntry::MechanizationPackage(item)
    } else if let Some(item) = registry.get_theorem_spec(id) {
        QaEntry::TheoremSpec(item)
    } else if let Some(item) = registry.get_obligation(id) {
        QaEntry::Obligation(item)
    } else if let Some(item) = registry.get_target_profile(id) {
        QaEntry::TargetProfile(item)
    } else if let Some(item) = registry.get_route_ledger(id) {
        QaEntry::RouteLedger(item)
    } else if let Some(item) = registry.get_certificate(id) {
        QaEntry::Certificate(item)
    } else if let Some(item) = registry.get_campaign(id) {
        QaEntry::Campaign(item)
    } else if let Some(item) = registry.get_campaign_portfolio(id) {
        QaEntry::CampaignPortfolio(item)
    } else if let Some(item) = registry.get_route_class(id) {
        QaEntry::RouteClass(item)
    } else if let Some(item) = registry.get_atlas_deficiency(id) {
        QaEntry::AtlasDeficiency(item)
    } else if let Some(item) = registry.get_adequacy_clause(id) {
        QaEntry::AdequacyClause(item)
    } else if let Some(item) = registry.get_burden_pack(id) {
        QaEntry::BurdenPack(item)
    } else if let Some(item) = registry.get_claim_packet(id) {
        QaEntry::ClaimPacket(item)
    } else if let Some(item) = registry.get_evidence_contract(id) {
        QaEntry::EvidenceContract(item)
    } else if let Some(item) = registry.get_benchmark_receipt(id) {
        QaEntry::BenchmarkReceipt(item)
    } else if let Some(item) = registry.get_challenge_receipt(id) {
        QaEntry::ChallengeReceipt(item)
    } else if let Some(item) = registry.get_reproducibility_packet(id) {
        QaEntry::ReproducibilityPacket(item)
    } else if let Some(item) = registry.get_surface_policy(id) {
        QaEntry::SurfacePolicy(item)
    } else if let Some(item) = registry.get_transform_receipt(id) {
        QaEntry::TransformReceipt(item)
    } else if let Some(item) = registry.get_roundtrip_report(id) {
        QaEntry::RoundTripReport(item)
    } else if let Some(item) = registry.get_capability_matrix(id) {
        QaEntry::CapabilityMatrix(item)
    } else if let Some(item) = registry.get_policy_object(id) {
        QaEntry::PolicyObject(item)
    } else {
        return None;
    };
    Some(QaDocument {
        entries: vec![entry],
    })
}

pub fn report_related_entries(
    registry: &dyn RegistryLookup,
    report: &CertificationReport,
) -> Vec<QaEntry> {
    let mut entries = Vec::new();
    for id in &report.burden_pack_ids {
        if let Some(item) = registry.get_burden_pack(id) {
            entries.push(QaEntry::BurdenPack(item));
        }
    }
    for id in &report.claim_packet_ids {
        if let Some(item) = registry.get_claim_packet(id) {
            entries.push(QaEntry::ClaimPacket(item));
        }
    }
    for id in &report.evidence_contract_ids {
        if let Some(item) = registry.get_evidence_contract(id) {
            entries.push(QaEntry::EvidenceContract(item));
        }
    }
    for id in &report.benchmark_receipt_ids {
        if let Some(item) = registry.get_benchmark_receipt(id) {
            entries.push(QaEntry::BenchmarkReceipt(item));
        }
    }
    for id in &report.challenge_receipt_ids {
        if let Some(item) = registry.get_challenge_receipt(id) {
            entries.push(QaEntry::ChallengeReceipt(item));
        }
    }
    for id in &report.reproducibility_packet_ids {
        if let Some(item) = registry.get_reproducibility_packet(id) {
            entries.push(QaEntry::ReproducibilityPacket(item));
        }
    }
    for record in &report.adequacy_records {
        if let Some(item) = registry.get_adequacy_clause(&record.clause_id) {
            entries.push(QaEntry::AdequacyClause(item));
        }
    }
    entries
}

pub fn report_to_validation_bundle_with_registry(
    report: &CertificationReport,
    registry: &dyn RegistryLookup,
) -> Result<QaDocument> {
    let mut document = report_to_document_with_registry(report, registry)?;
    let mut extra = Vec::new();
    if let Some(theorem) = registry.get_theorem_spec(&report.theorem_id) {
        for host in &theorem.hosts {
            if let Some(regime) = registry.get_regime(host) {
                extra.push(QaEntry::Regime(regime));
            }
        }
        for shape_id in &theorem.proof_shapes {
            if let Some(shape) = registry.get_proof_shape(shape_id) {
                extra.push(QaEntry::ProofShape(shape));
            }
        }
        extra.push(QaEntry::TheoremSpec(theorem));
    }
    if let Some(target) = registry.get_target_profile(&report.target_profile_id) {
        extra.push(QaEntry::TargetProfile(target));
    }
    if let Some(campaign_id) = &report.campaign_id {
        if let Some(campaign) = registry.get_campaign(campaign_id) {
            for obligation_id in &campaign.obligations {
                if let Some(obligation) = registry.get_obligation(obligation_id) {
                    extra.push(QaEntry::Obligation(obligation));
                }
            }
            extra.push(QaEntry::Campaign(campaign));
        }
    }
    if let Some(route_class_id) = &report.route_class_id {
        if let Some(route_class) = registry.get_route_class(route_class_id) {
            extra.push(QaEntry::RouteClass(route_class));
        }
    }
    for bridge_id in &report.selected_path {
        if let Some(bridge) = registry.get_bridge(bridge_id) {
            if let Some(source) = registry.get_regime(&bridge.src) {
                extra.push(QaEntry::Regime(source));
            }
            if let Some(target) = registry.get_regime(&bridge.tgt) {
                extra.push(QaEntry::Regime(target));
            }
            extra.push(QaEntry::Bridge(bridge));
        }
    }
    if let Some(cell_id) = &report.selected_atlas_cell {
        if let Some(cell) = registry.get_atlas_cell(cell_id) {
            extra.push(QaEntry::AtlasCell(cell));
        }
    }
    document.entries.extend(extra);
    Ok(QaDocument {
        entries: document
            .entries
            .into_iter()
            .fold(Vec::new(), |mut acc, entry| {
                let id = entry.id();
                if !acc.iter().any(|existing: &QaEntry| existing.id() == id) {
                    acc.push(entry);
                }
                acc
            }),
    })
}

pub fn report_to_document_with_registry(
    report: &CertificationReport,
    registry: &dyn RegistryLookup,
) -> Result<QaDocument> {
    let report_id = report_id(report);
    let ledger = RouteLedger {
        id: format!("TRL_{report_id}"),
        theorem: report.theorem_id.clone(),
        paths: vec![report.selected_path.clone()],
        budget: Budget {
            max_loss: report
                .candidates
                .first()
                .map(|item| item.loss_count)
                .unwrap_or_default(),
            allow_lossy_supported: true,
            require_proof: true,
        },
        losses: report
            .candidates
            .first()
            .map(|item| vec![format!("loss-count={}", item.loss_count)])
            .unwrap_or_default(),
        receipts: Vec::new(),
        normalized_path: report.selected_path.clone(),
    };
    let certificate = Certificate {
        id: format!("CRT_{report_id}"),
        theorem: report.theorem_id.clone(),
        route_ledger: ledger.id.clone(),
        proof_shapes: report
            .candidates
            .first()
            .map(|item| item.proof_shapes.clone())
            .unwrap_or_default(),
        receipts: report
            .reused_artifact_ids
            .iter()
            .cloned()
            .chain(
                report
                    .adequacy_records
                    .iter()
                    .filter(|item| item.verdict == l64_core::CertificationVerdict::Certified)
                    .map(|item| item.id.clone()),
            )
            .chain(report.payoff_receipt_ids.iter().cloned())
            .collect(),
        verdict: report.verdict.clone(),
    };
    let mut entries = vec![
        QaEntry::RouteLedger(ledger.clone()),
        QaEntry::Certificate(certificate.clone()),
    ];
    entries.extend(report_related_entries(registry, report));
    entries.extend(
        report
            .promotion_artifact_ids
            .iter()
            .map(|id| QaEntry::Object(promoted_operator_object(report, id))),
    );
    if let Some(policy_resolution) = &report.policy_resolution {
        entries.push(QaEntry::PolicyResolution(policy_resolution.clone()));
    }
    if let Some(envelope) = &report.execution_envelope {
        if let Some(manifest_id) = &envelope.manifest_id {
            if let Ok(manifest) = load_execution_manifest(manifest_id) {
                entries.push(QaEntry::ExecutionManifest(manifest));
            }
        }
        if let Some(lock_id) = &envelope.lock_id {
            if let Ok(lock) = load_bundle_lock(lock_id) {
                entries.push(QaEntry::BundleLock(lock));
            }
        }
        if let (Some(lock_id), Some(manifest_id)) = (&envelope.lock_id, &envelope.manifest_id) {
            entries.push(QaEntry::ReplayLockManifest(ReplayLockManifest {
                id: format!("RLM_{report_id}"),
                report_id: report_id.clone(),
                report_hash: envelope.report_hash.clone(),
                route_winner_hash: envelope.route_winner_hash.clone(),
                policy_hash: envelope.policy_hash.clone(),
                bundle_hash: envelope.bundle_hash.clone(),
            }));
            entries.push(QaEntry::LockReceipt(l64_core::LockReceipt {
                id: format!("LRC_{report_id}"),
                lock_id: lock_id.clone(),
                manifest_id: manifest_id.clone(),
                bundle_id: envelope.bundle_hash.clone(),
                receipt_ids: vec![format!("XFR_{report_id}")],
                verdict: report
                    .policy_resolution
                    .as_ref()
                    .map(|item| item.verdict.clone())
                    .unwrap_or(PolicyVerdict::Applied),
            }));
        }
        entries.push(QaEntry::TransformReceipt(FormatTransformReceipt {
            id: format!("XFR_{report_id}"),
            src_surface: SurfaceKind::Qc0,
            dst_surface: SurfaceKind::Qc0,
            object_ids: vec![ledger.id.clone(), certificate.id.clone()],
            transform_kind: TransformKind::Export,
            policy_id: envelope.policy_hash.clone(),
            defaults_used: vec![format!("bundle_hash={}", envelope.bundle_hash)],
            alias_expansions: Vec::new(),
            loss_classes: Vec::new(),
            hash_before: envelope.route_winner_hash.clone(),
            hash_after: envelope.report_hash.clone(),
            verdict: match envelope.replay_status {
                ReplayStatus::Fresh | ReplayStatus::CacheHit | ReplayStatus::ReplayOnly => {
                    TransformVerdict::Lossless
                }
                ReplayStatus::Invalidated => TransformVerdict::Invalid,
            },
            rollback_ref: None,
            replay_ref: Some(format!("{:?}", envelope.replay_status)),
        }));
    }
    entries.extend(
        report
            .deficiencies
            .iter()
            .cloned()
            .map(QaEntry::AtlasDeficiency),
    );
    entries.extend(
        report
            .adequacy_records
            .iter()
            .enumerate()
            .map(|(index, item)| {
                QaEntry::AtlasDeficiency(l64_core::adequacy_record_deficiency(
                    &report.theorem_id,
                    report.selected_atlas_cell.as_deref(),
                    format!("DGN_ADQ_{report_id}_{index}"),
                    item,
                ))
            }),
    );
    entries.extend(
        report
            .diagnostics
            .iter()
            .enumerate()
            .map(|(index, message)| {
                QaEntry::AtlasDeficiency(AtlasDeficiency {
                    id: format!("DGN_{report_id}_{index}"),
                    class: l64_core::AtlasDeficiencyClass::DNoCommutingProof,
                    atlas_cell: report.selected_atlas_cell.clone(),
                    theorem: Some(report.theorem_id.clone()),
                    message: message.clone(),
                    blocking_scope: None,
                    control_effects: Vec::new(),
                    suggested_seam: None,
                })
            }),
    );
    entries.extend(report.obligations.iter().enumerate().map(|(index, item)| {
        QaEntry::AtlasDeficiency(l64_core::obligation_status_deficiency(
            &report.theorem_id,
            report.selected_atlas_cell.as_deref(),
            format!("DGN_OBL_{report_id}_{index}"),
            item,
            false,
            None,
        ))
    }));
    if let Some(explanation) = &report.route_explanation {
        entries.push(QaEntry::AtlasDeficiency(
            l64_core::route_explanation_deficiency(
                &report.theorem_id,
                report.selected_atlas_cell.as_deref(),
                format!("DGN_ROUTE_{report_id}"),
                explanation,
                false,
            ),
        ));
    }
    Ok(QaDocument { entries })
}

pub fn load_report_document_with_registry(
    id: &str,
    registry: &dyn RegistryLookup,
) -> Result<QaDocument> {
    let path = report_cache_path(id)?;
    if path.exists() {
        let bytes = fs::read(path)?;
        let report = decode_locus_packet_report(&bytes).map_err(anyhow::Error::msg)?;
        return report_to_document_with_registry(&report, registry);
    }
    let legacy = legacy_report_cache_path(id)?;
    let text = fs::read_to_string(legacy)?;
    if let Ok(report) = serde_json::from_str::<CertificationReport>(&text) {
        return report_to_document_with_registry(&report, registry);
    }
    Ok(serde_json::from_str(&text)?)
}

fn promoted_operator_object(report: &CertificationReport, id: &str) -> l64_core::QcObject {
    let alias = if let Some(suffix) = id.strip_prefix("OPR_PROMOTED_") {
        suffix.replace('_', ".")
    } else {
        id.replace('_', ".")
    };
    l64_core::QcObject {
        id: id.to_string(),
        identity: l64_core::IdentityFace {
            tag: l64_core::ObjectTag::Opr,
            cid: format!("cid:{id}"),
            codebook: "QC0_CORE".into(),
            remap: "none".into(),
            lineage: format!("derived-from:{}", report.theorem_id),
        },
        structural: l64_core::StructuralFace {
            head: "operator".into(),
            args: vec![
                report.theorem_id.clone(),
                report
                    .campaign_id
                    .clone()
                    .unwrap_or_else(|| "THEOREM".into()),
            ],
            local_sections: vec!["first-order derivative composition".into()],
            morphism_hooks: report.selected_path.clone(),
        },
        constraint: l64_core::ConstraintFace {
            regime: "R_CALC".into(),
            contracts: vec!["chain-rule".into(), "first-order".into()],
            invariants: vec!["jet-compose".into(), "reduction-exact".into()],
            equivalence: "first-order jet equivalence".into(),
            admissibility: "promoted after exact certified discharge".into(),
        },
        evidence: l64_core::EvidenceFace {
            evidence_class: "DerivedPromotion".into(),
            traces: vec![report.theorem_id.clone()],
            receipts: vec![
                report
                    .certificate_id
                    .clone()
                    .unwrap_or_else(|| format!("CRT_{}", report_id(report))),
                format!(
                    "REPORT_{}_{}",
                    report.theorem_id,
                    report
                        .campaign_id
                        .clone()
                        .unwrap_or_else(|| "THEOREM".into())
                ),
            ],
            maturity: l64_core::EvidenceMaturity::Certified,
            gate_verdict: l64_core::GateVerdict::Pass,
        },
        alias: l64_core::AliasFace {
            aliases: vec![alias],
            profile_pack: vec!["STD".into(), "chain-rule".into()],
            qm_binding: "THS·ChainRule".into(),
            qa_binding: "OPR.Chain1".into(),
            projection_policy: "canonical-authored".into(),
        },
    }
}

pub fn load_receipt_store() -> Result<SurfaceReceiptStore> {
    let packet_path = receipt_store_packet_path()?;
    let path = receipt_store_path()?;
    if !packet_path.exists() && !path.exists() {
        return Ok(SurfaceReceiptStore {
            receipts: Vec::new(),
        });
    }
    read_section_packet_or_json(&packet_path, &path, LocusOpcode::CanonicalPayload)
        .map_err(anyhow::Error::msg)
}

pub fn persist_receipt(receipt: FormatTransformReceipt) -> Result<()> {
    let path = receipt_store_packet_path()?;
    let mut store = load_receipt_store()?;
    store.receipts.push(receipt);
    write_section_packet(
        &path,
        LocusPacketKind::CanonicalTransfer,
        LocusOpcode::CanonicalPayload,
        "transform_receipts",
        "surface_receipts.v1",
        &store,
        LocusCapabilityMask::default(),
        1,
    )
    .map_err(anyhow::Error::msg)?;
    Ok(())
}

pub fn dump_transform_receipt(id: &str) -> Result<FormatTransformReceipt> {
    load_receipt_store()?
        .receipts
        .into_iter()
        .find(|item| item.id == id)
        .ok_or_else(|| anyhow!("unknown transform receipt `{id}`"))
}

pub fn detect_surface_kind(path: &Path) -> Result<SurfaceKind> {
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
    {
        "qc0" => Ok(SurfaceKind::Qc0),
        "qm0" => Ok(SurfaceKind::Qm0),
        "qk0" => Ok(SurfaceKind::Qk0),
        "qa0" => Ok(SurfaceKind::Qa0),
        other => Err(anyhow!("unsupported surface extension `{other}`")),
    }
}

pub fn import_file(
    path: &Path,
    forced_kind: Option<SurfaceKind>,
    registry: &SeedRegistry,
) -> Result<(SurfaceArtifact, FormatTransformReceipt)> {
    let surface_kind = forced_kind.unwrap_or(detect_surface_kind(path)?);
    let text = fs::read_to_string(path)?;
    import_text(&text, surface_kind, registry)
}

pub fn import_text(
    text: &str,
    surface_kind: SurfaceKind,
    registry: &SeedRegistry,
) -> Result<(SurfaceArtifact, FormatTransformReceipt)> {
    let policy = default_policy_for(surface_kind.clone(), registry)?;
    let (header, document, defaults_used, alias_expansions, loss_classes) = match surface_kind {
        SurfaceKind::Qc0 => {
            let (header, document) = parse_qc0(text)?;
            (header, document, Vec::new(), Vec::new(), Vec::new())
        }
        SurfaceKind::Qm0 => {
            let (header, document) = parse_qm0(text)?;
            (header, document, Vec::new(), Vec::new(), Vec::new())
        }
        SurfaceKind::Qk0 => {
            let combo_pack = registry
                .get_combo_pack(policy.combo_pack.as_deref().unwrap_or_default())
                .ok_or_else(|| anyhow!("missing combo pack for qk0 import"))?;
            let (header, document) = qk0_to_document(text, &combo_pack)?;
            (
                header,
                document,
                vec!["combo-pack".into()],
                Vec::new(),
                Vec::new(),
            )
        }
        SurfaceKind::Qa0 => {
            let mut lines = text.lines();
            let header_line = lines.next().ok_or_else(|| anyhow!("missing qa0 header"))?;
            let header_payload = header_line
                .strip_prefix("!qa0 ")
                .ok_or_else(|| anyhow!("missing qa0 header"))?;
            let header: HeaderEnvelope = serde_json::from_str(header_payload)?;
            let document = l64_qa0::parse_document(&lines.collect::<Vec<_>>().join("\n"))?;
            (
                header,
                document,
                Vec::new(),
                Vec::new(),
                vec!["ascii-mirror".into()],
            )
        }
        _ => return Err(anyhow!("unsupported import surface")),
    };
    let receipt = build_receipt(
        format!("XFR-IMPORT-{:x}", simple_hash(text)),
        surface_kind.clone(),
        SurfaceKind::Qc0,
        &document,
        TransformKind::Import,
        &policy,
        defaults_used,
        alias_expansions,
        loss_classes,
        text,
        &render_semantic_json(&document)?,
    );
    persist_receipt(receipt.clone())?;
    Ok((SurfaceArtifact { header, document }, receipt))
}

pub fn export_document(
    document: &QaDocument,
    target: SurfaceKind,
    policy: &SurfacePolicy,
    registry: &SeedRegistry,
) -> Result<(String, FormatTransformReceipt)> {
    let header = HeaderEnvelope {
        surface_kind: target.clone(),
        version: "1".into(),
        policy_id: policy.id.clone(),
        capability_id: capability_for(target.clone(), registry)
            .ok()
            .map(|item| item.id),
    };
    let rendered = match target {
        SurfaceKind::Qc0 => render_qc0(&header, document),
        SurfaceKind::Qm0 => render_qm0(&header, document),
        SurfaceKind::Qk0 => {
            let combo_pack = registry
                .get_combo_pack(policy.combo_pack.as_deref().unwrap_or_default())
                .ok_or_else(|| anyhow!("missing combo pack for qk0 export"))?;
            document_to_qk0(&header, document, &combo_pack)
        }
        SurfaceKind::Qa0 => format!(
            "!qa0 {}\n{}",
            serde_json::to_string(&header)?,
            l64_qa0::normalize_document(document)?
        ),
        _ => return Err(anyhow!("unsupported export surface")),
    };

    let semantic = render_semantic_json(document)?;
    let receipt = build_receipt(
        format!(
            "XFR-EXPORT-{:x}",
            simple_hash(&(semantic.clone() + &rendered))
        ),
        SurfaceKind::Qc0,
        target.clone(),
        document,
        TransformKind::Export,
        policy,
        Vec::new(),
        Vec::new(),
        if target == SurfaceKind::Qa0 {
            vec!["ascii-mirror".into()]
        } else {
            Vec::new()
        },
        &semantic,
        &rendered,
    );
    cache_export(document, &target, policy, &rendered)?;
    persist_receipt(receipt.clone())?;
    Ok((rendered, receipt))
}

pub fn transcode_text(
    text: &str,
    src: SurfaceKind,
    dst: SurfaceKind,
    registry: &SeedRegistry,
) -> Result<(String, FormatTransformReceipt)> {
    let (artifact, import_receipt) = import_text(text, src.clone(), registry)?;
    let policy = default_policy_for(dst.clone(), registry)?;
    let (rendered, mut export_receipt) =
        export_document(&artifact.document, dst.clone(), &policy, registry)?;
    export_receipt.transform_kind = TransformKind::Transcode;
    export_receipt.src_surface = src;
    export_receipt.hash_before = import_receipt.hash_before;
    persist_receipt(export_receipt.clone())?;
    Ok((rendered, export_receipt))
}

pub fn normalize_surface(
    text: &str,
    kind: SurfaceKind,
    registry: &SeedRegistry,
) -> Result<(String, FormatTransformReceipt)> {
    transcode_text(text, kind.clone(), kind, registry)
}

pub fn roundtrip_check(
    text: &str,
    kind: SurfaceKind,
    registry: &SeedRegistry,
) -> Result<(RoundTripReport, Vec<FormatTransformReceipt>)> {
    let (artifact, import_receipt) = import_text(text, kind.clone(), registry)?;
    let policy = default_policy_for(kind.clone(), registry)?;
    let (normalized, export_receipt) =
        export_document(&artifact.document, kind.clone(), &policy, registry)?;
    let verdict = if normalize_newlines(text) == normalize_newlines(&normalized) {
        TransformVerdict::Lossless
    } else {
        TransformVerdict::ReceiptedLoss
    };
    let report = RoundTripReport {
        id: format!("RTP-{:x}", simple_hash(&(text.to_string() + &normalized))),
        surface_kind: kind,
        policy_id: policy.id,
        object_ids: object_ids(&artifact.document),
        receipt_ids: vec![import_receipt.id.clone(), export_receipt.id.clone()],
        verdict,
        fragility_vector: Vec::new(),
    };
    Ok((report, vec![import_receipt, export_receipt]))
}

pub fn capability_for(kind: SurfaceKind, registry: &SeedRegistry) -> Result<CapabilityMatrix> {
    registry
        .bundle()
        .capability_matrices
        .iter()
        .find(|item| item.surface_kind == kind)
        .cloned()
        .ok_or_else(|| anyhow!("missing capability matrix"))
}

pub fn surface_capabilities(registry: &SeedRegistry) -> Vec<CapabilityMatrix> {
    registry.bundle().capability_matrices.clone()
}

pub fn default_policy_for(kind: SurfaceKind, registry: &SeedRegistry) -> Result<SurfacePolicy> {
    let id = match kind {
        SurfaceKind::Qc0 => "POL_QC0_CORE",
        SurfaceKind::Qm0 => "POL_QM0_PACK_A",
        SurfaceKind::Qk0 => "POL_QK0_PACK_A",
        SurfaceKind::Qa0 => "POL_QA0_PACK_A",
        _ => return Err(anyhow!("unsupported surface policy")),
    };
    registry
        .get_surface_policy(id)
        .ok_or_else(|| anyhow!("missing policy `{id}`"))
}

pub fn surface_deficiencies(registry: &SeedRegistry) -> Vec<SurfaceDeficiency> {
    registry.bundle().surface_deficiencies.clone()
}

pub fn expand_qk0_file(path: &Path, registry: &SeedRegistry) -> Result<String> {
    let text = fs::read_to_string(path)?;
    let policy = default_policy_for(SurfaceKind::Qk0, registry)?;
    let combo_pack = registry
        .get_combo_pack(policy.combo_pack.as_deref().unwrap_or_default())
        .ok_or_else(|| anyhow!("missing combo pack"))?;
    Ok(expand_qk0(&text, &combo_pack)?)
}

fn cache_export(
    document: &QaDocument,
    kind: &SurfaceKind,
    policy: &SurfacePolicy,
    rendered: &str,
) -> Result<()> {
    let key = format!(
        "{:x}",
        simple_hash(&(render_semantic_json(document)? + &format!("{kind:?}{:?}", policy.id)))
    );
    let path = cache_root()?.join("exports").join(format!("{key}.cache"));
    fs::write(path, rendered)?;
    Ok(())
}

fn render_semantic_json(document: &QaDocument) -> Result<String> {
    Ok(serde_json::to_string(document)?)
}

fn simple_hash(input: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

fn build_receipt(
    id: String,
    src_surface: SurfaceKind,
    dst_surface: SurfaceKind,
    document: &QaDocument,
    transform_kind: TransformKind,
    policy: &SurfacePolicy,
    defaults_used: Vec<String>,
    alias_expansions: Vec<String>,
    loss_classes: Vec<String>,
    before: &str,
    after: &str,
) -> FormatTransformReceipt {
    FormatTransformReceipt {
        id,
        src_surface,
        dst_surface,
        object_ids: object_ids(document),
        transform_kind,
        policy_id: policy.id.clone(),
        defaults_used,
        alias_expansions,
        loss_classes: loss_classes.clone(),
        hash_before: format!("{:x}", simple_hash(before)),
        hash_after: format!("{:x}", simple_hash(after)),
        verdict: if loss_classes.is_empty() {
            TransformVerdict::Lossless
        } else {
            TransformVerdict::ReceiptedLoss
        },
        rollback_ref: None,
        replay_ref: Some(format!("policy={}", policy.id)),
    }
}

fn object_ids(document: &QaDocument) -> Vec<String> {
    document.entries.iter().map(QaEntry::id).collect()
}

fn normalize_newlines(input: &str) -> String {
    input.replace("\r\n", "\n").trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn qa0_to_qc0_transcodes_with_receipt() {
        let registry = SeedRegistry::load().unwrap();
        let qa0 = format!(
            "!qa0 {}\nobject [tag=CTX;cid=OBJ;codebook=CB;remap=none;lineage=seed]<head=carrier;args=x;locals=;hooks=>[regime=R_SET;contracts=;invariants=;equivalence=eq;admissibility=adm]{{evidence_class=Seed;traces=;receipts=;maturity=Validated;gate_verdict=Pass}}<<aliases=;profiles=;qm_binding=qm;qa_binding=qa;projection_policy=policy>>",
            serde_json::to_string(&HeaderEnvelope {
                surface_kind: SurfaceKind::Qa0,
                version: "1".into(),
                policy_id: "POL_QA0_PACK_A".into(),
                capability_id: Some("CAP_QA0_PACK_A".into())
            })
            .unwrap()
        );
        let (rendered, receipt) =
            transcode_text(&qa0, SurfaceKind::Qa0, SurfaceKind::Qc0, &registry).unwrap();
        assert!(rendered.starts_with("!qc0 "));
        assert_eq!(receipt.dst_surface, SurfaceKind::Qc0);
    }
}
