use l64_core::{
    BundleDependency, EvaluatorPolicyConfig, EvidencePreference, ExecutionManifest,
    MechanizationPolicyObject, OptimizationAxis, OptimizerBackend, OptimizerPolicy,
    OptimizerPolicyConfig, PolicyBinding, PolicyConflict, PolicyResolution, PolicyScope,
    PolicyTrace, PolicyVerdict, RegistryLookup, ReplayCachePolicyConfig, ReplayLockManifest,
    ReplayTrustClass, ReportPolicyConfig, SchedulerPolicyConfig, SurfaceKind,
    UnsupportedHandlingMode,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PolicyError {
    #[error("illegal policy conflict: {0}")]
    Conflict(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResolvedPolicyGraph {
    pub resolution: PolicyResolution,
    pub optimizer_backend: OptimizerBackend,
}

pub fn resolve_policy_graph(
    registry: &impl RegistryLookup,
    bundle_id: Option<&str>,
    theorem_id: Option<&str>,
    campaign_id: Option<&str>,
    target_profile_id: Option<&str>,
    strict: bool,
    fallback_optimizer: OptimizerPolicy,
) -> Result<ResolvedPolicyGraph, PolicyError> {
    let mut candidates = builtin_policies();
    candidates.extend(registry.policy_objects());
    let bindings = registry.policy_bindings();

    let scope = if let Some(id) = campaign_id {
        PolicyScope::Campaign(id.to_string())
    } else if let Some(id) = theorem_id {
        PolicyScope::Theorem(id.to_string())
    } else if let Some(id) = target_profile_id {
        PolicyScope::TargetProfile(id.to_string())
    } else if let Some(id) = bundle_id {
        PolicyScope::Bundle(id.to_string())
    } else {
        PolicyScope::Global
    };

    let applicable = applicable_policies(
        &candidates,
        &bindings,
        bundle_id,
        theorem_id,
        campaign_id,
        target_profile_id,
    );
    let mut trace_steps = vec!["resolved built-in policy baseline".to_string()];
    let mut conflicts = Vec::new();

    let mut optimizer = default_optimizer(fallback_optimizer);
    let mut evaluator = default_evaluator();
    let mut replay_cache = default_replay_cache();
    let mut report = default_report();
    let mut scheduler = default_scheduler();
    let mut applied_policy_ids = Vec::new();

    for policy in applicable {
        applied_policy_ids.push(policy.id.clone());
        trace_steps.push(format!("applied {:?} policy {}", policy.kind, policy.id));
        if let Some(config) = &policy.optimizer {
            if policy.kind != l64_core::PolicyKind::Optimizer && strict {
                conflicts.push(conflict_for(
                    &policy,
                    "optimizer config on non-optimizer policy",
                ));
                continue;
            }
            optimizer = config.clone();
        }
        if let Some(config) = &policy.evaluator {
            if policy.kind != l64_core::PolicyKind::Evaluator && strict {
                conflicts.push(conflict_for(
                    &policy,
                    "evaluator config on non-evaluator policy",
                ));
                continue;
            }
            evaluator = config.clone();
        }
        if let Some(config) = &policy.replay_cache {
            if policy.kind != l64_core::PolicyKind::ReplayCache && strict {
                conflicts.push(conflict_for(&policy, "replay config on non-replay policy"));
                continue;
            }
            replay_cache = config.clone();
        }
        if let Some(config) = &policy.report {
            if policy.kind != l64_core::PolicyKind::ReportExport && strict {
                conflicts.push(conflict_for(&policy, "report config on non-report policy"));
                continue;
            }
            report = config.clone();
        }
        if let Some(config) = &policy.scheduler {
            scheduler = config.clone();
        }
    }

    if strict {
        ensure_scope_conflicts(&mut conflicts, &applied_policy_ids, &candidates, &scope);
    }
    if strict && conflicts.iter().any(|item| item.illegal) {
        return Err(PolicyError::Conflict(
            conflicts
                .iter()
                .map(|item| item.message.clone())
                .collect::<Vec<_>>()
                .join("; "),
        ));
    }

    let resolution = PolicyResolution {
        id: format!("MPR_{}", simple_id(&applied_policy_ids.join("|"))),
        scope,
        applied_policy_ids,
        conflicts: conflicts.clone(),
        trace: PolicyTrace {
            id: format!("MPT_{}", simple_id(&trace_steps.join("|"))),
            steps: trace_steps,
        },
        optimizer: optimizer.clone(),
        evaluator,
        replay_cache,
        report,
        scheduler,
        verdict: if conflicts.is_empty() {
            PolicyVerdict::Applied
        } else {
            PolicyVerdict::Conflict
        },
    };

    Ok(ResolvedPolicyGraph {
        optimizer_backend: optimizer.backend.clone(),
        resolution,
    })
}

pub fn build_execution_manifest(
    bundle_id: &str,
    bundle_hash: &str,
    dependencies: Vec<BundleDependency>,
    resolution: &PolicyResolution,
    route_winner_ids: Vec<String>,
    evaluator_versions: Vec<String>,
    pack_versions: Vec<String>,
    report_ids: Vec<String>,
) -> ExecutionManifest {
    ExecutionManifest {
        id: format!("EXM_{}", simple_id(&(bundle_id.to_string() + bundle_hash))),
        bundle_id: bundle_id.to_string(),
        bundle_hash: bundle_hash.to_string(),
        dependency_graph: dependencies,
        policy_manifest: l64_core::PolicyManifest {
            id: format!("PMF_{}", simple_id(&resolution.id)),
            resolution_id: resolution.id.clone(),
            policy_ids: resolution.applied_policy_ids.clone(),
            policy_hash: simple_id(&serde_json::to_string(resolution).unwrap_or_default()),
        },
        route_winner_ids,
        evaluator_versions,
        pack_versions,
        report_ids,
        executed_plan_hash: None,
        executed_steps: Vec::new(),
        reused_artifacts: Vec::new(),
        rerun_artifacts: Vec::new(),
        reconciliation_summary: Vec::new(),
        root_resolution: None,
        scheduler_policy: None,
        execution_scope: None,
        lane_records: Vec::new(),
        schedule_hash: None,
        coherence_receipts: Vec::new(),
        ordering_receipt: None,
        obligation_plans: Vec::new(),
        obligation_lanes: Vec::new(),
        obligation_ordering_receipts: Vec::new(),
        obligation_merge_receipts: Vec::new(),
        replay_legality_checks: Vec::new(),
        replay_barrier_receipts: Vec::new(),
        replay_merge_receipts: Vec::new(),
        replay_divergence_records: Vec::new(),
        obligation_cache_shards: Vec::new(),
        obligation_write_sets: Vec::new(),
        obligation_collision_reports: Vec::new(),
        obligation_namespace_receipts: Vec::new(),
    }
}

pub fn build_replay_lock_manifest(
    report_id: &str,
    report_hash: &str,
    route_winner_hash: &str,
    policy_hash: &str,
    bundle_hash: &str,
) -> ReplayLockManifest {
    ReplayLockManifest {
        id: format!("RLM_{}", simple_id(&(report_id.to_string() + report_hash))),
        report_id: report_id.to_string(),
        report_hash: report_hash.to_string(),
        route_winner_hash: route_winner_hash.to_string(),
        policy_hash: policy_hash.to_string(),
        bundle_hash: bundle_hash.to_string(),
    }
}

fn applicable_policies<'a>(
    candidates: &'a [MechanizationPolicyObject],
    bindings: &[PolicyBinding],
    bundle_id: Option<&str>,
    theorem_id: Option<&str>,
    campaign_id: Option<&str>,
    target_profile_id: Option<&str>,
) -> Vec<&'a MechanizationPolicyObject> {
    let mut scored = candidates
        .iter()
        .filter_map(|policy| {
            let direct = match &policy.scope {
                PolicyScope::Global => Some(0_u32),
                PolicyScope::Bundle(id) if bundle_id == Some(id.as_str()) => Some(30),
                PolicyScope::Theorem(id) if theorem_id == Some(id.as_str()) => Some(40),
                PolicyScope::Campaign(id) if campaign_id == Some(id.as_str()) => Some(50),
                PolicyScope::TargetProfile(id) if target_profile_id == Some(id.as_str()) => {
                    Some(60)
                }
                _ => None,
            };
            let binding_bonus = bindings
                .iter()
                .filter(|binding| binding.policy_id == policy.id)
                .filter(|binding| match &binding.scope {
                    PolicyScope::Global => true,
                    PolicyScope::Bundle(id) => bundle_id == Some(id.as_str()),
                    PolicyScope::Theorem(id) => theorem_id == Some(id.as_str()),
                    PolicyScope::Campaign(id) => campaign_id == Some(id.as_str()),
                    PolicyScope::TargetProfile(id) => target_profile_id == Some(id.as_str()),
                })
                .map(|binding| binding.priority)
                .max()
                .unwrap_or_default();
            direct.map(|score| (score + binding_bonus, policy))
        })
        .collect::<Vec<_>>();
    scored.sort_by(|left, right| left.0.cmp(&right.0));
    scored.into_iter().map(|(_, policy)| policy).collect()
}

fn ensure_scope_conflicts(
    conflicts: &mut Vec<PolicyConflict>,
    applied_ids: &[String],
    candidates: &[MechanizationPolicyObject],
    scope: &PolicyScope,
) {
    let same_scope = candidates
        .iter()
        .filter(|item| &item.scope == scope)
        .collect::<Vec<_>>();
    for window in same_scope.windows(2) {
        if let [left, right] = window {
            if left.kind == right.kind
                && applied_ids.contains(&left.id)
                && applied_ids.contains(&right.id)
            {
                conflicts.push(PolicyConflict {
                    id: format!("MPC_{}", simple_id(&(left.id.clone() + &right.id))),
                    kind: left.kind.clone(),
                    left_policy_id: left.id.clone(),
                    right_policy_id: right.id.clone(),
                    message: format!("same-scope {:?} policies both applied", left.kind),
                    illegal: true,
                });
            }
        }
    }
}

fn conflict_for(policy: &MechanizationPolicyObject, message: &str) -> PolicyConflict {
    PolicyConflict {
        id: format!("MPC_{}", simple_id(&(policy.id.clone() + message))),
        kind: policy.kind.clone(),
        left_policy_id: policy.id.clone(),
        right_policy_id: policy.extends.clone().unwrap_or_default(),
        message: message.to_string(),
        illegal: true,
    }
}

fn default_optimizer(policy: OptimizerPolicy) -> OptimizerPolicyConfig {
    OptimizerPolicyConfig {
        optimizer_policy: policy.clone(),
        backend: OptimizerBackend::Lexicographic,
        active_axes: vec![
            OptimizationAxis::Lawfulness,
            OptimizationAxis::IdentityPreservation,
            OptimizationAxis::LossCompliance,
            OptimizationAxis::RollbackViability,
            OptimizationAxis::ProofShapeSatisfiability,
            OptimizationAxis::BundleResolution,
            OptimizationAxis::SurfaceTransitionPenalty,
            OptimizationAxis::ExecutionCost,
            OptimizationAxis::DerivedObligationDepth,
            OptimizationAxis::MaturityConfidence,
            OptimizationAxis::SymbolicFidelity,
            OptimizationAxis::ReceiptCompleteness,
        ],
        route_explanation_verbosity: "standard".into(),
        symbolic_fidelity_preferred: matches!(policy, OptimizerPolicy::SymbolicFidelityFirst),
        tie_break_rules: vec!["shorter-path".into(), "lower-loss".into()],
    }
}

fn default_evaluator() -> EvaluatorPolicyConfig {
    EvaluatorPolicyConfig {
        evidence_preference: EvidencePreference::RecomputeIfSupported,
        allow_approximation: true,
        unsupported_mode: UnsupportedHandlingMode::Permit,
        require_symbolic_fidelity_route: false,
        prefer_comp_replay: true,
    }
}

fn default_replay_cache() -> ReplayCachePolicyConfig {
    ReplayCachePolicyConfig {
        replay_allowed: true,
        exact_policy_match_required: true,
        survive_surface_only_changes: false,
        reuse_approximate_results: true,
        optimizer_change_invalidates: true,
        surface_pack_change_invalidates: true,
        trust_class: ReplayTrustClass::ExactPolicyOnly,
    }
}

fn default_report() -> ReportPolicyConfig {
    ReportPolicyConfig {
        export_surfaces: vec![SurfaceKind::Qc0, SurfaceKind::Qm0, SurfaceKind::Qa0],
        include_policy_trace: true,
        include_route_explanation: true,
        include_obligation_logs: true,
    }
}

fn default_scheduler() -> SchedulerPolicyConfig {
    SchedulerPolicyConfig {
        parallelization: l64_core::ParallelizationPolicy::Serialize,
        max_workers: 1,
        allow_parallel_replay: false,
        allow_parallel_certification: true,
        allow_parallel_exports: true,
        deterministic_ordering: true,
        allow_parallel_obligations: false,
        max_obligation_workers: 1,
        allow_parallel_obligation_replay: false,
        serialize_canonicalization_sensitive: true,
    }
}

fn builtin_policies() -> Vec<MechanizationPolicyObject> {
    vec![
        MechanizationPolicyObject {
            id: "MOP_GLOBAL_DEFAULT".into(),
            kind: l64_core::PolicyKind::Optimizer,
            scope: PolicyScope::Global,
            extends: None,
            optimizer: Some(default_optimizer(OptimizerPolicy::Conservative)),
            evaluator: None,
            replay_cache: None,
            report: None,
            scheduler: None,
            canonicalizer_mode: Some("canonical-default".into()),
            merge_policy: None,
            notes: vec!["seed global optimizer baseline".into()],
        },
        MechanizationPolicyObject {
            id: "MOP_GLOBAL_EVAL".into(),
            kind: l64_core::PolicyKind::Evaluator,
            scope: PolicyScope::Global,
            extends: None,
            optimizer: None,
            evaluator: Some(default_evaluator()),
            replay_cache: None,
            report: None,
            scheduler: None,
            canonicalizer_mode: None,
            merge_policy: None,
            notes: vec!["seed global evaluator baseline".into()],
        },
        MechanizationPolicyObject {
            id: "MOP_GLOBAL_REPLAY".into(),
            kind: l64_core::PolicyKind::ReplayCache,
            scope: PolicyScope::Global,
            extends: None,
            optimizer: None,
            evaluator: None,
            replay_cache: Some(default_replay_cache()),
            report: None,
            scheduler: None,
            canonicalizer_mode: None,
            merge_policy: None,
            notes: vec!["seed global replay baseline".into()],
        },
        MechanizationPolicyObject {
            id: "MOP_GLOBAL_REPORT".into(),
            kind: l64_core::PolicyKind::ReportExport,
            scope: PolicyScope::Global,
            extends: None,
            optimizer: None,
            evaluator: None,
            replay_cache: None,
            report: Some(default_report()),
            scheduler: Some(default_scheduler()),
            canonicalizer_mode: None,
            merge_policy: None,
            notes: vec!["seed global report baseline".into()],
        },
    ]
}

fn simple_id(input: &str) -> String {
    let mut hash: u64 = 1469598103934665603;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(1099511628211);
    }
    format!("{hash:x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use l64_core::RegistryLookup;

    #[derive(Default)]
    struct EmptyRegistry;
    impl RegistryLookup for EmptyRegistry {
        fn get_object(&self, _: &str) -> Option<l64_core::QcObject> {
            None
        }
        fn get_regime(&self, _: &str) -> Option<l64_core::RegimePack> {
            None
        }
        fn get_bridge(&self, _: &str) -> Option<l64_core::BridgeContract> {
            None
        }
        fn get_proof_shape(&self, _: &str) -> Option<l64_core::ProofShape> {
            None
        }
        fn get_atlas_cell(&self, _: &str) -> Option<l64_core::AtlasCell> {
            None
        }
        fn get_mechanization_package(&self, _: &str) -> Option<l64_core::MechanizationPackage> {
            None
        }
        fn get_theorem_spec(&self, _: &str) -> Option<l64_core::TheoremSpec> {
            None
        }
        fn get_obligation(&self, _: &str) -> Option<l64_core::Obligation> {
            None
        }
        fn get_target_profile(&self, _: &str) -> Option<l64_core::TargetProfile> {
            None
        }
        fn get_route_ledger(&self, _: &str) -> Option<l64_core::RouteLedger> {
            None
        }
        fn get_certificate(&self, _: &str) -> Option<l64_core::Certificate> {
            None
        }
        fn get_campaign(&self, _: &str) -> Option<l64_core::Campaign> {
            None
        }
        fn get_campaign_portfolio(&self, _: &str) -> Option<l64_core::CampaignPortfolio> {
            None
        }
        fn get_route_class(&self, _: &str) -> Option<l64_core::RouteClass> {
            None
        }
        fn get_atlas_deficiency(&self, _: &str) -> Option<l64_core::AtlasDeficiency> {
            None
        }
        fn atlas_deficiencies(&self) -> Vec<l64_core::AtlasDeficiency> {
            Vec::new()
        }
        fn get_adequacy_clause(&self, _: &str) -> Option<l64_core::AdequacyClause> {
            None
        }
        fn adequacy_clauses(&self) -> Vec<l64_core::AdequacyClause> {
            Vec::new()
        }
        fn get_codebook_pack(&self, _: &str) -> Option<l64_core::CodebookPack> {
            None
        }
        fn get_glyph_pack(&self, _: &str) -> Option<l64_core::GlyphPack> {
            None
        }
        fn get_combo_pack(&self, _: &str) -> Option<l64_core::ComboPack> {
            None
        }
        fn get_projection_policy(&self, _: &str) -> Option<l64_core::ProjectionPolicy> {
            None
        }
        fn get_alias_expansion_policy(&self, _: &str) -> Option<l64_core::AliasExpansionPolicy> {
            None
        }
        fn get_surface_policy(&self, _: &str) -> Option<l64_core::SurfacePolicy> {
            None
        }
        fn get_capability_matrix(&self, _: &str) -> Option<l64_core::CapabilityMatrix> {
            None
        }
        fn get_roundtrip_report(&self, _: &str) -> Option<l64_core::RoundTripReport> {
            None
        }
        fn get_transform_receipt(&self, _: &str) -> Option<l64_core::FormatTransformReceipt> {
            None
        }
        fn get_surface_deficiency(&self, _: &str) -> Option<l64_core::SurfaceDeficiency> {
            None
        }
        fn get_policy_object(&self, _: &str) -> Option<MechanizationPolicyObject> {
            None
        }
        fn policy_objects(&self) -> Vec<MechanizationPolicyObject> {
            Vec::new()
        }
        fn policy_bindings(&self) -> Vec<PolicyBinding> {
            Vec::new()
        }
        fn find_equivalence_class(&self, _: &str, _: &str) -> Option<l64_core::EquivalenceClass> {
            None
        }
        fn atlas_cells(&self) -> Vec<l64_core::AtlasCell> {
            Vec::new()
        }
    }

    #[test]
    fn resolves_builtin_policy_graph() {
        let resolved = resolve_policy_graph(
            &EmptyRegistry,
            Some("BND_LOCAL"),
            None,
            None,
            None,
            false,
            OptimizerPolicy::Conservative,
        )
        .unwrap();
        assert_eq!(resolved.resolution.verdict, PolicyVerdict::Applied);
        assert!(!resolved.resolution.applied_policy_ids.is_empty());
    }
}
