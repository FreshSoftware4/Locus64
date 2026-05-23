use l64_core::{
    AtlasCell, Budget, BurdenClass, CertificationVerdict, ComposeBridge, OptimizationAxis,
    OptimizerBackend, OptimizerPolicy, PolicyResolution, RegistryLookup, ReversibilityClass,
    RouteExplanation, RouteScoreVector, RouteSelection, SurfaceCompatibilityClass, SurfaceKind,
    SurfacePreferredTarget, SurfaceRequirement, SurfaceTransitionCost, WinnerState,
};
use l64_kernel::ConstitutionKernel;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompiledEdge {
    pub atlas_cell_id: String,
    pub src: String,
    pub tgt: String,
    pub burden_class: BurdenClass,
    pub path: Vec<String>,
    pub loss_count: usize,
    pub surface_penalty: usize,
    pub reversibility: Vec<ReversibilityClass>,
    pub proof_shapes: Vec<String>,
    pub winner_state: WinnerState,
    pub surface_transition: Option<SurfaceTransitionCost>,
}

#[derive(Debug, Clone)]
pub struct CompiledAtlas {
    pub edges: Vec<CompiledEdge>,
    by_src_tgt: HashMap<(String, String), Vec<usize>>,
    by_src_tgt_burden: HashMap<(String, String, BurdenClass), Vec<usize>>,
}

#[derive(Debug, Error)]
pub enum AtlasError {
    #[error("unknown bridge `{0}`")]
    UnknownBridge(String),
    #[error("no route found")]
    NoRoute,
}

impl CompiledAtlas {
    pub fn compile<R: RegistryLookup + ?Sized>(registry: &R) -> Result<Self, AtlasError> {
        let mut edges = Vec::new();
        let mut by_src_tgt = HashMap::new();
        let mut by_src_tgt_burden = HashMap::new();

        for cell in registry.atlas_cells() {
            let reversibility = cell
                .normalized_winner
                .iter()
                .map(|id| {
                    registry
                        .get_bridge(id)
                        .map(|bridge| bridge.reversibility)
                        .ok_or_else(|| AtlasError::UnknownBridge(id.clone()))
                })
                .collect::<Result<Vec<_>, _>>()?;

            let edge = CompiledEdge {
                atlas_cell_id: cell.id.clone(),
                src: cell.source_regime.clone(),
                tgt: cell.target_regime.clone(),
                burden_class: cell.burden_class.clone(),
                path: cell.normalized_winner.clone(),
                loss_count: cell.loss_profile.items.len(),
                surface_penalty: cell
                    .surface_transition
                    .as_ref()
                    .map(|item| item.total_penalty)
                    .unwrap_or_else(|| infer_surface_penalty(&cell, None)),
                reversibility,
                proof_shapes: cell.proof_shapes_checked.clone(),
                winner_state: cell.winner_state.clone(),
                surface_transition: cell.surface_transition.clone(),
            };
            let index = edges.len();
            edges.push(edge);
            by_src_tgt
                .entry((cell.source_regime.clone(), cell.target_regime.clone()))
                .or_insert_with(Vec::new)
                .push(index);
            by_src_tgt_burden
                .entry((
                    cell.source_regime.clone(),
                    cell.target_regime.clone(),
                    cell.burden_class.clone(),
                ))
                .or_insert_with(Vec::new)
                .push(index);
        }

        Ok(Self {
            edges,
            by_src_tgt,
            by_src_tgt_burden,
        })
    }

    pub fn compile_summary(&self) -> AtlasCompileSummary {
        AtlasCompileSummary {
            edge_count: self.edges.len(),
            indexed_src_tgt_pairs: self.by_src_tgt.len(),
            indexed_burden_pairs: self.by_src_tgt_burden.len(),
        }
    }

    pub fn select_policy_driven(
        &self,
        src: &str,
        tgt: &str,
        burden_class: Option<&BurdenClass>,
        budget: Option<&Budget>,
        surface_requirement: Option<&SurfaceRequirement>,
        preferred_target: Option<&SurfacePreferredTarget>,
        resolution: Option<&PolicyResolution>,
        bundle_resolution_ok: bool,
    ) -> Result<RouteSelection, AtlasError> {
        let indices = if let Some(burden_class) = burden_class {
            self.by_src_tgt_burden
                .get(&(src.to_string(), tgt.to_string(), burden_class.clone()))
                .cloned()
                .unwrap_or_default()
        } else {
            self.by_src_tgt
                .get(&(src.to_string(), tgt.to_string()))
                .cloned()
                .unwrap_or_default()
        };

        let mut candidate_edges = indices
            .into_iter()
            .filter_map(|index| self.edges.get(index).cloned())
            .filter(|edge| {
                budget
                    .map(|budget| edge.loss_count <= budget.max_loss)
                    .unwrap_or(true)
            })
            .collect::<Vec<_>>();

        for edge in &mut candidate_edges {
            edge.surface_penalty =
                score_surface_penalty(edge, surface_requirement, preferred_target);
        }

        let policy = resolution
            .map(|item| item.optimizer.optimizer_policy.clone())
            .unwrap_or(OptimizerPolicy::Conservative);
        let backend = resolution
            .map(|item| item.optimizer.backend.clone())
            .unwrap_or(OptimizerBackend::Lexicographic);
        let axes = resolution
            .map(|item| item.optimizer.active_axes.clone())
            .unwrap_or_else(optimizer_axes);

        candidate_edges.sort_by(|left, right| {
            let left_score = route_score(left, &policy, &axes, bundle_resolution_ok);
            let right_score = route_score(right, &policy, &axes, bundle_resolution_ok);
            left_score.cmp(&right_score)
        });

        let dominated_candidates = if backend == OptimizerBackend::ParetoBounded {
            pareto_dominated(&candidate_edges, &axes, bundle_resolution_ok)
        } else {
            candidate_edges
                .iter()
                .skip(1)
                .map(|edge| edge.atlas_cell_id.clone())
                .collect::<Vec<_>>()
        };

        let candidates = candidate_edges
            .iter()
            .map(to_atlas_cell_stub)
            .collect::<Vec<_>>();
        let winner = candidate_edges.first().map(to_atlas_cell_stub);
        if winner.is_none() {
            return Err(AtlasError::NoRoute);
        }

        Ok(RouteSelection {
            candidates,
            winner,
            reasons: vec![format!("compiled atlas {:?} selection", backend)],
            route_explanation: Some(RouteExplanation {
                optimizer_policy: policy,
                optimizer_backend: backend.clone(),
                policy_resolution_id: resolution.map(|item| item.id.clone()),
                winner_atlas_cell_id: candidate_edges
                    .first()
                    .map(|edge| edge.atlas_cell_id.clone()),
                winner_score: candidate_edges
                    .first()
                    .map(|edge| route_score_vector(edge, bundle_resolution_ok)),
                dominated_candidates,
                rejected_candidates: Vec::new(),
                axes_used: axes.clone(),
                explanation: vec![format!(
                    "winner chosen by constitutional {:?} optimizer",
                    backend
                )],
            }),
        })
    }

    pub fn select_lexicographic(
        &self,
        src: &str,
        tgt: &str,
        burden_class: Option<&BurdenClass>,
        budget: Option<&Budget>,
        surface_requirement: Option<&SurfaceRequirement>,
        preferred_target: Option<&SurfacePreferredTarget>,
        optimizer_policy: OptimizerPolicy,
        bundle_resolution_ok: bool,
    ) -> Result<RouteSelection, AtlasError> {
        let resolution = PolicyResolution {
            id: "MPR_INLINE".into(),
            scope: l64_core::PolicyScope::Global,
            applied_policy_ids: vec!["INLINE".into()],
            conflicts: Vec::new(),
            trace: l64_core::PolicyTrace {
                id: "MPT_INLINE".into(),
                steps: vec!["inline optimizer policy".into()],
            },
            optimizer: l64_core::OptimizerPolicyConfig {
                optimizer_policy,
                backend: OptimizerBackend::Lexicographic,
                active_axes: optimizer_axes(),
                route_explanation_verbosity: "standard".into(),
                symbolic_fidelity_preferred: false,
                tie_break_rules: vec!["shorter-path".into()],
            },
            evaluator: l64_core::EvaluatorPolicyConfig {
                evidence_preference: l64_core::EvidencePreference::RecomputeIfSupported,
                allow_approximation: true,
                unsupported_mode: l64_core::UnsupportedHandlingMode::Permit,
                require_symbolic_fidelity_route: false,
                prefer_comp_replay: true,
            },
            replay_cache: l64_core::ReplayCachePolicyConfig {
                replay_allowed: true,
                exact_policy_match_required: true,
                survive_surface_only_changes: false,
                reuse_approximate_results: false,
                optimizer_change_invalidates: true,
                surface_pack_change_invalidates: true,
                trust_class: l64_core::ReplayTrustClass::ExactPolicyOnly,
            },
            report: l64_core::ReportPolicyConfig {
                export_surfaces: vec![SurfaceKind::Qc0, SurfaceKind::Qm0, SurfaceKind::Qa0],
                include_policy_trace: true,
                include_route_explanation: true,
                include_obligation_logs: true,
            },
            scheduler: l64_core::SchedulerPolicyConfig {
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
            },
            verdict: l64_core::PolicyVerdict::Applied,
        };
        self.select_policy_driven(
            src,
            tgt,
            burden_class,
            budget,
            surface_requirement,
            preferred_target,
            Some(&resolution),
            bundle_resolution_ok,
        )
    }

    pub fn edge_by_atlas_cell(&self, atlas_cell_id: &str) -> Option<&CompiledEdge> {
        self.edges
            .iter()
            .find(|edge| edge.atlas_cell_id == atlas_cell_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AtlasCompileSummary {
    pub edge_count: usize,
    pub indexed_src_tgt_pairs: usize,
    pub indexed_burden_pairs: usize,
}

fn route_score(
    edge: &CompiledEdge,
    policy: &OptimizerPolicy,
    axes: &[OptimizationAxis],
    bundle_resolution_ok: bool,
) -> Vec<usize> {
    let vector = route_score_vector(edge, bundle_resolution_ok);
    let preferred_axes = if axes.is_empty() {
        match policy {
            OptimizerPolicy::Conservative => optimizer_axes(),
            OptimizerPolicy::SymbolicFidelityFirst => vec![
                OptimizationAxis::SymbolicFidelity,
                OptimizationAxis::SurfaceTransitionPenalty,
                OptimizationAxis::IdentityPreservation,
                OptimizationAxis::LossCompliance,
                OptimizationAxis::ProofShapeSatisfiability,
                OptimizationAxis::ExecutionCost,
            ],
            OptimizerPolicy::ExecutionFirst => vec![
                OptimizationAxis::ExecutionCost,
                OptimizationAxis::LossCompliance,
                OptimizationAxis::Lawfulness,
                OptimizationAxis::BundleResolution,
                OptimizationAxis::SymbolicFidelity,
            ],
            OptimizerPolicy::LowLoss => vec![
                OptimizationAxis::LossCompliance,
                OptimizationAxis::SurfaceTransitionPenalty,
                OptimizationAxis::RollbackViability,
                OptimizationAxis::IdentityPreservation,
                OptimizationAxis::ExecutionCost,
            ],
            OptimizerPolicy::BenchmarkFriendly => vec![
                OptimizationAxis::MaturityConfidence,
                OptimizationAxis::ExecutionCost,
                OptimizationAxis::ProofShapeSatisfiability,
                OptimizationAxis::ReceiptCompleteness,
                OptimizationAxis::LossCompliance,
            ],
        }
    } else {
        axes.to_vec()
    };
    preferred_axes
        .into_iter()
        .map(|axis| axis_value(&vector, &axis))
        .collect()
}

fn route_score_vector(edge: &CompiledEdge, bundle_resolution_ok: bool) -> RouteScoreVector {
    RouteScoreVector {
        lawfulness: usize::from(edge.winner_state != WinnerState::SeedWinner),
        identity_preservation: usize::from(
            edge.reversibility
                .contains(&ReversibilityClass::LossySupported),
        ),
        loss_compliance: edge.loss_count,
        rollback_viability: usize::from(
            edge.reversibility
                .contains(&ReversibilityClass::LossySupported),
        ),
        proof_shape_satisfiability: usize::from(edge.proof_shapes.is_empty()),
        bundle_resolution: usize::from(!bundle_resolution_ok),
        surface_transition_penalty: edge.surface_penalty,
        execution_cost: edge.path.len(),
        derived_obligation_depth: usize::from(edge.proof_shapes.is_empty()),
        maturity_confidence: usize::from(edge.winner_state != WinnerState::SeedWinner),
        symbolic_fidelity: match edge
            .surface_transition
            .as_ref()
            .map(|item| &item.compatibility)
        {
            Some(
                SurfaceCompatibilityClass::AuthorityPreserving
                | SurfaceCompatibilityClass::SymbolicFidelityPreserving,
            ) => 0,
            Some(SurfaceCompatibilityClass::DebugMirrorOnly) => 2,
            Some(SurfaceCompatibilityClass::IngressProjectionOnly) => 3,
            None => 1,
        },
        receipt_completeness: usize::from(edge.surface_transition.is_none()),
    }
}

fn optimizer_axes() -> Vec<OptimizationAxis> {
    vec![
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
    ]
}

fn axis_value(vector: &RouteScoreVector, axis: &OptimizationAxis) -> usize {
    match axis {
        OptimizationAxis::Lawfulness => vector.lawfulness,
        OptimizationAxis::IdentityPreservation => vector.identity_preservation,
        OptimizationAxis::LossCompliance => vector.loss_compliance,
        OptimizationAxis::RollbackViability => vector.rollback_viability,
        OptimizationAxis::ProofShapeSatisfiability => vector.proof_shape_satisfiability,
        OptimizationAxis::BundleResolution => vector.bundle_resolution,
        OptimizationAxis::SurfaceTransitionPenalty => vector.surface_transition_penalty,
        OptimizationAxis::ExecutionCost => vector.execution_cost,
        OptimizationAxis::DerivedObligationDepth => vector.derived_obligation_depth,
        OptimizationAxis::MaturityConfidence => vector.maturity_confidence,
        OptimizationAxis::SymbolicFidelity => vector.symbolic_fidelity,
        OptimizationAxis::ReceiptCompleteness => vector.receipt_completeness,
    }
}

fn pareto_dominated(
    edges: &[CompiledEdge],
    axes: &[OptimizationAxis],
    bundle_resolution_ok: bool,
) -> Vec<String> {
    let mut dominated = Vec::new();
    for (i, edge) in edges.iter().enumerate() {
        let score = route_score_vector(edge, bundle_resolution_ok);
        let is_dominated = edges.iter().enumerate().any(|(j, other)| {
            if i == j {
                return false;
            }
            let other_score = route_score_vector(other, bundle_resolution_ok);
            dominates(&other_score, &score, axes)
        });
        if is_dominated {
            dominated.push(edge.atlas_cell_id.clone());
        }
    }
    dominated
}

fn dominates(left: &RouteScoreVector, right: &RouteScoreVector, axes: &[OptimizationAxis]) -> bool {
    let mut strictly_better = false;
    for axis in axes {
        let left_value = axis_value(left, axis);
        let right_value = axis_value(right, axis);
        if left_value > right_value {
            return false;
        }
        if left_value < right_value {
            strictly_better = true;
        }
    }
    strictly_better
}

fn to_atlas_cell_stub(edge: &CompiledEdge) -> AtlasCell {
    AtlasCell {
        id: edge.atlas_cell_id.clone(),
        source_regime: edge.src.clone(),
        target_regime: edge.tgt.clone(),
        burden_class: edge.burden_class.clone(),
        proof_target: "compiled".into(),
        candidate_paths: vec![edge.path.clone()],
        normalized_winner: edge.path.clone(),
        winner_state: edge.winner_state.clone(),
        loss_profile: l64_core::LossProfile {
            items: vec!["compiled".into(); edge.loss_count],
        },
        proof_shapes_checked: edge.proof_shapes.clone(),
        recipe_maturity: l64_core::RecipeMaturity::Stable,
        failure_signatures: Vec::new(),
        side_conditions: vec!["compiled-atlas".into()],
        surface_transition: edge.surface_transition.clone(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TheoremExecution {
    pub theorem_id: String,
    pub selected_route: Vec<String>,
    pub verdict: CertificationVerdict,
}

pub fn run_seed_theorem<R: RegistryLookup + ?Sized>(
    registry: &R,
    theorem_id: &str,
    atlas: &CompiledAtlas,
) -> Result<TheoremExecution, AtlasError> {
    let theorem = registry
        .get_theorem_spec(theorem_id)
        .ok_or(AtlasError::NoRoute)?;
    let src = theorem.hosts.first().ok_or(AtlasError::NoRoute)?;
    let tgt = theorem.hosts.last().ok_or(AtlasError::NoRoute)?;
    let route = atlas.select_lexicographic(
        src,
        tgt,
        None,
        None,
        None,
        None,
        OptimizerPolicy::Conservative,
        true,
    )?;
    let selected = route.winner.ok_or(AtlasError::NoRoute)?;
    let kernel = ConstitutionKernel;
    let path = selected.normalized_winner.clone();
    let bridges = path
        .iter()
        .filter_map(|id| registry.get_bridge(id))
        .collect::<Vec<_>>();
    kernel
        .compose_bridge_path(&bridges, None)
        .map_err(|_| AtlasError::NoRoute)?;

    Ok(TheoremExecution {
        theorem_id: theorem.id,
        selected_route: path,
        verdict: theorem.verdict,
    })
}

fn infer_surface_penalty(cell: &AtlasCell, requirement: Option<&SurfaceRequirement>) -> usize {
    let mut penalty = 0usize;
    if cell
        .loss_profile
        .items
        .iter()
        .any(|item| item.contains("ascii") || item.contains("debug"))
    {
        penalty += 3;
    }
    if cell
        .side_conditions
        .iter()
        .any(|item| item.contains("qk-ingress-only") || item.contains("projection"))
    {
        penalty += 2;
    }
    if requirement
        .map(|item| item.require_symbolic_fidelity)
        .unwrap_or(false)
        && cell
            .loss_profile
            .items
            .iter()
            .any(|item| item.contains("ascii") || item.contains("debug"))
    {
        penalty += 4;
    }
    penalty
}

fn score_surface_penalty(
    edge: &CompiledEdge,
    requirement: Option<&SurfaceRequirement>,
    preferred_target: Option<&SurfacePreferredTarget>,
) -> usize {
    let mut penalty = edge.surface_penalty;
    if let Some(requirement) = requirement {
        if requirement.keyboard_projection_ingress_only
            && matches!(requirement.preferred_output, Some(SurfaceKind::Qk0))
        {
            penalty += 8;
        }
        if requirement.require_symbolic_fidelity
            && matches!(
                edge.surface_transition
                    .as_ref()
                    .map(|item| &item.compatibility),
                Some(
                    SurfaceCompatibilityClass::DebugMirrorOnly
                        | SurfaceCompatibilityClass::IngressProjectionOnly
                )
            )
        {
            penalty += 6;
        }
        if requirement.transform_receipts_mandatory && edge.surface_transition.is_none() {
            penalty += 3;
        }
    }
    if let Some(preferred) = preferred_target {
        if preferred.surface_kind == SurfaceKind::Qm0
            && matches!(
                edge.surface_transition
                    .as_ref()
                    .map(|item| &item.compatibility),
                Some(SurfaceCompatibilityClass::DebugMirrorOnly)
            )
        {
            penalty += 5;
        }
    }
    penalty
}

#[cfg(test)]
mod tests {
    use super::*;
    use l64_registry::SeedRegistry;

    #[test]
    fn compiled_atlas_selects_seed_route() {
        let registry = SeedRegistry::load().unwrap();
        let atlas = CompiledAtlas::compile(&registry).unwrap();
        let result = atlas
            .select_lexicographic(
                "R_TYP",
                "R_SET",
                None,
                None,
                None,
                None,
                OptimizerPolicy::Conservative,
                true,
            )
            .unwrap();
        assert_eq!(result.winner.unwrap().id, "A_TYPE_TO_SET");
    }
}
