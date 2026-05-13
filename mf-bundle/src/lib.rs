use anyhow::{Result, anyhow};
use mf_core::{
    AdequacyClause, AliasExpansionPolicy, ArtifactOrigin, AtlasCell, AtlasDeficiency,
    BenchmarkReceipt, BridgeContract, BundleConflict, BundleConflictPolicy, BundleDependency,
    BundleEntry, BundleExecutionReceipt, BundleManifest, BundleMergeReport, BurdenPack, Campaign,
    CampaignPortfolio, CapabilityMatrix, Certificate, ChallengeReceipt, ClaimPacket, CodebookPack,
    ComboPack, EquivalenceClass, EvidenceContract, ExecutionManifest, FormatTransformReceipt,
    GlyphPack, MechanizationPackage, MechanizationPolicyObject, Obligation,
    OverlayRegistryDescriptor, PolicyBinding, PolicyResolution, ProjectionPolicy, ProofShape,
    QaDocument, QaEntry, QcObject, RegimePack, RegistryBundle, RegistryLookup, ReplayLockManifest,
    ReproducibilityPacket, RoundTripReport, RouteClass, RouteLedger, SurfaceDeficiency,
    SurfaceKind, SurfacePolicy, TargetProfile, TheoremSpec, ensure_cache_subdir,
};
use mf_registry::SeedRegistry;
use mf_surfaces::import_file;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct OverlayRegistry {
    pub parent: SeedRegistry,
    pub bundle_id: String,
    pub local: RegistryBundle,
    pub merge_report: BundleMergeReport,
    pub import_receipts: Vec<FormatTransformReceipt>,
}

#[derive(Debug, Clone)]
pub struct BundleWorld {
    pub manifest: BundleManifest,
    pub overlay: OverlayRegistry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedBundleWorld {
    manifest: BundleManifest,
    bundle_id: String,
    local: RegistryBundle,
    merge_report: BundleMergeReport,
    import_receipts: Vec<FormatTransformReceipt>,
}

fn bundle_cache_root() -> Result<PathBuf> {
    Ok(ensure_cache_subdir("bundles").map_err(anyhow::Error::msg)?)
}

pub fn load_bundle_world(bundle_id: &str) -> Result<BundleWorld> {
    let path = bundle_cache_root()?.join(format!("{bundle_id}.json"));
    let cached: CachedBundleWorld = serde_json::from_str(&fs::read_to_string(path)?)?;
    Ok(BundleWorld {
        manifest: cached.manifest,
        overlay: OverlayRegistry {
            parent: SeedRegistry::load()?,
            bundle_id: cached.bundle_id,
            local: cached.local,
            merge_report: cached.merge_report,
            import_receipts: cached.import_receipts,
        },
    })
}

pub fn persist_bundle_world(world: &BundleWorld) -> Result<()> {
    let path = bundle_cache_root()?.join(format!("{}.json", world.manifest.id));
    let cached = CachedBundleWorld {
        manifest: world.manifest.clone(),
        bundle_id: world.overlay.bundle_id.clone(),
        local: world.overlay.local.clone(),
        merge_report: world.overlay.merge_report.clone(),
        import_receipts: world.overlay.import_receipts.clone(),
    };
    fs::write(path, serde_json::to_string_pretty(&cached)?)?;
    Ok(())
}

pub fn local_registry_from_document(document: &QaDocument) -> LocalRegistry {
    LocalRegistry {
        bundle_id: "DOC_LOCAL".into(),
        local: bundle_from_document(document),
    }
}

pub fn overlay_registry_from_document(
    parent: SeedRegistry,
    document: &QaDocument,
) -> OverlayRegistry {
    OverlayRegistry {
        parent,
        bundle_id: "DOC_OVERLAY".into(),
        local: bundle_from_document(document),
        merge_report: BundleMergeReport {
            id: "BMERGE_DOC_OVERLAY".into(),
            policy: BundleConflictPolicy::Reject,
            imported_entries: document.entries.len(),
            conflicts: Vec::new(),
            namespaced_entries: 0,
        },
        import_receipts: Vec::new(),
    }
}

pub fn import_bundle_file(
    path: &Path,
    forced_kind: Option<SurfaceKind>,
    policy: BundleConflictPolicy,
    namespace: Option<&str>,
) -> Result<BundleWorld> {
    let parent = SeedRegistry::load()?;
    let (artifact, import_receipt) = import_file(path, forced_kind, &parent)?;
    let bundle_id = path
        .file_stem()
        .and_then(|value| value.to_str())
        .map(|value| format!("BND_{}", value.to_ascii_uppercase().replace('-', "_")))
        .unwrap_or_else(|| "BND_IMPORTED".into());
    let document = if policy == BundleConflictPolicy::NamespacedImport {
        namespace_document(
            artifact.document,
            namespace.unwrap_or(&bundle_id.to_ascii_lowercase().replace("bnd_", "bnd.")),
        )
    } else {
        artifact.document
    };
    let local = bundle_from_document(&document);
    let merge_report = detect_conflicts(&parent, &local, &policy)?;
    if policy == BundleConflictPolicy::Reject && !merge_report.conflicts.is_empty() {
        return Err(anyhow!("bundle import rejected due to conflicts"));
    }
    let manifest = BundleManifest {
        id: bundle_id.clone(),
        entries: bundle_entries(&local),
        dependencies: bundle_dependencies(&local),
        merge_report: merge_report.clone(),
        overlay: OverlayRegistryDescriptor {
            id: format!("BOVR_{bundle_id}"),
            parent: "seed".into(),
            bundle_id: bundle_id.clone(),
            local_entries: bundle_entries(&local).len(),
            shadowed_entries: merge_report
                .conflicts
                .iter()
                .map(|item| item.id.clone())
                .collect(),
        },
        execution_receipt: BundleExecutionReceipt {
            id: format!("BREC_{bundle_id}"),
            bundle_id: bundle_id.clone(),
            theorem_ids: local
                .theorem_specs
                .iter()
                .map(|item| item.id.clone())
                .collect(),
            campaign_ids: local.campaigns.iter().map(|item| item.id.clone()).collect(),
            overlay_id: format!("BOVR_{bundle_id}"),
            import_receipt_ids: vec![import_receipt.id.clone()],
            merge_report_id: format!("BMER_{bundle_id}"),
        },
    };
    let world = BundleWorld {
        manifest,
        overlay: OverlayRegistry {
            parent,
            bundle_id,
            local,
            merge_report,
            import_receipts: vec![import_receipt],
        },
    };
    persist_bundle_world(&world)?;
    Ok(world)
}

fn detect_conflicts(
    parent: &SeedRegistry,
    local: &RegistryBundle,
    policy: &BundleConflictPolicy,
) -> Result<BundleMergeReport> {
    let mut conflicts = Vec::new();
    collect_conflicts(&mut conflicts, policy, &local.objects, |id| {
        parent
            .get_object(id)
            .map(|item| serde_json::to_value(item).unwrap())
    });
    collect_conflicts(&mut conflicts, policy, &local.regimes, |id| {
        parent
            .get_regime(id)
            .map(|item| serde_json::to_value(item).unwrap())
    });
    collect_conflicts(&mut conflicts, policy, &local.bridges, |id| {
        parent
            .get_bridge(id)
            .map(|item| serde_json::to_value(item).unwrap())
    });
    collect_conflicts(&mut conflicts, policy, &local.proof_shapes, |id| {
        parent
            .get_proof_shape(id)
            .map(|item| serde_json::to_value(item).unwrap())
    });
    collect_conflicts(&mut conflicts, policy, &local.atlas_cells, |id| {
        parent
            .get_atlas_cell(id)
            .map(|item| serde_json::to_value(item).unwrap())
    });
    collect_conflicts(&mut conflicts, policy, &local.theorem_specs, |id| {
        parent
            .get_theorem_spec(id)
            .map(|item| serde_json::to_value(item).unwrap())
    });
    collect_conflicts(&mut conflicts, policy, &local.target_profiles, |id| {
        parent
            .get_target_profile(id)
            .map(|item| serde_json::to_value(item).unwrap())
    });
    collect_conflicts(&mut conflicts, policy, &local.route_ledgers, |id| {
        parent
            .get_route_ledger(id)
            .map(|item| serde_json::to_value(item).unwrap())
    });
    collect_conflicts(&mut conflicts, policy, &local.certificates, |id| {
        parent
            .get_certificate(id)
            .map(|item| serde_json::to_value(item).unwrap())
    });
    collect_conflicts(&mut conflicts, policy, &local.campaigns, |id| {
        parent
            .get_campaign(id)
            .map(|item| serde_json::to_value(item).unwrap())
    });
    collect_conflicts(&mut conflicts, policy, &local.burden_packs, |id| {
        parent
            .get_burden_pack(id)
            .map(|item| serde_json::to_value(item).unwrap())
    });
    collect_conflicts(&mut conflicts, policy, &local.claim_packets, |id| {
        parent
            .get_claim_packet(id)
            .map(|item| serde_json::to_value(item).unwrap())
    });
    collect_conflicts(&mut conflicts, policy, &local.evidence_contracts, |id| {
        parent
            .get_evidence_contract(id)
            .map(|item| serde_json::to_value(item).unwrap())
    });
    collect_conflicts(&mut conflicts, policy, &local.benchmark_receipts, |id| {
        parent
            .get_benchmark_receipt(id)
            .map(|item| serde_json::to_value(item).unwrap())
    });
    collect_conflicts(&mut conflicts, policy, &local.challenge_receipts, |id| {
        parent
            .get_challenge_receipt(id)
            .map(|item| serde_json::to_value(item).unwrap())
    });
    collect_conflicts(
        &mut conflicts,
        policy,
        &local.reproducibility_packets,
        |id| {
            parent
                .get_reproducibility_packet(id)
                .map(|item| serde_json::to_value(item).unwrap())
        },
    );
    collect_conflicts(&mut conflicts, policy, &local.policy_objects, |id| {
        parent
            .get_policy_object(id)
            .map(|item| serde_json::to_value(item).unwrap())
    });

    Ok(BundleMergeReport {
        id: "BMER_IMPORTED".into(),
        policy: policy.clone(),
        imported_entries: bundle_entries(local).len(),
        namespaced_entries: usize::from(*policy == BundleConflictPolicy::NamespacedImport)
            * bundle_entries(local).len(),
        conflicts,
    })
}

fn collect_conflicts<T: serde::Serialize>(
    conflicts: &mut Vec<BundleConflict>,
    policy: &BundleConflictPolicy,
    items: &[T],
    existing: impl Fn(&str) -> Option<serde_json::Value>,
) where
    T: HasId,
{
    for item in items {
        if let Some(existing_value) = existing(item.id()) {
            let local_value = serde_json::to_value(item).unwrap();
            let exact = existing_value == local_value;
            let message = if exact {
                "exact-match".to_string()
            } else {
                "shadowed-or-conflicting".to_string()
            };
            if !exact || *policy != BundleConflictPolicy::ExactMatch {
                conflicts.push(BundleConflict {
                    id: item.id().to_string(),
                    kind: std::any::type_name::<T>()
                        .rsplit("::")
                        .next()
                        .unwrap_or("entry")
                        .into(),
                    policy: policy.clone(),
                    message,
                });
            }
        }
    }
}

fn bundle_entries(local: &RegistryBundle) -> Vec<BundleEntry> {
    registry_bundle_entries(local)
}

fn bundle_dependencies(local: &RegistryBundle) -> Vec<BundleDependency> {
    let mut deps = Vec::new();
    deps.extend(local.theorem_specs.iter().map(|item| BundleDependency {
        id: item.id.clone(),
        depends_on: item.bridges.clone(),
    }));
    deps.extend(local.adequacy_clauses.iter().map(|item| {
        BundleDependency {
            id: item.id.clone(),
            depends_on: item
                .bridge_ids
                .iter()
                .cloned()
                .chain(item.theorem_ids.iter().cloned())
                .chain(item.burden_pack_ids.iter().cloned())
                .chain(item.claim_packet_ids.iter().cloned())
                .chain(item.evidence_contract_ids.iter().cloned())
                .chain(item.benchmark_receipt_ids.iter().cloned())
                .chain(item.challenge_receipt_ids.iter().cloned())
                .chain(item.reproducibility_packet_ids.iter().cloned())
                .collect(),
        }
    }));
    deps.extend(local.campaigns.iter().map(|item| {
        BundleDependency {
            id: item.id.clone(),
            depends_on: std::iter::once(item.theorem.clone())
                .chain(std::iter::once(item.target_profile.clone()))
                .chain(std::iter::once(item.route_ledger.clone()))
                .chain(item.obligations.clone())
                .chain(item.certificates.clone())
                .collect(),
        }
    }));
    deps
}

fn bundle_from_document(document: &QaDocument) -> RegistryBundle {
    let mut bundle = RegistryBundle::default();
    for entry in &document.entries {
        match entry {
            QaEntry::Object(item) => bundle.objects.push(item.clone()),
            QaEntry::Regime(item) => bundle.regimes.push(item.clone()),
            QaEntry::Bridge(item) => bundle.bridges.push(item.clone()),
            QaEntry::ProofShape(item) => bundle.proof_shapes.push(item.clone()),
            QaEntry::AtlasCell(item) => bundle.atlas_cells.push(item.clone()),
            QaEntry::MechanizationPackage(item) => bundle.mechanization_packages.push(item.clone()),
            QaEntry::TheoremSpec(item) => bundle.theorem_specs.push(item.clone()),
            QaEntry::Obligation(item) => bundle.obligations.push(item.clone()),
            QaEntry::TargetProfile(item) => bundle.target_profiles.push(item.clone()),
            QaEntry::RouteLedger(item) => bundle.route_ledgers.push(item.clone()),
            QaEntry::Certificate(item) => bundle.certificates.push(item.clone()),
            QaEntry::Campaign(item) => bundle.campaigns.push(item.clone()),
            QaEntry::CampaignPortfolio(item) => bundle.campaign_portfolios.push(item.clone()),
            QaEntry::RouteClass(item) => bundle.route_classes.push(item.clone()),
            QaEntry::AtlasDeficiency(item) => bundle.atlas_deficiencies.push(item.clone()),
            QaEntry::AdequacyClause(item) => bundle.adequacy_clauses.push(item.clone()),
            QaEntry::BurdenPack(item) => bundle.burden_packs.push(item.clone()),
            QaEntry::ClaimPacket(item) => bundle.claim_packets.push(item.clone()),
            QaEntry::EvidenceContract(item) => bundle.evidence_contracts.push(item.clone()),
            QaEntry::BenchmarkReceipt(item) => bundle.benchmark_receipts.push(item.clone()),
            QaEntry::ChallengeReceipt(item) => bundle.challenge_receipts.push(item.clone()),
            QaEntry::ReproducibilityPacket(item) => {
                bundle.reproducibility_packets.push(item.clone())
            }
            QaEntry::SurfacePolicy(item) => bundle.surface_policies.push(item.clone()),
            QaEntry::TransformReceipt(item) => bundle.transform_receipts.push(item.clone()),
            QaEntry::RoundTripReport(item) => bundle.roundtrip_reports.push(item.clone()),
            QaEntry::CapabilityMatrix(item) => bundle.capability_matrices.push(item.clone()),
            QaEntry::SurfaceBudget(_) => {}
            QaEntry::PolicyObject(item) => bundle.policy_objects.push(item.clone()),
            QaEntry::PolicyBinding(item) => bundle.policy_bindings.push(item.clone()),
            QaEntry::PolicyResolution(item) => bundle.policy_resolutions.push(item.clone()),
            QaEntry::BundleLock(item) => bundle.bundle_locks.push(item.clone()),
            QaEntry::ExecutionManifest(item) => bundle.execution_manifests.push(item.clone()),
            QaEntry::ReplayLockManifest(item) => bundle.replay_lock_manifests.push(item.clone()),
            QaEntry::LockReceipt(_) => {}
            QaEntry::LockDiff(_) => {}
            QaEntry::RecomputationPlan(_) => {}
            QaEntry::PlanExecution(_) => {}
            QaEntry::PredictionAssessment(_) => {}
            QaEntry::Reconciliation(_) => {}
            QaEntry::RootResolution(_) => {}
        }
    }
    bundle
}

fn namespace_document(mut document: QaDocument, namespace: &str) -> QaDocument {
    let prefix = format!("{namespace}::");
    let id_map = collect_ids(&document)
        .into_iter()
        .map(|id| (id.clone(), format!("{prefix}{id}")))
        .collect::<HashMap<_, _>>();
    for entry in &mut document.entries {
        match entry {
            QaEntry::Object(item) => {
                item.id = remap(&item.id, &id_map);
                item.identity.cid = remap(&item.identity.cid, &id_map);
                item.constraint.regime = remap(&item.constraint.regime, &id_map);
                item.constraint.contracts = remap_vec(&item.constraint.contracts, &id_map);
                item.constraint.invariants = remap_vec(&item.constraint.invariants, &id_map);
            }
            QaEntry::Regime(item) => item.id = remap(&item.id, &id_map),
            QaEntry::Bridge(item) => {
                item.id = remap(&item.id, &id_map);
                item.src = remap(&item.src, &id_map);
                item.tgt = remap(&item.tgt, &id_map);
                item.receipts = remap_vec(&item.receipts, &id_map);
            }
            QaEntry::ProofShape(item) => {
                item.id = remap(&item.id, &id_map);
                item.receipts = remap_vec(&item.receipts, &id_map);
            }
            QaEntry::AtlasCell(item) => {
                item.id = remap(&item.id, &id_map);
                item.source_regime = remap(&item.source_regime, &id_map);
                item.target_regime = remap(&item.target_regime, &id_map);
                item.candidate_paths = item
                    .candidate_paths
                    .iter()
                    .map(|path| remap_vec(path, &id_map))
                    .collect();
                item.normalized_winner = remap_vec(&item.normalized_winner, &id_map);
                item.proof_shapes_checked = remap_vec(&item.proof_shapes_checked, &id_map);
            }
            QaEntry::MechanizationPackage(item) => item.id = remap(&item.id, &id_map),
            QaEntry::TheoremSpec(item) => {
                item.id = remap(&item.id, &id_map);
                item.hosts = remap_vec(&item.hosts, &id_map);
                item.bridges = remap_vec(&item.bridges, &id_map);
                item.proof_shapes = remap_vec(&item.proof_shapes, &id_map);
            }
            QaEntry::Obligation(item) => item.id = remap(&item.id, &id_map),
            QaEntry::TargetProfile(item) => item.id = remap(&item.id, &id_map),
            QaEntry::RouteLedger(item) => {
                item.id = remap(&item.id, &id_map);
                item.theorem = remap(&item.theorem, &id_map);
                item.paths = item
                    .paths
                    .iter()
                    .map(|path| remap_vec(path, &id_map))
                    .collect();
                item.receipts = remap_vec(&item.receipts, &id_map);
                item.normalized_path = remap_vec(&item.normalized_path, &id_map);
            }
            QaEntry::Certificate(item) => {
                item.id = remap(&item.id, &id_map);
                item.theorem = remap(&item.theorem, &id_map);
                item.route_ledger = remap(&item.route_ledger, &id_map);
                item.proof_shapes = remap_vec(&item.proof_shapes, &id_map);
                item.receipts = remap_vec(&item.receipts, &id_map);
            }
            QaEntry::Campaign(item) => {
                item.id = remap(&item.id, &id_map);
                item.theorem = remap(&item.theorem, &id_map);
                item.target_profile = remap(&item.target_profile, &id_map);
                item.route_ledger = remap(&item.route_ledger, &id_map);
                item.obligations = remap_vec(&item.obligations, &id_map);
                item.certificates = remap_vec(&item.certificates, &id_map);
                item.dependencies = remap_vec(&item.dependencies, &id_map);
            }
            QaEntry::CampaignPortfolio(item) => {
                item.id = remap(&item.id, &id_map);
                item.campaigns = remap_vec(&item.campaigns, &id_map);
            }
            QaEntry::RouteClass(item) => {
                item.id = remap(&item.id, &id_map);
                item.theorem = remap(&item.theorem, &id_map);
                item.target_profile = remap(&item.target_profile, &id_map);
                item.equivalent_paths = item
                    .equivalent_paths
                    .iter()
                    .map(|path| remap_vec(path, &id_map))
                    .collect();
                item.canonical_path = remap_vec(&item.canonical_path, &id_map);
            }
            QaEntry::AtlasDeficiency(item) => {
                item.id = remap(&item.id, &id_map);
                item.atlas_cell = item.atlas_cell.as_ref().map(|id| remap(id, &id_map));
                item.theorem = item.theorem.as_ref().map(|id| remap(id, &id_map));
            }
            QaEntry::AdequacyClause(item) => {
                item.id = remap(&item.id, &id_map);
                item.regime_ids = remap_vec(&item.regime_ids, &id_map);
                item.bridge_ids = remap_vec(&item.bridge_ids, &id_map);
                item.theorem_ids = remap_vec(&item.theorem_ids, &id_map);
                item.burden_pack_ids = remap_vec(&item.burden_pack_ids, &id_map);
                item.claim_packet_ids = remap_vec(&item.claim_packet_ids, &id_map);
                item.evidence_contract_ids = remap_vec(&item.evidence_contract_ids, &id_map);
                item.benchmark_receipt_ids = remap_vec(&item.benchmark_receipt_ids, &id_map);
                item.challenge_receipt_ids = remap_vec(&item.challenge_receipt_ids, &id_map);
                item.reproducibility_packet_ids =
                    remap_vec(&item.reproducibility_packet_ids, &id_map);
            }
            QaEntry::BurdenPack(item) => {
                item.id = remap(&item.id, &id_map);
                item.allowed_host_cluster = remap_vec(&item.allowed_host_cluster, &id_map);
                item.obligation_ids = remap_vec(&item.obligation_ids, &id_map);
                item.adequacy_clause_ids = remap_vec(&item.adequacy_clause_ids, &id_map);
                item.route_class_constraints = remap_vec(&item.route_class_constraints, &id_map);
                item.evidence_contract_ids = remap_vec(&item.evidence_contract_ids, &id_map);
            }
            QaEntry::ClaimPacket(item) => {
                item.id = remap(&item.id, &id_map);
            }
            QaEntry::EvidenceContract(item) => {
                item.id = remap(&item.id, &id_map);
            }
            QaEntry::BenchmarkReceipt(item) => {
                item.id = remap(&item.id, &id_map);
                item.claim_packet_id = remap(&item.claim_packet_id, &id_map);
                item.reproducibility_ref = remap(&item.reproducibility_ref, &id_map);
            }
            QaEntry::ChallengeReceipt(item) => {
                item.id = remap(&item.id, &id_map);
                item.claim_packet_id = remap(&item.claim_packet_id, &id_map);
            }
            QaEntry::ReproducibilityPacket(item) => {
                item.id = remap(&item.id, &id_map);
                item.claim_packet_id = remap(&item.claim_packet_id, &id_map);
                item.benchmark_refs = remap_vec(&item.benchmark_refs, &id_map);
                item.artifact_refs = remap_vec(&item.artifact_refs, &id_map);
            }
            QaEntry::SurfacePolicy(item) => item.id = remap(&item.id, &id_map),
            QaEntry::TransformReceipt(item) => {
                item.id = remap(&item.id, &id_map);
                item.object_ids = remap_vec(&item.object_ids, &id_map);
            }
            QaEntry::RoundTripReport(item) => {
                item.id = remap(&item.id, &id_map);
                item.object_ids = remap_vec(&item.object_ids, &id_map);
                item.receipt_ids = remap_vec(&item.receipt_ids, &id_map);
            }
            QaEntry::CapabilityMatrix(item) => item.id = remap(&item.id, &id_map),
            QaEntry::SurfaceBudget(item) => item.id = remap(&item.id, &id_map),
            QaEntry::PolicyObject(item) => {
                item.id = remap(&item.id, &id_map);
                item.extends = item.extends.as_ref().map(|id| remap(id, &id_map));
            }
            QaEntry::PolicyBinding(item) => {
                item.id = remap(&item.id, &id_map);
                item.policy_id = remap(&item.policy_id, &id_map);
                item.target_id = item.target_id.as_ref().map(|id| remap(id, &id_map));
            }
            QaEntry::PolicyResolution(item) => {
                item.id = remap(&item.id, &id_map);
                item.applied_policy_ids = remap_vec(&item.applied_policy_ids, &id_map);
                item.trace.id = remap(&item.trace.id, &id_map);
            }
            QaEntry::BundleLock(item) => {
                item.id = remap(&item.id, &id_map);
                item.bundle_id = remap(&item.bundle_id, &id_map);
                item.policy_resolution_id = remap(&item.policy_resolution_id, &id_map);
                item.report_ids = remap_vec(&item.report_ids, &id_map);
                item.manifest_id = remap(&item.manifest_id, &id_map);
            }
            QaEntry::ExecutionManifest(item) => {
                item.id = remap(&item.id, &id_map);
                item.bundle_id = remap(&item.bundle_id, &id_map);
                item.policy_manifest.id = remap(&item.policy_manifest.id, &id_map);
                item.policy_manifest.resolution_id =
                    remap(&item.policy_manifest.resolution_id, &id_map);
                item.policy_manifest.policy_ids =
                    remap_vec(&item.policy_manifest.policy_ids, &id_map);
                item.route_winner_ids = remap_vec(&item.route_winner_ids, &id_map);
                item.report_ids = remap_vec(&item.report_ids, &id_map);
            }
            QaEntry::ReplayLockManifest(item) => {
                item.id = remap(&item.id, &id_map);
                item.report_id = remap(&item.report_id, &id_map);
            }
            QaEntry::LockReceipt(item) => {
                item.id = remap(&item.id, &id_map);
                item.lock_id = remap(&item.lock_id, &id_map);
                item.manifest_id = remap(&item.manifest_id, &id_map);
                item.bundle_id = remap(&item.bundle_id, &id_map);
                item.receipt_ids = remap_vec(&item.receipt_ids, &id_map);
            }
            QaEntry::LockDiff(item) => {
                item.id = remap(&item.id, &id_map);
                item.left_lock_id = remap(&item.left_lock_id, &id_map);
                item.right_lock_id = remap(&item.right_lock_id, &id_map);
            }
            QaEntry::RecomputationPlan(_) => {}
            QaEntry::PlanExecution(_) => {}
            QaEntry::PredictionAssessment(_) => {}
            QaEntry::Reconciliation(_) => {}
            QaEntry::RootResolution(_) => {}
        }
    }
    document
}

fn collect_ids(document: &QaDocument) -> HashSet<String> {
    document.entries.iter().map(QaEntry::id).collect()
}

fn registry_bundle_entries(local: &RegistryBundle) -> Vec<BundleEntry> {
    let mut entries = Vec::new();
    entries.extend(local.objects.iter().cloned().map(QaEntry::Object));
    entries.extend(local.regimes.iter().cloned().map(QaEntry::Regime));
    entries.extend(local.bridges.iter().cloned().map(QaEntry::Bridge));
    entries.extend(local.proof_shapes.iter().cloned().map(QaEntry::ProofShape));
    entries.extend(local.atlas_cells.iter().cloned().map(QaEntry::AtlasCell));
    entries.extend(
        local
            .mechanization_packages
            .iter()
            .cloned()
            .map(QaEntry::MechanizationPackage),
    );
    entries.extend(
        local
            .theorem_specs
            .iter()
            .cloned()
            .map(QaEntry::TheoremSpec),
    );
    entries.extend(local.obligations.iter().cloned().map(QaEntry::Obligation));
    entries.extend(
        local
            .target_profiles
            .iter()
            .cloned()
            .map(QaEntry::TargetProfile),
    );
    entries.extend(
        local
            .route_ledgers
            .iter()
            .cloned()
            .map(QaEntry::RouteLedger),
    );
    entries.extend(local.certificates.iter().cloned().map(QaEntry::Certificate));
    entries.extend(local.campaigns.iter().cloned().map(QaEntry::Campaign));
    entries.extend(
        local
            .campaign_portfolios
            .iter()
            .cloned()
            .map(QaEntry::CampaignPortfolio),
    );
    entries.extend(local.route_classes.iter().cloned().map(QaEntry::RouteClass));
    entries.extend(
        local
            .atlas_deficiencies
            .iter()
            .cloned()
            .map(QaEntry::AtlasDeficiency),
    );
    entries.extend(
        local
            .adequacy_clauses
            .iter()
            .cloned()
            .map(QaEntry::AdequacyClause),
    );
    entries.extend(local.burden_packs.iter().cloned().map(QaEntry::BurdenPack));
    entries.extend(
        local
            .claim_packets
            .iter()
            .cloned()
            .map(QaEntry::ClaimPacket),
    );
    entries.extend(
        local
            .evidence_contracts
            .iter()
            .cloned()
            .map(QaEntry::EvidenceContract),
    );
    entries.extend(
        local
            .benchmark_receipts
            .iter()
            .cloned()
            .map(QaEntry::BenchmarkReceipt),
    );
    entries.extend(
        local
            .challenge_receipts
            .iter()
            .cloned()
            .map(QaEntry::ChallengeReceipt),
    );
    entries.extend(
        local
            .reproducibility_packets
            .iter()
            .cloned()
            .map(QaEntry::ReproducibilityPacket),
    );
    entries.extend(
        local
            .surface_policies
            .iter()
            .cloned()
            .map(QaEntry::SurfacePolicy),
    );
    entries.extend(
        local
            .transform_receipts
            .iter()
            .cloned()
            .map(QaEntry::TransformReceipt),
    );
    entries.extend(
        local
            .roundtrip_reports
            .iter()
            .cloned()
            .map(QaEntry::RoundTripReport),
    );
    entries.extend(
        local
            .capability_matrices
            .iter()
            .cloned()
            .map(QaEntry::CapabilityMatrix),
    );
    entries.extend(
        local
            .policy_objects
            .iter()
            .cloned()
            .map(QaEntry::PolicyObject),
    );
    entries.extend(
        local
            .policy_bindings
            .iter()
            .cloned()
            .map(QaEntry::PolicyBinding),
    );
    entries.extend(
        local
            .policy_resolutions
            .iter()
            .cloned()
            .map(QaEntry::PolicyResolution),
    );
    entries.extend(local.bundle_locks.iter().cloned().map(QaEntry::BundleLock));
    entries.extend(
        local
            .execution_manifests
            .iter()
            .cloned()
            .map(QaEntry::ExecutionManifest),
    );
    entries.extend(
        local
            .replay_lock_manifests
            .iter()
            .cloned()
            .map(QaEntry::ReplayLockManifest),
    );
    entries
        .into_iter()
        .map(|entry| BundleEntry {
            id: entry.id(),
            kind: entry.kind_tag().into(),
        })
        .collect()
}

fn remap(value: &str, map: &HashMap<String, String>) -> String {
    map.get(value).cloned().unwrap_or_else(|| value.to_string())
}

fn remap_vec(values: &[String], map: &HashMap<String, String>) -> Vec<String> {
    values.iter().map(|value| remap(value, map)).collect()
}

impl OverlayRegistry {
    pub fn local_only(&self) -> LocalRegistry {
        LocalRegistry {
            bundle_id: self.bundle_id.clone(),
            local: self.local.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalRegistry {
    pub bundle_id: String,
    pub local: RegistryBundle,
}

macro_rules! lookup_local_then_parent {
    ($self:expr, $field:ident, $id:expr, $parent_method:ident) => {
        $self
            .local
            .$field
            .iter()
            .find(|item| item.id == $id)
            .cloned()
            .or_else(|| $self.parent.$parent_method($id))
    };
}

macro_rules! lookup_local_only {
    ($self:expr, $field:ident, $id:expr) => {
        $self
            .local
            .$field
            .iter()
            .find(|item| item.id == $id)
            .cloned()
    };
}

impl RegistryLookup for OverlayRegistry {
    fn get_object(&self, id: &str) -> Option<QcObject> {
        lookup_local_then_parent!(self, objects, id, get_object)
    }
    fn get_object_origin(&self, id: &str) -> ArtifactOrigin {
        if self.local.objects.iter().any(|item| item.id == id) {
            ArtifactOrigin::Overlay
        } else {
            self.parent.get_object_origin(id)
        }
    }
    fn get_regime(&self, id: &str) -> Option<RegimePack> {
        lookup_local_then_parent!(self, regimes, id, get_regime)
    }
    fn get_bridge(&self, id: &str) -> Option<BridgeContract> {
        lookup_local_then_parent!(self, bridges, id, get_bridge)
    }
    fn get_proof_shape(&self, id: &str) -> Option<ProofShape> {
        lookup_local_then_parent!(self, proof_shapes, id, get_proof_shape)
    }
    fn get_atlas_cell(&self, id: &str) -> Option<AtlasCell> {
        lookup_local_then_parent!(self, atlas_cells, id, get_atlas_cell)
    }
    fn get_mechanization_package(&self, id: &str) -> Option<MechanizationPackage> {
        lookup_local_then_parent!(self, mechanization_packages, id, get_mechanization_package)
    }
    fn get_theorem_spec(&self, id: &str) -> Option<TheoremSpec> {
        lookup_local_then_parent!(self, theorem_specs, id, get_theorem_spec)
    }
    fn get_obligation(&self, id: &str) -> Option<Obligation> {
        lookup_local_then_parent!(self, obligations, id, get_obligation)
    }
    fn get_target_profile(&self, id: &str) -> Option<TargetProfile> {
        lookup_local_then_parent!(self, target_profiles, id, get_target_profile)
    }
    fn get_route_ledger(&self, id: &str) -> Option<RouteLedger> {
        lookup_local_then_parent!(self, route_ledgers, id, get_route_ledger)
    }
    fn get_certificate(&self, id: &str) -> Option<Certificate> {
        lookup_local_then_parent!(self, certificates, id, get_certificate)
    }
    fn get_campaign(&self, id: &str) -> Option<Campaign> {
        lookup_local_then_parent!(self, campaigns, id, get_campaign)
    }
    fn get_campaign_portfolio(&self, id: &str) -> Option<CampaignPortfolio> {
        lookup_local_then_parent!(self, campaign_portfolios, id, get_campaign_portfolio)
    }
    fn get_route_class(&self, id: &str) -> Option<RouteClass> {
        lookup_local_then_parent!(self, route_classes, id, get_route_class)
    }
    fn get_atlas_deficiency(&self, id: &str) -> Option<AtlasDeficiency> {
        lookup_local_then_parent!(self, atlas_deficiencies, id, get_atlas_deficiency)
    }
    fn atlas_deficiencies(&self) -> Vec<AtlasDeficiency> {
        let mut items = self.local.atlas_deficiencies.clone();
        items.extend(self.parent.atlas_deficiencies());
        dedupe_by_id(items, |item| item.id.clone())
    }
    fn get_adequacy_clause(&self, id: &str) -> Option<AdequacyClause> {
        lookup_local_then_parent!(self, adequacy_clauses, id, get_adequacy_clause)
    }
    fn adequacy_clauses(&self) -> Vec<AdequacyClause> {
        let mut items = self.local.adequacy_clauses.clone();
        items.extend(self.parent.adequacy_clauses());
        dedupe_by_id(items, |item| item.id.clone())
    }
    fn get_burden_pack(&self, id: &str) -> Option<BurdenPack> {
        lookup_local_then_parent!(self, burden_packs, id, get_burden_pack)
    }
    fn burden_packs(&self) -> Vec<BurdenPack> {
        let mut items = self.local.burden_packs.clone();
        items.extend(self.parent.burden_packs());
        dedupe_by_id(items, |item| item.id.clone())
    }
    fn get_claim_packet(&self, id: &str) -> Option<ClaimPacket> {
        lookup_local_then_parent!(self, claim_packets, id, get_claim_packet)
    }
    fn claim_packets(&self) -> Vec<ClaimPacket> {
        let mut items = self.local.claim_packets.clone();
        items.extend(self.parent.claim_packets());
        dedupe_by_id(items, |item| item.id.clone())
    }
    fn get_evidence_contract(&self, id: &str) -> Option<EvidenceContract> {
        lookup_local_then_parent!(self, evidence_contracts, id, get_evidence_contract)
    }
    fn evidence_contracts(&self) -> Vec<EvidenceContract> {
        let mut items = self.local.evidence_contracts.clone();
        items.extend(self.parent.evidence_contracts());
        dedupe_by_id(items, |item| item.id.clone())
    }
    fn get_benchmark_receipt(&self, id: &str) -> Option<BenchmarkReceipt> {
        lookup_local_then_parent!(self, benchmark_receipts, id, get_benchmark_receipt)
    }
    fn benchmark_receipts(&self) -> Vec<BenchmarkReceipt> {
        let mut items = self.local.benchmark_receipts.clone();
        items.extend(self.parent.benchmark_receipts());
        dedupe_by_id(items, |item| item.id.clone())
    }
    fn get_challenge_receipt(&self, id: &str) -> Option<ChallengeReceipt> {
        lookup_local_then_parent!(self, challenge_receipts, id, get_challenge_receipt)
    }
    fn challenge_receipts(&self) -> Vec<ChallengeReceipt> {
        let mut items = self.local.challenge_receipts.clone();
        items.extend(self.parent.challenge_receipts());
        dedupe_by_id(items, |item| item.id.clone())
    }
    fn get_reproducibility_packet(&self, id: &str) -> Option<ReproducibilityPacket> {
        lookup_local_then_parent!(
            self,
            reproducibility_packets,
            id,
            get_reproducibility_packet
        )
    }
    fn reproducibility_packets(&self) -> Vec<ReproducibilityPacket> {
        let mut items = self.local.reproducibility_packets.clone();
        items.extend(self.parent.reproducibility_packets());
        dedupe_by_id(items, |item| item.id.clone())
    }
    fn get_codebook_pack(&self, id: &str) -> Option<CodebookPack> {
        lookup_local_then_parent!(self, codebook_packs, id, get_codebook_pack)
    }
    fn get_glyph_pack(&self, id: &str) -> Option<GlyphPack> {
        lookup_local_then_parent!(self, glyph_packs, id, get_glyph_pack)
    }
    fn get_combo_pack(&self, id: &str) -> Option<ComboPack> {
        lookup_local_then_parent!(self, combo_packs, id, get_combo_pack)
    }
    fn get_projection_policy(&self, id: &str) -> Option<ProjectionPolicy> {
        lookup_local_then_parent!(self, projection_policies, id, get_projection_policy)
    }
    fn get_alias_expansion_policy(&self, id: &str) -> Option<AliasExpansionPolicy> {
        lookup_local_then_parent!(
            self,
            alias_expansion_policies,
            id,
            get_alias_expansion_policy
        )
    }
    fn get_surface_policy(&self, id: &str) -> Option<SurfacePolicy> {
        lookup_local_then_parent!(self, surface_policies, id, get_surface_policy)
    }
    fn get_capability_matrix(&self, id: &str) -> Option<CapabilityMatrix> {
        lookup_local_then_parent!(self, capability_matrices, id, get_capability_matrix)
    }
    fn get_roundtrip_report(&self, id: &str) -> Option<RoundTripReport> {
        lookup_local_then_parent!(self, roundtrip_reports, id, get_roundtrip_report)
    }
    fn get_transform_receipt(&self, id: &str) -> Option<FormatTransformReceipt> {
        lookup_local_then_parent!(self, transform_receipts, id, get_transform_receipt)
    }
    fn get_surface_deficiency(&self, id: &str) -> Option<SurfaceDeficiency> {
        lookup_local_then_parent!(self, surface_deficiencies, id, get_surface_deficiency)
    }
    fn get_policy_object(&self, id: &str) -> Option<MechanizationPolicyObject> {
        lookup_local_then_parent!(self, policy_objects, id, get_policy_object)
    }
    fn policy_objects(&self) -> Vec<MechanizationPolicyObject> {
        let mut items = self.local.policy_objects.clone();
        items.extend(self.parent.policy_objects());
        items
    }
    fn policy_bindings(&self) -> Vec<PolicyBinding> {
        let mut items = self.local.policy_bindings.clone();
        items.extend(self.parent.policy_bindings());
        items
    }
    fn find_equivalence_class(&self, object_id: &str, regime: &str) -> Option<EquivalenceClass> {
        self.local
            .equivalence_classes
            .iter()
            .find(|item| {
                item.regime == regime && item.members.iter().any(|member| member == object_id)
            })
            .cloned()
            .or_else(|| self.parent.find_equivalence_class(object_id, regime))
    }
    fn atlas_cells(&self) -> Vec<AtlasCell> {
        let mut items = self.local.atlas_cells.clone();
        items.extend(self.parent.atlas_cells());
        items
    }
}

impl RegistryLookup for LocalRegistry {
    fn get_object(&self, id: &str) -> Option<QcObject> {
        lookup_local_only!(self, objects, id)
    }
    fn get_object_origin(&self, id: &str) -> ArtifactOrigin {
        if self.local.objects.iter().any(|item| item.id == id) {
            ArtifactOrigin::Overlay
        } else {
            ArtifactOrigin::Unknown
        }
    }
    fn get_regime(&self, id: &str) -> Option<RegimePack> {
        lookup_local_only!(self, regimes, id)
    }
    fn get_bridge(&self, id: &str) -> Option<BridgeContract> {
        lookup_local_only!(self, bridges, id)
    }
    fn get_proof_shape(&self, id: &str) -> Option<ProofShape> {
        lookup_local_only!(self, proof_shapes, id)
    }
    fn get_atlas_cell(&self, id: &str) -> Option<AtlasCell> {
        lookup_local_only!(self, atlas_cells, id)
    }
    fn get_mechanization_package(&self, id: &str) -> Option<MechanizationPackage> {
        lookup_local_only!(self, mechanization_packages, id)
    }
    fn get_theorem_spec(&self, id: &str) -> Option<TheoremSpec> {
        lookup_local_only!(self, theorem_specs, id)
    }
    fn get_obligation(&self, id: &str) -> Option<Obligation> {
        lookup_local_only!(self, obligations, id)
    }
    fn get_target_profile(&self, id: &str) -> Option<TargetProfile> {
        lookup_local_only!(self, target_profiles, id)
    }
    fn get_route_ledger(&self, id: &str) -> Option<RouteLedger> {
        lookup_local_only!(self, route_ledgers, id)
    }
    fn get_certificate(&self, id: &str) -> Option<Certificate> {
        lookup_local_only!(self, certificates, id)
    }
    fn get_campaign(&self, id: &str) -> Option<Campaign> {
        lookup_local_only!(self, campaigns, id)
    }
    fn get_campaign_portfolio(&self, id: &str) -> Option<CampaignPortfolio> {
        lookup_local_only!(self, campaign_portfolios, id)
    }
    fn get_route_class(&self, id: &str) -> Option<RouteClass> {
        lookup_local_only!(self, route_classes, id)
    }
    fn get_atlas_deficiency(&self, id: &str) -> Option<AtlasDeficiency> {
        lookup_local_only!(self, atlas_deficiencies, id)
    }
    fn atlas_deficiencies(&self) -> Vec<AtlasDeficiency> {
        self.local.atlas_deficiencies.clone()
    }
    fn get_adequacy_clause(&self, id: &str) -> Option<AdequacyClause> {
        lookup_local_only!(self, adequacy_clauses, id)
    }
    fn adequacy_clauses(&self) -> Vec<AdequacyClause> {
        self.local.adequacy_clauses.clone()
    }
    fn get_burden_pack(&self, id: &str) -> Option<BurdenPack> {
        lookup_local_only!(self, burden_packs, id)
    }
    fn burden_packs(&self) -> Vec<BurdenPack> {
        self.local.burden_packs.clone()
    }
    fn get_claim_packet(&self, id: &str) -> Option<ClaimPacket> {
        lookup_local_only!(self, claim_packets, id)
    }
    fn claim_packets(&self) -> Vec<ClaimPacket> {
        self.local.claim_packets.clone()
    }
    fn get_evidence_contract(&self, id: &str) -> Option<EvidenceContract> {
        lookup_local_only!(self, evidence_contracts, id)
    }
    fn evidence_contracts(&self) -> Vec<EvidenceContract> {
        self.local.evidence_contracts.clone()
    }
    fn get_benchmark_receipt(&self, id: &str) -> Option<BenchmarkReceipt> {
        lookup_local_only!(self, benchmark_receipts, id)
    }
    fn benchmark_receipts(&self) -> Vec<BenchmarkReceipt> {
        self.local.benchmark_receipts.clone()
    }
    fn get_challenge_receipt(&self, id: &str) -> Option<ChallengeReceipt> {
        lookup_local_only!(self, challenge_receipts, id)
    }
    fn challenge_receipts(&self) -> Vec<ChallengeReceipt> {
        self.local.challenge_receipts.clone()
    }
    fn get_reproducibility_packet(&self, id: &str) -> Option<ReproducibilityPacket> {
        lookup_local_only!(self, reproducibility_packets, id)
    }
    fn reproducibility_packets(&self) -> Vec<ReproducibilityPacket> {
        self.local.reproducibility_packets.clone()
    }
    fn get_codebook_pack(&self, id: &str) -> Option<CodebookPack> {
        lookup_local_only!(self, codebook_packs, id)
    }
    fn get_glyph_pack(&self, id: &str) -> Option<GlyphPack> {
        lookup_local_only!(self, glyph_packs, id)
    }
    fn get_combo_pack(&self, id: &str) -> Option<ComboPack> {
        lookup_local_only!(self, combo_packs, id)
    }
    fn get_projection_policy(&self, id: &str) -> Option<ProjectionPolicy> {
        lookup_local_only!(self, projection_policies, id)
    }
    fn get_alias_expansion_policy(&self, id: &str) -> Option<AliasExpansionPolicy> {
        lookup_local_only!(self, alias_expansion_policies, id)
    }
    fn get_surface_policy(&self, id: &str) -> Option<SurfacePolicy> {
        lookup_local_only!(self, surface_policies, id)
    }
    fn get_capability_matrix(&self, id: &str) -> Option<CapabilityMatrix> {
        lookup_local_only!(self, capability_matrices, id)
    }
    fn get_roundtrip_report(&self, id: &str) -> Option<RoundTripReport> {
        lookup_local_only!(self, roundtrip_reports, id)
    }
    fn get_transform_receipt(&self, id: &str) -> Option<FormatTransformReceipt> {
        lookup_local_only!(self, transform_receipts, id)
    }
    fn get_surface_deficiency(&self, id: &str) -> Option<SurfaceDeficiency> {
        lookup_local_only!(self, surface_deficiencies, id)
    }
    fn get_policy_object(&self, id: &str) -> Option<MechanizationPolicyObject> {
        lookup_local_only!(self, policy_objects, id)
    }
    fn policy_objects(&self) -> Vec<MechanizationPolicyObject> {
        self.local.policy_objects.clone()
    }
    fn policy_bindings(&self) -> Vec<PolicyBinding> {
        self.local.policy_bindings.clone()
    }
    fn find_equivalence_class(&self, object_id: &str, regime: &str) -> Option<EquivalenceClass> {
        self.local
            .equivalence_classes
            .iter()
            .find(|item| {
                item.regime == regime && item.members.iter().any(|member| member == object_id)
            })
            .cloned()
    }
    fn atlas_cells(&self) -> Vec<AtlasCell> {
        self.local.atlas_cells.clone()
    }
}

trait HasId {
    fn id(&self) -> &str;
}

fn dedupe_by_id<T>(items: Vec<T>, id: impl Fn(&T) -> String) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();
    for item in items {
        let item_id = id(&item);
        if seen.insert(item_id) {
            deduped.push(item);
        }
    }
    deduped
}

macro_rules! impl_has_id {
    ($($t:ty),+ $(,)?) => {
        $(impl HasId for $t {
            fn id(&self) -> &str { &self.id }
        })+
    };
}

impl_has_id!(
    QcObject,
    RegimePack,
    BridgeContract,
    ProofShape,
    AtlasCell,
    TheoremSpec,
    TargetProfile,
    RouteLedger,
    Certificate,
    Campaign,
    AdequacyClause,
    BurdenPack,
    ClaimPacket,
    EvidenceContract,
    BenchmarkReceipt,
    ChallengeReceipt,
    ReproducibilityPacket,
    MechanizationPolicyObject,
    PolicyBinding,
    PolicyResolution,
    mf_core::BundleLock,
    ExecutionManifest,
    ReplayLockManifest
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlay_prefers_local_theorem() {
        let mut local = RegistryBundle::default();
        local.theorem_specs.push(TheoremSpec {
            id: "THS_LOCAL".into(),
            statement: "local".into(),
            hosts: vec!["R_SET".into(), "R_SET".into()],
            bridges: vec![],
            operators: vec![],
            target_equivalence: "eq".into(),
            obligations: vec![],
            primary_zone: mf_core::ProofMechanismZone::PmzSemantic,
            verdict: mf_core::CertificationVerdict::RouteFound,
            proof_shapes: vec![],
        });
        let overlay = OverlayRegistry {
            parent: SeedRegistry::load().unwrap(),
            bundle_id: "BND_TEST".into(),
            local,
            merge_report: BundleMergeReport {
                id: "BMER".into(),
                policy: BundleConflictPolicy::Reject,
                imported_entries: 1,
                namespaced_entries: 0,
                conflicts: Vec::new(),
            },
            import_receipts: Vec::new(),
        };
        assert!(overlay.get_theorem_spec("THS_LOCAL").is_some());
        assert!(overlay.get_theorem_spec("THS_CHAIN_RULE").is_some());
    }
}
