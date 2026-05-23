use l64_atlas::CompiledAtlas;
use l64_core::{
    ArtifactOrigin, AtlasDeficiencyClass, CalibrationPressureMap, CandidateAdequacyClause,
    CandidateAtlasCell, CandidateCampaign, CandidateCheckerExtension, CandidatePayoffTask,
    CandidateSemanticLeaf, CertificationCandidate, CertificationReport, CertificationVerdict,
    CheckProofShape, CheckerReceipt, CheckerReceiptKind, CoverageDecision,
    DeterministicExecutionEnvelope, DistressVector, EvidenceExactness, EvidencePreference,
    ExecutionClosureReceipt, Frontier, FrontierLedger, GeneratedStatus, GenerationReceipt,
    GeneratorContract, GenomeArtifactClass, GenomeSurface, HelpRequest, LocusCapabilityMask,
    LocusOpcode, LocusPacket, LocusPacketHeader, LocusPacketKind, LocusSection, Obligation,
    ObligationCacheShard, ObligationCollisionReport, ObligationConcurrencyClass, ObligationDagEdge,
    ObligationDagNode, ObligationEvaluationMode, ObligationEvidenceReceipt, ObligationGroup,
    ObligationKind, ObligationLaneRecord, ObligationMergeReceipt, ObligationNamespaceReceipt,
    ObligationOrderingReceipt, ObligationPlan, ObligationStatus, ObligationWriteSet,
    OptimizerPolicy, OverridePressureReceipt, PromotionCandidate, ProofCoverageDispatch,
    ProofCoverageEnvelope, ProposalKind, RecipeDelta, RecipeRecord, RegistryLookup,
    ReplayBarrierReceipt, ReplayDivergenceRecord, ReplayLegalityCheck, ReplayMergeReceipt,
    ReplayStatus, RequiredProofShapeFamily, ResidualVerificationReceipt, ReuseDecisionReceipt,
    ReuseLegalityReceipt, RouteScoreVector, SearchCompartment, UnsupportedHandlingMode,
    VerticalCompoundingBundle, ensure_cache_subdir,
};
use l64_kernel::ConstitutionKernel;
use l64_locus::{decode_section_payload, decode_summary as decode_locus_summary};
use l64_policy::resolve_policy_graph;
use l64_runtime::{
    HostExecutionResult, exec_ch_inh_type_witness, exec_ch_norm_type_witness,
    exec_chain_rule_jet_compose, exec_chain_rule_reduction, exec_host,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    fs,
    hash::{Hash, Hasher},
    path::PathBuf,
    sync::mpsc,
    thread,
};
use thiserror::Error;

const EVALUATOR_VERSION: &str = "l64-cert-obl-v8";

#[derive(Debug, Error)]
pub enum CertError {
    #[error("unknown theorem `{0}`")]
    UnknownTheorem(String),
    #[error("unknown campaign `{0}`")]
    UnknownCampaign(String),
    #[error("unknown target profile `{0}`")]
    UnknownTargetProfile(String),
    #[error("replay-only requested but no cached report exists")]
    ReplayMiss,
    #[error("{0}")]
    Message(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CertificationOptions {
    pub optimizer_policy: OptimizerPolicy,
    pub bundle_hash: String,
    pub policy_hash: String,
    pub bundle_id: Option<String>,
    pub evaluator_policy: Option<String>,
    pub cache_policy: Option<String>,
    pub no_cache: bool,
    pub replay_only: bool,
    pub strict_derived: bool,
    pub strict_policy: bool,
    pub force_parallel_obligations: bool,
    pub max_obligation_workers: Option<usize>,
}

impl Default for CertificationOptions {
    fn default() -> Self {
        Self {
            optimizer_policy: OptimizerPolicy::Conservative,
            bundle_hash: "seed".into(),
            policy_hash: "default".into(),
            bundle_id: None,
            evaluator_policy: None,
            cache_policy: None,
            no_cache: false,
            replay_only: false,
            strict_derived: false,
            strict_policy: false,
            force_parallel_obligations: false,
            max_obligation_workers: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CachedCertificationReport {
    pub cache_key: String,
    pub theorem_id: String,
    pub campaign_id: Option<String>,
    pub target_profile_id: String,
    pub bundle_hash: String,
    pub policy_hash: String,
    pub route_winner_hash: String,
    pub evaluator_version: String,
    pub report: CertificationReport,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ExecutionCache {
    pub reports: Vec<CachedCertificationReport>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct CachedExecutionPacketHeader {
    pub cache_key: String,
    pub theorem_id: String,
    pub campaign_id: Option<String>,
    pub target_profile_id: String,
    pub bundle_hash: String,
    pub policy_hash: String,
    pub route_winner_hash: String,
    pub evaluator_version: String,
    pub report_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct CacheStats {
    pub report_count: usize,
    pub theorem_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InvalidationExplanation {
    pub report_id: String,
    pub valid: bool,
    pub reasons: Vec<String>,
}

pub trait ObligationEvaluator {
    fn supports(&self, regime: &str, kind: &ObligationKind) -> bool;
    fn evaluate(&self, regime: &str, obligation: &Obligation) -> Option<ObligationStatus>;
    fn version(&self) -> &'static str;
}

#[derive(Debug, Default)]
pub struct SeedObligationEvaluator;

impl ObligationEvaluator for SeedObligationEvaluator {
    fn supports(&self, regime: &str, kind: &ObligationKind) -> bool {
        matches!(
            (regime, kind),
            (
                "R_SET",
                ObligationKind::OblEq
                    | ObligationKind::OblFin
                    | ObligationKind::OblAdm
                    | ObligationKind::OblKnt
            ) | (
                "R_ALG",
                ObligationKind::OblAdm | ObligationKind::OblKnt | ObligationKind::OblEq
            ) | ("R_TOP", ObligationKind::OblLoc | ObligationKind::OblGlu)
                | (
                    "R_CALC",
                    ObligationKind::OblAdm | ObligationKind::OblTol | ObligationKind::OblRed
                )
                | (
                    "R_PROB",
                    ObligationKind::OblTol | ObligationKind::OblObs | ObligationKind::OblAde
                )
                | (
                    "R_COMP",
                    ObligationKind::OblRed | ObligationKind::OblObs | ObligationKind::OblTol
                )
                | ("R_LOG", ObligationKind::OblAde)
                | ("R_TYP", ObligationKind::OblRed | ObligationKind::OblAde)
        )
    }

    fn evaluate(&self, regime: &str, obligation: &Obligation) -> Option<ObligationStatus> {
        let result = exec_host(regime, None).ok()?;
        obligation_from_result(regime, obligation.clone(), result)
    }

    fn version(&self) -> &'static str {
        EVALUATOR_VERSION
    }
}

#[derive(Debug, Clone)]
struct ObligationExecutionArtifacts {
    statuses: Vec<ObligationStatus>,
    plan: ObligationPlan,
    lanes: Vec<ObligationLaneRecord>,
    ordering_receipt: ObligationOrderingReceipt,
    merge_receipt: ObligationMergeReceipt,
    replay_legality_checks: Vec<ReplayLegalityCheck>,
    replay_barrier_receipts: Vec<ReplayBarrierReceipt>,
    replay_merge_receipt: ReplayMergeReceipt,
    replay_divergence_records: Vec<ReplayDivergenceRecord>,
    cache_shards: Vec<ObligationCacheShard>,
    write_sets: Vec<ObligationWriteSet>,
    collision_reports: Vec<ObligationCollisionReport>,
    namespace_receipt: ObligationNamespaceReceipt,
    notes: Vec<String>,
}

pub fn certify_derived_campaign(
    registry: &(impl RegistryLookup + Sync),
    atlas: &CompiledAtlas,
    campaign_id: &str,
) -> Result<CertificationReport, CertError> {
    certify_derived_campaign_with_options(
        registry,
        atlas,
        campaign_id,
        &CertificationOptions::default(),
    )
}

pub fn certify_derived_campaign_with_options(
    registry: &(impl RegistryLookup + Sync),
    atlas: &CompiledAtlas,
    campaign_id: &str,
    options: &CertificationOptions,
) -> Result<CertificationReport, CertError> {
    let campaign = registry
        .get_campaign(campaign_id)
        .ok_or_else(|| CertError::UnknownCampaign(campaign_id.to_string()))?;
    certify_derived_theorem_with_options(
        registry,
        atlas,
        &campaign.theorem,
        &campaign.target_profile,
        Some(&campaign.id),
        options,
    )
}

pub fn certify_derived_theorem(
    registry: &(impl RegistryLookup + Sync),
    atlas: &CompiledAtlas,
    theorem_id: &str,
    target_profile_id: &str,
    campaign_id: Option<&str>,
) -> Result<CertificationReport, CertError> {
    certify_derived_theorem_with_options(
        registry,
        atlas,
        theorem_id,
        target_profile_id,
        campaign_id,
        &CertificationOptions::default(),
    )
}

pub fn certify_derived_theorem_with_options(
    registry: &(impl RegistryLookup + Sync),
    atlas: &CompiledAtlas,
    theorem_id: &str,
    target_profile_id: &str,
    campaign_id: Option<&str>,
    options: &CertificationOptions,
) -> Result<CertificationReport, CertError> {
    let theorem = registry
        .get_theorem_spec(theorem_id)
        .ok_or_else(|| CertError::UnknownTheorem(theorem_id.to_string()))?;
    let target = registry
        .get_target_profile(target_profile_id)
        .ok_or_else(|| CertError::UnknownTargetProfile(target_profile_id.to_string()))?;
    let mut resolved_policy = resolve_policy_graph(
        registry,
        options.bundle_id.as_deref(),
        Some(&theorem.id),
        campaign_id,
        Some(target_profile_id),
        options.strict_policy,
        target
            .optimizer_policy
            .clone()
            .unwrap_or_else(|| options.optimizer_policy.clone()),
    )
    .map_err(|err| CertError::Message(err.to_string()))?;
    if let Some(policy_id) = &options.evaluator_policy {
        let policy = registry
            .get_policy_object(policy_id)
            .ok_or_else(|| CertError::Message(format!("unknown evaluator policy `{policy_id}`")))?;
        let evaluator = policy.evaluator.ok_or_else(|| {
            CertError::Message(format!(
                "policy `{policy_id}` does not carry evaluator config"
            ))
        })?;
        resolved_policy
            .resolution
            .applied_policy_ids
            .push(policy_id.clone());
        resolved_policy.resolution.evaluator = evaluator;
        resolved_policy
            .resolution
            .trace
            .steps
            .push(format!("cli-selected evaluator policy {policy_id}"));
    }
    if let Some(policy_id) = &options.cache_policy {
        let policy = registry
            .get_policy_object(policy_id)
            .ok_or_else(|| CertError::Message(format!("unknown cache policy `{policy_id}`")))?;
        let replay_cache = policy.replay_cache.ok_or_else(|| {
            CertError::Message(format!(
                "policy `{policy_id}` does not carry replay/cache config"
            ))
        })?;
        resolved_policy
            .resolution
            .applied_policy_ids
            .push(policy_id.clone());
        resolved_policy.resolution.replay_cache = replay_cache;
        resolved_policy
            .resolution
            .trace
            .steps
            .push(format!("cli-selected cache policy {policy_id}"));
    }
    let optimizer_policy = resolved_policy
        .resolution
        .optimizer
        .optimizer_policy
        .clone();
    let effective_policy_hash =
        stable_hash(&serde_json::to_string(&resolved_policy.resolution).unwrap_or_default());
    let cache_key = certification_cache_key(
        &theorem.id,
        target_profile_id,
        campaign_id,
        &options.bundle_hash,
        &effective_policy_hash,
        &optimizer_policy,
    );
    if !options.no_cache && resolved_policy.resolution.replay_cache.replay_allowed {
        if let Some(mut cached) = load_cached_report(&cache_key)? {
            if options.replay_only
                || cached.cache_key == cache_key
                || cache_entry_valid(
                    &cached,
                    options,
                    &effective_policy_hash,
                    Some(&resolved_policy.resolution),
                )
            {
                cached.report.execution_envelope =
                    cached.report.execution_envelope.as_ref().map(|env| {
                        let mut env = env.clone();
                        env.replay_status = if options.replay_only {
                            ReplayStatus::ReplayOnly
                        } else {
                            ReplayStatus::CacheHit
                        };
                        env
                    });
                return Ok(cached.report);
            }
        } else if options.replay_only {
            return Err(CertError::ReplayMiss);
        }
    } else if options.replay_only {
        return Err(CertError::ReplayMiss);
    }

    let kernel = ConstitutionKernel;
    kernel
        .validate_theorem_spec(&theorem, registry)
        .map_err(|err| CertError::Message(err.to_string()))?;
    kernel
        .validate_target_profile(&target)
        .map_err(|err| CertError::Message(err.to_string()))?;

    let route = atlas
        .select_policy_driven(
            theorem
                .hosts
                .first()
                .ok_or_else(|| CertError::Message("missing theorem source host".into()))?,
            theorem
                .hosts
                .last()
                .ok_or_else(|| CertError::Message("missing theorem target host".into()))?,
            Some(&target.burden_class),
            Some(&l64_core::Budget {
                max_loss: target.loss_ceiling,
                allow_lossy_supported: target
                    .allowed_bridge_classes
                    .contains(&l64_core::ReversibilityClass::LossySupported),
                require_proof: true,
            }),
            target.surface_requirement.as_ref(),
            target.preferred_surface_target.as_ref(),
            Some(&resolved_policy.resolution),
            true,
        )
        .map_err(|err| CertError::Message(err.to_string()))?;

    let winner = route
        .winner
        .clone()
        .ok_or_else(|| CertError::Message("no winner selected".into()))?;
    let edge = atlas
        .edge_by_atlas_cell(&winner.id)
        .ok_or_else(|| CertError::Message("compiled winner edge missing".into()))?;
    let proof_family_ok = proof_family_ok(
        &target.required_proof_shape_family,
        &edge.proof_shapes,
        registry,
    );
    let evaluator = SeedObligationEvaluator;
    let obligation_artifacts = campaign_id
        .and_then(|id| registry.get_campaign(id))
        .map(|campaign| {
            let obligations = campaign
                .obligations
                .into_iter()
                .filter_map(|id| registry.get_obligation(&id))
                .collect::<Vec<_>>();
            evaluate_obligations(
                registry,
                &theorem.id,
                campaign_id,
                &theorem.hosts,
                obligations,
                &evaluator,
                &resolved_policy.resolution,
                options,
            )
        })
        .transpose()?
        .unwrap_or_else(|| ObligationExecutionArtifacts {
            statuses: Vec::new(),
            plan: ObligationPlan {
                id: format!("OPL_{}", stable_hash(&theorem.id)),
                theorem_id: theorem.id.clone(),
                campaign_id: campaign_id.map(ToString::to_string),
                nodes: Vec::new(),
                edges: Vec::new(),
                groups: Vec::new(),
                notes: vec!["no campaign obligations were declared".into()],
            },
            lanes: Vec::new(),
            ordering_receipt: ObligationOrderingReceipt {
                id: format!("OORD_{}", stable_hash(&theorem.id)),
                ordered_group_ids: Vec::new(),
                notes: vec!["no obligation groups were scheduled".into()],
            },
            merge_receipt: ObligationMergeReceipt {
                id: format!("OMER_{}", stable_hash(&theorem.id)),
                merged_obligation_ids: Vec::new(),
                notes: vec!["no obligation merge required".into()],
            },
            replay_legality_checks: Vec::new(),
            replay_barrier_receipts: Vec::new(),
            replay_merge_receipt: ReplayMergeReceipt {
                id: format!("RMR_{}", stable_hash(&theorem.id)),
                reused_obligation_ids: Vec::new(),
                rerun_obligation_ids: Vec::new(),
                notes: vec!["no obligation replay merge required".into()],
            },
            replay_divergence_records: Vec::new(),
            cache_shards: Vec::new(),
            write_sets: Vec::new(),
            collision_reports: Vec::new(),
            namespace_receipt: ObligationNamespaceReceipt {
                id: format!("ONR_{}", stable_hash(&theorem.id)),
                namespace_id: "default".into(),
                shard_ids: Vec::new(),
                notes: vec!["no obligation namespace activity".into()],
            },
            notes: Vec::new(),
        });
    let obligations = obligation_artifacts.statuses.clone();

    if options.strict_derived
        && obligations.iter().any(|item| {
            matches!(
                item.evaluation_mode,
                ObligationEvaluationMode::StoredReceiptUsed | ObligationEvaluationMode::Unsupported
            )
        })
    {
        return Err(CertError::Message(
            "strict-derived rejected stored/unsupported obligations".into(),
        ));
    }

    let winner_vector = route
        .route_explanation
        .as_ref()
        .and_then(|item| item.winner_score.clone());
    let candidates = vec![CertificationCandidate {
        atlas_cell_id: winner.id.clone(),
        path: edge.path.clone(),
        loss_count: edge.loss_count,
        proof_shapes: edge.proof_shapes.clone(),
        route_class_id: registry
            .get_route_class(&format!("RTC_{}", theorem.id.trim_start_matches("THS_")))
            .map(|item| item.id),
        score: winner_vector
            .as_ref()
            .map(score_vector_to_legacy)
            .unwrap_or_else(|| {
                vec![
                    edge.loss_count,
                    edge.path.len(),
                    usize::from(!proof_family_ok),
                ]
            }),
        route_score: winner_vector.clone(),
    }];

    let certificate =
        registry.get_certificate(&format!("CRT_{}", theorem.id.trim_start_matches("THS_")));
    let mut verdict = if let Some(certificate) = &certificate {
        certificate.verdict.clone()
    } else if proof_family_ok {
        CertificationVerdict::RouteFound
    } else {
        CertificationVerdict::Underspecified
    };
    if obligations
        .iter()
        .any(|item| item.verdict == CertificationVerdict::BlockedContradiction)
    {
        verdict = CertificationVerdict::BlockedContradiction;
    } else if obligations
        .iter()
        .any(|item| item.verdict == CertificationVerdict::BlockedOpen)
    {
        verdict = CertificationVerdict::BlockedOpen;
    } else if obligations
        .iter()
        .any(|item| item.verdict == CertificationVerdict::Underspecified)
    {
        verdict = CertificationVerdict::Underspecified;
    } else if resolved_policy.resolution.evaluator.unsupported_mode
        == UnsupportedHandlingMode::StrictFail
        && obligations
            .iter()
            .any(|item| item.evaluation_mode == ObligationEvaluationMode::Unsupported)
    {
        verdict = CertificationVerdict::BlockedOpen;
    }
    let adequacy_records = evaluate_active_adequacy_clauses(
        registry,
        &theorem.id,
        Some(&edge.atlas_cell_id),
        &edge.path,
        &obligations,
    );
    let checker_receipts = collect_checker_receipts(
        registry,
        &theorem,
        &target,
        &edge.proof_shapes,
        campaign_id,
        &obligations,
    );
    let deficiencies = collect_campaign_deficiencies(
        registry,
        &theorem.id,
        campaign_id,
        Some(&edge.atlas_cell_id),
        &obligations,
        &adequacy_records,
        proof_family_ok,
    );
    if deficiency_blocks_campaign(&deficiencies)
        && !matches!(
            verdict,
            CertificationVerdict::BlockedOpen | CertificationVerdict::BlockedContradiction
        )
    {
        verdict = if deficiencies
            .iter()
            .any(|item| item.class == l64_core::AtlasDeficiencyClass::DOpenConjectural)
        {
            CertificationVerdict::Underspecified
        } else {
            CertificationVerdict::BlockedOpen
        };
    }
    if proof_family_ok
        && deficiencies.is_empty()
        && obligations
            .iter()
            .all(|item| item.verdict == CertificationVerdict::Certified)
        && matches!(
            verdict,
            CertificationVerdict::Benchmarked | CertificationVerdict::RouteFound
        )
    {
        verdict = CertificationVerdict::Certified;
    }
    let promotion_artifact_ids = collect_promotion_artifact_ids(
        &theorem,
        &target,
        &obligations,
        &adequacy_records,
        &deficiencies,
        &verdict,
    );
    let reused_artifact_ids = collect_reused_artifact_ids(registry, &theorem.id, &obligations);
    let default_selected_artifact_ids =
        collect_default_selected_artifact_ids(registry, &theorem.id, &obligations);
    let payoff_receipt_ids =
        collect_payoff_receipt_ids(&theorem.id, &obligations, &default_selected_artifact_ids);
    if verdict == CertificationVerdict::Certified && !reused_artifact_ids.is_empty() {
        verdict = CertificationVerdict::Integrated;
    }

    let route_winner_hash = stable_hash(&format!("{}::{}", winner.id, edge.path.join("->")));
    let mut reasons = vec![format!(
        "derived certification from compiled atlas using {:?}",
        optimizer_policy
    )];
    if let Some(reason) = flagship_selection_reason(&theorem.id, campaign_id) {
        reasons.push(reason);
    }
    let mut report = CertificationReport {
        theorem_id: theorem.id.clone(),
        campaign_id: campaign_id.map(ToString::to_string),
        target_profile_id: target.id.clone(),
        verdict,
        selected_atlas_cell: Some(winner.id),
        selected_path: edge.path.clone(),
        route_class_id: registry
            .get_route_class(&format!("RTC_{}", theorem.id.trim_start_matches("THS_")))
            .map(|item| item.id),
        certificate_id: certificate.map(|item| item.id),
        candidates,
        obligations,
        reasons,
        diagnostics: if proof_family_ok {
            Vec::new()
        } else {
            vec!["required proof-shape family not satisfied".into()]
        },
        deficiencies,
        adequacy_records,
        checker_receipts,
        burden_pack_ids: collect_related_burden_pack_ids(registry, campaign_id),
        claim_packet_ids: collect_related_claim_packet_ids(registry, campaign_id),
        evidence_contract_ids: collect_related_evidence_contract_ids(registry, campaign_id),
        benchmark_receipt_ids: collect_related_benchmark_receipt_ids(registry, campaign_id),
        challenge_receipt_ids: collect_related_challenge_receipt_ids(registry, campaign_id),
        reproducibility_packet_ids: collect_related_reproducibility_packet_ids(
            registry,
            campaign_id,
        ),
        promotion_artifact_ids,
        reused_artifact_ids,
        default_selected_artifact_ids,
        payoff_receipt_ids,
        policy_resolution: Some(resolved_policy.resolution.clone()),
        route_explanation: route.route_explanation.clone(),
        execution_envelope: None,
        reconciliation_summary: Vec::new(),
        obligation_plan: Some(obligation_artifacts.plan.clone()),
        obligation_lanes: obligation_artifacts.lanes.clone(),
        obligation_ordering_receipt: Some(obligation_artifacts.ordering_receipt.clone()),
        obligation_merge_receipt: Some(obligation_artifacts.merge_receipt.clone()),
        replay_legality_checks: obligation_artifacts.replay_legality_checks.clone(),
        replay_barrier_receipts: obligation_artifacts.replay_barrier_receipts.clone(),
        replay_merge_receipt: Some(obligation_artifacts.replay_merge_receipt.clone()),
        replay_divergence_records: obligation_artifacts.replay_divergence_records.clone(),
        obligation_cache_shards: obligation_artifacts.cache_shards.clone(),
        reuse_legality_receipts: Vec::new(),
        reuse_decision_receipts: Vec::new(),
        residual_verification_receipts: Vec::new(),
        obligation_write_sets: obligation_artifacts.write_sets.clone(),
        obligation_collision_reports: obligation_artifacts.collision_reports.clone(),
        obligation_namespace_receipt: Some(obligation_artifacts.namespace_receipt.clone()),
    };
    let coverage = derive_proof_coverage_envelope(&report);
    report.reuse_legality_receipts = derive_reuse_legality_receipts(&report);
    report.reuse_decision_receipts = derive_reuse_decision_receipts(&report, &coverage);
    report.residual_verification_receipts =
        derive_residual_verification_receipts(&report, &coverage);
    let obligation_replay_keys = report
        .obligations
        .iter()
        .map(|item| {
            stable_hash(&format!(
                "{}::{:?}::{:?}",
                item.obligation_id, item.kind, item.evaluation_mode
            ))
        })
        .collect::<Vec<_>>();
    let report_hash = stable_hash(&format!(
        "{}|{}|{:?}|{}|{}|{}|{}|{}|{}",
        report.theorem_id,
        report
            .campaign_id
            .clone()
            .unwrap_or_else(|| "THEOREM".into()),
        report.verdict,
        report.selected_atlas_cell.clone().unwrap_or_default(),
        route_winner_hash,
        report
            .obligations
            .iter()
            .map(|item| format!(
                "{}:{:?}:{:?}",
                item.obligation_id, item.verdict, item.evaluation_mode
            ))
            .collect::<Vec<_>>()
            .join("|"),
        report
            .deficiencies
            .iter()
            .map(|item| item.id.as_str())
            .collect::<Vec<_>>()
            .join("|"),
        report
            .adequacy_records
            .iter()
            .map(|item| format!("{}:{:?}", item.clause_id, item.verdict))
            .collect::<Vec<_>>()
            .join("|"),
        report
            .checker_receipts
            .iter()
            .map(|item| format!("{}:{:?}", item.subject_id, item.verdict))
            .collect::<Vec<_>>()
            .join("|")
    ));
    report.execution_envelope = Some(DeterministicExecutionEnvelope {
        bundle_hash: options.bundle_hash.clone(),
        bundle_id: options.bundle_id.clone(),
        policy_hash: effective_policy_hash.clone(),
        policy_resolution_id: Some(resolved_policy.resolution.id.clone()),
        manifest_id: None,
        lock_id: None,
        route_winner_hash: route_winner_hash.clone(),
        obligation_replay_keys,
        report_hash: report_hash.clone(),
        replay_status: ReplayStatus::Fresh,
        executed_plan_id: None,
        reconciliation_id: None,
    });
    report.diagnostics.extend(obligation_artifacts.notes);
    if let Some(eq_status) = report
        .obligations
        .iter()
        .find(|item| item.obligation_id == "OBL_CHAIN_EQ")
    {
        if !eq_status.receipts.is_empty() {
            report.diagnostics.push(format!(
                "chain-rule equivalence receipts: {}",
                eq_status
                    .receipts
                    .iter()
                    .map(|item| receipt_leaf_counts(item).0 + receipt_leaf_counts(item).1)
                    .sum::<usize>()
            ));
        }
        if eq_status.evaluation_mode == ObligationEvaluationMode::RecomputedExact
            && eq_status.verdict == CertificationVerdict::Certified
        {
            report.diagnostics.push(
                "chain-rule equivalence fully discharged through executable jet transport".into(),
            );
            report
                .reasons
                .push("runtime discharged the Chain Rule equivalence witness strongly enough to lift the seeded benchmark certificate".into());
        }
    }
    if let Some(red_status) = report
        .obligations
        .iter()
        .find(|item| item.obligation_id == "OBL_CHAIN_RED")
    {
        if !red_status.receipts.is_empty() {
            report.diagnostics.push(format!(
                "chain-rule reduction receipts: {}",
                red_status
                    .receipts
                    .iter()
                    .map(|item| receipt_leaf_counts(item).0 + receipt_leaf_counts(item).1)
                    .sum::<usize>()
            ));
        }
        if red_status.evaluation_mode == ObligationEvaluationMode::RecomputedExact
            && red_status.verdict == CertificationVerdict::Certified
        {
            report
                .reasons
                .push("runtime discharged the Chain Rule reduction witness exactly on the active finite route-local model".into());
        }
    }
    if !report.promotion_artifact_ids.is_empty() {
        report.diagnostics.push(format!(
            "promotion artifacts realized: {}",
            report.promotion_artifact_ids.join(",")
        ));
    }
    if !report.reused_artifact_ids.is_empty() {
        report.diagnostics.push(format!(
            "reused integrated artifacts: {}",
            report.reused_artifact_ids.join(",")
        ));
        report
            .reasons
            .push("reused promoted Chain₁ operator to reduce future certification work on the active derivative-composition burden".into());
    }
    if !report.default_selected_artifact_ids.is_empty() {
        report.diagnostics.push(format!(
            "default-selected integrated artifacts: {}",
            report.default_selected_artifact_ids.join(",")
        ));
        report
            .reasons
            .push("selector/certifier default-selected the canonical Chain₁ operator for the active derivative-composition burden".into());
    }
    if !report.payoff_receipt_ids.is_empty() {
        report.diagnostics.push(format!(
            "operator payoff receipts: {}",
            report.payoff_receipt_ids.join(",")
        ));
        report
            .reasons
            .push(payoff_reason(&theorem.id, &report.payoff_receipt_ids));
    }
    if !report.checker_receipts.is_empty() {
        report.diagnostics.push(format!(
            "checker receipts: {}",
            report.checker_receipts.len()
        ));
    }
    if !report.adequacy_records.is_empty() {
        report.diagnostics.push(format!(
            "active adequacy clauses: {}",
            report.adequacy_records.len()
        ));
        if report
            .adequacy_records
            .iter()
            .all(|item| item.verdict == CertificationVerdict::Certified)
        {
            report.reasons.push(adequacy_cluster_reason(&theorem.id));
        }
    }

    if !options.no_cache && resolved_policy.resolution.replay_cache.replay_allowed {
        persist_cached_report(CachedCertificationReport {
            cache_key,
            theorem_id: theorem.id.clone(),
            campaign_id: campaign_id.map(ToString::to_string),
            target_profile_id: target.id,
            bundle_hash: options.bundle_hash.clone(),
            policy_hash: effective_policy_hash,
            route_winner_hash,
            evaluator_version: evaluator.version().into(),
            report: report.clone(),
        })?;
    }

    Ok(report)
}

pub fn replay_report(report_id: &str) -> Result<CertificationReport, CertError> {
    let cache = load_execution_cache().map_err(|err| CertError::Message(err.to_string()))?;
    cache
        .reports
        .into_iter()
        .find(|item| report_storage_id(&item.report) == report_id)
        .map(|item| item.report)
        .ok_or(CertError::ReplayMiss)
}

pub fn cache_stats() -> Result<CacheStats, CertError> {
    let cache = load_execution_cache().map_err(|err| CertError::Message(err.to_string()))?;
    Ok(CacheStats {
        report_count: cache.reports.len(),
        theorem_ids: cache
            .reports
            .into_iter()
            .map(|item| item.theorem_id)
            .collect(),
    })
}

pub fn clear_cache(scope: Option<&str>) -> Result<(), CertError> {
    let path = execution_cache_path().map_err(|err| CertError::Message(err.to_string()))?;
    let legacy =
        execution_cache_legacy_path().map_err(|err| CertError::Message(err.to_string()))?;
    match scope {
        Some("all") | None | Some("reports") | Some("obligations") => {
            if path.exists() {
                fs::remove_dir_all(&path).map_err(|err| CertError::Message(err.to_string()))?;
            }
            if legacy.exists() {
                fs::remove_file(&legacy).map_err(|err| CertError::Message(err.to_string()))?;
            }
        }
        Some(_) => {}
    }
    Ok(())
}

pub fn explain_invalidation(
    report_id: &str,
    bundle_hash: &str,
    policy_hash: &str,
) -> Result<InvalidationExplanation, CertError> {
    let cache = load_execution_cache().map_err(|err| CertError::Message(err.to_string()))?;
    let Some(cached) = cache
        .reports
        .into_iter()
        .find(|item| report_storage_id(&item.report) == report_id)
    else {
        return Ok(InvalidationExplanation {
            report_id: report_id.into(),
            valid: false,
            reasons: vec!["report not found in execution cache".into()],
        });
    };
    let mut reasons = Vec::new();
    if cached.bundle_hash != bundle_hash {
        reasons.push("bundle hash changed".into());
    }
    if cached.policy_hash != policy_hash {
        let exact_required = cached
            .report
            .policy_resolution
            .as_ref()
            .map(|item| item.replay_cache.exact_policy_match_required)
            .unwrap_or(true);
        if exact_required {
            reasons.push("policy hash changed and replay policy requires exact match".into());
        } else {
            reasons.push("policy hash changed but replay policy may permit reuse".into());
        }
    }
    if cached.evaluator_version != EVALUATOR_VERSION {
        reasons.push("evaluator version changed".into());
    }
    Ok(InvalidationExplanation {
        report_id: report_id.into(),
        valid: reasons.is_empty(),
        reasons,
    })
}

fn proof_family_ok(
    family: &RequiredProofShapeFamily,
    proof_shapes: &[String],
    registry: &(impl RegistryLookup + Sync),
) -> bool {
    match family {
        RequiredProofShapeFamily::Minimal | RequiredProofShapeFamily::MixedBattery => {
            !proof_shapes.is_empty()
        }
        RequiredProofShapeFamily::Triangle => proof_shapes.iter().any(|id| {
            registry
                .get_proof_shape(id)
                .map(|shape| shape.kind == l64_core::ProofShapeKind::Triangle)
                .unwrap_or(false)
        }),
        RequiredProofShapeFamily::Square => proof_shapes.iter().any(|id| {
            registry
                .get_proof_shape(id)
                .map(|shape| shape.kind == l64_core::ProofShapeKind::Square)
                .unwrap_or(false)
        }),
        RequiredProofShapeFamily::Diamond => proof_shapes.iter().any(|id| {
            registry
                .get_proof_shape(id)
                .map(|shape| shape.kind == l64_core::ProofShapeKind::Diamond)
                .unwrap_or(false)
        }),
        RequiredProofShapeFamily::Pentagon => proof_shapes.iter().any(|id| {
            registry
                .get_proof_shape(id)
                .map(|shape| shape.kind == l64_core::ProofShapeKind::Pentagon)
                .unwrap_or(false)
        }),
        RequiredProofShapeFamily::Hexagon => proof_shapes.iter().any(|id| {
            registry
                .get_proof_shape(id)
                .map(|shape| shape.kind == l64_core::ProofShapeKind::Hexagon)
                .unwrap_or(false)
        }),
    }
}

fn flagship_selection_reason(theorem_id: &str, campaign_id: Option<&str>) -> Option<String> {
    match (theorem_id, campaign_id) {
        ("THS_CHAIN_RULE", Some("CPG_CHAIN_RULE")) => Some(
            "flagship adequacy-first campaign selected: Chain Rule maximizes bridge reuse on the seeded R_TOP→R_CALC atlas while keeping proof-shape novelty low".into(),
        ),
        ("THS_BAYES_BRACE", Some("CPG_BAYES_BRACE")) => Some(
            "next flagship selected: BayesBrace reuses the active adequacy/control machinery on the seeded R_TOP→R_PROB atlas with the lowest host-law novelty beyond the Chain Rule cluster".into(),
        ),
        ("THS_CH_NORM", Some("CPG_CH_NORM")) => Some(
            "next flagship selected: CH_Norm is the lowest-friction logic/type campaign because the seeded R_TYP→R_SET atlas and current runtime already support its first normalization witnesses".into(),
        ),
        ("THS_EXEC_INFER", Some("CPG_EXEC_INFER")) => Some(
            "cheap seeded closure selected: ExecInfer is the lowest-friction A_PROB→COMP campaign because the existing probability and computation kernels already discharge its executable realization obligations".into(),
        ),
        ("THS_PROB_JUDG", Some("CPG_PROB_JUDG")) => Some(
            "cheap seeded closure selected: ProbJudg reuses the active probability adequacy layer on the seeded A_PROB→LOG atlas without introducing new host-law machinery".into(),
        ),
        ("THS_CERT_PROP", Some("CPG_CERT_PROP")) => Some(
            "cheap seeded closure selected: CertProp is the lowest-friction A_COMP→LOG campaign because the computation kernel already exposes the certified property witnesses its route requires".into(),
        ),
        ("THS_CH_INH", Some("CPG_CH_INH")) => Some(
            "cheap seeded closure selected: CH-Inh is the next bounded type/algebra campaign because the seeded A_TYPE→ALG atlas already exists even though its exact inhabitance semantics are still colder".into(),
        ),
        _ => None,
    }
}

fn collect_campaign_deficiencies(
    registry: &(impl RegistryLookup + Sync),
    theorem_id: &str,
    campaign_id: Option<&str>,
    atlas_cell_id: Option<&str>,
    obligations: &[ObligationStatus],
    adequacy_records: &[l64_core::AdequacyRecord],
    proof_family_ok: bool,
) -> Vec<l64_core::AtlasDeficiency> {
    let chain_rule_eq_closed = theorem_id == "THS_CHAIN_RULE"
        && obligations.iter().any(|item| {
            item.obligation_id == "OBL_CHAIN_EQ"
                && item.evaluation_mode == ObligationEvaluationMode::RecomputedExact
                && item.verdict == CertificationVerdict::Certified
        });
    let mut deficiencies = registry
        .atlas_deficiencies()
        .into_iter()
        .filter(|item| item.theorem.as_deref() == Some(theorem_id))
        .filter(|item| atlas_cell_id.is_none() || item.atlas_cell.as_deref() == atlas_cell_id)
        .filter(|item| !(chain_rule_eq_closed && item.id == "DGN_CHAIN_RULE_ADEQUACY"))
        .collect::<Vec<_>>();

    if !proof_family_ok {
        deficiencies.push(l64_core::AtlasDeficiency {
            id: format!("DGN_{}_PROOF_FAMILY", theorem_id),
            class: l64_core::AtlasDeficiencyClass::DNoCommutingProof,
            atlas_cell: atlas_cell_id.map(ToString::to_string),
            theorem: Some(theorem_id.to_string()),
            message: "required proof-shape family not satisfied".into(),
            blocking_scope: l64_core::blocking_scope(true),
            control_effects: l64_core::blocking_control_effects(true),
            suggested_seam: Some(
                "proof-shape family closure is still required for this campaign".into(),
            ),
        });
    }

    for obligation in obligations {
        let receipt_residual = obligation.receipts.iter().any(receipt_has_residual);
        if !receipt_residual
            && (matches!(
                obligation.evaluation_mode,
                ObligationEvaluationMode::Unsupported
                    | ObligationEvaluationMode::StoredReceiptUsed
                    | ObligationEvaluationMode::RecomputedPartial
            ) || matches!(
                obligation.verdict,
                CertificationVerdict::BlockedOpen
                    | CertificationVerdict::BlockedContradiction
                    | CertificationVerdict::Underspecified
            ))
        {
            deficiencies.push(l64_core::obligation_status_deficiency(
                theorem_id,
                atlas_cell_id,
                format!(
                    "DGN_OBL_{}_{}_{}",
                    theorem_id,
                    campaign_id.unwrap_or("THEOREM"),
                    obligation.obligation_id
                ),
                obligation,
                true,
                Some(format!(
                    "obligation {} remains constitutionally open on the active campaign",
                    obligation.obligation_id
                )),
            ));
        }
        for receipt in &obligation.receipts {
            collect_receipt_deficiencies(
                &mut deficiencies,
                theorem_id,
                atlas_cell_id,
                &obligation.obligation_id,
                receipt,
            );
        }
    }

    for record in adequacy_records {
        if record.verdict != CertificationVerdict::Certified {
            deficiencies.push(adequacy_record_deficiency(
                theorem_id,
                atlas_cell_id,
                record,
            ));
        }
    }

    deficiencies.sort_by(|left, right| left.id.cmp(&right.id));
    deficiencies.dedup_by(|left, right| left.id == right.id);
    deficiencies
}

fn evaluate_active_adequacy_clauses(
    registry: &(impl RegistryLookup + Sync),
    theorem_id: &str,
    atlas_cell_id: Option<&str>,
    selected_path: &[String],
    obligations: &[ObligationStatus],
) -> Vec<l64_core::AdequacyRecord> {
    let theorem_scoped_clauses = registry
        .adequacy_clauses()
        .into_iter()
        .filter(|item| {
            item.theorem_ids.is_empty() || item.theorem_ids.iter().any(|id| id == theorem_id)
        })
        .collect::<Vec<_>>();
    if theorem_scoped_clauses.is_empty()
        || (!supports_active_adequacy(theorem_id)
            && theorem_scoped_clauses.iter().all(|clause| {
                !matches!(
                    clause.kind,
                    l64_core::AdequacyClauseKind::ProjectionInterpretation
                        | l64_core::AdequacyClauseKind::ContainmentInterpretation
                        | l64_core::AdequacyClauseKind::ClosureInterpretation
                        | l64_core::AdequacyClauseKind::RunningLawInterpretation
                        | l64_core::AdequacyClauseKind::EvidenceContractInterpretation
                        | l64_core::AdequacyClauseKind::BenchmarkInterpretation
                        | l64_core::AdequacyClauseKind::StressInterpretation
                        | l64_core::AdequacyClauseKind::ChallengeInterpretation
                )
            }))
    {
        return Vec::new();
    }
    let top = exec_host("R_TOP", None).ok();
    let calc = exec_host("R_CALC", None).ok();
    let prob = exec_host("R_PROB", None).ok();
    let _set = exec_host("R_SET", None).ok();
    let typ = exec_host("R_TYP", None).ok();
    let comp = exec_host("R_COMP", None).ok();
    let log = exec_host("R_LOG", None).ok();
    let alg = exec_host("R_ALG", None).ok();
    let ledger_id = format!("TRL_{}", theorem_id.trim_start_matches("THS_"));
    let ledger = registry.get_route_ledger(&ledger_id);

    let mut records = theorem_scoped_clauses
        .into_iter()
        .map(|clause| {
            let (verdict, computed, detail, receipt_ids) = match clause.id.as_str() {
                "ADQ_CHAIN_CALC_OBJECT" => {
                    let (ok, detail) = match &calc {
                        Some(HostExecutionResult::Calculus {
                            local_linear_witness,
                            finite_difference_derivative,
                            ..
                        }) => (
                            *local_linear_witness && *finite_difference_derivative,
                            format!(
                                "local_linear={} finite_difference={}",
                                local_linear_witness, finite_difference_derivative
                            ),
                        ),
                        _ => (false, "missing R_CALC executable witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_CHAIN_CALC_OBJECT_WIT".into()],
                    )
                }
                "ADQ_CHAIN_TOP_THREAD" => {
                    let (ok, detail) = match &top {
                        Some(HostExecutionResult::Topology {
                            continuity_holds,
                            overlap_compatible,
                            ..
                        }) => (
                            *continuity_holds && *overlap_compatible,
                            format!(
                                "continuity={} overlap={}",
                                continuity_holds, overlap_compatible
                            ),
                        ),
                        _ => (false, "missing R_TOP executable witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_CHAIN_TOP_THREAD_WIT".into()],
                    )
                }
                "ADQ_CHAIN_CALC_EQ" => {
                    adequacy_from_obligation(obligations, "OBL_CHAIN_EQ", "ADQ_CHAIN_EQ_RECEIPT")
                }
                "ADQ_CHAIN_CALC_TOLL" => {
                    adequacy_from_obligation(obligations, "OBL_CHAIN_RED", "ADQ_CHAIN_RED_RECEIPT")
                }
                "ADQ_CHAIN_TOP_KNOT" => {
                    let (ok, detail) = match &top {
                        Some(HostExecutionResult::Topology {
                            cover_legal,
                            overlap_compatible,
                            ..
                        }) => (
                            *cover_legal && *overlap_compatible,
                            format!("cover={} overlap={}", cover_legal, overlap_compatible),
                        ),
                        _ => (false, "missing R_TOP knot witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_CHAIN_TOP_KNOT_WIT".into()],
                    )
                }
                "ADQ_CHAIN_TOP_CALC_BRIDGE" => {
                    let has_ref = ledger
                        .as_ref()
                        .map(|item| item.receipts.iter().any(|receipt| receipt == "Ref_1"))
                        .unwrap_or(false);
                    let selected_direct = selected_path == ["B_TOP_TO_CALC".to_string()];
                    let on_cell = atlas_cell_id == Some("A_TOP_TO_CALC");
                    (
                        if has_ref && selected_direct && on_cell {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        format!(
                            "atlas_cell={:?} selected_path={} receipt_ref1={}",
                            atlas_cell_id,
                            selected_path.join("->"),
                            has_ref
                        ),
                        vec!["ADQ_CHAIN_BRIDGE_SOUND_WIT".into()],
                    )
                }
                "ADQ_BAYES_PROB_OBJECT" => {
                    let (ok, detail) = match &prob {
                        Some(HostExecutionResult::Probability {
                            normalized,
                            pushforward_ok,
                            conditioning_legal,
                            ..
                        }) => (
                            *normalized && *pushforward_ok && *conditioning_legal,
                            format!(
                                "normalized={} pushforward={} conditioning={}",
                                normalized, pushforward_ok, conditioning_legal
                            ),
                        ),
                        _ => (false, "missing R_PROB executable witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_BAYES_PROB_OBJECT_WIT".into()],
                    )
                }
                "ADQ_BAYES_TOP_THREAD" => {
                    let (ok, detail) = match &top {
                        Some(HostExecutionResult::Topology {
                            continuity_holds,
                            overlap_compatible,
                            ..
                        }) => (
                            *continuity_holds && *overlap_compatible,
                            format!(
                                "continuity={} overlap={}",
                                continuity_holds, overlap_compatible
                            ),
                        ),
                        _ => (false, "missing R_TOP executable witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_BAYES_TOP_THREAD_WIT".into()],
                    )
                }
                "ADQ_BAYES_PROB_EQ" => {
                    adequacy_from_obligation(obligations, "OBL_BAYES_ADE", "ADQ_BAYES_ADE_RECEIPT")
                }
                "ADQ_BAYES_PROB_TOLL" => {
                    adequacy_from_obligation(obligations, "OBL_BAYES_TOL", "ADQ_BAYES_TOL_RECEIPT")
                }
                "ADQ_BAYES_TOP_KNOT" => {
                    let (ok, detail) = match &top {
                        Some(HostExecutionResult::Topology {
                            cover_legal,
                            overlap_compatible,
                            ..
                        }) => (
                            *cover_legal && *overlap_compatible,
                            format!("cover={} overlap={}", cover_legal, overlap_compatible),
                        ),
                        _ => (false, "missing R_TOP knot witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_BAYES_TOP_KNOT_WIT".into()],
                    )
                }
                "ADQ_BAYES_TOP_PROB_BRIDGE" => {
                    let has_measprob = ledger
                        .as_ref()
                        .map(|item| item.receipts.iter().any(|receipt| receipt == "MeasProb_mu"))
                        .unwrap_or(false);
                    let selected_direct = selected_path == ["B_TOP_TO_PROB".to_string()];
                    let on_cell = atlas_cell_id == Some("A_TOP_TO_PROB");
                    (
                        if has_measprob && selected_direct && on_cell {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        format!(
                            "atlas_cell={:?} selected_path={} receipt_measprob={}",
                            atlas_cell_id,
                            selected_path.join("->"),
                            has_measprob
                        ),
                        vec!["ADQ_BAYES_BRIDGE_SOUND_WIT".into()],
                    )
                }
                "ADQ_CH_TYP_OBJECT" => {
                    let (ok, detail) = match &typ {
                        Some(HostExecutionResult::TypeTheory {
                            witness_inhabited, ..
                        }) => (
                            *witness_inhabited,
                            format!("witness_inhabited={witness_inhabited}"),
                        ),
                        _ => (false, "missing R_TYP executable witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_CH_TYP_OBJECT_WIT".into()],
                    )
                }
                "ADQ_CH_TYP_THREAD" => {
                    let (ok, detail) = match &typ {
                        Some(HostExecutionResult::TypeTheory {
                            witness_inhabited,
                            normalization_correspondence,
                        }) => (
                            *witness_inhabited && *normalization_correspondence,
                            format!(
                                "witness_inhabited={} normalization_correspondence={}",
                                witness_inhabited, normalization_correspondence
                            ),
                        ),
                        _ => (false, "missing R_TYP executable thread witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_CH_TYP_THREAD_WIT".into()],
                    )
                }
                "ADQ_CH_SET_EQ" => {
                    adequacy_from_obligation(obligations, "OBL_CH_EQ", "ADQ_CH_EQ_RECEIPT")
                }
                "ADQ_CH_TYP_TOLL" => {
                    adequacy_from_obligation(obligations, "OBL_CH_RED", "ADQ_CH_RED_RECEIPT")
                }
                "ADQ_CH_TYP_KNOT" => {
                    let (ok, detail) = match &typ {
                        Some(HostExecutionResult::TypeTheory {
                            witness_inhabited,
                            normalization_correspondence,
                        }) => (
                            *witness_inhabited && *normalization_correspondence,
                            format!(
                                "witness_inhabited={} normalization_correspondence={}",
                                witness_inhabited, normalization_correspondence
                            ),
                        ),
                        _ => (false, "missing R_TYP executable knot witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_CH_TYP_KNOT_WIT".into()],
                    )
                }
                "ADQ_CH_TYP_SET_BRIDGE" => {
                    let has_rig = ledger
                        .as_ref()
                        .map(|item| item.receipts.iter().any(|receipt| receipt == "Rig"))
                        .unwrap_or(false);
                    let selected_direct = selected_path == ["B_TYPE_TO_SET".to_string()];
                    let on_cell = atlas_cell_id == Some("A_TYPE_TO_SET");
                    (
                        if has_rig && selected_direct && on_cell {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        format!(
                            "atlas_cell={:?} selected_path={} receipt_rig={}",
                            atlas_cell_id,
                            selected_path.join("->"),
                            has_rig
                        ),
                        vec!["ADQ_CH_TYP_SET_BRIDGE_WIT".into()],
                    )
                }
                "ADQ_EXEC_PROB_OBJECT" => {
                    let (ok, detail) = match &prob {
                        Some(HostExecutionResult::Probability {
                            normalized,
                            pushforward_ok,
                            conditioning_legal,
                            ..
                        }) => (
                            *normalized && *pushforward_ok && *conditioning_legal,
                            format!(
                                "normalized={} pushforward={} conditioning={}",
                                normalized, pushforward_ok, conditioning_legal
                            ),
                        ),
                        _ => (false, "missing R_PROB executable witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_EXEC_PROB_OBJECT_WIT".into()],
                    )
                }
                "ADQ_EXEC_PROB_THREAD" => adequacy_from_obligation(
                    obligations,
                    "OBL_EXEC_TOL",
                    "ADQ_EXEC_PROB_THREAD_WIT",
                ),
                "ADQ_EXEC_COMP_EQ" => adequacy_from_obligation(
                    obligations,
                    "OBL_EXEC_ADE",
                    "ADQ_EXEC_COMP_EQ_RECEIPT",
                ),
                "ADQ_EXEC_COMP_TOLL" => adequacy_from_obligation(
                    obligations,
                    "OBL_EXEC_TOL",
                    "ADQ_EXEC_COMP_TOLL_RECEIPT",
                ),
                "ADQ_EXEC_COMP_KNOT" => {
                    let (ok, detail) = match &comp {
                        Some(HostExecutionResult::Computation {
                            reached_normal_form,
                            replayable_trace,
                            ..
                        }) => (
                            *reached_normal_form && *replayable_trace,
                            format!(
                                "normal_form={} replayable_trace={}",
                                reached_normal_form, replayable_trace
                            ),
                        ),
                        _ => (false, "missing R_COMP executable knot witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_EXEC_COMP_KNOT_WIT".into()],
                    )
                }
                "ADQ_EXEC_PROB_COMP_BRIDGE" => {
                    let has_exec = ledger
                        .as_ref()
                        .map(|item| item.receipts.iter().any(|receipt| receipt == "Exec"))
                        .unwrap_or(false);
                    let selected_direct = selected_path == ["B_PROB_TO_COMP".to_string()];
                    let on_cell = atlas_cell_id == Some("A_PROB_TO_COMP");
                    (
                        if has_exec && selected_direct && on_cell {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        format!(
                            "atlas_cell={:?} selected_path={} receipt_exec={}",
                            atlas_cell_id,
                            selected_path.join("->"),
                            has_exec
                        ),
                        vec!["ADQ_EXEC_PROB_COMP_BRIDGE_WIT".into()],
                    )
                }
                "ADQ_JDG_PROB_OBJECT" => {
                    let (ok, detail) = match &prob {
                        Some(HostExecutionResult::Probability { normalized, .. }) => {
                            (*normalized, format!("normalized={normalized}"))
                        }
                        _ => (false, "missing R_PROB executable witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_JDG_PROB_OBJECT_WIT".into()],
                    )
                }
                "ADQ_JDG_PROB_THREAD" => {
                    adequacy_from_obligation(obligations, "OBL_JDG_TOL", "ADQ_JDG_PROB_THREAD_WIT")
                }
                "ADQ_JDG_LOG_EQ" => {
                    let (ok, detail) = match &log {
                        Some(HostExecutionResult::Logic {
                            proposition_well_formed,
                            witness_available,
                        }) => (
                            *proposition_well_formed && *witness_available,
                            format!(
                                "logic proposition={} witness={}",
                                proposition_well_formed, witness_available
                            ),
                        ),
                        _ => (false, "missing R_LOG executable witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_JDG_LOG_EQ_RECEIPT".into()],
                    )
                }
                "ADQ_JDG_PROB_TOLL" => adequacy_from_obligation(
                    obligations,
                    "OBL_JDG_TOL",
                    "ADQ_JDG_PROB_TOLL_RECEIPT",
                ),
                "ADQ_JDG_PROB_KNOT" => {
                    let (ok, detail) = match &prob {
                        Some(HostExecutionResult::Probability {
                            normalized,
                            conditioning_legal,
                            pushforward_ok,
                            ..
                        }) => (
                            *normalized && *conditioning_legal && *pushforward_ok,
                            format!(
                                "normalized={} conditioning={} pushforward={}",
                                normalized, conditioning_legal, pushforward_ok
                            ),
                        ),
                        _ => (false, "missing R_PROB executable knot witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_JDG_PROB_KNOT_WIT".into()],
                    )
                }
                "ADQ_JDG_PROB_LOG_BRIDGE" => {
                    let has_probjudg = ledger
                        .as_ref()
                        .map(|item| {
                            item.receipts
                                .iter()
                                .any(|receipt| receipt == "ProbJudg_phi")
                        })
                        .unwrap_or(false);
                    let selected_direct = selected_path == ["B_PROB_TO_LOG".to_string()];
                    let on_cell = atlas_cell_id == Some("A_PROB_TO_LOG");
                    (
                        if has_probjudg && selected_direct && on_cell {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        format!(
                            "atlas_cell={:?} selected_path={} receipt_probjudg={}",
                            atlas_cell_id,
                            selected_path.join("->"),
                            has_probjudg
                        ),
                        vec!["ADQ_JDG_PROB_LOG_BRIDGE_WIT".into()],
                    )
                }
                "ADQ_CERT_COMP_OBJECT" => {
                    let (ok, detail) = match &comp {
                        Some(HostExecutionResult::Computation {
                            reached_normal_form,
                            observationally_equivalent,
                            ..
                        }) => (
                            *reached_normal_form && *observationally_equivalent,
                            format!(
                                "normal_form={} observationally_equivalent={}",
                                reached_normal_form, observationally_equivalent
                            ),
                        ),
                        _ => (false, "missing R_COMP executable witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_CERT_COMP_OBJECT_WIT".into()],
                    )
                }
                "ADQ_CERT_COMP_THREAD" => {
                    let (ok, detail) = match &comp {
                        Some(HostExecutionResult::Computation {
                            replayable_trace, ..
                        }) => (
                            *replayable_trace,
                            format!("replayable_trace={replayable_trace}"),
                        ),
                        _ => (false, "missing R_COMP replay witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_CERT_COMP_THREAD_WIT".into()],
                    )
                }
                "ADQ_CERT_LOG_EQ" => {
                    let (ok, detail) = match &log {
                        Some(HostExecutionResult::Logic {
                            proposition_well_formed,
                            witness_available,
                        }) => (
                            *proposition_well_formed && *witness_available,
                            format!(
                                "logic proposition={} witness={}",
                                proposition_well_formed, witness_available
                            ),
                        ),
                        _ => (false, "missing R_LOG executable witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_CERT_LOG_EQ_RECEIPT".into()],
                    )
                }
                "ADQ_CERT_COMP_TOLL" => adequacy_from_obligation(
                    obligations,
                    "OBL_CERT_TOL",
                    "ADQ_CERT_COMP_TOLL_RECEIPT",
                ),
                "ADQ_CERT_COMP_KNOT" => {
                    adequacy_from_obligation(obligations, "OBL_CERT_RED", "ADQ_CERT_COMP_KNOT_WIT")
                }
                "ADQ_CERT_COMP_LOG_BRIDGE" => {
                    let has_cert = ledger
                        .as_ref()
                        .map(|item| item.receipts.iter().any(|receipt| receipt == "Cert_phi"))
                        .unwrap_or(false);
                    let selected_direct = selected_path == ["B_COMP_TO_LOG".to_string()];
                    let on_cell = atlas_cell_id == Some("A_COMP_TO_LOG");
                    (
                        if has_cert && selected_direct && on_cell {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        format!(
                            "atlas_cell={:?} selected_path={} receipt_cert={}",
                            atlas_cell_id,
                            selected_path.join("->"),
                            has_cert
                        ),
                        vec!["ADQ_CERT_COMP_LOG_BRIDGE_WIT".into()],
                    )
                }
                "ADQ_CHI_TYP_OBJECT" => {
                    let (ok, detail) = match &typ {
                        Some(HostExecutionResult::TypeTheory {
                            witness_inhabited, ..
                        }) => (
                            *witness_inhabited,
                            format!("witness_inhabited={witness_inhabited}"),
                        ),
                        _ => (false, "missing R_TYP executable witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_CHI_TYP_OBJECT_WIT".into()],
                    )
                }
                "ADQ_CHI_TYP_THREAD" => {
                    let (ok, detail) = match &typ {
                        Some(HostExecutionResult::TypeTheory {
                            witness_inhabited,
                            normalization_correspondence,
                        }) => (
                            *witness_inhabited && *normalization_correspondence,
                            format!(
                                "witness_inhabited={} normalization_correspondence={}",
                                witness_inhabited, normalization_correspondence
                            ),
                        ),
                        _ => (false, "missing R_TYP executable thread witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_CHI_TYP_THREAD_WIT".into()],
                    )
                }
                "ADQ_CHI_ALG_EQ" => {
                    adequacy_from_obligation(obligations, "OBL_CHI_EQ", "ADQ_CHI_ALG_EQ_RECEIPT")
                }
                "ADQ_CHI_TYP_TOLL" => {
                    adequacy_from_obligation(obligations, "OBL_CHI_ADE", "ADQ_CHI_TYP_TOLL_RECEIPT")
                }
                "ADQ_CHI_TYP_KNOT" => {
                    let (ok, detail) = match &typ {
                        Some(HostExecutionResult::TypeTheory {
                            witness_inhabited,
                            normalization_correspondence,
                        }) => (
                            *witness_inhabited && *normalization_correspondence,
                            format!(
                                "witness_inhabited={} normalization_correspondence={}",
                                witness_inhabited, normalization_correspondence
                            ),
                        ),
                        _ => (false, "missing R_TYP executable knot witness".into()),
                    };
                    (
                        if ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        detail,
                        vec!["ADQ_CHI_TYP_KNOT_WIT".into()],
                    )
                }
                "ADQ_CHI_TYP_ALG_BRIDGE" => {
                    let has_lift = ledger
                        .as_ref()
                        .map(|item| {
                            item.receipts
                                .iter()
                                .any(|receipt| receipt == "Lift^Typ->Alg")
                        })
                        .unwrap_or(false);
                    let selected_direct = selected_path == ["B_TYPE_TO_ALG".to_string()];
                    let on_cell = atlas_cell_id == Some("A_TYPE_TO_ALG");
                    let alg_ok = matches!(
                        &alg,
                        Some(HostExecutionResult::Algebra {
                            closure_holds: true,
                            associative: true,
                            ..
                        })
                    );
                    (
                        if has_lift && selected_direct && on_cell && alg_ok {
                            CertificationVerdict::Certified
                        } else {
                            CertificationVerdict::BlockedOpen
                        },
                        true,
                        format!(
                            "atlas_cell={:?} selected_path={} receipt_lift={} algebra_ready={}",
                            atlas_cell_id,
                            selected_path.join("->"),
                            has_lift,
                            alg_ok
                        ),
                        vec!["ADQ_CHI_TYP_ALG_BRIDGE_WIT".into()],
                    )
                }
                _ => generic_adequacy_evaluation(registry, &clause),
            };
            l64_core::AdequacyRecord {
                id: format!("ADR_{}_{}", theorem_id, clause.id),
                clause_id: clause.id,
                kind: clause.kind,
                verdict,
                computed,
                blocking: clause.blocking,
                detail,
                receipt_ids,
            }
        })
        .collect::<Vec<_>>();
    records.sort_by(|left, right| left.id.cmp(&right.id));
    records
}

fn adequacy_from_obligation(
    obligations: &[ObligationStatus],
    obligation_id: &str,
    receipt_id: &str,
) -> (CertificationVerdict, bool, String, Vec<String>) {
    if let Some(item) = obligations
        .iter()
        .find(|item| item.obligation_id == obligation_id)
    {
        (
            item.verdict.clone(),
            item.evaluation_mode != ObligationEvaluationMode::Unsupported,
            item.detail.clone(),
            vec![receipt_id.into()],
        )
    } else {
        (
            CertificationVerdict::Underspecified,
            false,
            format!("missing obligation {obligation_id}"),
            vec![receipt_id.into()],
        )
    }
}

fn generic_adequacy_evaluation(
    registry: &(impl RegistryLookup + Sync),
    clause: &l64_core::AdequacyClause,
) -> (CertificationVerdict, bool, String, Vec<String>) {
    generic_adequacy_evaluator(&clause.kind)(registry, clause)
}

type GenericAdequacyEvaluator = fn(
    &(dyn RegistryLookup + Sync),
    &l64_core::AdequacyClause,
) -> (CertificationVerdict, bool, String, Vec<String>);

fn generic_adequacy_evaluator(kind: &l64_core::AdequacyClauseKind) -> GenericAdequacyEvaluator {
    match kind {
        l64_core::AdequacyClauseKind::EvidenceContractInterpretation => {
            eval_evidence_contract_adequacy
        }
        l64_core::AdequacyClauseKind::BenchmarkInterpretation => eval_benchmark_adequacy,
        l64_core::AdequacyClauseKind::StressInterpretation => eval_stress_adequacy,
        l64_core::AdequacyClauseKind::ChallengeInterpretation => eval_challenge_adequacy,
        l64_core::AdequacyClauseKind::ProjectionInterpretation
        | l64_core::AdequacyClauseKind::ContainmentInterpretation
        | l64_core::AdequacyClauseKind::ClosureInterpretation
        | l64_core::AdequacyClauseKind::RunningLawInterpretation => eval_missing_host_pack_adequacy,
        _ => eval_underspecified_adequacy,
    }
}

fn eval_evidence_contract_adequacy(
    registry: &(dyn RegistryLookup + Sync),
    clause: &l64_core::AdequacyClause,
) -> (CertificationVerdict, bool, String, Vec<String>) {
    let contracts = clause
        .evidence_contract_ids
        .iter()
        .filter_map(|id| registry.get_evidence_contract(id))
        .collect::<Vec<_>>();
    if contracts.len() != clause.evidence_contract_ids.len() || contracts.is_empty() {
        return (
            CertificationVerdict::BlockedOpen,
            true,
            "required evidence contracts missing".into(),
            clause.evidence_contract_ids.clone(),
        );
    }
    (
        CertificationVerdict::Certified,
        true,
        format!("evidence_contracts={}", contracts.len()),
        clause.evidence_contract_ids.clone(),
    )
}

fn eval_benchmark_adequacy(
    registry: &(dyn RegistryLookup + Sync),
    clause: &l64_core::AdequacyClause,
) -> (CertificationVerdict, bool, String, Vec<String>) {
    let receipts = clause
        .benchmark_receipt_ids
        .iter()
        .filter_map(|id| registry.get_benchmark_receipt(id))
        .collect::<Vec<_>>();
    let ok = receipts.len() == clause.benchmark_receipt_ids.len()
        && !receipts.is_empty()
        && receipts
            .iter()
            .all(|item| item.verdict == CertificationVerdict::Certified);
    (
        if ok {
            CertificationVerdict::Certified
        } else {
            CertificationVerdict::BlockedOpen
        },
        true,
        format!("benchmark_receipts={} all_certified={}", receipts.len(), ok),
        clause.benchmark_receipt_ids.clone(),
    )
}

fn eval_stress_adequacy(
    registry: &(dyn RegistryLookup + Sync),
    clause: &l64_core::AdequacyClause,
) -> (CertificationVerdict, bool, String, Vec<String>) {
    let receipts = clause
        .benchmark_receipt_ids
        .iter()
        .filter_map(|id| registry.get_benchmark_receipt(id))
        .filter(|item| item.role == l64_core::BenchmarkRole::Stress)
        .collect::<Vec<_>>();
    let ok = !receipts.is_empty()
        && receipts
            .iter()
            .all(|item| item.verdict == CertificationVerdict::Certified);
    (
        if ok {
            CertificationVerdict::Certified
        } else {
            CertificationVerdict::BlockedOpen
        },
        true,
        format!("stress_receipts={} all_certified={}", receipts.len(), ok),
        receipts.into_iter().map(|item| item.id).collect(),
    )
}

fn eval_challenge_adequacy(
    registry: &(dyn RegistryLookup + Sync),
    clause: &l64_core::AdequacyClause,
) -> (CertificationVerdict, bool, String, Vec<String>) {
    let receipts = clause
        .challenge_receipt_ids
        .iter()
        .filter_map(|id| registry.get_challenge_receipt(id))
        .collect::<Vec<_>>();
    let ok = !receipts.is_empty()
        && receipts
            .iter()
            .all(|item| item.status != l64_core::ChallengeStatus::Open);
    (
        if ok {
            CertificationVerdict::Certified
        } else {
            CertificationVerdict::BlockedOpen
        },
        true,
        format!("challenge_receipts={} all_closed={}", receipts.len(), ok),
        clause.challenge_receipt_ids.clone(),
    )
}

fn eval_missing_host_pack_adequacy(
    _registry: &(dyn RegistryLookup + Sync),
    clause: &l64_core::AdequacyClause,
) -> (CertificationVerdict, bool, String, Vec<String>) {
    (
        CertificationVerdict::BlockedOpen,
        true,
        "host semantic pack not yet implemented for this burden".into(),
        clause.burden_pack_ids.clone(),
    )
}

fn eval_underspecified_adequacy(
    _registry: &(dyn RegistryLookup + Sync),
    _clause: &l64_core::AdequacyClause,
) -> (CertificationVerdict, bool, String, Vec<String>) {
    (
        CertificationVerdict::Underspecified,
        false,
        "no executable adequacy evaluator registered for this clause".into(),
        Vec::new(),
    )
}

fn adequacy_record_deficiency(
    theorem_id: &str,
    atlas_cell_id: Option<&str>,
    record: &l64_core::AdequacyRecord,
) -> l64_core::AtlasDeficiency {
    l64_core::adequacy_record_deficiency(
        theorem_id,
        atlas_cell_id,
        format!("DGN_ADQ_{}_{}", theorem_id, record.clause_id),
        record,
    )
}

fn campaign_related_burden_packs(
    registry: &(impl RegistryLookup + Sync),
    campaign_id: Option<&str>,
) -> Vec<l64_core::BurdenPack> {
    let Some(campaign_id) = campaign_id else {
        return Vec::new();
    };
    let Some(campaign) = registry.get_campaign(campaign_id) else {
        return Vec::new();
    };
    registry
        .burden_packs()
        .into_iter()
        .filter(|pack| {
            pack.obligation_ids
                .iter()
                .any(|id| campaign.obligations.iter().any(|candidate| candidate == id))
        })
        .collect()
}

fn collect_related_burden_pack_ids(
    registry: &(impl RegistryLookup + Sync),
    campaign_id: Option<&str>,
) -> Vec<String> {
    campaign_related_burden_packs(registry, campaign_id)
        .into_iter()
        .map(|item| item.id)
        .collect()
}

fn collect_related_evidence_contract_ids(
    registry: &(impl RegistryLookup + Sync),
    campaign_id: Option<&str>,
) -> Vec<String> {
    campaign_related_burden_packs(registry, campaign_id)
        .into_iter()
        .flat_map(|item| item.evidence_contract_ids)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn collect_related_claim_packet_ids(
    registry: &(impl RegistryLookup + Sync),
    campaign_id: Option<&str>,
) -> Vec<String> {
    let contract_ids = collect_related_evidence_contract_ids(registry, campaign_id);
    let expected_kinds = registry
        .evidence_contracts()
        .into_iter()
        .filter(|contract| contract_ids.iter().any(|id| id == &contract.id))
        .flat_map(|contract| contract.required_evidence_kinds)
        .collect::<std::collections::BTreeSet<_>>();
    registry
        .claim_packets()
        .into_iter()
        .filter(|packet| {
            expected_kinds.is_empty() || expected_kinds.contains(&packet.target_sector)
        })
        .map(|packet| packet.id)
        .collect()
}

fn collect_related_benchmark_receipt_ids(
    registry: &(impl RegistryLookup + Sync),
    campaign_id: Option<&str>,
) -> Vec<String> {
    let claim_ids = collect_related_claim_packet_ids(registry, campaign_id);
    registry
        .benchmark_receipts()
        .into_iter()
        .filter(|item| claim_ids.iter().any(|id| id == &item.claim_packet_id))
        .map(|item| item.id)
        .collect()
}

fn collect_related_challenge_receipt_ids(
    registry: &(impl RegistryLookup + Sync),
    campaign_id: Option<&str>,
) -> Vec<String> {
    let claim_ids = collect_related_claim_packet_ids(registry, campaign_id);
    registry
        .challenge_receipts()
        .into_iter()
        .filter(|item| claim_ids.iter().any(|id| id == &item.claim_packet_id))
        .map(|item| item.id)
        .collect()
}

fn collect_related_reproducibility_packet_ids(
    registry: &(impl RegistryLookup + Sync),
    campaign_id: Option<&str>,
) -> Vec<String> {
    let claim_ids = collect_related_claim_packet_ids(registry, campaign_id);
    registry
        .reproducibility_packets()
        .into_iter()
        .filter(|item| claim_ids.iter().any(|id| id == &item.claim_packet_id))
        .map(|item| item.id)
        .collect()
}

fn deficiency_blocks_campaign(deficiencies: &[l64_core::AtlasDeficiency]) -> bool {
    deficiencies.iter().any(|item| {
        item.control_effects
            .iter()
            .any(|effect| effect == &l64_core::DeficiencyControlEffect::BlockCampaign)
    })
}

fn deficiency_blocks_promotion(deficiencies: &[l64_core::AtlasDeficiency]) -> bool {
    deficiencies.iter().any(|item| {
        item.control_effects
            .iter()
            .any(|effect| effect == &l64_core::DeficiencyControlEffect::BlockPromotion)
    })
}

fn collect_receipt_deficiencies(
    deficiencies: &mut Vec<l64_core::AtlasDeficiency>,
    theorem_id: &str,
    atlas_cell_id: Option<&str>,
    obligation_id: &str,
    receipt: &ObligationEvidenceReceipt,
) {
    if receipt.subreceipts.is_empty() {
        if receipt.verdict != CertificationVerdict::Certified {
            deficiencies.push(l64_core::receipt_deficiency(
                theorem_id,
                atlas_cell_id,
                format!("DGN_EQR_{}_{}", obligation_id, receipt.id),
                receipt,
                l64_core::AtlasDeficiencyClass::DNoAdequacy,
                l64_core::AtlasDeficiencyClass::DOpenConjectural,
                format!(
                    "receipt {} remains open and still blocks semantic discharge",
                    receipt.id
                ),
            ));
        }
        return;
    }

    for subreceipt in &receipt.subreceipts {
        collect_receipt_deficiencies(
            deficiencies,
            theorem_id,
            atlas_cell_id,
            obligation_id,
            subreceipt,
        );
    }
}

fn receipt_has_computed_failure(receipt: &ObligationEvidenceReceipt) -> bool {
    if receipt.subreceipts.is_empty() {
        return receipt.computed
            && matches!(
                receipt.verdict,
                CertificationVerdict::BlockedContradiction | CertificationVerdict::BlockedOpen
            );
    }
    receipt.subreceipts.iter().any(receipt_has_computed_failure)
}

fn receipt_has_residual(receipt: &ObligationEvidenceReceipt) -> bool {
    if receipt.subreceipts.is_empty() {
        return !receipt.computed || receipt.verdict != CertificationVerdict::Certified;
    }
    receipt.subreceipts.iter().any(receipt_has_residual)
}

fn receipt_leaf_counts(receipt: &ObligationEvidenceReceipt) -> (usize, usize) {
    if receipt.subreceipts.is_empty() {
        let success =
            usize::from(receipt.computed && receipt.verdict == CertificationVerdict::Certified);
        let residual = usize::from(receipt.verdict != CertificationVerdict::Certified);
        return (success, residual);
    }
    receipt
        .subreceipts
        .iter()
        .map(receipt_leaf_counts)
        .fold((0usize, 0usize), |(sa, ra), (sb, rb)| (sa + sb, ra + rb))
}

fn evaluate_flagship_equivalence_obligation(
    registry: &(impl RegistryLookup + Sync),
    theorem_id: &str,
    obligation: &Obligation,
) -> Option<ObligationStatus> {
    if theorem_id == "THS_CH_INH" && obligation.id == "OBL_CHI_EQ" {
        let witness = exec_ch_inh_type_witness().ok()?;
        let mut subreceipts = vec![
            ObligationEvidenceReceipt {
                id: "CHI_EQ_QUOTIENT_TRANSPORT".into(),
                label: "quotient transport agreement".into(),
                verdict: if witness.quotient_transport_exact {
                    CertificationVerdict::Certified
                } else {
                    CertificationVerdict::BlockedOpen
                },
                computed: true,
                detail: format!(
                    "direct_carrier={:?} lifted_carrier={:?}",
                    witness.direct_carrier, witness.lifted_carrier
                ),
                subreceipts: Vec::new(),
            },
            ObligationEvidenceReceipt {
                id: "CHI_EQ_HOMOMORPHISM_PRESERVE".into(),
                label: "homomorphism preservation".into(),
                verdict: if witness.homomorphism_preserved_exact {
                    CertificationVerdict::Certified
                } else {
                    CertificationVerdict::BlockedOpen
                },
                computed: true,
                detail: "lifted algebra operations preserve the inherited carrier witness".into(),
                subreceipts: Vec::new(),
            },
        ];
        if witness.proof_relevance_preserved {
            subreceipts.push(ObligationEvidenceReceipt {
                id: "CHI_EQ_PROOF_TERM_TRANSPORT".into(),
                label: "proof-term transport".into(),
                verdict: CertificationVerdict::Certified,
                computed: true,
                detail: format!(
                    "direct_proofs={:?} lifted_proofs={:?}",
                    witness.direct_proof_terms, witness.lifted_proof_terms
                ),
                subreceipts: Vec::new(),
            });
            subreceipts.push(ObligationEvidenceReceipt {
                id: "CHI_EQ_PROOF_TERM_NORMAL_FORM".into(),
                label: "proof-term normal-form agreement".into(),
                verdict: CertificationVerdict::Certified,
                computed: true,
                detail: "proof-relevant carrier witnesses normalize to the same inherited representative".into(),
                subreceipts: Vec::new(),
            });
        } else {
            subreceipts.push(ObligationEvidenceReceipt {
                id: "CHI_EQ_PROOF_RELEVANCE_BOUNDARY".into(),
                label: "proof-relevant equality boundary".into(),
                verdict: CertificationVerdict::Underspecified,
                computed: false,
                detail: witness.residual_boundary.clone().unwrap_or_else(|| {
                    "proof-relevant algebra equality boundary remains open".into()
                }),
                subreceipts: Vec::new(),
            });
        }
        let computed_failure = subreceipts.iter().any(receipt_has_computed_failure);
        let residual_scaffold = subreceipts.iter().any(receipt_has_residual);
        let (computed_successes, residuals) = subreceipts
            .iter()
            .map(receipt_leaf_counts)
            .fold((0usize, 0usize), |(sa, ra), (sb, rb)| (sa + sb, ra + rb));
        return Some(ObligationStatus {
            obligation_id: obligation.id.clone(),
            kind: obligation.kind.clone(),
            verdict: if computed_failure || residual_scaffold {
                CertificationVerdict::BlockedOpen
            } else {
                CertificationVerdict::Certified
            },
            evaluation_mode: if residual_scaffold {
                ObligationEvaluationMode::RecomputedPartial
            } else {
                ObligationEvaluationMode::RecomputedExact
            },
            detail: format!(
                "CH-Inh algebra equality factored into {} computed receipts with {} residual blocker(s)",
                computed_successes, residuals
            ),
            receipts: vec![ObligationEvidenceReceipt {
                id: "CHI_EQ_INHERITANCE".into(),
                label: "proof-relevant inherited algebra equality".into(),
                verdict: if residual_scaffold {
                    CertificationVerdict::BlockedOpen
                } else {
                    CertificationVerdict::Certified
                },
                computed: !residual_scaffold,
                detail: "algebra equality decomposed into quotient transport, homomorphism preservation, and proof-relevance witness structure".into(),
                subreceipts,
            }],
        });
    }
    if !is_chain_rule_family(theorem_id) || obligation.id != "OBL_CHAIN_EQ" {
        return None;
    }
    if let Some(operator) = chain_rule_promoted_operator(registry) {
        let (apply_id, apply_label, reuse_id, reuse_label, detail) = match operator.origin {
            ArtifactOrigin::Seed => (
                "EQR_CHAIN_OPERATOR_DEFAULT_APPLICABILITY",
                "default promoted operator applicability",
                "EQR_CHAIN_OPERATOR_DEFAULT_SELECTION",
                "default promoted operator selection",
                format!(
                    "default-selected canonical Chain₁ operator `{}` for first-order equivalence discharge",
                    operator.id
                ),
            ),
            ArtifactOrigin::Overlay | ArtifactOrigin::Unknown => (
                "EQR_CHAIN_OPERATOR_APPLICABILITY",
                "promoted operator applicability",
                "EQR_CHAIN_OPERATOR_REUSE",
                "promoted operator reuse",
                format!(
                    "reused promoted Chain₁ operator witness `{}` for first-order equivalence discharge",
                    operator.id
                ),
            ),
        };
        return Some(ObligationStatus {
            obligation_id: obligation.id.clone(),
            kind: obligation.kind.clone(),
            verdict: CertificationVerdict::Certified,
            evaluation_mode: ObligationEvaluationMode::RecomputedExact,
            detail,
            receipts: vec![
                ObligationEvidenceReceipt {
                    id: apply_id.into(),
                    label: apply_label.into(),
                    verdict: CertificationVerdict::Certified,
                    computed: true,
                    detail: format!(
                        "promoted operator `{}` matches first-order derivative composition burden",
                        operator.id
                    ),
                    subreceipts: Vec::new(),
                },
                ObligationEvidenceReceipt {
                    id: reuse_id.into(),
                    label: reuse_label.into(),
                    verdict: CertificationVerdict::Certified,
                    computed: true,
                    detail: match operator.origin {
                        ArtifactOrigin::Seed => {
                            "default-selected integrated Chain₁ witness instead of expanding the full composed-jet stack"
                                .into()
                        }
                        ArtifactOrigin::Overlay | ArtifactOrigin::Unknown => {
                            "reused integrated Chain₁ witness instead of expanding the full composed-jet stack".into()
                        }
                    },
                    subreceipts: Vec::new(),
                },
            ],
        });
    }
    let top = exec_host("R_TOP", None).ok()?;
    let calc = exec_host("R_CALC", None).ok()?;
    let ledger = registry.get_route_ledger("TRL_CHAIN_RULE");

    let mut receipts = Vec::new();
    if let HostExecutionResult::Topology {
        cover_legal,
        continuity_holds,
        overlap_compatible,
        ..
    } = top
    {
        receipts.push(ObligationEvidenceReceipt {
            id: "EQR_CHAIN_LOC_TRANSPORT".into(),
            label: "brace-local transport".into(),
            verdict: if cover_legal && continuity_holds && overlap_compatible {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedContradiction
            },
            computed: true,
            detail: format!(
                "cover={} continuity={} overlap={}",
                cover_legal, continuity_holds, overlap_compatible
            ),
            subreceipts: Vec::new(),
        });
    }
    if let HostExecutionResult::Calculus {
        local_linear_witness,
        finite_difference_derivative,
        symbolic_only,
        ..
    } = calc
    {
        receipts.push(ObligationEvidenceReceipt {
            id: "EQR_CHAIN_DERIV_REP".into(),
            label: "first-order derivative representative".into(),
            verdict: if local_linear_witness && finite_difference_derivative {
                CertificationVerdict::Certified
            } else if symbolic_only {
                CertificationVerdict::Underspecified
            } else {
                CertificationVerdict::BlockedOpen
            },
            computed: true,
            detail: format!(
                "local_linear={} finite_difference={} symbolic_only={}",
                local_linear_witness, finite_difference_derivative, symbolic_only
            ),
            subreceipts: Vec::new(),
        });
    }
    if let Some(ledger) = ledger {
        let has_can = ledger.receipts.iter().any(|item| item == "Can");
        let has_red = ledger.receipts.iter().any(|item| item == "Red");
        let route_is_direct = ledger.normalized_path == vec!["B_TOP_TO_CALC".to_string()];
        receipts.push(ObligationEvidenceReceipt {
            id: "EQR_CHAIN_ROUTE_BINDING".into(),
            label: "bridge-route equivalence transport".into(),
            verdict: if has_can && has_red && route_is_direct {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedOpen
            },
            computed: true,
            detail: format!(
                "route={} canonical_receipt={} reduction_receipt={}",
                ledger.normalized_path.join("->"),
                has_can,
                has_red
            ),
            subreceipts: Vec::new(),
        });
    }
    if let Ok(jet) = exec_chain_rule_jet_compose() {
        let mut subreceipts = Vec::new();
        subreceipts.push(ObligationEvidenceReceipt {
            id: "EQR_CHAIN_JET_LINEAR_AGREE".into(),
            label: "composed local linear form agreement".into(),
            verdict: if jet.local_linear_form_agrees {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedOpen
            },
            computed: true,
            detail: format!(
                "direct={:?} composed={:?}",
                jet.direct.entries, jet.composed.entries
            ),
            subreceipts: Vec::new(),
        });
        subreceipts.push(ObligationEvidenceReceipt {
            id: "EQR_CHAIN_JET_NORMAL_FORM".into(),
            label: "route-normalized representative agreement".into(),
            verdict: if jet.normalized_representative_agrees {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedOpen
            },
            computed: true,
            detail: format!(
                "normalized_direct={:?} normalized_composed={:?}",
                jet.direct.entries, jet.composed.entries
            ),
            subreceipts: Vec::new(),
        });
        subreceipts.push(ObligationEvidenceReceipt {
            id: "EQR_CHAIN_JET_REDUCTION_PRESERVE".into(),
            label: "reduction-preservation agreement".into(),
            verdict: if jet.reduction_preserves_form {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedOpen
            },
            computed: true,
            detail: "finite-difference representative agrees with composed local linear form"
                .into(),
            subreceipts: Vec::new(),
        });
        subreceipts.push(ObligationEvidenceReceipt {
            id: "EQR_CHAIN_JET_CHART_OVERLAP".into(),
            label: "chart-overlap compatibility".into(),
            verdict: if jet.chart_overlap_compatible {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedOpen
            },
            computed: true,
            detail: format!(
                "transition_direct={:?} transition_composed={:?}",
                jet.direct_in_transition.entries, jet.composed_in_transition.entries
            ),
            subreceipts: Vec::new(),
        });
        subreceipts.push(ObligationEvidenceReceipt {
            id: "EQR_CHAIN_JET_TRANSITION_TRANSPORT".into(),
            label: "transition transport agreement".into(),
            verdict: if jet.transition_transport_agrees {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedOpen
            },
            computed: true,
            detail: format!(
                "transition_direct={:?} transition_composed={:?}",
                jet.direct_in_transition.entries, jet.composed_in_transition.entries
            ),
            subreceipts: Vec::new(),
        });
        subreceipts.push(ObligationEvidenceReceipt {
            id: "EQR_CHAIN_JET_GAUGE_NORMALIZE".into(),
            label: "gauge-normalized representative agreement".into(),
            verdict: if jet.gauge_normalized_representative_agrees {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedOpen
            },
            computed: true,
            detail: format!(
                "normalized_transition_direct={:?} normalized_transition_composed={:?}",
                jet.direct_in_transition.entries, jet.composed_in_transition.entries
            ),
            subreceipts: Vec::new(),
        });
        if let Some(boundary) = jet.residual_boundary {
            subreceipts.push(ObligationEvidenceReceipt {
                id: "EQR_CHAIN_JET_GAUGE_BOUNDARY".into(),
                label: "chart-global gauge boundary".into(),
                verdict: CertificationVerdict::Underspecified,
                computed: false,
                detail: boundary,
                subreceipts: Vec::new(),
            });
        }
        let jet_computed_failure = subreceipts.iter().any(receipt_has_computed_failure);
        let jet_residual = subreceipts.iter().any(receipt_has_residual);
        let jet_successes = subreceipts
            .iter()
            .filter(|item| item.computed && item.verdict == CertificationVerdict::Certified)
            .count();
        receipts.push(ObligationEvidenceReceipt {
            id: "EQR_CHAIN_JET_COMPOSE".into(),
            label: "composite first-order jet equality".into(),
            verdict: if jet_computed_failure {
                CertificationVerdict::BlockedOpen
            } else if jet_residual {
                CertificationVerdict::Benchmarked
            } else {
                CertificationVerdict::Certified
            },
            computed: true,
            detail: format!(
                "computed {} composed-jet subreceipts; residual_boundary={}",
                jet_successes, jet_residual
            ),
            subreceipts,
        });
    }

    let computed_failure = receipts.iter().any(receipt_has_computed_failure);
    let residual_scaffold = receipts.iter().any(receipt_has_residual);
    let verdict = if computed_failure {
        CertificationVerdict::BlockedOpen
    } else if residual_scaffold {
        CertificationVerdict::Benchmarked
    } else {
        CertificationVerdict::Certified
    };
    let (computed_successes, residuals) = receipts
        .iter()
        .map(receipt_leaf_counts)
        .fold((0usize, 0usize), |(sa, ra), (sb, rb)| (sa + sb, ra + rb));

    Some(ObligationStatus {
        obligation_id: obligation.id.clone(),
        kind: obligation.kind.clone(),
        verdict,
        evaluation_mode: if residual_scaffold {
            ObligationEvaluationMode::RecomputedPartial
        } else {
            ObligationEvaluationMode::RecomputedExact
        },
        detail: format!(
            "chain-rule equivalence factored into {} computed receipts with {} residual blocker(s)",
            computed_successes, residuals
        ),
        receipts,
    })
}

fn evaluate_flagship_reduction_obligation(
    registry: &(impl RegistryLookup + Sync),
    theorem_id: &str,
    obligation: &Obligation,
) -> Option<ObligationStatus> {
    if theorem_id == "THS_CH_NORM" && obligation.id == "OBL_CH_RED" {
        let witness = exec_ch_norm_type_witness().ok()?;
        let receipts = vec![
            ObligationEvidenceReceipt {
                id: "CHN_RED_BETA_ETA_NORMALIZE".into(),
                label: "beta-eta normalization".into(),
                verdict: if witness.beta_eta_normalization_exact {
                    CertificationVerdict::Certified
                } else {
                    CertificationVerdict::BlockedOpen
                },
                computed: true,
                detail: format!(
                    "direct_normal_form={:?} collapsed_normal_form={:?}",
                    witness.direct_normal_form, witness.collapsed_normal_form
                ),
                subreceipts: Vec::new(),
            },
            ObligationEvidenceReceipt {
                id: "CHN_RED_EXTENSIONAL_NORMAL_FORM".into(),
                label: "extensional carrier normal-form agreement".into(),
                verdict: if witness.extensional_normal_form_agrees {
                    CertificationVerdict::Certified
                } else {
                    CertificationVerdict::BlockedOpen
                },
                computed: true,
                detail: "collapsed carrier and direct witness normalize to the same representative"
                    .into(),
                subreceipts: Vec::new(),
            },
        ];
        let residual_scaffold = receipts.iter().any(receipt_has_residual);
        return Some(ObligationStatus {
            obligation_id: obligation.id.clone(),
            kind: obligation.kind.clone(),
            verdict: if residual_scaffold {
                CertificationVerdict::BlockedOpen
            } else {
                CertificationVerdict::Certified
            },
            evaluation_mode: if residual_scaffold {
                ObligationEvaluationMode::RecomputedPartial
            } else {
                ObligationEvaluationMode::RecomputedExact
            },
            detail: "CH-Norm reduction witness discharged through exact route-local normalization checks".into(),
            receipts,
        });
    }
    if !is_chain_rule_family(theorem_id) || obligation.id != "OBL_CHAIN_RED" {
        return None;
    }
    if let Some(operator) = chain_rule_promoted_operator(registry) {
        let (apply_id, apply_label, reuse_id, reuse_label, detail) = match operator.origin {
            ArtifactOrigin::Seed => (
                "RED_CHAIN_OPERATOR_DEFAULT_APPLICABILITY",
                "default promoted operator applicability",
                "RED_CHAIN_OPERATOR_DEFAULT_SELECTION",
                "default promoted operator selection",
                format!(
                    "default-selected canonical Chain₁ operator `{}` for reduction exactness",
                    operator.id
                ),
            ),
            ArtifactOrigin::Overlay | ArtifactOrigin::Unknown => (
                "RED_CHAIN_OPERATOR_APPLICABILITY",
                "promoted operator applicability",
                "RED_CHAIN_OPERATOR_REUSE",
                "promoted operator reuse",
                format!(
                    "reused promoted Chain₁ operator witness `{}` for reduction exactness",
                    operator.id
                ),
            ),
        };
        return Some(ObligationStatus {
            obligation_id: obligation.id.clone(),
            kind: obligation.kind.clone(),
            verdict: CertificationVerdict::Certified,
            evaluation_mode: ObligationEvaluationMode::RecomputedExact,
            detail,
            receipts: vec![
                ObligationEvidenceReceipt {
                    id: apply_id.into(),
                    label: apply_label.into(),
                    verdict: CertificationVerdict::Certified,
                    computed: true,
                    detail: format!(
                        "promoted operator `{}` is valid on the active R_TOP→R_CALC route",
                        operator.id
                    ),
                    subreceipts: Vec::new(),
                },
                ObligationEvidenceReceipt {
                    id: reuse_id.into(),
                    label: reuse_label.into(),
                    verdict: CertificationVerdict::Certified,
                    computed: true,
                    detail: match operator.origin {
                        ArtifactOrigin::Seed => {
                            "default-selected integrated Chain₁ witness instead of recomputing the full remainder-reduction micro-kernel"
                                .into()
                        }
                        ArtifactOrigin::Overlay | ArtifactOrigin::Unknown => {
                            "reused integrated Chain₁ witness instead of recomputing the full remainder-reduction micro-kernel"
                                .into()
                        }
                    },
                    subreceipts: Vec::new(),
                },
            ],
        });
    }
    let reduction = exec_chain_rule_reduction().ok()?;
    let mut receipts = Vec::new();
    receipts.push(ObligationEvidenceReceipt {
        id: "RED_CHAIN_FINITE_DIFF_AGREE".into(),
        label: "finite-difference agreement".into(),
        verdict: if reduction.finite_difference_agrees {
            CertificationVerdict::Certified
        } else {
            CertificationVerdict::BlockedOpen
        },
        computed: true,
        detail: format!(
            "direct_difference={:?} composed_difference={:?}",
            reduction.direct_difference, reduction.composed_difference
        ),
        subreceipts: Vec::new(),
    });
    receipts.push(ObligationEvidenceReceipt {
        id: "RED_CHAIN_NORMAL_FORM".into(),
        label: "reduction normal-form preservation".into(),
        verdict: if reduction.reduction_normal_form_preserved {
            CertificationVerdict::Certified
        } else {
            CertificationVerdict::BlockedOpen
        },
        computed: true,
        detail: format!(
            "remainder_normal_form={:?}",
            reduction.remainder_normal_form
        ),
        subreceipts: Vec::new(),
    });
    receipts.push(ObligationEvidenceReceipt {
        id: "RED_CHAIN_REMAINDER_TRANSPORT".into(),
        label: "remainder transport compatibility".into(),
        verdict: if reduction.remainder_transport_compatible {
            CertificationVerdict::Certified
        } else {
            CertificationVerdict::BlockedOpen
        },
        computed: true,
        detail:
            "route-local remainder witness remains stable under the active brace/chart transport"
                .into(),
        subreceipts: Vec::new(),
    });
    receipts.push(ObligationEvidenceReceipt {
        id: "RED_CHAIN_EXACT_ZERO".into(),
        label: "exact first-order remainder vanishing".into(),
        verdict: if reduction.remainder_exact_zero {
            CertificationVerdict::Certified
        } else {
            CertificationVerdict::Underspecified
        },
        computed: true,
        detail: format!(
            "remainder_normal_form={:?}",
            reduction.remainder_normal_form
        ),
        subreceipts: Vec::new(),
    });
    if let Some(boundary) = reduction.residual_boundary {
        receipts.push(ObligationEvidenceReceipt {
            id: "RED_CHAIN_EXACTNESS_BOUNDARY".into(),
            label: "route-local reduction exactness boundary".into(),
            verdict: CertificationVerdict::Underspecified,
            computed: false,
            detail: boundary,
            subreceipts: Vec::new(),
        });
    }

    let computed_failure = receipts.iter().any(receipt_has_computed_failure);
    let residual_scaffold = receipts.iter().any(receipt_has_residual);
    let verdict = if computed_failure {
        CertificationVerdict::BlockedOpen
    } else if residual_scaffold {
        CertificationVerdict::Benchmarked
    } else {
        CertificationVerdict::Certified
    };
    let (computed_successes, residuals) = receipts
        .iter()
        .map(receipt_leaf_counts)
        .fold((0usize, 0usize), |(sa, ra), (sb, rb)| (sa + sb, ra + rb));

    Some(ObligationStatus {
        obligation_id: obligation.id.clone(),
        kind: obligation.kind.clone(),
        verdict,
        evaluation_mode: if residual_scaffold {
            ObligationEvaluationMode::RecomputedPartial
        } else {
            ObligationEvaluationMode::RecomputedExact
        },
        detail: format!(
            "chain-rule reduction factored into {} computed receipts with {} residual blocker(s)",
            computed_successes, residuals
        ),
        receipts,
    })
}

fn evaluate_flagship_type_obligation(
    theorem_id: &str,
    obligation: &Obligation,
) -> Option<ObligationStatus> {
    match (theorem_id, obligation.id.as_str()) {
        ("THS_CH_NORM", "OBL_CH_ADE") => {
            let witness = exec_ch_norm_type_witness().ok()?;
            let receipts = vec![
                ObligationEvidenceReceipt {
                    id: "CHN_ADE_INHABITANCE_PRESERVE".into(),
                    label: "inhabitance preservation".into(),
                    verdict: if witness.inhabitance_preserved {
                        CertificationVerdict::Certified
                    } else {
                        CertificationVerdict::BlockedOpen
                    },
                    computed: true,
                    detail: format!("direct_normal_form={:?}", witness.direct_normal_form),
                    subreceipts: Vec::new(),
                },
                ObligationEvidenceReceipt {
                    id: "CHN_ADE_CARRIER_COLLAPSE".into(),
                    label: "carrier-collapse exactness".into(),
                    verdict: if witness.carrier_collapse_exact {
                        CertificationVerdict::Certified
                    } else {
                        CertificationVerdict::BlockedOpen
                    },
                    computed: true,
                    detail:
                        "extensional carrier collapse preserves the canonical normalization witness"
                            .into(),
                    subreceipts: Vec::new(),
                },
            ];
            let residual = receipts.iter().any(receipt_has_residual);
            Some(ObligationStatus {
                obligation_id: obligation.id.clone(),
                kind: obligation.kind.clone(),
                verdict: if residual {
                    CertificationVerdict::BlockedOpen
                } else {
                    CertificationVerdict::Certified
                },
                evaluation_mode: if residual {
                    ObligationEvaluationMode::RecomputedPartial
                } else {
                    ObligationEvaluationMode::RecomputedExact
                },
                detail: "CH-Norm adequacy witness discharged through exact inhabitance and carrier-collapse checks".into(),
                receipts,
            })
        }
        ("THS_CH_INH", "OBL_CHI_ADE") => {
            let witness = exec_ch_inh_type_witness().ok()?;
            let receipts = vec![
                ObligationEvidenceReceipt {
                    id: "CHI_ADE_INHABITANCE_TRANSPORT".into(),
                    label: "inhabitance transport".into(),
                    verdict: if witness.inhabitance_transport_exact {
                        CertificationVerdict::Certified
                    } else {
                        CertificationVerdict::BlockedOpen
                    },
                    computed: true,
                    detail: format!("direct_carrier={:?}", witness.direct_carrier),
                    subreceipts: Vec::new(),
                },
                ObligationEvidenceReceipt {
                    id: "CHI_ADE_CARRIER_LIFT".into(),
                    label: "carrier lift exactness".into(),
                    verdict: if witness.carrier_lift_exact {
                        CertificationVerdict::Certified
                    } else {
                        CertificationVerdict::BlockedOpen
                    },
                    computed: true,
                    detail: format!("lifted_carrier={:?}", witness.lifted_carrier),
                    subreceipts: Vec::new(),
                },
            ];
            let residual = receipts.iter().any(receipt_has_residual);
            Some(ObligationStatus {
                obligation_id: obligation.id.clone(),
                kind: obligation.kind.clone(),
                verdict: if residual {
                    CertificationVerdict::BlockedOpen
                } else {
                    CertificationVerdict::Certified
                },
                evaluation_mode: if residual {
                    ObligationEvaluationMode::RecomputedPartial
                } else {
                    ObligationEvaluationMode::RecomputedExact
                },
                detail: "CH-Inh adequacy witness discharged through exact inhabitance transport and carrier lift checks".into(),
                receipts,
            })
        }
        _ => None,
    }
}

fn collect_promotion_artifact_ids(
    theorem: &l64_core::TheoremSpec,
    target: &l64_core::TargetProfile,
    obligations: &[ObligationStatus],
    adequacy_records: &[l64_core::AdequacyRecord],
    deficiencies: &[l64_core::AtlasDeficiency],
    verdict: &CertificationVerdict,
) -> Vec<String> {
    if !is_chain_rule_family(&theorem.id)
        || target.promotion_goal != l64_core::PromotionGoal::PromoteOperator
        || *verdict != CertificationVerdict::Certified
        || adequacy_records
            .iter()
            .any(|item| item.blocking && item.verdict != CertificationVerdict::Certified)
        || deficiency_blocks_promotion(deficiencies)
        || obligations
            .iter()
            .any(|item| item.evaluation_mode != ObligationEvaluationMode::RecomputedExact)
    {
        return Vec::new();
    }
    theorem
        .operators
        .iter()
        .map(|operator| {
            format!(
                "OPR_PROMOTED_{}",
                operator.replace('.', "_").to_ascii_uppercase()
            )
        })
        .collect()
}

fn collect_reused_artifact_ids(
    registry: &dyn RegistryLookup,
    theorem_id: &str,
    obligations: &[ObligationStatus],
) -> Vec<String> {
    if !is_chain_rule_family(theorem_id) {
        return Vec::new();
    }
    let Some(operator) = chain_rule_promoted_operator(registry) else {
        return Vec::new();
    };
    if obligations.iter().any(|item| {
        matches!(
            item.obligation_id.as_str(),
            "OBL_CHAIN_EQ" | "OBL_CHAIN_RED"
        ) && item.receipts.iter().any(|receipt| {
            matches!(
                receipt.id.as_str(),
                "EQR_CHAIN_OPERATOR_REUSE"
                    | "RED_CHAIN_OPERATOR_REUSE"
                    | "EQR_CHAIN_OPERATOR_DEFAULT_SELECTION"
                    | "RED_CHAIN_OPERATOR_DEFAULT_SELECTION"
            )
        })
    }) {
        vec![operator.id]
    } else {
        Vec::new()
    }
}

fn collect_checker_receipts(
    registry: &(impl RegistryLookup + Sync),
    theorem: &l64_core::TheoremSpec,
    target: &l64_core::TargetProfile,
    proof_shape_ids: &[String],
    campaign_id: Option<&str>,
    obligations: &[ObligationStatus],
) -> Vec<CheckerReceipt> {
    let kernel = ConstitutionKernel::default();
    let mut receipts = Vec::new();

    receipts.push(checker_receipt(
        CheckerReceiptKind::TheoremSpec,
        &theorem.id,
        kernel
            .validate_theorem_spec(theorem, registry)
            .map(|_| "theorem specification validated".to_string()),
    ));
    receipts.push(checker_receipt(
        CheckerReceiptKind::TargetProfile,
        &target.id,
        kernel
            .validate_target_profile(target)
            .map(|_| "target profile validated".to_string()),
    ));
    let ledger_id = format!("TRL_{}", theorem.id.trim_start_matches("THS_"));
    if let Some(ledger) = registry.get_route_ledger(&ledger_id) {
        receipts.push(checker_receipt(
            CheckerReceiptKind::RouteLedger,
            &ledger.id,
            kernel
                .validate_route_ledger(&ledger, registry)
                .map(|_| "route ledger validated".to_string()),
        ));
    }
    if let Some(campaign_id) = campaign_id {
        if let Some(campaign) = registry.get_campaign(campaign_id) {
            receipts.push(checker_receipt(
                CheckerReceiptKind::Campaign,
                &campaign.id,
                kernel
                    .validate_campaign(&campaign, registry)
                    .map(|_| "campaign validated".to_string()),
            ));
        }
    }
    let certificate_id = format!("CRT_{}", theorem.id.trim_start_matches("THS_"));
    if let Some(certificate) = registry.get_certificate(&certificate_id) {
        receipts.push(checker_receipt(
            CheckerReceiptKind::Certificate,
            &certificate.id,
            kernel
                .validate_certificate(&certificate, registry)
                .map(|_| "certificate validated".to_string()),
        ));
    }
    for proof_shape_id in proof_shape_ids {
        if let Some(shape) = registry.get_proof_shape(proof_shape_id) {
            receipts.push(checker_receipt(
                CheckerReceiptKind::ProofShape,
                &shape.id,
                kernel
                    .check_proof_shape(&shape, registry)
                    .map_err(l64_kernel::KernelError::Message)
                    .map(|_| "proof shape validated".to_string()),
            ));
        }
    }
    for clause in registry.adequacy_clauses().into_iter().filter(|item| {
        item.theorem_ids.is_empty() || item.theorem_ids.iter().any(|id| id == &theorem.id)
    }) {
        let clause_id = clause.id.clone();
        receipts.push(checker_receipt(
            CheckerReceiptKind::AdequacyClause,
            &clause_id,
            kernel
                .validate_adequacy_clause(&clause, registry)
                .map(|_| "adequacy clause validated".to_string()),
        ));
    }
    for obligation in obligations {
        receipts.push(checker_receipt(
            CheckerReceiptKind::ObligationVerdict,
            &obligation.obligation_id,
            kernel
                .validate_obligation_status(obligation)
                .map(|_| "obligation verdict validated".to_string()),
        ));
    }
    for pack_id in collect_related_burden_pack_ids(registry, campaign_id) {
        if let Some(pack) = registry.get_burden_pack(&pack_id) {
            receipts.push(checker_receipt(
                CheckerReceiptKind::BurdenPack,
                &pack.id,
                Ok("burden pack validated structurally".to_string()),
            ));
        }
    }
    for packet_id in collect_related_claim_packet_ids(registry, campaign_id) {
        if let Some(packet) = registry.get_claim_packet(&packet_id) {
            receipts.push(checker_receipt(
                CheckerReceiptKind::ClaimPacket,
                &packet.id,
                Ok("claim packet validated structurally".to_string()),
            ));
        }
    }
    for contract_id in collect_related_evidence_contract_ids(registry, campaign_id) {
        if let Some(contract) = registry.get_evidence_contract(&contract_id) {
            receipts.push(checker_receipt(
                CheckerReceiptKind::EvidenceContract,
                &contract.id,
                Ok("evidence contract validated structurally".to_string()),
            ));
        }
    }
    for receipt_id in collect_related_benchmark_receipt_ids(registry, campaign_id) {
        if let Some(receipt) = registry.get_benchmark_receipt(&receipt_id) {
            receipts.push(checker_receipt(
                CheckerReceiptKind::BenchmarkReceipt,
                &receipt.id,
                Ok("benchmark receipt validated structurally".to_string()),
            ));
        }
    }
    for receipt_id in collect_related_challenge_receipt_ids(registry, campaign_id) {
        if let Some(receipt) = registry.get_challenge_receipt(&receipt_id) {
            receipts.push(checker_receipt(
                CheckerReceiptKind::ChallengeReceipt,
                &receipt.id,
                Ok("challenge receipt validated structurally".to_string()),
            ));
        }
    }
    for packet_id in collect_related_reproducibility_packet_ids(registry, campaign_id) {
        if let Some(packet) = registry.get_reproducibility_packet(&packet_id) {
            receipts.push(checker_receipt(
                CheckerReceiptKind::ReproducibilityPacket,
                &packet.id,
                Ok("reproducibility packet validated structurally".to_string()),
            ));
        }
    }

    receipts.sort_by(|left, right| left.id.cmp(&right.id));
    receipts
}

fn checker_receipt(
    kind: CheckerReceiptKind,
    subject_id: &str,
    result: Result<String, l64_kernel::KernelError>,
) -> CheckerReceipt {
    let id = format!(
        "CHK_{}",
        stable_hash(&(subject_id.to_string() + &format!("{kind:?}")))
    );
    match result {
        Ok(detail) => CheckerReceipt {
            id,
            kind,
            subject_id: subject_id.to_string(),
            verdict: CertificationVerdict::Certified,
            detail,
        },
        Err(error) => CheckerReceipt {
            id,
            kind,
            subject_id: subject_id.to_string(),
            verdict: CertificationVerdict::BlockedOpen,
            detail: error.to_string(),
        },
    }
}

fn collect_default_selected_artifact_ids(
    registry: &dyn RegistryLookup,
    theorem_id: &str,
    obligations: &[ObligationStatus],
) -> Vec<String> {
    if !is_chain_rule_family(theorem_id) {
        return Vec::new();
    }
    let Some(operator) = chain_rule_promoted_operator(registry) else {
        return Vec::new();
    };
    if operator.origin != ArtifactOrigin::Seed {
        return Vec::new();
    }
    if obligations.iter().any(|item| {
        matches!(
            item.obligation_id.as_str(),
            "OBL_CHAIN_EQ" | "OBL_CHAIN_RED"
        ) && item.receipts.iter().any(|receipt| {
            matches!(
                receipt.id.as_str(),
                "EQR_CHAIN_OPERATOR_DEFAULT_SELECTION" | "RED_CHAIN_OPERATOR_DEFAULT_SELECTION"
            )
        })
    }) {
        vec![operator.id]
    } else {
        Vec::new()
    }
}

fn chain_rule_promoted_operator(
    registry: &dyn RegistryLookup,
) -> Option<ChainRulePromotedOperatorContext> {
    let object = registry.get_object("OPR_PROMOTED_OPR_CHAIN1")?;
    if object.constraint.regime == "R_CALC"
        && matches!(
            object.evidence.maturity,
            l64_core::EvidenceMaturity::Validated | l64_core::EvidenceMaturity::Certified
        )
        && object.evidence.gate_verdict == l64_core::GateVerdict::Pass
    {
        Some(ChainRulePromotedOperatorContext {
            id: object.id,
            origin: registry.get_object_origin("OPR_PROMOTED_OPR_CHAIN1"),
        })
    } else {
        None
    }
}

fn collect_payoff_receipt_ids(
    theorem_id: &str,
    obligations: &[ObligationStatus],
    default_selected_artifact_ids: &[String],
) -> Vec<String> {
    if default_selected_artifact_ids.is_empty() {
        return Vec::new();
    }
    let mut receipts = Vec::new();
    match theorem_id {
        "THS_CHAIN_RULE_RECIPE" => {
            if obligations.iter().any(|item| {
                item.obligation_id == "OBL_CHAIN_EQ"
                    && item
                        .receipts
                        .iter()
                        .any(|receipt| receipt.id == "EQR_CHAIN_OPERATOR_DEFAULT_SELECTION")
            }) {
                receipts.push("PAY_CHAIN1_SECOND_BURDEN_DEFAULT_REUSE".into());
            }
            if obligations.len() < 4 {
                receipts.push(format!(
                    "PAY_CHAIN1_OBLIGATION_COUNT_REDUCTION_{}_TO_{}",
                    4,
                    obligations.len()
                ));
            }
        }
        "THS_CHAIN_RULE_TRANSPORT" => {
            if obligations.iter().any(|item| {
                item.obligation_id == "OBL_CHAIN_EQ"
                    && item
                        .receipts
                        .iter()
                        .any(|receipt| receipt.id == "EQR_CHAIN_OPERATOR_DEFAULT_SELECTION")
            }) {
                receipts.push("PAY_CHAIN1_ADJACENT_TRANSPORT_DEFAULT_REUSE".into());
            }
            if obligations
                .iter()
                .any(|item| item.obligation_id == "OBL_CHAIN_LOC")
            {
                receipts.push("PAY_CHAIN1_LOCALITY_WITNESS_RETAINED".into());
            }
            if obligations.len() < 4 {
                receipts.push(format!(
                    "PAY_CHAIN1_OBLIGATION_COUNT_REDUCTION_{}_TO_{}",
                    4,
                    obligations.len()
                ));
            }
        }
        _ => {}
    }
    receipts
}

fn is_chain_rule_family(theorem_id: &str) -> bool {
    matches!(
        theorem_id,
        "THS_CHAIN_RULE" | "THS_CHAIN_RULE_RECIPE" | "THS_CHAIN_RULE_TRANSPORT"
    )
}

fn supports_active_adequacy(theorem_id: &str) -> bool {
    is_chain_rule_family(theorem_id)
        || matches!(
            theorem_id,
            "THS_BAYES_BRACE"
                | "THS_CH_NORM"
                | "THS_EXEC_INFER"
                | "THS_PROB_JUDG"
                | "THS_CERT_PROP"
                | "THS_CH_INH"
        )
}

fn payoff_reason(theorem_id: &str, payoff_receipt_ids: &[String]) -> String {
    match theorem_id {
        "THS_CHAIN_RULE_RECIPE" => {
            "default-selected Chain₁ reduced the adjacent recipe burden relative to the full flagship obligation stack"
                .into()
        }
        "THS_CHAIN_RULE_TRANSPORT" => {
            if payoff_receipt_ids
                .iter()
                .any(|item| item == "PAY_CHAIN1_LOCALITY_WITNESS_RETAINED")
            {
                "default-selected Chain₁ reduced the adjacent transport burden while retaining a live locality obligation on the same route cluster"
                    .into()
            } else {
                "default-selected Chain₁ reduced the adjacent transport burden on the same route cluster".into()
            }
        }
        _ => format!("operator payoff receipts: {}", payoff_receipt_ids.join(",")),
    }
}

fn adequacy_cluster_reason(theorem_id: &str) -> String {
    match theorem_id {
        "THS_BAYES_BRACE" => {
            "active adequacy clauses for the touched R_TOP/R_PROB/bridge cluster all discharged on the selected route"
                .into()
        }
        "THS_CH_NORM" => {
            "active adequacy clauses for the touched R_TYP/R_SET/bridge cluster all discharged on the selected route"
                .into()
        }
        "THS_EXEC_INFER" => {
            "active adequacy clauses for the touched R_PROB/R_COMP/bridge cluster all discharged on the selected route"
                .into()
        }
        "THS_PROB_JUDG" => {
            "active adequacy clauses for the touched R_PROB/R_LOG/bridge cluster all discharged on the selected route"
                .into()
        }
        "THS_CERT_PROP" => {
            "active adequacy clauses for the touched R_COMP/R_LOG/bridge cluster all discharged on the selected route"
                .into()
        }
        "THS_CH_INH" => {
            "active adequacy clauses for the touched R_TYP/R_ALG/bridge cluster all discharged on the selected route"
                .into()
        }
        _ => {
            "active adequacy clauses for the touched R_TOP/R_CALC/bridge cluster all discharged on the selected route"
                .into()
        }
    }
}

fn evaluate_obligation(
    registry: &(impl RegistryLookup + Sync),
    theorem_id: &str,
    theorem_hosts: &[String],
    obligation: Obligation,
    evaluator: &(impl ObligationEvaluator + Sync),
    evaluator_policy: &l64_core::EvaluatorPolicyConfig,
) -> ObligationStatus {
    if let Some(status) = evaluate_flagship_type_obligation(theorem_id, &obligation) {
        return apply_evaluator_policy(status, evaluator_policy);
    }
    if let Some(status) =
        evaluate_flagship_equivalence_obligation(registry, theorem_id, &obligation)
    {
        return apply_evaluator_policy(status, evaluator_policy);
    }
    if let Some(status) = evaluate_flagship_reduction_obligation(registry, theorem_id, &obligation)
    {
        return apply_evaluator_policy(status, evaluator_policy);
    }
    for regime in theorem_hosts {
        if evaluator.supports(regime, &obligation.kind) {
            if let Some(status) = evaluator.evaluate(regime, &obligation) {
                return apply_evaluator_policy(status, evaluator_policy);
            }
        }
    }
    let can_use_stored = matches!(
        evaluator_policy.evidence_preference,
        EvidencePreference::PreferStored | EvidencePreference::StoredOnlyWhenUnavailable
    );
    if can_use_stored && registry.get_transform_receipt(&obligation.id).is_some() {
        return ObligationStatus {
            obligation_id: obligation.id,
            kind: obligation.kind,
            verdict: obligation.status,
            evaluation_mode: ObligationEvaluationMode::StoredReceiptUsed,
            detail: "stored transform receipt reused".into(),
            receipts: Vec::new(),
        };
    }
    let mut unsupported = ObligationStatus {
        obligation_id: obligation.id,
        kind: obligation.kind,
        verdict: if evaluator_policy.unsupported_mode == UnsupportedHandlingMode::StrictFail {
            CertificationVerdict::BlockedOpen
        } else {
            obligation.status
        },
        evaluation_mode: ObligationEvaluationMode::Unsupported,
        detail: "current executable kernel does not support recomputation for this obligation"
            .into(),
        receipts: Vec::new(),
    };
    if evaluator_policy.unsupported_mode == UnsupportedHandlingMode::StrictFail {
        unsupported
            .detail
            .push_str("; strict evaluator policy forbids unsupported fallback");
    }
    unsupported
}

fn evaluate_obligations(
    registry: &(impl RegistryLookup + Sync),
    theorem_id: &str,
    campaign_id: Option<&str>,
    theorem_hosts: &[String],
    obligations: Vec<Obligation>,
    evaluator: &(impl ObligationEvaluator + Sync),
    policy_resolution: &l64_core::PolicyResolution,
    options: &CertificationOptions,
) -> Result<ObligationExecutionArtifacts, CertError> {
    let namespace = ensure_cache_subdir("obligation-shards").map_err(CertError::Message)?;
    let namespace_id = namespace
        .parent()
        .and_then(|path| path.file_name())
        .map(|item| item.to_string_lossy().to_string())
        .unwrap_or_else(|| "default".into());
    let mut sorted = obligations;
    sorted.sort_by(|left, right| left.id.cmp(&right.id));
    let mut nodes = Vec::new();
    let mut legality = Vec::new();
    let mut shards = Vec::new();
    let mut write_sets = Vec::new();
    let mut collisions = Vec::new();
    let mut reused_ids = Vec::new();
    let mut rerun_ids = Vec::new();
    let mut notes = Vec::new();

    for obligation in &sorted {
        let class = obligation_concurrency_class(&obligation.kind, policy_resolution);
        let replay_allowed = obligation_replay_allowed(obligation, policy_resolution, options);
        legality.push(ReplayLegalityCheck {
            id: format!(
                "RLC_{}",
                stable_hash(&(theorem_id.to_string() + &obligation.id))
            ),
            obligation_id: obligation.id.clone(),
            allowed: replay_allowed,
            reasons: replay_legality_reasons(obligation, &class, policy_resolution, options),
        });
        if replay_allowed {
            reused_ids.push(obligation.id.clone());
        } else {
            rerun_ids.push(obligation.id.clone());
        }
        let barrier_after = matches!(
            class,
            ObligationConcurrencyClass::CanonicalizationSensitive
                | ObligationConcurrencyClass::StrictlySerialized
        );
        nodes.push(ObligationDagNode {
            id: format!("ODN_{}", obligation.id),
            obligation_id: obligation.id.clone(),
            concurrency_class: class.clone(),
            barrier_after,
            notes: vec![format!("obligation class {:?}", class)],
        });
    }
    let (edges, barriers, group_waves, wave_notes) =
        build_obligation_waves(theorem_id, &sorted, policy_resolution);
    let groups = group_waves.iter().flatten().cloned().collect::<Vec<_>>();
    notes.extend(wave_notes);

    let allow_parallel = (policy_resolution.scheduler.allow_parallel_obligations
        || options.force_parallel_obligations)
        && options
            .max_obligation_workers
            .unwrap_or(policy_resolution.scheduler.max_obligation_workers)
            > 1
        && groups.len() > 1;
    let worker_count = if allow_parallel {
        options
            .max_obligation_workers
            .unwrap_or(policy_resolution.scheduler.max_obligation_workers)
            .min(groups.len())
            .max(1)
    } else {
        1
    };
    let lanes = assign_obligation_lanes(&group_waves, worker_count, allow_parallel);

    let mut results = Vec::new();
    if allow_parallel {
        let mut any_parallel_wave = false;
        let mut next_index = 0usize;
        for wave in &group_waves {
            if wave.len() > 1 {
                any_parallel_wave = true;
                let (tx, rx) = mpsc::channel();
                thread::scope(|scope| {
                    for (offset, group) in wave.iter().cloned().enumerate() {
                        let tx = tx.clone();
                        let obligations = sorted.clone();
                        let hosts = theorem_hosts.to_vec();
                        let policy = policy_resolution.evaluator.clone();
                        scope.spawn(move || {
                            let mut statuses = Vec::new();
                            for obligation_id in &group.obligation_ids {
                                if let Some(obligation) =
                                    obligations.iter().find(|item| &item.id == obligation_id)
                                {
                                    statuses.push(evaluate_obligation(
                                        registry,
                                        theorem_id,
                                        &hosts,
                                        obligation.clone(),
                                        evaluator,
                                        &policy,
                                    ));
                                }
                            }
                            let _ = tx.send((offset, group, statuses));
                        });
                    }
                });
                drop(tx);
                let mut wave_results = Vec::new();
                for item in rx {
                    wave_results.push(item);
                }
                wave_results.sort_by_key(|(offset, _, _)| *offset);
                for (_, group, statuses) in wave_results {
                    results.push((next_index, group, statuses));
                    next_index += 1;
                }
            } else {
                for group in wave {
                    let mut statuses = Vec::new();
                    for obligation_id in &group.obligation_ids {
                        if let Some(obligation) =
                            sorted.iter().find(|item| &item.id == obligation_id)
                        {
                            statuses.push(evaluate_obligation(
                                registry,
                                theorem_id,
                                theorem_hosts,
                                obligation.clone(),
                                evaluator,
                                &policy_resolution.evaluator,
                            ));
                        }
                    }
                    results.push((next_index, group.clone(), statuses));
                    next_index += 1;
                }
            }
        }
        if any_parallel_wave {
            notes.push("independent obligation groups executed in parallel".into());
        } else {
            notes.push("obligation barriers limited the campaign to serialized waves despite parallel policy".into());
        }
    } else {
        for (index, group) in groups.iter().cloned().enumerate() {
            let mut statuses = Vec::new();
            for obligation_id in &group.obligation_ids {
                if let Some(obligation) = sorted.iter().find(|item| &item.id == obligation_id) {
                    statuses.push(evaluate_obligation(
                        registry,
                        theorem_id,
                        theorem_hosts,
                        obligation.clone(),
                        evaluator,
                        &policy_resolution.evaluator,
                    ));
                }
            }
            results.push((index, group, statuses));
        }
        notes.push(
            "obligation execution remained serialized under current policy or barrier conditions"
                .into(),
        );
    }

    let mut statuses = Vec::new();
    for (_, group, group_statuses) in &results {
        shards.push(ObligationCacheShard {
            id: format!("OCS_{}", stable_hash(&(theorem_id.to_string() + &group.id))),
            obligation_ids: group.obligation_ids.clone(),
            namespace_id: namespace_id.clone(),
        });
        write_sets.push(ObligationWriteSet {
            id: format!(
                "OWS_{}",
                stable_hash(&(theorem_id.to_string() + &group.id + "|writes"))
            ),
            artifact_ids: group
                .obligation_ids
                .iter()
                .map(|id| format!("OBL_RESULT_{id}"))
                .collect(),
            notes: vec!["obligation results staged and merged deterministically".into()],
        });
        if group.obligation_ids.len() > 1
            && matches!(
                group.concurrency_class,
                ObligationConcurrencyClass::ReplaySafeWriteSensitive
                    | ObligationConcurrencyClass::CacheSensitive
            )
        {
            collisions.push(ObligationCollisionReport {
                id: format!("OCR_{}", stable_hash(&(group.id.clone() + "|collision"))),
                obligation_ids: group.obligation_ids.clone(),
                reasons: vec!["group remained within a single deterministic lane due to cache/write sensitivity".into()],
            });
        }
        statuses.extend(group_statuses.clone());
    }
    statuses.sort_by(|left, right| left.obligation_id.cmp(&right.obligation_id));

    let ordering_receipt = ObligationOrderingReceipt {
        id: format!(
            "OORD_{}",
            stable_hash(&(theorem_id.to_string() + "|obligations"))
        ),
        ordered_group_ids: groups.iter().map(|item| item.id.clone()).collect(),
        notes: vec!["obligation groups merged in stable group order".into()],
    };
    let merge_receipt = ObligationMergeReceipt {
        id: format!("OMER_{}", stable_hash(&(theorem_id.to_string() + "|merge"))),
        merged_obligation_ids: statuses
            .iter()
            .map(|item| item.obligation_id.clone())
            .collect(),
        notes: vec![
            "obligation statuses merged deterministically into certification report".into(),
        ],
    };
    let replay_merge_receipt = ReplayMergeReceipt {
        id: format!(
            "RMR_{}",
            stable_hash(&(theorem_id.to_string() + "|replay-merge"))
        ),
        reused_obligation_ids: reused_ids,
        rerun_obligation_ids: rerun_ids,
        notes: vec!["replay legality checked at obligation granularity before merge".into()],
    };
    let namespace_receipt = ObligationNamespaceReceipt {
        id: format!(
            "ONR_{}",
            stable_hash(&(theorem_id.to_string() + "|namespace"))
        ),
        namespace_id: namespace_id.clone(),
        shard_ids: shards.iter().map(|item| item.id.clone()).collect(),
        notes: vec!["obligation shards resolved beneath the active cache namespace".into()],
    };
    let plan = ObligationPlan {
        id: format!(
            "OPL_{}",
            stable_hash(&(theorem_id.to_string() + campaign_id.unwrap_or("THEOREM")))
        ),
        theorem_id: theorem_id.into(),
        campaign_id: campaign_id.map(ToString::to_string),
        nodes,
        edges,
        groups: groups.clone(),
        notes: notes.clone(),
    };
    Ok(ObligationExecutionArtifacts {
        statuses,
        plan,
        lanes,
        ordering_receipt,
        merge_receipt,
        replay_legality_checks: legality,
        replay_barrier_receipts: barriers,
        replay_merge_receipt,
        replay_divergence_records: Vec::new(),
        cache_shards: shards,
        write_sets,
        collision_reports: collisions,
        namespace_receipt,
        notes,
    })
}

fn obligation_concurrency_class(
    kind: &ObligationKind,
    policy_resolution: &l64_core::PolicyResolution,
) -> ObligationConcurrencyClass {
    if policy_resolution
        .scheduler
        .serialize_canonicalization_sensitive
    {
        match kind {
            ObligationKind::OblEq | ObligationKind::OblRed => {
                return ObligationConcurrencyClass::CanonicalizationSensitive;
            }
            _ => {}
        }
    }
    match kind {
        ObligationKind::OblLoc | ObligationKind::OblGlu => {
            ObligationConcurrencyClass::ReplaySafeWriteSensitive
        }
        ObligationKind::OblTol | ObligationKind::OblObs => {
            ObligationConcurrencyClass::CacheSensitive
        }
        ObligationKind::OblAde => ObligationConcurrencyClass::StrictlySerialized,
        _ => ObligationConcurrencyClass::ParallelSafe,
    }
}

fn obligation_replay_allowed(
    obligation: &Obligation,
    policy_resolution: &l64_core::PolicyResolution,
    options: &CertificationOptions,
) -> bool {
    if !policy_resolution.replay_cache.replay_allowed || options.no_cache {
        return false;
    }
    if !(policy_resolution.scheduler.allow_parallel_obligation_replay
        || options.force_parallel_obligations)
        && options.replay_only
    {
        return false;
    }
    !matches!(
        obligation_concurrency_class(&obligation.kind, policy_resolution),
        ObligationConcurrencyClass::StrictlySerialized
    )
}

fn replay_legality_reasons(
    obligation: &Obligation,
    class: &ObligationConcurrencyClass,
    policy_resolution: &l64_core::PolicyResolution,
    options: &CertificationOptions,
) -> Vec<String> {
    let mut reasons = Vec::new();
    if !policy_resolution.replay_cache.replay_allowed {
        reasons.push("replay policy disabled obligation reuse".into());
    }
    if options.no_cache {
        reasons.push("no-cache execution requested".into());
    }
    if matches!(class, ObligationConcurrencyClass::StrictlySerialized) {
        reasons.push("strictly serialized obligation cannot participate in parallel replay".into());
    }
    if options.replay_only && !policy_resolution.scheduler.allow_parallel_obligation_replay {
        reasons.push("scheduler policy forbids parallel obligation replay".into());
    }
    if reasons.is_empty() {
        reasons.push(format!(
            "obligation {} is eligible for replay-aware grouped execution",
            obligation.id
        ));
    }
    reasons
}

fn build_obligation_waves(
    theorem_id: &str,
    obligations: &[Obligation],
    policy_resolution: &l64_core::PolicyResolution,
) -> (
    Vec<ObligationDagEdge>,
    Vec<ReplayBarrierReceipt>,
    Vec<Vec<ObligationGroup>>,
    Vec<String>,
) {
    use std::collections::{BTreeMap, BTreeSet, HashMap};

    let classes = obligations
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                obligation_concurrency_class(&item.kind, policy_resolution),
            )
        })
        .collect::<HashMap<_, _>>();
    let dependencies = obligation_dependency_pairs(theorem_id, obligations, policy_resolution);
    let mut incoming = obligations
        .iter()
        .map(|item| (item.id.clone(), 0usize))
        .collect::<HashMap<_, _>>();
    let mut outgoing = obligations
        .iter()
        .map(|item| (item.id.clone(), Vec::<String>::new()))
        .collect::<HashMap<_, _>>();
    let mut edges = Vec::new();
    let mut barriers = Vec::new();

    for (from, to, reason) in dependencies {
        if let Some(count) = incoming.get_mut(&to) {
            *count += 1;
        }
        outgoing.entry(from.clone()).or_default().push(to.clone());
        edges.push(ObligationDagEdge {
            id: format!("ODE_{}", stable_hash(&(from.clone() + &to))),
            from: format!("ODN_{from}"),
            to: format!("ODN_{to}"),
            reason: reason.clone(),
        });
        barriers.push(ReplayBarrierReceipt {
            id: format!("RBR_{}", stable_hash(&(from.clone() + &to))),
            obligation_ids: vec![from, to],
            reason,
        });
    }

    let obligation_ids = obligations
        .iter()
        .map(|item| item.id.clone())
        .collect::<BTreeSet<_>>();
    let mut remaining = obligation_ids.clone();
    let mut waves = Vec::new();
    let mut notes = Vec::new();

    while !remaining.is_empty() {
        let mut ready = obligations
            .iter()
            .filter(|item| remaining.contains(&item.id))
            .filter(|item| incoming.get(&item.id).copied().unwrap_or_default() == 0)
            .map(|item| item.id.clone())
            .collect::<Vec<_>>();
        ready.sort();
        if ready.is_empty() {
            if let Some(fallback) = remaining.iter().next().cloned() {
                ready.push(fallback.clone());
                notes.push(format!(
                    "obligation cycle fallback engaged at {fallback}; forced deterministic serialization"
                ));
            }
        }

        let mut buckets = BTreeMap::<String, Vec<String>>::new();
        for id in &ready {
            let class = classes
                .get(id)
                .cloned()
                .unwrap_or(ObligationConcurrencyClass::StrictlySerialized);
            let bucket_key = match class {
                ObligationConcurrencyClass::ParallelSafe => "parallel".to_string(),
                _ => id.clone(),
            };
            buckets.entry(bucket_key).or_default().push(id.clone());
        }

        let mut wave_groups = buckets
            .into_values()
            .map(|ids| {
                let class = classes
                    .get(&ids[0])
                    .cloned()
                    .unwrap_or(ObligationConcurrencyClass::StrictlySerialized);
                let serialized_reason = if ids.len() == 1 {
                    match class {
                        ObligationConcurrencyClass::ParallelSafe => Some(
                            "dependency wave contained a single parallel-safe obligation".into(),
                        ),
                        ObligationConcurrencyClass::ReplaySafeWriteSensitive => Some(
                            "write-sensitive obligation isolated into its own deterministic group"
                                .into(),
                        ),
                        ObligationConcurrencyClass::CacheSensitive => Some(
                            "cache-sensitive obligation isolated into its own deterministic group"
                                .into(),
                        ),
                        ObligationConcurrencyClass::CanonicalizationSensitive => Some(
                            "canonicalization-sensitive obligation forms an explicit barrier group"
                                .into(),
                        ),
                        ObligationConcurrencyClass::StrictlySerialized => Some(
                            "strictly serialized obligation forms an explicit barrier group".into(),
                        ),
                    }
                } else {
                    None
                };
                ObligationGroup {
                    id: format!("OGR_{}", stable_hash(&ids.join("|"))),
                    concurrency_class: class,
                    obligation_ids: ids,
                    serialized_reason,
                }
            })
            .collect::<Vec<_>>();
        wave_groups.sort_by(|left, right| left.id.cmp(&right.id));
        if wave_groups.len() > 1 {
            notes.push(format!(
                "dependency wave {} exposes {} independent obligation groups",
                waves.len(),
                wave_groups.len()
            ));
        }
        waves.push(wave_groups);

        for id in ready {
            remaining.remove(&id);
            if let Some(nexts) = outgoing.get(&id) {
                for next in nexts {
                    if let Some(count) = incoming.get_mut(next) {
                        *count = count.saturating_sub(1);
                    }
                }
            }
        }
    }

    (edges, barriers, waves, notes)
}

fn obligation_dependency_pairs(
    theorem_id: &str,
    obligations: &[Obligation],
    policy_resolution: &l64_core::PolicyResolution,
) -> Vec<(String, String, String)> {
    let ids = obligations
        .iter()
        .map(|item| item.id.as_str())
        .collect::<std::collections::HashSet<_>>();
    if theorem_id == "THS_CHAIN_RULE" {
        let mut pairs = Vec::new();
        if ids.contains("OBL_CHAIN_ADM") && ids.contains("OBL_CHAIN_RED") {
            pairs.push((
                "OBL_CHAIN_ADM".into(),
                "OBL_CHAIN_RED".into(),
                "chain rule reduction depends on derivative admissibility".into(),
            ));
        }
        if ids.contains("OBL_CHAIN_LOC") && ids.contains("OBL_CHAIN_RED") {
            pairs.push((
                "OBL_CHAIN_LOC".into(),
                "OBL_CHAIN_RED".into(),
                "chain rule remainder reduction depends on brace-local compatibility".into(),
            ));
        }
        if ids.contains("OBL_CHAIN_RED") && ids.contains("OBL_CHAIN_EQ") {
            pairs.push((
                "OBL_CHAIN_RED".into(),
                "OBL_CHAIN_EQ".into(),
                "first-order equivalence discharge waits on remainder reduction".into(),
            ));
        }
        return pairs;
    }

    let mut pairs = Vec::new();
    for window in obligations.windows(2) {
        if let [left, right] = window {
            let left_class = obligation_concurrency_class(&left.kind, policy_resolution);
            let right_class = obligation_concurrency_class(&right.kind, policy_resolution);
            if matches!(
                left_class,
                ObligationConcurrencyClass::CanonicalizationSensitive
                    | ObligationConcurrencyClass::StrictlySerialized
            ) || matches!(
                right_class,
                ObligationConcurrencyClass::CanonicalizationSensitive
                    | ObligationConcurrencyClass::StrictlySerialized
            ) {
                pairs.push((
                    left.id.clone(),
                    right.id.clone(),
                    "canonicalization-sensitive or strict obligation barrier".into(),
                ));
            }
        }
    }
    pairs
}

fn assign_obligation_lanes(
    waves: &[Vec<ObligationGroup>],
    worker_count: usize,
    allow_parallel: bool,
) -> Vec<ObligationLaneRecord> {
    let lane_count = if allow_parallel {
        worker_count.max(1)
    } else {
        1
    };
    let mut lanes = (0..lane_count)
        .map(|index| ObligationLaneRecord {
            lane_id: format!("OLAN_{index}"),
            group_ids: Vec::new(),
            obligation_ids: Vec::new(),
            serialized_reason: if allow_parallel {
                None
            } else {
                Some("obligation policy or barriers required serialized execution".into())
            },
        })
        .collect::<Vec<_>>();
    for wave in waves {
        for (index, group) in wave.iter().enumerate() {
            let lane_index = if allow_parallel {
                index % lane_count
            } else {
                0
            };
            lanes[lane_index].group_ids.push(group.id.clone());
            lanes[lane_index]
                .obligation_ids
                .extend(group.obligation_ids.clone());
        }
    }
    lanes
}

fn apply_evaluator_policy(
    mut status: ObligationStatus,
    evaluator_policy: &l64_core::EvaluatorPolicyConfig,
) -> ObligationStatus {
    if !evaluator_policy.allow_approximation
        && matches!(
            status.evaluation_mode,
            ObligationEvaluationMode::RecomputedApproximate
                | ObligationEvaluationMode::RecomputedPartial
        )
    {
        status.verdict = CertificationVerdict::Underspecified;
        status
            .detail
            .push_str("; approximation disallowed by evaluator policy");
    }
    status
}

fn obligation_from_result(
    regime: &str,
    obligation: Obligation,
    result: HostExecutionResult,
) -> Option<ObligationStatus> {
    let (verdict, mode, detail) = match (regime, obligation.kind.clone(), result) {
        (
            "R_SET",
            ObligationKind::OblEq,
            HostExecutionResult::Set {
                extensional_equal, ..
            },
        ) => (
            if extensional_equal {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedContradiction
            },
            ObligationEvaluationMode::RecomputedExact,
            "finite extensional equality checked".into(),
        ),
        ("R_SET", ObligationKind::OblFin, HostExecutionResult::Set { powerset_bound, .. }) => (
            CertificationVerdict::Certified,
            ObligationEvaluationMode::RecomputedExact,
            format!("finite powerset bound {powerset_bound}"),
        ),
        (
            "R_SET",
            ObligationKind::OblAdm,
            HostExecutionResult::Set {
                total_function_graph,
                injective,
                surjective,
                ..
            },
        ) => (
            if total_function_graph {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedContradiction
            },
            ObligationEvaluationMode::RecomputedPartial,
            format!(
                "finite function graph total={total_function_graph} injective={injective} surjective={surjective}"
            ),
        ),
        (
            "R_SET",
            ObligationKind::OblKnt,
            HostExecutionResult::Set {
                injective,
                surjective,
                ..
            },
        ) => (
            if injective && surjective {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedOpen
            },
            ObligationEvaluationMode::RecomputedPartial,
            format!("finite map injective={injective} surjective={surjective}"),
        ),
        (
            "R_ALG",
            ObligationKind::OblAdm,
            HostExecutionResult::Algebra {
                closure_holds,
                associative,
                distributive,
                ..
            },
        ) => (
            if closure_holds && associative {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedContradiction
            },
            ObligationEvaluationMode::RecomputedExact,
            format!(
                "closure={closure_holds} associative={associative} distributive={distributive}"
            ),
        ),
        (
            "R_ALG",
            ObligationKind::OblKnt,
            HostExecutionResult::Algebra {
                identity_holds,
                inverse_holds,
                ..
            },
        ) => (
            if identity_holds && inverse_holds {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedContradiction
            },
            ObligationEvaluationMode::RecomputedExact,
            format!("identity={identity_holds} inverse={inverse_holds}"),
        ),
        (
            "R_ALG",
            ObligationKind::OblEq,
            HostExecutionResult::Algebra {
                quotient_compatible,
                homomorphism_preserving,
                ..
            },
        ) => (
            if quotient_compatible && homomorphism_preserving {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedOpen
            },
            ObligationEvaluationMode::RecomputedPartial,
            format!(
                "quotient-compatible={quotient_compatible} homomorphism={homomorphism_preserving}"
            ),
        ),
        (
            "R_TOP",
            ObligationKind::OblLoc,
            HostExecutionResult::Topology {
                cover_legal,
                continuity_holds,
                overlap_compatible,
                ..
            },
        ) => (
            if cover_legal && continuity_holds && overlap_compatible {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedContradiction
            },
            ObligationEvaluationMode::RecomputedExact,
            format!(
                "cover={cover_legal} continuity={continuity_holds} overlap={overlap_compatible}"
            ),
        ),
        (
            "R_TOP",
            ObligationKind::OblGlu,
            HostExecutionResult::Topology {
                gluing_compatible,
                obstruction,
                ..
            },
        ) => (
            if gluing_compatible {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedOpen
            },
            ObligationEvaluationMode::RecomputedPartial,
            obstruction.unwrap_or_else(|| "finite gluing witness constructed".into()),
        ),
        (
            "R_CALC",
            ObligationKind::OblAdm,
            HostExecutionResult::Calculus {
                local_linear_witness,
                ..
            },
        ) => (
            if local_linear_witness {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedOpen
            },
            ObligationEvaluationMode::RecomputedExact,
            "local linear witness checked".into(),
        ),
        (
            "R_CALC",
            ObligationKind::OblTol,
            HostExecutionResult::Calculus {
                accumulation_ok, ..
            },
        ) => (
            if accumulation_ok {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedContradiction
            },
            ObligationEvaluationMode::RecomputedApproximate,
            "finite accumulation/toll check".into(),
        ),
        (
            "R_CALC",
            ObligationKind::OblRed,
            HostExecutionResult::Calculus {
                finite_difference_derivative,
                symbolic_only,
                ..
            },
        ) => (
            if finite_difference_derivative {
                CertificationVerdict::Certified
            } else if symbolic_only {
                CertificationVerdict::Underspecified
            } else {
                CertificationVerdict::BlockedOpen
            },
            if symbolic_only {
                ObligationEvaluationMode::RecomputedPartial
            } else {
                ObligationEvaluationMode::RecomputedApproximate
            },
            format!("finite-difference derivative witness; symbolic_only={symbolic_only}"),
        ),
        (
            "R_PROB",
            ObligationKind::OblTol,
            HostExecutionResult::Probability {
                normalized,
                expectation,
                conditioning_legal,
                pushforward_ok,
                ..
            },
        ) => (
            if normalized && conditioning_legal && pushforward_ok {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedContradiction
            },
            ObligationEvaluationMode::RecomputedExact,
            format!(
                "normalized={normalized} conditioning={conditioning_legal} pushforward={pushforward_ok} expectation={}/{}",
                expectation.num, expectation.den
            ),
        ),
        (
            "R_PROB",
            ObligationKind::OblObs,
            HostExecutionResult::Probability { independent, .. },
        ) => (
            if independent {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedOpen
            },
            ObligationEvaluationMode::RecomputedApproximate,
            format!("finite independence check independent={independent}"),
        ),
        (
            "R_PROB",
            ObligationKind::OblAde,
            HostExecutionResult::Probability {
                normalized,
                conditioning_legal,
                pushforward_ok,
                ..
            },
        ) => (
            if pushforward_ok && normalized && conditioning_legal {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedOpen
            },
            if pushforward_ok && normalized && conditioning_legal {
                ObligationEvaluationMode::RecomputedExact
            } else {
                ObligationEvaluationMode::RecomputedPartial
            },
            format!(
                "finite pushforward check pushforward_ok={} normalized={} conditioning={}",
                pushforward_ok, normalized, conditioning_legal
            ),
        ),
        (
            "R_COMP",
            ObligationKind::OblRed,
            HostExecutionResult::Computation {
                reached_normal_form,
                replayable_trace,
                ..
            },
        ) => (
            if reached_normal_form {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedContradiction
            },
            ObligationEvaluationMode::RecomputedExact,
            format!(
                "normal form reached={reached_normal_form} replayable_trace={replayable_trace}"
            ),
        ),
        (
            "R_COMP",
            ObligationKind::OblObs,
            HostExecutionResult::Computation {
                observationally_equivalent,
                ..
            },
        ) => (
            if observationally_equivalent {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedContradiction
            },
            ObligationEvaluationMode::RecomputedExact,
            format!("observational equivalence={observationally_equivalent}"),
        ),
        ("R_COMP", ObligationKind::OblTol, HostExecutionResult::Computation { cost, .. }) => (
            CertificationVerdict::Certified,
            ObligationEvaluationMode::RecomputedExact,
            format!("cost accumulation={}", cost.0),
        ),
        (
            "R_LOG",
            ObligationKind::OblAde,
            HostExecutionResult::Logic {
                proposition_well_formed,
                witness_available,
            },
        ) => (
            if proposition_well_formed && witness_available {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedOpen
            },
            ObligationEvaluationMode::RecomputedPartial,
            format!("logic proposition={proposition_well_formed} witness={witness_available}"),
        ),
        (
            "R_TYP",
            ObligationKind::OblAde,
            HostExecutionResult::TypeTheory {
                witness_inhabited, ..
            },
        ) => (
            if witness_inhabited {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedOpen
            },
            ObligationEvaluationMode::RecomputedPartial,
            format!("type witness inhabited={witness_inhabited}"),
        ),
        (
            "R_TYP",
            ObligationKind::OblRed,
            HostExecutionResult::TypeTheory {
                normalization_correspondence,
                ..
            },
        ) => (
            if normalization_correspondence {
                CertificationVerdict::Certified
            } else {
                CertificationVerdict::BlockedOpen
            },
            ObligationEvaluationMode::RecomputedApproximate,
            format!("normalization correspondence={normalization_correspondence}"),
        ),
        _ => return None,
    };
    Some(ObligationStatus {
        obligation_id: obligation.id,
        kind: obligation.kind,
        verdict,
        evaluation_mode: mode,
        detail,
        receipts: Vec::new(),
    })
}

fn execution_cache_root() -> Result<PathBuf, CertError> {
    ensure_cache_subdir("execution").map_err(CertError::Message)
}

fn execution_cache_legacy_path() -> Result<PathBuf, CertError> {
    Ok(execution_cache_root()?.join("reports.json"))
}

fn execution_cache_entries_root() -> Result<PathBuf, CertError> {
    let root = execution_cache_root()?.join("reports");
    fs::create_dir_all(&root).map_err(|err| CertError::Message(err.to_string()))?;
    Ok(root)
}

fn execution_cache_path() -> Result<PathBuf, CertError> {
    execution_cache_entries_root()
}

fn execution_cache_entry_path(cache_key: &str) -> Result<PathBuf, CertError> {
    Ok(execution_cache_entries_root()?.join(format!("{}.locus", cache_key_filename(cache_key))))
}

fn cache_key_filename(cache_key: &str) -> String {
    cache_key
        .chars()
        .map(|ch| match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' => ch,
            _ => '_',
        })
        .collect()
}

fn load_execution_cache() -> Result<ExecutionCache, CertError> {
    let root = execution_cache_entries_root()?;
    let mut cache = ExecutionCache::default();
    let mut saw_binary = false;
    for entry in fs::read_dir(&root).map_err(|err| CertError::Message(err.to_string()))? {
        let entry = entry.map_err(|err| CertError::Message(err.to_string()))?;
        let path = entry.path();
        if path.extension().and_then(|item| item.to_str()) != Some("locus") {
            continue;
        }
        saw_binary = true;
        match fs::read(&path)
            .map_err(|err| CertError::Message(err.to_string()))
            .and_then(|bytes| decode_cached_report_packet(&bytes))
        {
            Ok(report) => cache.reports.push(report),
            Err(_) => {
                let _ = fs::remove_file(&path);
            }
        }
    }
    if saw_binary {
        return Ok(cache);
    }
    let legacy = execution_cache_legacy_path()?;
    if !legacy.exists() {
        return Ok(cache);
    }
    let text = fs::read_to_string(&legacy).map_err(|err| CertError::Message(err.to_string()))?;
    match serde_json::from_str(&text) {
        Ok(cache) => Ok(cache),
        Err(_) => {
            let _ = fs::remove_file(&legacy);
            Ok(ExecutionCache::default())
        }
    }
}

fn load_cached_report(cache_key: &str) -> Result<Option<CachedCertificationReport>, CertError> {
    let path = execution_cache_entry_path(cache_key)?;
    if path.exists() {
        let bytes = fs::read(&path).map_err(|err| CertError::Message(err.to_string()))?;
        match decode_cached_report_packet(&bytes) {
            Ok(report) => return Ok(Some(report)),
            Err(_) => {
                let _ = fs::remove_file(&path);
                return Ok(None);
            }
        }
    }
    let cache = load_execution_cache()?;
    Ok(cache
        .reports
        .into_iter()
        .find(|item| item.cache_key == cache_key))
}

fn persist_cached_report(entry: CachedCertificationReport) -> Result<(), CertError> {
    let path = execution_cache_entry_path(&entry.cache_key)?;
    let bytes = encode_cached_report_packet(&entry)?;
    fs::write(path, bytes).map_err(|err| CertError::Message(err.to_string()))
}

fn cache_entry_valid(
    entry: &CachedCertificationReport,
    options: &CertificationOptions,
    current_policy_hash: &str,
    resolution: Option<&l64_core::PolicyResolution>,
) -> bool {
    let exact_policy_required = resolution
        .map(|item| item.replay_cache.exact_policy_match_required)
        .unwrap_or(true);
    let approx_ok = resolution
        .map(|item| item.replay_cache.reuse_approximate_results)
        .unwrap_or(false);
    let policy_ok = if exact_policy_required {
        entry.policy_hash == current_policy_hash
            || entry
                .report
                .execution_envelope
                .as_ref()
                .map(|env| env.policy_hash.as_str())
                == Some(current_policy_hash)
    } else {
        true
    };
    let approx_ok_for_report = approx_ok
        || !entry.report.obligations.iter().any(|item| {
            matches!(
                item.evaluation_mode,
                ObligationEvaluationMode::RecomputedApproximate
                    | ObligationEvaluationMode::RecomputedPartial
            )
        });
    entry.bundle_hash == options.bundle_hash
        && policy_ok
        && (entry.policy_hash == current_policy_hash || !exact_policy_required)
        && approx_ok_for_report
        && entry.evaluator_version == EVALUATOR_VERSION
}

fn certification_cache_key(
    theorem_id: &str,
    target_profile_id: &str,
    campaign_id: Option<&str>,
    bundle_hash: &str,
    policy_hash: &str,
    optimizer_policy: &OptimizerPolicy,
) -> String {
    stable_hash(&format!(
        "{theorem_id}|{target_profile_id}|{}|{bundle_hash}|{policy_hash}|{:?}",
        campaign_id.unwrap_or("THEOREM"),
        optimizer_policy
    ))
}

fn report_storage_id(report: &CertificationReport) -> String {
    format!(
        "REPORT_{}_{}",
        report.theorem_id,
        report
            .campaign_id
            .clone()
            .unwrap_or_else(|| "THEOREM".into())
    )
}

fn stable_hash(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

fn derive_reuse_legality_receipts(report: &CertificationReport) -> Vec<ReuseLegalityReceipt> {
    let subject_id = report
        .campaign_id
        .clone()
        .unwrap_or_else(|| report.theorem_id.clone());
    let lawful = !report.reused_artifact_ids.is_empty()
        && report
            .replay_legality_checks
            .iter()
            .all(|item| item.allowed);
    let mut basis = Vec::new();
    if !report.reused_artifact_ids.is_empty() {
        basis.push("reused-artifact-lineage-present".into());
    }
    if report
        .replay_legality_checks
        .iter()
        .all(|item| item.allowed)
    {
        basis.push("replay-legality-passed".into());
    } else {
        basis.push("replay-legality-blocked".into());
    }
    if !report.selected_path.is_empty() {
        basis.push("route-selection-resolved".into());
    }
    vec![ReuseLegalityReceipt {
        id: format!(
            "RLR_{}",
            stable_hash(&(subject_id.clone() + "|reuse-legality"))
        ),
        subject_id: subject_id.clone(),
        lawful,
        basis,
        lineage_refs: vec![format!("LIN_{subject_id}")],
        policy_scope: report
            .policy_resolution
            .as_ref()
            .map(|item| item.id.clone()),
    }]
}

fn derive_reuse_decision_receipts(
    _report: &CertificationReport,
    coverage: &ProofCoverageEnvelope,
) -> Vec<ReuseDecisionReceipt> {
    let subject_id = coverage.subject_id.clone();
    let fallback_required = !matches!(
        coverage.decision,
        CoverageDecision::Exact
            | CoverageDecision::Subsumed
            | CoverageDecision::TransportEquivalent
    ) || !coverage.residual_obligation_ids.is_empty();
    let reason = if fallback_required {
        "reuse path left residual work; fallback verification remains required"
    } else {
        "reuse path fully discharged the active coverage seam"
    };
    vec![ReuseDecisionReceipt {
        id: format!(
            "RDR_{}",
            stable_hash(&(subject_id.clone() + "|reuse-decision"))
        ),
        subject_id,
        reused_artifact_ids: coverage.covered_by.clone(),
        skipped_obligation_ids: coverage.skipped_obligation_ids.clone(),
        fallback_required,
        reason: reason.into(),
    }]
}

fn derive_residual_verification_receipts(
    report: &CertificationReport,
    coverage: &ProofCoverageEnvelope,
) -> Vec<ResidualVerificationReceipt> {
    let subject_id = coverage.subject_id.clone();
    let fully_discharged = coverage.residual_obligation_ids.is_empty();
    let mut notes = Vec::new();
    if fully_discharged {
        notes.push("no residual obligations remain after lawful reuse evaluation".into());
    } else {
        notes.push("residual obligations remain explicit and must not be silently skipped".into());
    }
    if report.reused_artifact_ids.is_empty() {
        notes.push("no promoted artifact reuse was available on this run".into());
    }
    vec![ResidualVerificationReceipt {
        id: format!(
            "RVR_{}",
            stable_hash(&(subject_id.clone() + "|residual-verification"))
        ),
        subject_id,
        residual_obligation_ids: coverage.residual_obligation_ids.clone(),
        fully_discharged,
        notes,
    }]
}

pub fn derive_execution_closure_receipt(report: &CertificationReport) -> ExecutionClosureReceipt {
    let subject_id = report
        .campaign_id
        .clone()
        .unwrap_or_else(|| report.theorem_id.clone());
    let exactness = if report.verdict == CertificationVerdict::Certified
        && report
            .obligations
            .iter()
            .all(|item| item.verdict == CertificationVerdict::Certified)
    {
        EvidenceExactness::WitnessBacked
    } else if report.obligations.iter().any(|item| {
        matches!(
            item.verdict,
            CertificationVerdict::BlockedOpen | CertificationVerdict::BlockedContradiction
        )
    }) {
        EvidenceExactness::CounterexampleCandidate
    } else {
        EvidenceExactness::Undischarged
    };
    let reuse_reported = !report.reused_artifact_ids.is_empty()
        || !report.reuse_legality_receipts.is_empty()
        || !report.reuse_decision_receipts.is_empty();
    let residuals_reported = !report.residual_verification_receipts.is_empty()
        || report
            .obligations
            .iter()
            .any(|item| item.verdict != CertificationVerdict::Certified);
    ExecutionClosureReceipt {
        id: format!("ECR_{}", stable_hash(&(subject_id.clone() + "|execution-closure"))),
        subject_id,
        exactness,
        lower_lineage_required: true,
        reuse_reported,
        residuals_reported,
        promotion_gate_visible: report.verdict == CertificationVerdict::Certified
            || !report.deficiencies.is_empty(),
        notes: vec![
            "execution/certification closure is explicit; exact proof, numeric evidence, and residual work must not be merged".into(),
        ],
    }
}

fn score_vector_to_legacy(score: &RouteScoreVector) -> Vec<usize> {
    vec![
        score.lawfulness,
        score.loss_compliance,
        score.surface_transition_penalty,
        score.execution_cost,
        score.symbolic_fidelity,
    ]
}

pub fn derive_proof_coverage_envelope(report: &CertificationReport) -> ProofCoverageEnvelope {
    let subject_id = report
        .campaign_id
        .clone()
        .unwrap_or_else(|| report.theorem_id.clone());
    let decision = if !report.default_selected_artifact_ids.is_empty() {
        CoverageDecision::Subsumed
    } else if !report.reused_artifact_ids.is_empty() {
        CoverageDecision::TransportEquivalent
    } else {
        CoverageDecision::Fallback
    };
    let skipped_obligation_ids = if report.reused_artifact_ids.is_empty() {
        Vec::new()
    } else {
        report
            .obligations
            .iter()
            .filter(|item| item.verdict == CertificationVerdict::Certified)
            .map(|item| item.obligation_id.clone())
            .collect()
    };
    let residual_obligation_ids = report
        .obligations
        .iter()
        .filter(|item| item.verdict != CertificationVerdict::Certified)
        .map(|item| item.obligation_id.clone())
        .collect();
    let reuse_envelope_hash = stable_hash(&format!(
        "{}|{}|{}|{}",
        report.theorem_id,
        report.target_profile_id,
        report.selected_atlas_cell.clone().unwrap_or_default(),
        report.selected_path.join(",")
    ));
    ProofCoverageEnvelope {
        subject_id,
        decision,
        covered_by: report.reused_artifact_ids.clone(),
        skipped_obligation_ids,
        residual_obligation_ids,
        reuse_envelope_hash,
        payoff_receipt_ids: report.payoff_receipt_ids.clone(),
    }
}

pub fn dispatch_proof_coverage(report: &CertificationReport) -> ProofCoverageDispatch {
    let coverage = derive_proof_coverage_envelope(report);
    let lineage_id = format!("LIN_{}", coverage.subject_id);
    let route_fast_path = matches!(
        coverage.decision,
        CoverageDecision::Exact
            | CoverageDecision::Subsumed
            | CoverageDecision::TransportEquivalent
    ) && coverage.residual_obligation_ids.is_empty();
    let reuse_legality_receipts = derive_reuse_legality_receipts(report);
    let reuse_decision_receipts = derive_reuse_decision_receipts(report, &coverage);
    let residual_verification_receipts = derive_residual_verification_receipts(report, &coverage);
    let reason = match coverage.decision {
        CoverageDecision::Exact => "exact promoted coverage available",
        CoverageDecision::Subsumed => "subsuming promoted coverage available",
        CoverageDecision::TransportEquivalent => "transport-equivalent promoted coverage available",
        CoverageDecision::Fallback => "fallback to proof path required",
        CoverageDecision::Unsupported => "no lawful proof coverage exists yet",
    }
    .to_string();
    ProofCoverageDispatch {
        subject_id: coverage.subject_id,
        decision: coverage.decision,
        route_fast_path,
        reason,
        covered_by: coverage.covered_by,
        residual_obligation_ids: coverage.residual_obligation_ids.clone(),
        lineage_refs: vec![lineage_id],
        reuse_legality_receipts,
        reuse_decision_receipts,
        residual_verification_receipts,
    }
}

pub fn derive_distress_vector(
    report: &CertificationReport,
    bundle: &VerticalCompoundingBundle,
) -> DistressVector {
    let repeated_blocker_ids = report
        .deficiencies
        .iter()
        .map(|item| item.id.clone())
        .collect();
    let checker_gaps = bundle
        .checker_extensions
        .iter()
        .map(|item| item.object_family.clone())
        .collect();
    let route_scarcity = if bundle.atlas_cells.is_empty() && !report.deficiencies.is_empty() {
        vec![report.selected_path.join("->")]
    } else {
        Vec::new()
    };
    let payoff_drought =
        report.payoff_receipt_ids.is_empty() && !report.promotion_artifact_ids.is_empty();
    let stalled_frontier_motion =
        !report.deficiencies.is_empty() && report.reused_artifact_ids.is_empty();
    DistressVector {
        repeated_blocker_ids,
        checker_gaps,
        route_scarcity,
        payoff_drought,
        stalled_frontier_motion,
    }
}

pub fn derive_help_request(
    report: &CertificationReport,
    distress: &DistressVector,
) -> Option<HelpRequest> {
    let severe = distress.stalled_frontier_motion
        || !distress.route_scarcity.is_empty()
        || !distress.repeated_blocker_ids.is_empty();
    if !severe {
        return None;
    }
    Some(HelpRequest {
        id: format!(
            "HELP_{}",
            report
                .campaign_id
                .clone()
                .unwrap_or_else(|| report.theorem_id.clone())
        ),
        task_ref: report
            .campaign_id
            .clone()
            .unwrap_or_else(|| report.theorem_id.clone()),
        distress_vector: distress.clone(),
        minimal_reproduction: report_storage_id(report),
        requested_capacity: if !distress.route_scarcity.is_empty() {
            "atlas-or-shell-expansion".into()
        } else if !distress.checker_gaps.is_empty() {
            "checker-extension".into()
        } else {
            "semantic-leaf".into()
        },
        expected_relief: "convert repeated blocker into bounded verified next seam".into(),
    })
}

pub fn derive_recipe_records(
    report: &CertificationReport,
    bundle: &VerticalCompoundingBundle,
) -> (
    Vec<RecipeRecord>,
    Vec<RecipeDelta>,
    Vec<PromotionCandidate>,
    CalibrationPressureMap,
    Vec<OverridePressureReceipt>,
) {
    let seam = report
        .campaign_id
        .clone()
        .unwrap_or_else(|| report.theorem_id.clone());
    let recipes = vec![RecipeRecord {
        id: format!("RCP_{seam}"),
        seam: seam.clone(),
        tactic_family: if report.deficiencies.is_empty() {
            "coverage-reuse".into()
        } else {
            "semantic-closure".into()
        },
        generated_status: GeneratedStatus::VerifiedCandidate,
        expected_relief: "lower future frontier cost on sibling burdens".into(),
    }];
    let recipe_deltas = vec![RecipeDelta {
        id: format!("RDEL_{seam}"),
        recipe_record_id: format!("RCP_{seam}"),
        changed_fields: vec!["coverage".into(), "checker".into(), "frontier".into()],
        strengthening_value: report.checker_receipts.len()
            + report.adequacy_records.len()
            + bundle.payoff_tasks.len(),
    }];
    let promotion_candidates = report
        .promotion_artifact_ids
        .iter()
        .map(|artifact_id| PromotionCandidate {
            id: format!("PRCND_{artifact_id}"),
            artifact_id: artifact_id.clone(),
            candidate_kind: "operator-or-certificate".into(),
            generated_status: GeneratedStatus::VerifiedCandidate,
            proof_grade: if report.verdict == CertificationVerdict::Certified {
                "certified".into()
            } else {
                "bounded".into()
            },
        })
        .collect();
    let calibration_pressure = CalibrationPressureMap {
        override_pressure: report.reasons.len(),
        emitted_duplication: bundle.campaigns.len() + bundle.atlas_cells.len(),
        conformance_friction: report.deficiencies.len(),
        extension_cost: bundle.compartments.len() + bundle.help_requests.len(),
    };
    let override_pressure_receipts = report
        .reasons
        .iter()
        .enumerate()
        .map(|(idx, note)| OverridePressureReceipt {
            id: format!("OPR_{seam}_{idx}"),
            source_id: seam.clone(),
            pressure_class: "reason".into(),
            notes: vec![note.clone()],
        })
        .collect();
    (
        recipes,
        recipe_deltas,
        promotion_candidates,
        calibration_pressure,
        override_pressure_receipts,
    )
}

pub fn build_vertical_compounding_bundle(
    registry: &impl RegistryLookup,
    report: &CertificationReport,
) -> VerticalCompoundingBundle {
    let subject_id = report
        .campaign_id
        .clone()
        .unwrap_or_else(|| report.theorem_id.clone());
    let frontier = Frontier {
        id: format!("FRT_{subject_id}"),
        title: format!("frontier for {subject_id}"),
        burden_class: if report.deficiencies.is_empty() {
            "reuse-expansion".into()
        } else {
            "semantic-closure".into()
        },
        blocker_ids: report
            .deficiencies
            .iter()
            .map(|item| item.id.clone())
            .collect(),
        closure_witness: if report.verdict == CertificationVerdict::Certified {
            Some(format!("CLS_{subject_id}"))
        } else {
            None
        },
        residual_budget: if report.verdict == CertificationVerdict::Certified {
            None
        } else {
            Some("bounded-seam".into())
        },
        strengthening_value: report.checker_receipts.len() + report.adequacy_records.len(),
        coldness: if report.deficiencies.is_empty() {
            "mid-to-cold".into()
        } else {
            "mid-friction".into()
        },
    };

    let semantic_leaves = report
        .deficiencies
        .iter()
        .map(|item| CandidateSemanticLeaf {
            id: format!("CND_LEAF_{}", item.id),
            frontier_id: frontier.id.clone(),
            description: item.message.clone(),
            generated_status: GeneratedStatus::VerifiedCandidate,
            expected_relief: "narrow semantic closure or sharper blocker".into(),
        })
        .collect();

    let adequacy_clauses = report
        .adequacy_records
        .iter()
        .filter(|item| item.verdict != CertificationVerdict::Certified)
        .map(|item| CandidateAdequacyClause {
            id: format!("CND_ADQ_{}", item.clause_id),
            frontier_id: frontier.id.clone(),
            clause_stub: item.clause_id.clone(),
            generated_status: GeneratedStatus::Suggestion,
            expected_relief: "exact adequacy discharge".into(),
        })
        .collect();

    let checker_extensions = report
        .checker_receipts
        .iter()
        .filter(|item| item.verdict != CertificationVerdict::Certified)
        .map(|item| CandidateCheckerExtension {
            id: format!("CND_CHK_{}", item.subject_id),
            frontier_id: frontier.id.clone(),
            object_family: format!("{:?}", item.kind),
            generated_status: GeneratedStatus::Suggestion,
            expected_relief: "increase reusable machine validation".into(),
        })
        .collect();

    let campaigns = if report.verdict == CertificationVerdict::Certified {
        Vec::new()
    } else {
        registry
            .atlas_cells()
            .into_iter()
            .take(3)
            .map(|cell| CandidateCampaign {
                id: format!("CND_CPG_{}", cell.id),
                theorem_id: report.theorem_id.clone(),
                target_profile_id: report.target_profile_id.clone(),
                generated_status: GeneratedStatus::Scaffold,
                expected_relief: format!("probe atlas cell {}", cell.id),
            })
            .collect()
    };

    let atlas_cells = report
        .deficiencies
        .iter()
        .filter(|item| matches!(item.class, AtlasDeficiencyClass::DNoRoute))
        .map(|item| CandidateAtlasCell {
            id: format!("CND_ATL_{}", item.id),
            src: report.selected_path.first().cloned().unwrap_or_default(),
            tgt: report.selected_path.last().cloned().unwrap_or_default(),
            generated_status: GeneratedStatus::Suggestion,
            expected_relief: item.message.clone(),
        })
        .collect();

    let payoff_tasks = report
        .promotion_artifact_ids
        .iter()
        .map(|artifact_id| CandidatePayoffTask {
            id: format!("CND_PAY_{artifact_id}"),
            artifact_id: artifact_id.clone(),
            burden_hint: report.theorem_id.clone(),
            generated_status: GeneratedStatus::Suggestion,
            expected_relief: "prove broader lawful fast-path coverage".into(),
        })
        .collect();

    let compartments = if report.deficiencies.is_empty() {
        Vec::new()
    } else {
        vec![SearchCompartment {
            id: format!("SCMP_{subject_id}"),
            task_fingerprint: format!("{}:{}", report.theorem_id, report.target_profile_id),
            allowed_grammars: vec!["semantic-leaf".into(), "checker-extension".into()],
            budget: "bounded-micro-generator".into(),
            evaluator_profile: EVALUATOR_VERSION.into(),
            admissibility_surface: vec!["campaign-truth".into(), "adequacy".into()],
            kill_criteria: vec!["cold-blocker".into(), "no-frontier-motion".into()],
            expected_relief: "resolve active blocker without widening host law".into(),
        }]
    };

    let generator_contracts = vec![GeneratorContract {
        id: format!("GCTR_{subject_id}"),
        proposal_kind: if report.deficiencies.is_empty() {
            ProposalKind::CheckerExtension
        } else {
            ProposalKind::SemanticLeaf
        },
        seam: subject_id.clone(),
        verifier: EVALUATOR_VERSION.into(),
        compatibility_path: report.selected_path.join("->"),
    }];

    let generation_receipts = vec![GenerationReceipt {
        id: format!("GRCPT_{subject_id}"),
        generator_contract_id: format!("GCTR_{subject_id}"),
        output_ids: vec![frontier.id.clone()],
        generated_status: GeneratedStatus::VerifiedCandidate,
        notes: vec!["derived from live campaign truth".into()],
    }];

    let mut bundle = VerticalCompoundingBundle {
        frontier_ledger: FrontierLedger {
            frontiers: vec![frontier],
            notes: vec!["generated from certification truth".into()],
        },
        semantic_leaves,
        adequacy_clauses,
        checker_extensions,
        campaigns,
        atlas_cells,
        payoff_tasks,
        compartments,
        generator_contracts,
        generation_receipts,
        distress: None,
        help_requests: Vec::new(),
        recipes: Vec::new(),
        recipe_deltas: Vec::new(),
        promotion_candidates: Vec::new(),
        calibration_pressure: None,
        override_pressure_receipts: Vec::new(),
        lineage_refs: vec![format!("LIN_{subject_id}")],
        lineage_required: true,
    };
    let distress = derive_distress_vector(report, &bundle);
    let help_request = derive_help_request(report, &distress);
    let (
        recipes,
        recipe_deltas,
        promotion_candidates,
        calibration_pressure,
        override_pressure_receipts,
    ) = derive_recipe_records(report, &bundle);
    bundle.distress = Some(distress);
    if let Some(item) = help_request {
        bundle.help_requests.push(item);
    }
    bundle.recipes = recipes;
    bundle.recipe_deltas = recipe_deltas;
    bundle.promotion_candidates = promotion_candidates;
    bundle.calibration_pressure = Some(calibration_pressure);
    bundle.override_pressure_receipts = override_pressure_receipts;
    bundle
}

pub fn encode_locus_packet_for_report(report: &CertificationReport) -> Result<Vec<u8>, CertError> {
    let capabilities = LocusCapabilityMask {
        has_checker: !report.checker_receipts.is_empty(),
        has_adequacy: !report.adequacy_records.is_empty(),
        has_route: !report.selected_path.is_empty(),
        has_certificate: report.certificate_id.is_some(),
        has_frontier: !report.deficiencies.is_empty(),
        has_forensic: !report.diagnostics.is_empty() || !report.reasons.is_empty(),
    };
    let coverage = derive_proof_coverage_envelope(report);
    let mut sections = Vec::new();
    sections.push(LocusSection {
        opcode: LocusOpcode::CanonicalPayload,
        flags: 0,
        subject_id: report.theorem_id.clone(),
        payload: bincode::serialize(report).map_err(|err| CertError::Message(err.to_string()))?,
    });
    sections.push(LocusSection {
        opcode: LocusOpcode::Coverage,
        flags: 0,
        subject_id: coverage.subject_id.clone(),
        payload: bincode::serialize(&coverage)
            .map_err(|err| CertError::Message(err.to_string()))?,
    });
    if !report.adequacy_records.is_empty() {
        sections.push(LocusSection {
            opcode: LocusOpcode::AdequacyTable,
            flags: report.adequacy_records.len() as u16,
            subject_id: report.theorem_id.clone(),
            payload: bincode::serialize(&report.adequacy_records)
                .map_err(|err| CertError::Message(err.to_string()))?,
        });
    }
    if !report.checker_receipts.is_empty() {
        sections.push(LocusSection {
            opcode: LocusOpcode::Checker,
            flags: report.checker_receipts.len() as u16,
            subject_id: report.theorem_id.clone(),
            payload: bincode::serialize(&report.checker_receipts)
                .map_err(|err| CertError::Message(err.to_string()))?,
        });
    }
    let packet = LocusPacket {
        header: LocusPacketHeader {
            artifact_class: GenomeArtifactClass::Gene,
            surface: GenomeSurface::Dna,
            kind: LocusPacketKind::CertificationEnvelope,
            version_major: 1,
            version_minor: 0,
            authority_tier: if report.verdict == CertificationVerdict::Certified {
                3
            } else {
                2
            },
            capabilities,
            grammar_id: "report-envelope.v1".into(),
            schema_hash: stable_hash(&format!(
                "{}|{}|{}",
                report.theorem_id, report.target_profile_id, EVALUATOR_VERSION
            )),
            integrity_hash: stable_hash(&serde_json::to_string(report).unwrap_or_default()),
            strand_manifest: vec!["core".into(), "cert".into(), "trace".into()],
            feature_flags: 0,
            root_subject_id: report
                .campaign_id
                .clone()
                .unwrap_or_else(|| report.theorem_id.clone()),
        },
        sections,
    };
    l64_core::encode_locus_packet(&packet).map_err(CertError::Message)
}

pub fn decode_locus_packet_report(bytes: &[u8]) -> Result<CertificationReport, CertError> {
    decode_section_payload(bytes, LocusOpcode::CanonicalPayload)
        .map_err(|err| CertError::Message(err.to_string()))
}

pub fn decode_locus_packet_summary(
    bytes: &[u8],
) -> Result<std::collections::BTreeMap<String, String>, CertError> {
    decode_locus_summary(bytes).map_err(|err| CertError::Message(err.to_string()))
}

fn encode_cached_report_packet(entry: &CachedCertificationReport) -> Result<Vec<u8>, CertError> {
    let coverage = derive_proof_coverage_envelope(&entry.report);
    let header_payload = CachedExecutionPacketHeader {
        cache_key: entry.cache_key.clone(),
        theorem_id: entry.theorem_id.clone(),
        campaign_id: entry.campaign_id.clone(),
        target_profile_id: entry.target_profile_id.clone(),
        bundle_hash: entry.bundle_hash.clone(),
        policy_hash: entry.policy_hash.clone(),
        route_winner_hash: entry.route_winner_hash.clone(),
        evaluator_version: entry.evaluator_version.clone(),
        report_id: report_storage_id(&entry.report),
    };
    let packet = LocusPacket {
        header: LocusPacketHeader {
            artifact_class: GenomeArtifactClass::Genome,
            surface: GenomeSurface::Dna,
            kind: LocusPacketKind::CertificationEnvelope,
            version_major: 1,
            version_minor: 1,
            authority_tier: if entry.report.verdict == CertificationVerdict::Certified {
                3
            } else {
                2
            },
            capabilities: LocusCapabilityMask {
                has_checker: !entry.report.checker_receipts.is_empty(),
                has_adequacy: !entry.report.adequacy_records.is_empty(),
                has_route: !entry.report.selected_path.is_empty(),
                has_certificate: entry.report.certificate_id.is_some(),
                has_frontier: !entry.report.deficiencies.is_empty(),
                has_forensic: !entry.report.diagnostics.is_empty()
                    || !entry.report.reasons.is_empty(),
            },
            grammar_id: "cached-report-envelope.v1".into(),
            schema_hash: stable_hash(&format!(
                "cache|{}|{}|{}",
                entry.theorem_id, entry.target_profile_id, entry.evaluator_version
            )),
            integrity_hash: stable_hash(&serde_json::to_string(&entry.report).unwrap_or_default()),
            strand_manifest: vec!["core".into(), "cert".into(), "trace".into()],
            feature_flags: 0,
            root_subject_id: header_payload.report_id.clone(),
        },
        sections: vec![
            LocusSection {
                opcode: LocusOpcode::Header,
                flags: 0,
                subject_id: header_payload.report_id.clone(),
                payload: bincode::serialize(&header_payload)
                    .map_err(|err| CertError::Message(err.to_string()))?,
            },
            LocusSection {
                opcode: LocusOpcode::CanonicalPayload,
                flags: 0,
                subject_id: entry.theorem_id.clone(),
                payload: bincode::serialize(&entry.report)
                    .map_err(|err| CertError::Message(err.to_string()))?,
            },
            LocusSection {
                opcode: LocusOpcode::Coverage,
                flags: 0,
                subject_id: coverage.subject_id.clone(),
                payload: bincode::serialize(&coverage)
                    .map_err(|err| CertError::Message(err.to_string()))?,
            },
        ],
    };
    l64_core::encode_locus_packet(&packet).map_err(CertError::Message)
}

fn decode_cached_report_packet(bytes: &[u8]) -> Result<CachedCertificationReport, CertError> {
    let packet = l64_core::decode_locus_packet(bytes).map_err(CertError::Message)?;
    let header = packet
        .sections
        .iter()
        .find(|item| item.opcode == LocusOpcode::Header)
        .ok_or_else(|| CertError::Message("cached locus packet missing header".into()))
        .and_then(|item| {
            bincode::deserialize::<CachedExecutionPacketHeader>(&item.payload)
                .map_err(|err| CertError::Message(err.to_string()))
        })?;
    let report = packet
        .sections
        .iter()
        .find(|item| item.opcode == LocusOpcode::CanonicalPayload)
        .ok_or_else(|| CertError::Message("cached locus packet missing canonical payload".into()))
        .and_then(|item| {
            bincode::deserialize::<CertificationReport>(&item.payload)
                .map_err(|err| CertError::Message(err.to_string()))
        })?;
    Ok(CachedCertificationReport {
        cache_key: header.cache_key,
        theorem_id: header.theorem_id,
        campaign_id: header.campaign_id,
        target_profile_id: header.target_profile_id,
        bundle_hash: header.bundle_hash,
        policy_hash: header.policy_hash,
        route_winner_hash: header.route_winner_hash,
        evaluator_version: header.evaluator_version,
        report,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use l64_bundle::import_bundle_file;
    use l64_registry::SeedRegistry;
    use serial_test::serial;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn test_namespace(label: &str) -> String {
        format!(
            "l64_cert_{}_{}",
            label,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        )
    }

    #[test]
    #[serial]
    fn derived_certification_replays_seed_chain_rule() {
        unsafe {
            std::env::set_var("MF_CACHE_NAMESPACE", test_namespace("seed_chain"));
        }
        clear_cache(Some("all")).unwrap();
        let registry = SeedRegistry::load().unwrap();
        let atlas = CompiledAtlas::compile(&registry).unwrap();
        let report = certify_derived_campaign(&registry, &atlas, "CPG_CHAIN_RULE").unwrap();
        assert_eq!(report.selected_atlas_cell.as_deref(), Some("A_TOP_TO_CALC"));
        let eq = report
            .obligations
            .iter()
            .find(|item| item.obligation_id == "OBL_CHAIN_EQ")
            .unwrap();
        assert_eq!(report.verdict, CertificationVerdict::Integrated);
        assert_eq!(
            eq.evaluation_mode,
            ObligationEvaluationMode::RecomputedExact
        );
        assert!(
            eq.receipts
                .iter()
                .any(|item| item.id == "EQR_CHAIN_OPERATOR_DEFAULT_SELECTION")
        );
        assert!(
            !report
                .deficiencies
                .iter()
                .any(|item| item.id == "DGN_CHAIN_RULE_ADEQUACY")
        );
        let red = report
            .obligations
            .iter()
            .find(|item| item.obligation_id == "OBL_CHAIN_RED")
            .unwrap();
        assert_eq!(
            red.evaluation_mode,
            ObligationEvaluationMode::RecomputedExact
        );
        assert!(
            red.receipts
                .iter()
                .any(|item| item.id == "RED_CHAIN_OPERATOR_DEFAULT_SELECTION")
        );
        assert!(
            report
                .promotion_artifact_ids
                .iter()
                .any(|item| item == "OPR_PROMOTED_OPR_CHAIN1")
        );
        assert!(
            report
                .reused_artifact_ids
                .iter()
                .any(|item| item == "OPR_PROMOTED_OPR_CHAIN1")
        );
        assert!(
            report
                .default_selected_artifact_ids
                .iter()
                .any(|item| item == "OPR_PROMOTED_OPR_CHAIN1")
        );
    }

    #[test]
    #[serial]
    fn derived_certification_reuses_chain1_on_adjacent_transport_shell() {
        unsafe {
            std::env::set_var("MF_CACHE_NAMESPACE", test_namespace("seed_chain_transport"));
        }
        let registry = SeedRegistry::load().unwrap();
        let atlas = CompiledAtlas::compile(&registry).unwrap();
        let report = certify_derived_campaign_with_options(
            &registry,
            &atlas,
            "CPG_CHAIN_RULE_TRANSPORT",
            &CertificationOptions {
                no_cache: true,
                bundle_hash: "seed".into(),
                policy_hash: "test-chain-transport".into(),
                ..CertificationOptions::default()
            },
        )
        .unwrap();
        assert_eq!(
            report.campaign_id.as_deref(),
            Some("CPG_CHAIN_RULE_TRANSPORT")
        );
        assert_eq!(report.verdict, CertificationVerdict::Integrated);
        assert!(
            report
                .default_selected_artifact_ids
                .iter()
                .any(|item| item == "OPR_PROMOTED_OPR_CHAIN1")
        );
        assert!(
            report
                .payoff_receipt_ids
                .iter()
                .any(|item| item == "PAY_CHAIN1_ADJACENT_TRANSPORT_DEFAULT_REUSE")
        );
        assert!(
            report
                .payoff_receipt_ids
                .iter()
                .any(|item| item == "PAY_CHAIN1_LOCALITY_WITNESS_RETAINED")
        );
        assert!(
            report
                .payoff_receipt_ids
                .iter()
                .any(|item| item == "PAY_CHAIN1_OBLIGATION_COUNT_REDUCTION_4_TO_3")
        );
        assert_eq!(report.obligations.len(), 3);
        assert!(
            report
                .obligations
                .iter()
                .any(|item| item.obligation_id == "OBL_CHAIN_LOC")
        );
    }

    #[test]
    #[serial]
    fn derived_certification_realizes_bayes_brace_on_top_prob_cluster() {
        unsafe {
            std::env::set_var("MF_CACHE_NAMESPACE", test_namespace("seed_bayes_brace"));
        }
        clear_cache(Some("all")).unwrap();
        let registry = SeedRegistry::load().unwrap();
        let atlas = CompiledAtlas::compile(&registry).unwrap();
        let report = certify_derived_campaign(&registry, &atlas, "CPG_BAYES_BRACE").unwrap();
        assert_eq!(report.campaign_id.as_deref(), Some("CPG_BAYES_BRACE"));
        assert_eq!(report.selected_atlas_cell.as_deref(), Some("A_TOP_TO_PROB"));
        assert_eq!(report.verdict, CertificationVerdict::Certified);
        assert!(
            report
                .adequacy_records
                .iter()
                .any(|item| item.clause_id == "ADQ_BAYES_TOP_PROB_BRIDGE")
        );
        assert!(report.deficiencies.is_empty());
        let adequacy = report
            .obligations
            .iter()
            .find(|item| item.obligation_id == "OBL_BAYES_ADE")
            .unwrap();
        assert_eq!(
            adequacy.evaluation_mode,
            ObligationEvaluationMode::RecomputedExact
        );
    }

    #[test]
    #[serial]
    fn derived_certification_realizes_ch_norm_on_exact_type_normalization_witness() {
        unsafe {
            std::env::set_var("MF_CACHE_NAMESPACE", test_namespace("seed_ch_norm"));
        }
        let registry = SeedRegistry::load().unwrap();
        let atlas = CompiledAtlas::compile(&registry).unwrap();
        let report = certify_derived_campaign_with_options(
            &registry,
            &atlas,
            "CPG_CH_NORM",
            &CertificationOptions {
                no_cache: true,
                bundle_hash: "seed".into(),
                policy_hash: "test-ch-norm".into(),
                ..CertificationOptions::default()
            },
        )
        .unwrap();
        assert_eq!(report.campaign_id.as_deref(), Some("CPG_CH_NORM"));
        assert_eq!(report.selected_atlas_cell.as_deref(), Some("A_TYPE_TO_SET"));
        assert_eq!(report.verdict, CertificationVerdict::Certified);
        assert!(
            report
                .adequacy_records
                .iter()
                .any(|item| item.clause_id == "ADQ_CH_TYP_SET_BRIDGE")
        );
        assert!(report.deficiencies.is_empty());
        let adequacy = report
            .obligations
            .iter()
            .find(|item| item.obligation_id == "OBL_CH_ADE")
            .unwrap();
        assert_eq!(
            adequacy.evaluation_mode,
            ObligationEvaluationMode::RecomputedExact
        );
        assert!(
            adequacy
                .receipts
                .iter()
                .any(|item| item.id == "CHN_ADE_CARRIER_COLLAPSE")
        );
        let reduction = report
            .obligations
            .iter()
            .find(|item| item.obligation_id == "OBL_CH_RED")
            .unwrap();
        assert_eq!(
            reduction.evaluation_mode,
            ObligationEvaluationMode::RecomputedExact
        );
        assert!(
            reduction
                .receipts
                .iter()
                .any(|item| item.id == "CHN_RED_BETA_ETA_NORMALIZE")
        );
    }

    #[test]
    #[serial]
    fn derived_certification_realizes_exec_infer_on_prob_comp_cluster() {
        unsafe {
            std::env::set_var("MF_CACHE_NAMESPACE", test_namespace("seed_exec_infer"));
        }
        clear_cache(Some("all")).unwrap();
        let registry = SeedRegistry::load().unwrap();
        let atlas = CompiledAtlas::compile(&registry).unwrap();
        let report = certify_derived_campaign(&registry, &atlas, "CPG_EXEC_INFER").unwrap();
        assert_eq!(report.campaign_id.as_deref(), Some("CPG_EXEC_INFER"));
        assert_eq!(
            report.selected_atlas_cell.as_deref(),
            Some("A_PROB_TO_COMP")
        );
        assert_eq!(report.verdict, CertificationVerdict::Certified);
        assert!(
            report
                .adequacy_records
                .iter()
                .any(|item| item.clause_id == "ADQ_EXEC_PROB_COMP_BRIDGE")
        );
        assert!(report.deficiencies.is_empty());
        let red = report
            .obligations
            .iter()
            .find(|item| item.obligation_id == "OBL_EXEC_RED")
            .unwrap();
        assert_eq!(
            red.evaluation_mode,
            ObligationEvaluationMode::RecomputedExact
        );
    }

    #[test]
    #[serial]
    fn derived_certification_realizes_prob_judg_on_prob_log_cluster() {
        unsafe {
            std::env::set_var("MF_CACHE_NAMESPACE", test_namespace("seed_prob_judg"));
        }
        clear_cache(Some("all")).unwrap();
        let registry = SeedRegistry::load().unwrap();
        let atlas = CompiledAtlas::compile(&registry).unwrap();
        let report = certify_derived_campaign(&registry, &atlas, "CPG_PROB_JUDG").unwrap();
        assert_eq!(report.campaign_id.as_deref(), Some("CPG_PROB_JUDG"));
        assert_eq!(report.selected_atlas_cell.as_deref(), Some("A_PROB_TO_LOG"));
        assert_eq!(report.verdict, CertificationVerdict::Certified);
        assert!(
            report
                .adequacy_records
                .iter()
                .any(|item| item.clause_id == "ADQ_JDG_PROB_LOG_BRIDGE")
        );
        assert!(report.deficiencies.is_empty());
        let ade = report
            .obligations
            .iter()
            .find(|item| item.obligation_id == "OBL_JDG_ADE")
            .unwrap();
        assert_eq!(
            ade.evaluation_mode,
            ObligationEvaluationMode::RecomputedExact
        );
    }

    #[test]
    #[serial]
    fn derived_certification_realizes_cert_prop_on_comp_log_cluster() {
        unsafe {
            std::env::set_var("MF_CACHE_NAMESPACE", test_namespace("seed_cert_prop"));
        }
        clear_cache(Some("all")).unwrap();
        let registry = SeedRegistry::load().unwrap();
        let atlas = CompiledAtlas::compile(&registry).unwrap();
        let report = certify_derived_campaign(&registry, &atlas, "CPG_CERT_PROP").unwrap();
        assert_eq!(report.campaign_id.as_deref(), Some("CPG_CERT_PROP"));
        assert_eq!(report.selected_atlas_cell.as_deref(), Some("A_COMP_TO_LOG"));
        assert_eq!(report.verdict, CertificationVerdict::Certified);
        assert!(
            report
                .adequacy_records
                .iter()
                .any(|item| item.clause_id == "ADQ_CERT_COMP_LOG_BRIDGE")
        );
        assert!(report.deficiencies.is_empty());
        let obs = report
            .obligations
            .iter()
            .find(|item| item.obligation_id == "OBL_CERT_OBS")
            .unwrap();
        assert_eq!(
            obs.evaluation_mode,
            ObligationEvaluationMode::RecomputedExact
        );
    }

    #[test]
    #[serial]
    fn derived_certification_realizes_ch_inh_on_exact_type_algebra_witness() {
        unsafe {
            std::env::set_var("MF_CACHE_NAMESPACE", test_namespace("seed_ch_inh"));
        }
        clear_cache(Some("all")).unwrap();
        let registry = SeedRegistry::load().unwrap();
        let atlas = CompiledAtlas::compile(&registry).unwrap();
        let report = certify_derived_campaign(&registry, &atlas, "CPG_CH_INH").unwrap();
        assert_eq!(report.campaign_id.as_deref(), Some("CPG_CH_INH"));
        assert_eq!(report.selected_atlas_cell.as_deref(), Some("A_TYPE_TO_ALG"));
        assert_eq!(report.verdict, CertificationVerdict::Certified);
        assert!(
            report
                .adequacy_records
                .iter()
                .any(|item| item.clause_id == "ADQ_CHI_TYP_ALG_BRIDGE")
        );
        assert!(report.deficiencies.is_empty());
        let adequacy = report
            .obligations
            .iter()
            .find(|item| item.obligation_id == "OBL_CHI_ADE")
            .unwrap();
        assert_eq!(
            adequacy.evaluation_mode,
            ObligationEvaluationMode::RecomputedExact
        );
        assert!(
            adequacy
                .receipts
                .iter()
                .any(|item| item.id == "CHI_ADE_CARRIER_LIFT")
        );
        let eq = report
            .obligations
            .iter()
            .find(|item| item.obligation_id == "OBL_CHI_EQ")
            .unwrap();
        assert_eq!(
            eq.evaluation_mode,
            ObligationEvaluationMode::RecomputedExact
        );
        assert!(eq.receipts.iter().any(|item| {
            item.id == "CHI_EQ_INHERITANCE"
                && item
                    .subreceipts
                    .iter()
                    .any(|sub| sub.id == "CHI_EQ_PROOF_TERM_TRANSPORT")
                && item
                    .subreceipts
                    .iter()
                    .any(|sub| sub.id == "CHI_EQ_PROOF_TERM_NORMAL_FORM")
        }));
    }

    #[test]
    #[serial]
    fn seed_registry_exposes_chain_rule_adequacy_clauses() {
        let registry = SeedRegistry::load().unwrap();
        let clauses = registry.adequacy_clauses();
        assert!(
            clauses
                .iter()
                .any(|item| item.id == "ADQ_CHAIN_TOP_CALC_BRIDGE")
        );
        assert!(clauses.iter().any(|item| item.id == "ADQ_CHAIN_CALC_EQ"));
        assert!(
            clauses
                .iter()
                .any(|item| item.id == "ADQ_BAYES_TOP_PROB_BRIDGE")
        );
        assert!(clauses.iter().any(|item| item.id == "ADQ_BAYES_PROB_EQ"));
        assert!(
            clauses
                .iter()
                .any(|item| item.id == "ADQ_CH_TYP_SET_BRIDGE")
        );
        assert!(clauses.iter().any(|item| item.id == "ADQ_CH_SET_EQ"));
        assert!(
            clauses
                .iter()
                .any(|item| item.id == "ADQ_EXEC_PROB_COMP_BRIDGE")
        );
        assert!(
            clauses
                .iter()
                .any(|item| item.id == "ADQ_JDG_PROB_LOG_BRIDGE")
        );
        assert!(
            clauses
                .iter()
                .any(|item| item.id == "ADQ_CERT_COMP_LOG_BRIDGE")
        );
        assert!(
            clauses
                .iter()
                .any(|item| item.id == "ADQ_CHI_TYP_ALG_BRIDGE")
        );
    }

    #[test]
    #[serial]
    fn certification_reports_checker_receipts_for_active_campaign_objects() {
        unsafe {
            std::env::set_var(
                "MF_CACHE_NAMESPACE",
                test_namespace("seed_checker_receipts"),
            );
        }
        clear_cache(Some("all")).unwrap();
        let registry = SeedRegistry::load().unwrap();
        let atlas = CompiledAtlas::compile(&registry).unwrap();
        let report = certify_derived_campaign(&registry, &atlas, "CPG_EXEC_INFER").unwrap();
        assert!(
            report
                .checker_receipts
                .iter()
                .any(|item| item.kind == CheckerReceiptKind::TheoremSpec
                    && item.subject_id == "THS_EXEC_INFER")
        );
        assert!(
            report
                .checker_receipts
                .iter()
                .any(|item| item.kind == CheckerReceiptKind::Campaign
                    && item.subject_id == "CPG_EXEC_INFER")
        );
        assert!(
            report
                .checker_receipts
                .iter()
                .any(|item| item.kind == CheckerReceiptKind::AdequacyClause
                    && item.subject_id == "ADQ_EXEC_PROB_COMP_BRIDGE")
        );
        assert!(
            report
                .checker_receipts
                .iter()
                .any(|item| item.kind == CheckerReceiptKind::ObligationVerdict
                    && item.subject_id == "OBL_EXEC_ADE")
        );
        assert!(
            report
                .checker_receipts
                .iter()
                .all(|item| item.verdict == CertificationVerdict::Certified)
        );
    }

    #[test]
    #[serial]
    fn cache_roundtrip_works() {
        unsafe {
            std::env::set_var("MF_CACHE_NAMESPACE", test_namespace("cache_roundtrip"));
        }
        clear_cache(Some("all")).unwrap();
        let registry = SeedRegistry::load().unwrap();
        let atlas = CompiledAtlas::compile(&registry).unwrap();
        let options = CertificationOptions {
            bundle_hash: "test-bundle".into(),
            policy_hash: "test-policy".into(),
            ..CertificationOptions::default()
        };
        let first =
            certify_derived_campaign_with_options(&registry, &atlas, "CPG_CHAIN_RULE", &options)
                .unwrap();
        let second =
            certify_derived_campaign_with_options(&registry, &atlas, "CPG_CHAIN_RULE", &options)
                .unwrap();
        assert_eq!(first.theorem_id, second.theorem_id);
        assert!(matches!(
            second
                .execution_envelope
                .as_ref()
                .map(|item| &item.replay_status),
            Some(ReplayStatus::CacheHit) | Some(ReplayStatus::Fresh)
        ));
    }

    #[test]
    #[serial]
    fn imported_overlay_bundle_certifies_directly() {
        unsafe {
            std::env::set_var("MF_CACHE_NAMESPACE", test_namespace("overlay_direct"));
        }
        let dir = std::env::temp_dir().join(format!(
            "l64_cert_bundle_test_{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&dir);
        let file = dir.join("bundle.qc0");
        fs::write(
            &file,
            r#"!qc0 {"surface_kind":"Qc0","version":"1","policy_id":"POL_QC0_CORE","capability_id":"CAP_QC0_CORE"}
policy-object {"id":"MOP_BND_BUNDLE_TEST_SCHED","kind":"ReportExport","scope":{"Bundle":"BND_BUNDLE"},"extends":null,"optimizer":null,"evaluator":null,"replay_cache":null,"report":{"export_surfaces":["Qc0"],"include_policy_trace":true,"include_route_explanation":true,"include_obligation_logs":true},"scheduler":{"parallelization":"ParallelIndependent","max_workers":2,"allow_parallel_replay":true,"allow_parallel_certification":true,"allow_parallel_exports":true,"deterministic_ordering":true,"allow_parallel_obligations":true,"max_obligation_workers":2,"allow_parallel_obligation_replay":true,"serialize_canonicalization_sensitive":true},"canonicalizer_mode":null,"merge_policy":null,"notes":["test scheduler"]}
proof {"id":"PS_T","kind":"Square","nodes":["a","b","c","d"],"edges":[{"from":"a","to":"b","label":"f"},{"from":"b","to":"d","label":"g"},{"from":"a","to":"c","label":"h"},{"from":"c","to":"d","label":"i"}],"equations":["g∘f=i∘h"],"target_equivalence":"eq","receipts":["r"],"gate":"Pass"}
bridge {"id":"B_T","src":"R_TOP","tgt":"R_CALC","id_pres":"pres","eq_pres":"eq","forget":[],"enrich":["der"],"loss":[],"reversibility":"Enriching","receipts":["r"],"rollback":"allowed"}
atlas {"id":"A_T","source_regime":"R_TOP","target_regime":"R_CALC","burden_class":"General","proof_target":"test","candidate_paths":[["B_T"]],"normalized_winner":["B_T"],"winner_state":"Candidate","loss_profile":{"items":[]},"proof_shapes_checked":["PS_T"],"recipe_maturity":"Stable","failure_signatures":[],"side_conditions":[],"surface_transition":{"compatibility":"AuthorityPreserving","penalties":[],"total_penalty":0}}
theorem {"id":"THS_T","statement":"test","hosts":["R_TOP","R_CALC"],"bridges":["B_T"],"operators":["OPR.T"],"target_equivalence":"eq","obligations":["OblAdm"],"primary_zone":"PmzStructural","verdict":"Benchmarked","proof_shapes":["PS_T"]}
obligation {"id":"OBL_T","kind":"OblAdm","description":"adm","status":"Benchmarked"}
target {"id":"TGT_T","burden_class":"General","host_cluster":["R_TOP","R_CALC"],"target_equivalence":"eq","allowed_bridge_classes":["Enriching"],"loss_ceiling":1,"rollback_ceiling":1,"required_receipt_class":"RC","required_proof_shape_family":"Square","promotion_goal":"PromoteOperator","primary_zone":"PmzStructural","surface_requirement":null,"preferred_surface_target":null,"optimizer_policy":null,"policy_binding_ids":[]}
ledger {"id":"TRL_T","theorem":"THS_T","paths":[["B_T"]],"budget":{"max_loss":1,"allow_lossy_supported":false,"require_proof":true},"losses":[],"receipts":[],"normalized_path":["B_T"]}
campaign {"id":"CPG_T","theorem":"THS_T","target_profile":"TGT_T","route_ledger":"TRL_T","obligations":["OBL_T"],"certificates":[],"dependencies":[],"campaign_class":"CBasic","verdict":"Benchmarked","payoff":["test"]}"#,
        )
        .unwrap();
        let world =
            import_bundle_file(&file, None, l64_core::BundleConflictPolicy::Reject, None).unwrap();
        let atlas = CompiledAtlas::compile(&world.overlay).unwrap();
        let report = certify_derived_campaign_with_options(
            &world.overlay,
            &atlas,
            "CPG_T",
            &CertificationOptions {
                bundle_hash: "bundle".into(),
                policy_hash: "policy".into(),
                bundle_id: Some(world.manifest.id.clone()),
                no_cache: true,
                ..CertificationOptions::default()
            },
        )
        .unwrap();
        let _json = serde_json::to_string(&report).unwrap();
        assert_eq!(report.selected_atlas_cell.as_deref(), Some("A_T"));
    }

    #[test]
    #[serial]
    fn imported_multi_campaign_bundle_serializes_reports() {
        unsafe {
            std::env::set_var("MF_CACHE_NAMESPACE", test_namespace("multi_bundle"));
        }
        let dir = std::env::temp_dir().join("l64_cert_bundle_test_multi");
        let _ = fs::create_dir_all(&dir);
        let file = dir.join("bundle.qc0");
        fs::write(
            &file,
            r#"!qc0 {"surface_kind":"Qc0","version":"1","policy_id":"POL_QC0_CORE","capability_id":"CAP_QC0_CORE"}
policy-object {"id":"MOP_BND_PAR_SCHED","kind":"ReportExport","scope":{"Bundle":"BND_MULTI"},"extends":null,"optimizer":null,"evaluator":null,"replay_cache":null,"report":{"export_surfaces":["Qc0"],"include_policy_trace":true,"include_route_explanation":true,"include_obligation_logs":true},"scheduler":{"parallelization":"ParallelIndependent","max_workers":2,"allow_parallel_replay":false,"allow_parallel_certification":true,"allow_parallel_exports":true,"deterministic_ordering":true,"allow_parallel_obligations":true,"max_obligation_workers":2,"allow_parallel_obligation_replay":false,"serialize_canonicalization_sensitive":true},"canonicalizer_mode":null,"merge_policy":null,"notes":["parallel scheduler policy"]}
proof {"id":"PS_PAR","kind":"Square","nodes":["a","b","c","d"],"edges":[{"from":"a","to":"b","label":"f"},{"from":"b","to":"d","label":"g"},{"from":"a","to":"c","label":"h"},{"from":"c","to":"d","label":"i"}],"equations":["g∘f=i∘h"],"target_equivalence":"eq","receipts":["r"],"gate":"Pass"}
bridge {"id":"B_PAR","src":"R_TOP","tgt":"R_CALC","id_pres":"pres","eq_pres":"eq","forget":[],"enrich":["der"],"loss":[],"reversibility":"Enriching","receipts":["r"],"rollback":"allowed"}
atlas {"id":"A_PAR","source_regime":"R_TOP","target_regime":"R_CALC","burden_class":"General","proof_target":"parallel","candidate_paths":[["B_PAR"]],"normalized_winner":["B_PAR"],"winner_state":"Candidate","loss_profile":{"items":[]},"proof_shapes_checked":["PS_PAR"],"recipe_maturity":"Stable","failure_signatures":[],"side_conditions":[],"surface_transition":{"compatibility":"AuthorityPreserving","penalties":[],"total_penalty":0}}
theorem {"id":"THS_PAR_A","statement":"parallel A","hosts":["R_TOP","R_CALC"],"bridges":["B_PAR"],"operators":["OPR.ParA"],"target_equivalence":"eq","obligations":["OblAdm"],"primary_zone":"PmzStructural","verdict":"Benchmarked","proof_shapes":["PS_PAR"]}
theorem {"id":"THS_PAR_B","statement":"parallel B","hosts":["R_TOP","R_CALC"],"bridges":["B_PAR"],"operators":["OPR.ParB"],"target_equivalence":"eq","obligations":["OblAdm"],"primary_zone":"PmzStructural","verdict":"Benchmarked","proof_shapes":["PS_PAR"]}
obligation {"id":"OBL_PAR","kind":"OblAdm","description":"adm","status":"Benchmarked"}
target {"id":"TGT_PAR","burden_class":"General","host_cluster":["R_TOP","R_CALC"],"target_equivalence":"eq","allowed_bridge_classes":["Enriching"],"loss_ceiling":1,"rollback_ceiling":1,"required_receipt_class":"RC","required_proof_shape_family":"Square","promotion_goal":"PromoteOperator","primary_zone":"PmzStructural","surface_requirement":null,"preferred_surface_target":null,"optimizer_policy":null,"policy_binding_ids":[]}
ledger {"id":"TRL_PAR_A","theorem":"THS_PAR_A","paths":[["B_PAR"]],"budget":{"max_loss":1,"allow_lossy_supported":false,"require_proof":true},"losses":[],"receipts":[],"normalized_path":["B_PAR"]}
ledger {"id":"TRL_PAR_B","theorem":"THS_PAR_B","paths":[["B_PAR"]],"budget":{"max_loss":1,"allow_lossy_supported":false,"require_proof":true},"losses":[],"receipts":[],"normalized_path":["B_PAR"]}
campaign {"id":"CPG_PAR_A","theorem":"THS_PAR_A","target_profile":"TGT_PAR","route_ledger":"TRL_PAR_A","obligations":["OBL_PAR"],"certificates":[],"dependencies":[],"campaign_class":"CBasic","verdict":"Benchmarked","payoff":["parallel"]}
campaign {"id":"CPG_PAR_B","theorem":"THS_PAR_B","target_profile":"TGT_PAR","route_ledger":"TRL_PAR_B","obligations":["OBL_PAR"],"certificates":[],"dependencies":[],"campaign_class":"CBasic","verdict":"Benchmarked","payoff":["parallel"]}"#,
        )
        .unwrap();
        let world =
            import_bundle_file(&file, None, l64_core::BundleConflictPolicy::Reject, None).unwrap();
        let atlas = CompiledAtlas::compile(&world.overlay).unwrap();
        let reports = ["CPG_PAR_A", "CPG_PAR_B"]
            .iter()
            .map(|id| {
                certify_derived_campaign_with_options(
                    &world.overlay,
                    &atlas,
                    id,
                    &CertificationOptions {
                        bundle_hash: "bundle".into(),
                        policy_hash: "policy".into(),
                        bundle_id: Some(world.manifest.id.clone()),
                        no_cache: true,
                        ..CertificationOptions::default()
                    },
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        let _json = serde_json::to_string_pretty(&reports).unwrap();
        assert_eq!(reports.len(), 2);
    }

    #[test]
    #[serial]
    fn imported_kernel_claim_bundle_computes_native_evidence_adequacy_and_checker_receipts() {
        unsafe {
            std::env::set_var(
                "MF_CACHE_NAMESPACE",
                test_namespace("imported_kernel_claim"),
            );
        }
        let dir = std::env::temp_dir().join(format!(
            "l64_cert_kernel_claim_{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&dir);
        let file = dir.join("bundle.qc0");
        fs::write(
            &file,
            r#"!qc0 {"surface_kind":"Qc0","version":"1","policy_id":"POL_QC0_CORE","capability_id":"CAP_QC0_CORE"}
proof {"id":"PS_G1","kind":"Square","nodes":["a","b","c","d"],"edges":[{"from":"a","to":"b","label":"f"},{"from":"b","to":"d","label":"g"},{"from":"a","to":"c","label":"h"},{"from":"c","to":"d","label":"i"}],"equations":["g∘f=i∘h"],"target_equivalence":"eq","receipts":["r"],"gate":"Pass"}
bridge {"id":"B_G1","src":"R_TOP","tgt":"R_CALC","id_pres":"pres","eq_pres":"eq","forget":[],"enrich":["der"],"loss":[],"reversibility":"Enriching","receipts":["r"],"rollback":"allowed"}
atlas {"id":"A_G1","source_regime":"R_TOP","target_regime":"R_CALC","burden_class":"ImportedKernelClaim","proof_target":"kernel-claim","candidate_paths":[["B_G1"]],"normalized_winner":["B_G1"],"winner_state":"Candidate","loss_profile":{"items":[]},"proof_shapes_checked":["PS_G1"],"recipe_maturity":"Stable","failure_signatures":[],"side_conditions":[],"surface_transition":{"compatibility":"AuthorityPreserving","penalties":[],"total_penalty":0}}
theorem {"id":"THS_G1","statement":"imported kernel claim","hosts":["R_TOP","R_CALC"],"bridges":["B_G1"],"operators":["OPR.G1"],"target_equivalence":"eq","obligations":["OblAdm"],"primary_zone":"PmzStructural","verdict":"Benchmarked","proof_shapes":["PS_G1"]}
obligation {"id":"OBL_G1","kind":"OblAdm","description":"imported admissibility","status":"Benchmarked"}
target {"id":"TGT_G1","burden_class":"ImportedKernelClaim","host_cluster":["R_TOP","R_CALC"],"target_equivalence":"eq","allowed_bridge_classes":["Enriching"],"loss_ceiling":1,"rollback_ceiling":1,"required_receipt_class":"RC","required_proof_shape_family":"Square","promotion_goal":"OpenBlocked","primary_zone":"PmzStructural","surface_requirement":null,"preferred_surface_target":null,"optimizer_policy":null,"policy_binding_ids":[]}
ledger {"id":"TRL_G1","theorem":"THS_G1","paths":[["B_G1"]],"budget":{"max_loss":1,"allow_lossy_supported":false,"require_proof":true},"losses":[],"receipts":["Trace_G1"],"normalized_path":["B_G1"]}
campaign {"id":"CPG_G1","theorem":"THS_G1","target_profile":"TGT_G1","route_ledger":"TRL_G1","obligations":["OBL_G1"],"certificates":[],"dependencies":[],"campaign_class":"CBasic","verdict":"Benchmarked","payoff":["kernel-claim"]}
burden-pack {"id":"BPK_G1","allowed_host_cluster":["R_TOP","R_CALC"],"obligation_ids":["OBL_G1"],"adequacy_clause_ids":["ADQ_G1_EVID","ADQ_G1_BENCH","ADQ_G1_STRESS","ADQ_G1_CHALLENGE"],"required_proof_shape_family":"Square","route_class_constraints":[],"evidence_contract_ids":["ECT_G1"],"promotion_ceiling":"Certified","blocker_taxonomy":["DEvidenceContract","DBenchmarkGap","DStressGap","DChallengeGap"]}
claim-packet {"id":"CLM_G1","claim_class":"Kernel","authority_state":"Evidence","target_sector":"kernel-claim","statement":"G1 imported claim","assumptions":["A1"],"open_caveats":[]}
evidence-contract {"id":"ECT_G1","required_evidence_kinds":["kernel-claim"],"required_benchmark_roles":["TargetCase","Stress"],"requires_stress":true,"requires_challenge":true,"admissibility_thresholds":["stable"],"promotion_ceiling":"Certified"}
benchmark-receipt {"id":"BMR_G1_TARGET","claim_packet_id":"CLM_G1","role":"TargetCase","verdict":"Certified","metrics":{"score":"1.0"},"reproducibility_ref":"RPK_G1"}
benchmark-receipt {"id":"BMR_G1_STRESS","claim_packet_id":"CLM_G1","role":"Stress","verdict":"Certified","metrics":{"stress":"pass"},"reproducibility_ref":"RPK_G1"}
challenge-receipt {"id":"CHR_G1","claim_packet_id":"CLM_G1","grounds":["baseline challenge"],"required_response":"addressed","status":"Addressed"}
reproducibility-packet {"id":"RPK_G1","claim_packet_id":"CLM_G1","derivation_path":["lab","bundle"],"code_refs":["src"],"benchmark_refs":["BMR_G1_TARGET","BMR_G1_STRESS"],"artifact_refs":["CLM_G1"]}
adequacy {"id":"ADQ_G1_EVID","kind":"EvidenceContractInterpretation","regime_ids":["R_TOP","R_CALC"],"bridge_ids":[],"theorem_ids":["THS_G1"],"burden_pack_ids":["BPK_G1"],"claim_packet_ids":["CLM_G1"],"evidence_contract_ids":["ECT_G1"],"benchmark_receipt_ids":[],"challenge_receipt_ids":[],"reproducibility_packet_ids":["RPK_G1"],"description":"evidence contract present","blocking":true}
adequacy {"id":"ADQ_G1_BENCH","kind":"BenchmarkInterpretation","regime_ids":["R_TOP","R_CALC"],"bridge_ids":[],"theorem_ids":["THS_G1"],"burden_pack_ids":["BPK_G1"],"claim_packet_ids":["CLM_G1"],"evidence_contract_ids":["ECT_G1"],"benchmark_receipt_ids":["BMR_G1_TARGET","BMR_G1_STRESS"],"challenge_receipt_ids":[],"reproducibility_packet_ids":["RPK_G1"],"description":"benchmark coverage present","blocking":true}
adequacy {"id":"ADQ_G1_STRESS","kind":"StressInterpretation","regime_ids":["R_TOP","R_CALC"],"bridge_ids":[],"theorem_ids":["THS_G1"],"burden_pack_ids":["BPK_G1"],"claim_packet_ids":["CLM_G1"],"evidence_contract_ids":["ECT_G1"],"benchmark_receipt_ids":["BMR_G1_TARGET","BMR_G1_STRESS"],"challenge_receipt_ids":[],"reproducibility_packet_ids":["RPK_G1"],"description":"stress coverage present","blocking":true}
adequacy {"id":"ADQ_G1_CHALLENGE","kind":"ChallengeInterpretation","regime_ids":["R_TOP","R_CALC"],"bridge_ids":[],"theorem_ids":["THS_G1"],"burden_pack_ids":["BPK_G1"],"claim_packet_ids":["CLM_G1"],"evidence_contract_ids":["ECT_G1"],"benchmark_receipt_ids":[],"challenge_receipt_ids":["CHR_G1"],"reproducibility_packet_ids":["RPK_G1"],"description":"challenge addressed","blocking":true}"#,
        )
        .unwrap();
        let world =
            import_bundle_file(&file, None, l64_core::BundleConflictPolicy::Reject, None).unwrap();
        let atlas = CompiledAtlas::compile(&world.overlay).unwrap();
        let report = certify_derived_campaign_with_options(
            &world.overlay,
            &atlas,
            "CPG_G1",
            &CertificationOptions {
                bundle_hash: "bundle".into(),
                policy_hash: "policy".into(),
                bundle_id: Some(world.manifest.id.clone()),
                no_cache: true,
                ..CertificationOptions::default()
            },
        )
        .unwrap();
        assert!(
            report
                .adequacy_records
                .iter()
                .any(|item| item.clause_id == "ADQ_G1_EVID"
                    && item.verdict == CertificationVerdict::Certified)
        );
        assert!(
            report
                .adequacy_records
                .iter()
                .any(|item| item.clause_id == "ADQ_G1_STRESS"
                    && item.verdict == CertificationVerdict::Certified)
        );
        assert!(
            report
                .checker_receipts
                .iter()
                .any(|item| item.kind == CheckerReceiptKind::BurdenPack
                    && item.subject_id == "BPK_G1")
        );
        assert!(report.checker_receipts.iter().any(|item| item.kind
            == CheckerReceiptKind::ClaimPacket
            && item.subject_id == "CLM_G1"));
        assert!(report.burden_pack_ids.iter().any(|item| item == "BPK_G1"));
        assert!(report.claim_packet_ids.iter().any(|item| item == "CLM_G1"));
        assert!(
            report
                .evidence_contract_ids
                .iter()
                .any(|item| item == "ECT_G1")
        );
        assert!(
            report
                .benchmark_receipt_ids
                .iter()
                .any(|item| item == "BMR_G1_STRESS")
        );
        assert!(
            report
                .challenge_receipt_ids
                .iter()
                .any(|item| item == "CHR_G1")
        );
        assert!(
            report
                .reproducibility_packet_ids
                .iter()
                .any(|item| item == "RPK_G1")
        );
    }

    #[test]
    #[serial]
    fn imported_kernel_claim_bundle_sharpens_missing_stress_gap() {
        unsafe {
            std::env::set_var(
                "MF_CACHE_NAMESPACE",
                test_namespace("imported_kernel_claim_gap"),
            );
        }
        let dir = std::env::temp_dir().join(format!(
            "l64_cert_kernel_claim_gap_{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&dir);
        let file = dir.join("bundle.qc0");
        fs::write(
            &file,
            r#"!qc0 {"surface_kind":"Qc0","version":"1","policy_id":"POL_QC0_CORE","capability_id":"CAP_QC0_CORE"}
proof {"id":"PS_G2","kind":"Square","nodes":["a","b","c","d"],"edges":[{"from":"a","to":"b","label":"f"},{"from":"b","to":"d","label":"g"},{"from":"a","to":"c","label":"h"},{"from":"c","to":"d","label":"i"}],"equations":["g∘f=i∘h"],"target_equivalence":"eq","receipts":["r"],"gate":"Pass"}
bridge {"id":"B_G2","src":"R_TOP","tgt":"R_CALC","id_pres":"pres","eq_pres":"eq","forget":[],"enrich":["der"],"loss":[],"reversibility":"Enriching","receipts":["r"],"rollback":"allowed"}
atlas {"id":"A_G2","source_regime":"R_TOP","target_regime":"R_CALC","burden_class":"ImportedKernelClaim","proof_target":"kernel-claim","candidate_paths":[["B_G2"]],"normalized_winner":["B_G2"],"winner_state":"Candidate","loss_profile":{"items":[]},"proof_shapes_checked":["PS_G2"],"recipe_maturity":"Stable","failure_signatures":[],"side_conditions":[],"surface_transition":{"compatibility":"AuthorityPreserving","penalties":[],"total_penalty":0}}
theorem {"id":"THS_G2","statement":"imported kernel claim gap","hosts":["R_TOP","R_CALC"],"bridges":["B_G2"],"operators":["OPR.G2"],"target_equivalence":"eq","obligations":["OblAdm"],"primary_zone":"PmzStructural","verdict":"Benchmarked","proof_shapes":["PS_G2"]}
obligation {"id":"OBL_G2","kind":"OblAdm","description":"imported admissibility","status":"Benchmarked"}
target {"id":"TGT_G2","burden_class":"ImportedKernelClaim","host_cluster":["R_TOP","R_CALC"],"target_equivalence":"eq","allowed_bridge_classes":["Enriching"],"loss_ceiling":1,"rollback_ceiling":1,"required_receipt_class":"RC","required_proof_shape_family":"Square","promotion_goal":"OpenBlocked","primary_zone":"PmzStructural","surface_requirement":null,"preferred_surface_target":null,"optimizer_policy":null,"policy_binding_ids":[]}
ledger {"id":"TRL_G2","theorem":"THS_G2","paths":[["B_G2"]],"budget":{"max_loss":1,"allow_lossy_supported":false,"require_proof":true},"losses":[],"receipts":["Trace_G2"],"normalized_path":["B_G2"]}
campaign {"id":"CPG_G2","theorem":"THS_G2","target_profile":"TGT_G2","route_ledger":"TRL_G2","obligations":["OBL_G2"],"certificates":[],"dependencies":[],"campaign_class":"CBasic","verdict":"Benchmarked","payoff":["kernel-claim"]}
burden-pack {"id":"BPK_G2","allowed_host_cluster":["R_TOP","R_CALC"],"obligation_ids":["OBL_G2"],"adequacy_clause_ids":["ADQ_G2_STRESS"],"required_proof_shape_family":"Square","route_class_constraints":[],"evidence_contract_ids":["ECT_G2"],"promotion_ceiling":"Certified","blocker_taxonomy":["DStressGap"]}
claim-packet {"id":"CLM_G2","claim_class":"Kernel","authority_state":"Evidence","target_sector":"kernel-claim","statement":"G2 imported claim","assumptions":["A1"],"open_caveats":[]}
evidence-contract {"id":"ECT_G2","required_evidence_kinds":["kernel-claim"],"required_benchmark_roles":["Stress"],"requires_stress":true,"requires_challenge":false,"admissibility_thresholds":["stable"],"promotion_ceiling":"Certified"}
benchmark-receipt {"id":"BMR_G2_TARGET","claim_packet_id":"CLM_G2","role":"TargetCase","verdict":"Certified","metrics":{"score":"1.0"},"reproducibility_ref":"RPK_G2"}
reproducibility-packet {"id":"RPK_G2","claim_packet_id":"CLM_G2","derivation_path":["lab","bundle"],"code_refs":["src"],"benchmark_refs":["BMR_G2_TARGET"],"artifact_refs":["CLM_G2"]}
adequacy {"id":"ADQ_G2_STRESS","kind":"StressInterpretation","regime_ids":["R_TOP","R_CALC"],"bridge_ids":[],"theorem_ids":["THS_G2"],"burden_pack_ids":["BPK_G2"],"claim_packet_ids":["CLM_G2"],"evidence_contract_ids":["ECT_G2"],"benchmark_receipt_ids":["BMR_G2_TARGET"],"challenge_receipt_ids":[],"reproducibility_packet_ids":["RPK_G2"],"description":"stress coverage required","blocking":true}"#,
        )
        .unwrap();
        let world =
            import_bundle_file(&file, None, l64_core::BundleConflictPolicy::Reject, None).unwrap();
        let atlas = CompiledAtlas::compile(&world.overlay).unwrap();
        let report = certify_derived_campaign_with_options(
            &world.overlay,
            &atlas,
            "CPG_G2",
            &CertificationOptions {
                bundle_hash: "bundle".into(),
                policy_hash: "policy".into(),
                bundle_id: Some(world.manifest.id.clone()),
                no_cache: true,
                ..CertificationOptions::default()
            },
        )
        .unwrap();
        assert!(
            report
                .adequacy_records
                .iter()
                .any(|item| item.clause_id == "ADQ_G2_STRESS"
                    && item.verdict == CertificationVerdict::BlockedOpen)
        );
        assert!(
            report
                .deficiencies
                .iter()
                .any(|item| item.class == l64_core::AtlasDeficiencyClass::DStressGap)
        );
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct ChainRulePromotedOperatorContext {
    id: String,
    origin: ArtifactOrigin,
}
