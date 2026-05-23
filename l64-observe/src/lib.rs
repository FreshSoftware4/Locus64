use anyhow::{Result, anyhow};
use l64_atlas::CompiledAtlas;
use l64_bundle::{BundleWorld, load_bundle_world};
use l64_cert::{
    CertificationOptions, certify_derived_campaign_with_options,
    certify_derived_theorem_with_options, replay_report,
};
use l64_core::{
    BundleLock, CertificationReport, DriftReport, ExecutionEvent, ExecutionEventKind,
    ExecutionManifest, ExecutionScheduleHash, ExecutionScope, ExplanationObject, ImpactPrediction,
    LocusCapabilityMask, LocusOpcode, LocusPacketKind, ObligationDrift, ObligationStatus,
    ObservationEdge, ObservationGraph, ObservationNode, ObservationNodeKind, ObservationRecord,
    OrderingReceipt, ParallelizationPolicy, PlanExecutionRecord, PolicyResolution,
    PredictionAssessment, PredictionAssessmentOutcome, PredictionClass, RecomputationPlan,
    RecomputationStep, RecomputationStepKind, ReconciliationRecord, RegistryLookup, ReplayStatus,
    ReuseTrustClass, RiskClassification, RouteExplanation, ScheduledStepStatus,
    SchedulerDecisionReceipt, SchedulerPolicy, SemanticDiff, SemanticDiffClass,
    StepExecutionOutcome, TheoremDrift, resolve_cache_root, runtime_root_report,
};
use l64_locus::{read_section_packet_or_json, write_section_packet};
use l64_policy::resolve_policy_graph;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fs,
    path::PathBuf,
    sync::mpsc,
    thread,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationArtifact {
    pub record: ObservationRecord,
    pub graph: ObservationGraph,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObserveRunResult {
    pub artifact: ObservationArtifact,
    pub persisted_path: String,
}

fn receipt_count(receipts: &[l64_core::ObligationEvidenceReceipt]) -> usize {
    receipts
        .iter()
        .map(|receipt| 1 + receipt_count(&receipt.subreceipts))
        .sum()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PredictionTarget {
    ProposedBundle {
        bundle_id: String,
        policy_resolution: Option<PolicyResolution>,
        theorem_ids: Vec<String>,
        campaign_ids: Vec<String>,
        route_candidate_ids: Vec<String>,
        dependency_ids: Vec<String>,
    },
    PolicyOverride {
        policy_id: String,
        kind: String,
        notes: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PredictionRecord {
    pub prediction: ImpactPrediction,
    pub target: PredictionTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutePlanOptions {
    pub dry_run: bool,
    pub no_cache: bool,
    pub strict: bool,
    pub force_parallel: bool,
    pub force_parallel_obligations: bool,
    pub force_serialized: bool,
    pub max_workers: Option<usize>,
    pub max_obligation_workers: Option<usize>,
    pub strict_determinism: bool,
}

impl Default for ExecutePlanOptions {
    fn default() -> Self {
        Self {
            dry_run: false,
            no_cache: false,
            strict: false,
            force_parallel: false,
            force_parallel_obligations: false,
            force_serialized: false,
            max_workers: None,
            max_obligation_workers: None,
            strict_determinism: true,
        }
    }
}

#[derive(Debug, Clone)]
struct ExecutedTaskBatch {
    reports: Vec<CertificationReport>,
    lane_records: Vec<l64_core::LaneExecutionRecord>,
    schedule_hash: ExecutionScheduleHash,
    coherence_receipts: Vec<l64_core::ConcurrencyCoherenceReceipt>,
    ordering_receipt: OrderingReceipt,
    explanation: Vec<String>,
}

impl ExecutedTaskBatch {
    fn empty() -> Self {
        Self {
            reports: Vec::new(),
            lane_records: Vec::new(),
            schedule_hash: ExecutionScheduleHash {
                id: "SCHH_EMPTY".into(),
                hash: "empty".into(),
            },
            coherence_receipts: Vec::new(),
            ordering_receipt: OrderingReceipt {
                id: "ORD_EMPTY".into(),
                ordered_step_ids: Vec::new(),
                notes: vec!["no execution steps".into()],
            },
            explanation: Vec::new(),
        }
    }
}

fn cache_root() -> Result<PathBuf> {
    let root = PathBuf::from(
        resolve_cache_root()
            .map_err(anyhow::Error::msg)?
            .absolute_path,
    )
    .join("observe");
    fs::create_dir_all(&root)?;
    Ok(root)
}

fn observations_root() -> Result<PathBuf> {
    let root = cache_root()?.join("observations");
    fs::create_dir_all(&root)?;
    Ok(root)
}

fn diffs_root() -> Result<PathBuf> {
    let root = cache_root()?.join("diffs");
    fs::create_dir_all(&root)?;
    Ok(root)
}

fn predictions_root() -> Result<PathBuf> {
    let root = cache_root()?.join("predictions");
    fs::create_dir_all(&root)?;
    Ok(root)
}

fn plans_root() -> Result<PathBuf> {
    let root = cache_root()?.join("plans");
    fs::create_dir_all(&root)?;
    Ok(root)
}

fn explanations_root() -> Result<PathBuf> {
    let root = cache_root()?.join("explanations");
    fs::create_dir_all(&root)?;
    Ok(root)
}

fn assessments_root() -> Result<PathBuf> {
    let root = cache_root()?.join("assessments");
    fs::create_dir_all(&root)?;
    Ok(root)
}

fn executions_root() -> Result<PathBuf> {
    let root = cache_root()?.join("executions");
    fs::create_dir_all(&root)?;
    Ok(root)
}

fn reconciliations_root() -> Result<PathBuf> {
    let root = cache_root()?.join("reconciliations");
    fs::create_dir_all(&root)?;
    Ok(root)
}

fn locus_store_path(root: &PathBuf, id: &str) -> PathBuf {
    root.join(format!("{id}.locus"))
}

fn legacy_json_store_path(root: &PathBuf, id: &str) -> PathBuf {
    root.join(format!("{id}.json"))
}

fn persist_store_payload<T: Serialize>(
    root: PathBuf,
    id: &str,
    kind: LocusPacketKind,
    opcode: LocusOpcode,
    schema_hash: &str,
    payload: &T,
    capabilities: LocusCapabilityMask,
) -> Result<PathBuf> {
    let path = locus_store_path(&root, id);
    write_section_packet(
        &path,
        kind,
        opcode,
        id,
        schema_hash,
        payload,
        capabilities,
        1,
    )
    .map_err(anyhow::Error::msg)?;
    Ok(path)
}

fn load_store_payload<T: DeserializeOwned>(
    root: PathBuf,
    id: &str,
    opcode: LocusOpcode,
) -> Result<T> {
    let locus_path = locus_store_path(&root, id);
    let legacy = legacy_json_store_path(&root, id);
    read_section_packet_or_json(&locus_path, &legacy, opcode).map_err(anyhow::Error::msg)
}

pub fn observe_report(
    report: &CertificationReport,
    manifest: Option<&ExecutionManifest>,
    lock: Option<&BundleLock>,
) -> Result<ObserveRunResult> {
    let report_id = report_id(report);
    let record_id = format!("OBS_{report_id}");
    let graph_id = format!("OBG_{report_id}");
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut events = Vec::new();
    let mut choices = Vec::new();
    let mut obligation_traces = Vec::new();
    let mut route_events = Vec::new();
    let mut cache_events = Vec::new();

    nodes.push(ObservationNode {
        id: format!("OBN_{report_id}_REPORT"),
        kind: ObservationNodeKind::Report,
        ref_id: report_id.clone(),
        label: format!("report {}", report.theorem_id),
        attributes: btreemap([("verdict", format!("{:?}", report.verdict))]),
    });
    nodes.push(ObservationNode {
        id: format!("OBN_{report_id}_THEOREM"),
        kind: ObservationNodeKind::Theorem,
        ref_id: report.theorem_id.clone(),
        label: report.theorem_id.clone(),
        attributes: BTreeMap::new(),
    });
    edges.push(edge(
        &format!("OBN_{report_id}_THEOREM"),
        &format!("OBN_{report_id}_REPORT"),
        "reported-as",
    ));

    if let Some(campaign_id) = &report.campaign_id {
        nodes.push(ObservationNode {
            id: format!("OBN_{report_id}_CAMPAIGN"),
            kind: ObservationNodeKind::Campaign,
            ref_id: campaign_id.clone(),
            label: campaign_id.clone(),
            attributes: BTreeMap::new(),
        });
        edges.push(edge(
            &format!("OBN_{report_id}_CAMPAIGN"),
            &format!("OBN_{report_id}_REPORT"),
            "certified-as",
        ));
    }

    if let Some(policy_resolution) = &report.policy_resolution {
        nodes.push(ObservationNode {
            id: format!("OBN_{report_id}_POLICY"),
            kind: ObservationNodeKind::PolicyResolution,
            ref_id: policy_resolution.id.clone(),
            label: policy_resolution.id.clone(),
            attributes: btreemap([
                ("scope", format!("{:?}", policy_resolution.scope)),
                ("verdict", format!("{:?}", policy_resolution.verdict)),
            ]),
        });
        edges.push(edge(
            &format!("OBN_{report_id}_POLICY"),
            &format!("OBN_{report_id}_REPORT"),
            "governs",
        ));
        for policy_id in &policy_resolution.applied_policy_ids {
            let node_id = format!("OBN_{report_id}_APPLIED_{policy_id}");
            nodes.push(ObservationNode {
                id: node_id.clone(),
                kind: ObservationNodeKind::PolicyObject,
                ref_id: policy_id.clone(),
                label: policy_id.clone(),
                attributes: BTreeMap::new(),
            });
            edges.push(edge(
                &node_id,
                &format!("OBN_{report_id}_POLICY"),
                "applied-by",
            ));
        }
    }

    if let Some(winner) = &report.selected_atlas_cell {
        nodes.push(ObservationNode {
            id: format!("OBN_{report_id}_ROUTE"),
            kind: ObservationNodeKind::RouteWinner,
            ref_id: winner.clone(),
            label: winner.clone(),
            attributes: btreemap([("path", report.selected_path.join("->"))]),
        });
        edges.push(edge(
            &format!("OBN_{report_id}_ROUTE"),
            &format!("OBN_{report_id}_REPORT"),
            "selected-for",
        ));
        events.push(ExecutionEvent {
            id: format!("EVT_{report_id}_ROUTE"),
            kind: ExecutionEventKind::RouteSelected,
            summary: format!("selected route winner {winner}"),
            related_ids: vec![winner.clone(), report_id.clone()],
        });
    }

    if let Some(explanation) = &report.route_explanation {
        route_events.push(route_trace(report, explanation));
        choices.push(mk_choice(report, explanation));
    }

    for obligation in &report.obligations {
        let node_id = format!("OBN_{report_id}_OBL_{}", obligation.obligation_id);
        nodes.push(ObservationNode {
            id: node_id.clone(),
            kind: ObservationNodeKind::Obligation,
            ref_id: obligation.obligation_id.clone(),
            label: obligation.obligation_id.clone(),
            attributes: btreemap([
                ("verdict", format!("{:?}", obligation.verdict)),
                ("mode", format!("{:?}", obligation.evaluation_mode)),
                (
                    "receipt_count",
                    receipt_count(&obligation.receipts).to_string(),
                ),
            ]),
        });
        edges.push(edge(
            &node_id,
            &format!("OBN_{report_id}_REPORT"),
            "evaluated-for",
        ));
        obligation_traces.push(mk_obligation_trace(report, obligation));
        events.push(ExecutionEvent {
            id: format!("EVT_{report_id}_OBL_{}", obligation.obligation_id),
            kind: ExecutionEventKind::ObligationEvaluated,
            summary: format!(
                "obligation {} {:?} {:?}",
                obligation.obligation_id, obligation.verdict, obligation.evaluation_mode
            ),
            related_ids: vec![report_id.clone(), obligation.obligation_id.clone()],
        });
    }

    if let Some(envelope) = &report.execution_envelope {
        if let Some(bundle_id) = &envelope.bundle_id {
            nodes.push(ObservationNode {
                id: format!("OBN_{report_id}_BUNDLE"),
                kind: ObservationNodeKind::Bundle,
                ref_id: bundle_id.clone(),
                label: bundle_id.clone(),
                attributes: btreemap([("bundle_hash", envelope.bundle_hash.clone())]),
            });
            edges.push(edge(
                &format!("OBN_{report_id}_BUNDLE"),
                &format!("OBN_{report_id}_REPORT"),
                "executed-as",
            ));
        }
        cache_events.push(l64_core::CacheReplayEvent {
            id: format!("CCE_{report_id}"),
            report_id: report_id.clone(),
            replay_status: envelope.replay_status.clone(),
            reason: format!("report replay status {:?}", envelope.replay_status),
        });
        events.push(ExecutionEvent {
            id: format!("EVT_{report_id}_CACHE"),
            kind: match envelope.replay_status {
                ReplayStatus::Fresh => ExecutionEventKind::CacheMiss,
                ReplayStatus::CacheHit => ExecutionEventKind::CacheHit,
                ReplayStatus::ReplayOnly => ExecutionEventKind::ReplayRun,
                ReplayStatus::Invalidated => ExecutionEventKind::ReplayRun,
            },
            summary: format!("replay status {:?}", envelope.replay_status),
            related_ids: vec![report_id.clone()],
        });
    }

    if let Some(manifest) = manifest {
        nodes.push(ObservationNode {
            id: format!("OBN_{report_id}_MANIFEST"),
            kind: ObservationNodeKind::Manifest,
            ref_id: manifest.id.clone(),
            label: manifest.id.clone(),
            attributes: btreemap([
                ("bundle_id", manifest.bundle_id.clone()),
                ("policy_hash", manifest.policy_manifest.policy_hash.clone()),
            ]),
        });
        edges.push(edge(
            &format!("OBN_{report_id}_MANIFEST"),
            &format!("OBN_{report_id}_REPORT"),
            "materializes",
        ));
    }

    if let Some(lock) = lock {
        nodes.push(ObservationNode {
            id: format!("OBN_{report_id}_LOCK"),
            kind: ObservationNodeKind::Lock,
            ref_id: lock.id.clone(),
            label: lock.id.clone(),
            attributes: btreemap([("manifest_id", lock.manifest_id.clone())]),
        });
        edges.push(edge(
            &format!("OBN_{report_id}_LOCK"),
            &format!("OBN_{report_id}_REPORT"),
            "locks",
        ));
    }

    let graph = ObservationGraph {
        id: graph_id.clone(),
        record_id: record_id.clone(),
        nodes,
        edges,
        choices,
        obligation_traces,
        route_events,
        cache_events,
    };
    let record = ObservationRecord {
        id: record_id.clone(),
        report_id: report_id.clone(),
        theorem_id: report.theorem_id.clone(),
        campaign_id: report.campaign_id.clone(),
        bundle_id: report
            .execution_envelope
            .as_ref()
            .and_then(|item| item.bundle_id.clone()),
        manifest_id: manifest.map(|item| item.id.clone()),
        lock_id: lock.map(|item| item.id.clone()),
        graph_id,
        events,
    };
    let artifact = ObservationArtifact { record, graph };
    let path = persist_store_payload(
        observations_root()?,
        &record_id,
        LocusPacketKind::CanonicalTransfer,
        LocusOpcode::Forensic,
        "observe_artifact.v1",
        &artifact,
        LocusCapabilityMask {
            has_forensic: true,
            ..Default::default()
        },
    )?;
    Ok(ObserveRunResult {
        artifact,
        persisted_path: path.display().to_string(),
    })
}

pub fn load_observation(id: &str) -> Result<ObservationArtifact> {
    load_store_payload(observations_root()?, id, LocusOpcode::Forensic)
}

pub fn compare_reports(
    left: &CertificationReport,
    right: &CertificationReport,
) -> Result<SemanticDiff> {
    let diff = semantic_diff_between_reports(left, right);
    persist_diff(&diff)?;
    Ok(diff)
}

pub fn compare_manifests(
    left: &ExecutionManifest,
    right: &ExecutionManifest,
) -> Result<SemanticDiff> {
    let mut summary = Vec::new();
    let mut changed_policies = Vec::new();
    let mut changed_routes = Vec::new();
    let mut changed_deps = Vec::new();
    if left.policy_manifest.policy_hash != right.policy_manifest.policy_hash {
        summary.push("policy graph changed".into());
        changed_policies.extend(right.policy_manifest.policy_ids.clone());
    }
    if left.route_winner_ids != right.route_winner_ids {
        summary.push("route winners changed".into());
        changed_routes.extend(symmetric_diff(
            &left.route_winner_ids,
            &right.route_winner_ids,
        ));
    }
    if left.dependency_graph != right.dependency_graph {
        summary.push("bundle dependency graph changed".into());
        changed_deps.extend(diff_dependencies(
            &left.dependency_graph,
            &right.dependency_graph,
        ));
    }
    let diff = SemanticDiff {
        id: format!("DIF_{}", stable_id(&(left.id.clone() + &right.id))),
        left_id: left.id.clone(),
        right_id: right.id.clone(),
        class: classify(
            !changed_policies.is_empty(),
            !changed_routes.is_empty(),
            false,
            false,
            !changed_deps.is_empty(),
        ),
        summary: if summary.is_empty() {
            vec!["no semantic manifest drift".into()]
        } else {
            summary
        },
        theorem_drifts: Vec::new(),
        changed_policy_ids: changed_policies,
        changed_route_ids: changed_routes,
        changed_dependency_ids: changed_deps,
    };
    persist_diff(&diff)?;
    Ok(diff)
}

pub fn compare_locks(
    left_lock: &BundleLock,
    left_manifest: &ExecutionManifest,
    right_lock: &BundleLock,
    right_manifest: &ExecutionManifest,
) -> Result<SemanticDiff> {
    let mut diff = compare_manifests(left_manifest, right_manifest)?;
    diff.left_id = left_lock.id.clone();
    diff.right_id = right_lock.id.clone();
    if left_lock.report_ids != right_lock.report_ids {
        diff.summary.push("locked report set changed".into());
    }
    persist_diff(&diff)?;
    Ok(diff)
}

pub fn compare_report_manifest(
    report: &CertificationReport,
    manifest: &ExecutionManifest,
) -> Result<SemanticDiff> {
    let report_id = report_id(report);
    let route_change = report
        .selected_atlas_cell
        .as_ref()
        .map(|winner| !manifest.route_winner_ids.contains(winner))
        .unwrap_or(false);
    let policy_change = report
        .execution_envelope
        .as_ref()
        .map(|item| item.policy_hash != manifest.policy_manifest.policy_hash)
        .unwrap_or(true);
    let diff = SemanticDiff {
        id: format!("DIF_{}", stable_id(&(report_id.clone() + &manifest.id))),
        left_id: report_id,
        right_id: manifest.id.clone(),
        class: classify(policy_change, route_change, false, false, false),
        summary: {
            let mut summary = Vec::new();
            if policy_change {
                summary.push("report policy provenance differs from manifest".into());
            }
            if route_change {
                summary.push("report route winner not present in manifest".into());
            }
            if summary.is_empty() {
                summary.push("report matches manifest semantically".into());
            }
            summary
        },
        theorem_drifts: Vec::new(),
        changed_policy_ids: manifest.policy_manifest.policy_ids.clone(),
        changed_route_ids: if route_change {
            manifest.route_winner_ids.clone()
        } else {
            Vec::new()
        },
        changed_dependency_ids: Vec::new(),
    };
    persist_diff(&diff)?;
    Ok(diff)
}

pub fn compare_bundle_worlds(left: &BundleWorld, right: &BundleWorld) -> Result<SemanticDiff> {
    let mut summary = Vec::new();
    let changed_dependencies =
        diff_dependencies(&left.manifest.dependencies, &right.manifest.dependencies);
    let changed_policies = symmetric_diff(
        &left
            .overlay
            .local
            .policy_objects
            .iter()
            .map(|item| item.id.clone())
            .collect::<Vec<_>>(),
        &right
            .overlay
            .local
            .policy_objects
            .iter()
            .map(|item| item.id.clone())
            .collect::<Vec<_>>(),
    );
    let local_semantics_changed = left.overlay.local != right.overlay.local;
    if local_semantics_changed {
        summary.push("bundle-local semantic payload changed".into());
    }
    if !changed_policies.is_empty() {
        summary.push("bundle-local policy objects changed".into());
    }
    if !changed_dependencies.is_empty() {
        summary.push("bundle dependency graph changed".into());
    }
    let diff = SemanticDiff {
        id: format!(
            "DIF_{}",
            stable_id(&(left.manifest.id.clone() + &right.manifest.id))
        ),
        left_id: left.manifest.id.clone(),
        right_id: right.manifest.id.clone(),
        class: if !local_semantics_changed {
            SemanticDiffClass::NoSemanticChange
        } else {
            classify(
                !changed_policies.is_empty(),
                false,
                false,
                false,
                !changed_dependencies.is_empty(),
            )
        },
        summary: if summary.is_empty() {
            vec!["surface-only or filename-only change lowered to identical semantic bundle".into()]
        } else {
            summary
        },
        theorem_drifts: Vec::new(),
        changed_policy_ids: changed_policies,
        changed_route_ids: Vec::new(),
        changed_dependency_ids: changed_dependencies,
    };
    persist_diff(&diff)?;
    Ok(diff)
}

pub fn explain_drift(diff: &SemanticDiff) -> Result<ExplanationObject> {
    let mut details = diff.summary.clone();
    for theorem in &diff.theorem_drifts {
        if theorem.route_before != theorem.route_after {
            details.push(format!(
                "theorem {} changed route from {:?} to {:?}",
                theorem.theorem_id, theorem.route_before, theorem.route_after
            ));
        }
        for obligation in &theorem.obligation_drifts {
            details.push(format!(
                "obligation {} drifted from {:?}/{:?} to {:?}/{:?}: {}",
                obligation.obligation_id,
                obligation.verdict_before,
                obligation.mode_before,
                obligation.verdict_after,
                obligation.mode_after,
                obligation.meaning
            ));
        }
    }
    let explanation = ExplanationObject {
        id: format!("EXC_{}", stable_id(&diff.id)),
        title: format!("constitutional drift for {}", diff.id),
        details,
        related_ids: vec![diff.left_id.clone(), diff.right_id.clone(), diff.id.clone()],
    };
    persist_store_payload(
        explanations_root()?,
        &explanation.id,
        LocusPacketKind::CanonicalTransfer,
        LocusOpcode::Forensic,
        "explanation_object.v1",
        &explanation,
        LocusCapabilityMask {
            has_forensic: true,
            ..Default::default()
        },
    )?;
    Ok(explanation)
}

pub fn drift_report(diff: &SemanticDiff) -> Result<DriftReport> {
    let report = DriftReport {
        id: format!("DRF_{}", stable_id(&diff.id)),
        diff_id: diff.id.clone(),
        summary: diff.summary.clone(),
        theorem_drifts: diff.theorem_drifts.clone(),
    };
    persist_store_payload(
        explanations_root()?,
        &format!("{}.drift", report.id),
        LocusPacketKind::CanonicalTransfer,
        LocusOpcode::Forensic,
        "drift_report.v1",
        &report,
        LocusCapabilityMask {
            has_forensic: true,
            ..Default::default()
        },
    )?;
    Ok(report)
}

pub fn predict_from_bundle_change(
    baseline_report: &CertificationReport,
    baseline_bundle: Option<&BundleWorld>,
    proposed_bundle: &BundleWorld,
    proposed_resolution: Option<&PolicyResolution>,
) -> Result<PredictionRecord> {
    let mut reasons = Vec::new();
    let mut class = PredictionClass::InconclusivePrediction;
    let mut confidence = RiskClassification::Unknown;

    if let Some(base_bundle) = baseline_bundle {
        let bundle_diff = compare_bundle_worlds(base_bundle, proposed_bundle)?;
        match bundle_diff.class {
            SemanticDiffClass::NoSemanticChange => {
                class = PredictionClass::NoImpactPredicted;
                confidence = RiskClassification::Exact;
                reasons.push("proposed bundle lowers to the same semantic overlay world".into());
            }
            SemanticDiffClass::PolicyOnly => {
                if let Some(resolution) = proposed_resolution {
                    let base = baseline_report.policy_resolution.as_ref();
                    if let Some(base) = base {
                        if base.replay_cache != resolution.replay_cache
                            && base.optimizer == resolution.optimizer
                            && base.evaluator == resolution.evaluator
                        {
                            class = PredictionClass::ReplayOnlyImpact;
                            confidence = RiskClassification::Strong;
                            reasons.push("replay/cache policy changed without route or evaluator policy changes".into());
                        } else if base.optimizer != resolution.optimizer {
                            class = PredictionClass::RouteWinnerChangeLikely;
                            confidence = RiskClassification::Strong;
                            reasons.push("optimizer backend or active axes changed".into());
                        } else if base.evaluator != resolution.evaluator {
                            class = classify_evaluator_prediction(
                                baseline_report,
                                resolution,
                                &mut reasons,
                            );
                            confidence = RiskClassification::Strong;
                        } else {
                            class = PredictionClass::ReportHashChangeLikely;
                            confidence = RiskClassification::Moderate;
                            reasons.push("policy provenance changed without obvious route or obligation trigger".into());
                        }
                    }
                }
            }
            SemanticDiffClass::BundleDependencyOnly => {
                class = PredictionClass::RouteWinnerChangeLikely;
                confidence = RiskClassification::Moderate;
                reasons.push(
                    "bundle dependency graph changed, so route winners may need re-selection"
                        .into(),
                );
            }
            _ => {
                class = PredictionClass::CertificationVerdictChangeLikely;
                confidence = RiskClassification::Moderate;
                reasons.extend(bundle_diff.summary);
            }
        }
    }

    if let Some(resolution) = proposed_resolution {
        if let Some(base) = &baseline_report.policy_resolution {
            if base.optimizer != resolution.optimizer {
                class = PredictionClass::RouteWinnerChangeLikely;
                confidence = RiskClassification::Strong;
                reasons.push("policy graph changes route ranking rules".into());
            }
            if base.evaluator != resolution.evaluator {
                class = classify_evaluator_prediction(baseline_report, resolution, &mut reasons);
                confidence = RiskClassification::Strong;
            }
            if base.replay_cache != resolution.replay_cache
                && matches!(class, PredictionClass::NoImpactPredicted)
            {
                class = PredictionClass::ReplayOnlyImpact;
                confidence = RiskClassification::Strong;
                reasons.push("replay/cache reuse rules changed".into());
            }
        }
    }

    let prediction = ImpactPrediction {
        id: format!(
            "PRD_{}",
            stable_id(&(report_id(baseline_report) + &proposed_bundle.manifest.id))
        ),
        baseline_id: report_id(baseline_report),
        proposed_id: proposed_bundle.manifest.id.clone(),
        class,
        confidence,
        reasons,
        affected_theorems: vec![baseline_report.theorem_id.clone()],
        affected_reports: vec![report_id(baseline_report)],
    };
    let record = PredictionRecord {
        prediction,
        target: PredictionTarget::ProposedBundle {
            bundle_id: proposed_bundle.manifest.id.clone(),
            policy_resolution: proposed_resolution.cloned(),
            theorem_ids: proposed_bundle
                .overlay
                .local
                .theorem_specs
                .iter()
                .map(|item| item.id.clone())
                .collect(),
            campaign_ids: proposed_bundle
                .overlay
                .local
                .campaigns
                .iter()
                .map(|item| item.id.clone())
                .collect(),
            route_candidate_ids: proposed_bundle
                .overlay
                .local
                .atlas_cells
                .iter()
                .map(|item| item.id.clone())
                .collect(),
            dependency_ids: proposed_bundle
                .manifest
                .dependencies
                .iter()
                .map(|item| item.id.clone())
                .collect(),
        },
    };
    persist_prediction(&record)?;
    Ok(record)
}

pub fn predict_from_policy_override(
    baseline_report: &CertificationReport,
    policy_id: &str,
    kind: &str,
    notes: Vec<String>,
) -> Result<PredictionRecord> {
    let mut reasons = vec![format!(
        "proposed policy override {policy_id} of kind {kind}"
    )];
    let (class, confidence) = match kind {
        "Optimizer" => (
            PredictionClass::RouteWinnerChangeLikely,
            RiskClassification::Strong,
        ),
        "Evaluator" => (
            if baseline_report.obligations.iter().any(|item| {
                matches!(
                    item.evaluation_mode,
                    l64_core::ObligationEvaluationMode::Unsupported
                        | l64_core::ObligationEvaluationMode::RecomputedApproximate
                        | l64_core::ObligationEvaluationMode::RecomputedPartial
                )
            }) {
                PredictionClass::ObligationOutcomeChangeLikely
            } else {
                PredictionClass::ReportHashChangeLikely
            },
            RiskClassification::Strong,
        ),
        "ReplayCache" => (
            PredictionClass::ReplayOnlyImpact,
            RiskClassification::Strong,
        ),
        _ => (
            PredictionClass::InconclusivePrediction,
            RiskClassification::Unknown,
        ),
    };
    reasons.extend(notes.clone());
    let record = PredictionRecord {
        prediction: ImpactPrediction {
            id: format!(
                "PRD_{}",
                stable_id(&(report_id(baseline_report) + policy_id))
            ),
            baseline_id: report_id(baseline_report),
            proposed_id: policy_id.to_string(),
            class,
            confidence,
            reasons,
            affected_theorems: vec![baseline_report.theorem_id.clone()],
            affected_reports: vec![report_id(baseline_report)],
        },
        target: PredictionTarget::PolicyOverride {
            policy_id: policy_id.to_string(),
            kind: kind.to_string(),
            notes,
        },
    };
    persist_prediction(&record)?;
    Ok(record)
}

pub fn load_prediction(id: &str) -> Result<PredictionRecord> {
    load_store_payload(predictions_root()?, id, LocusOpcode::Frontier)
}

pub fn load_diff(id: &str) -> Result<SemanticDiff> {
    load_store_payload(diffs_root()?, id, LocusOpcode::Coverage)
}

pub fn load_plan(id: &str) -> Result<RecomputationPlan> {
    load_store_payload(plans_root()?, id, LocusOpcode::Proposal)
}

pub fn load_execution(id: &str) -> Result<PlanExecutionRecord> {
    load_store_payload(executions_root()?, id, LocusOpcode::ReceiptTable)
}

pub fn load_reconciliation(id: &str) -> Result<ReconciliationRecord> {
    load_store_payload(reconciliations_root()?, id, LocusOpcode::Coverage)
}

pub fn plan_recompute_from_prediction(record: &PredictionRecord) -> Result<RecomputationPlan> {
    let mut steps = Vec::new();
    let mut reusable_artifacts = Vec::new();
    let mut invalidated_artifacts = Vec::new();
    match record.prediction.class {
        PredictionClass::NoImpactPredicted => {
            reusable_artifacts.extend(record.prediction.affected_reports.clone());
            steps.push(step(
                "Reuse",
                RecomputationStepKind::Reuse,
                &record.prediction.baseline_id,
                "semantic bundle unchanged",
            ));
        }
        PredictionClass::ReplayOnlyImpact => {
            reusable_artifacts.extend(record.prediction.affected_reports.clone());
            steps.push(step(
                "Replay",
                RecomputationStepKind::Replay,
                &record.prediction.baseline_id,
                "policy change affects replay/cache path only",
            ));
        }
        PredictionClass::ReportHashChangeLikely => {
            reusable_artifacts.push(record.prediction.baseline_id.clone());
            steps.push(step(
                "ReExport",
                RecomputationStepKind::ReExportReport,
                &record.prediction.baseline_id,
                "report provenance likely changes without deeper semantic drift",
            ));
        }
        PredictionClass::RouteWinnerChangeLikely => {
            invalidated_artifacts.push(record.prediction.baseline_id.clone());
            steps.push(step(
                "ReRoute",
                RecomputationStepKind::ReRoute,
                &record.prediction.proposed_id,
                "route ranking or atlas dependencies changed",
            ));
            steps.push(step(
                "ReCert",
                RecomputationStepKind::ReCertifyTheorem,
                &record.prediction.baseline_id,
                "route winner may drift",
            ));
            steps.push(step(
                "ReManifest",
                RecomputationStepKind::RebuildManifestLock,
                &record.prediction.proposed_id,
                "route winner hash participates in manifests and locks",
            ));
        }
        PredictionClass::ObligationOutcomeChangeLikely => {
            invalidated_artifacts.push(record.prediction.baseline_id.clone());
            steps.push(step(
                "ReEval",
                RecomputationStepKind::ReEvaluateObligation,
                &record.prediction.baseline_id,
                "evaluator policy may change obligation outcome",
            ));
            steps.push(step(
                "ReCert",
                RecomputationStepKind::ReCertifyTheorem,
                &record.prediction.baseline_id,
                "verdict may drift after obligation replay",
            ));
        }
        PredictionClass::CertificationVerdictChangeLikely => {
            invalidated_artifacts.push(record.prediction.baseline_id.clone());
            steps.push(step(
                "ReRoute",
                RecomputationStepKind::ReRoute,
                &record.prediction.proposed_id,
                "mixed structural drift",
            ));
            steps.push(step(
                "ReEval",
                RecomputationStepKind::ReEvaluateObligation,
                &record.prediction.baseline_id,
                "mixed structural drift",
            ));
            steps.push(step(
                "ReRun",
                RecomputationStepKind::ReRunCampaign,
                &record.prediction.baseline_id,
                "campaign report may change",
            ));
            steps.push(step(
                "ReManifest",
                RecomputationStepKind::RebuildManifestLock,
                &record.prediction.proposed_id,
                "report and route hashes changed",
            ));
        }
        PredictionClass::BundleConflictRisk | PredictionClass::InconclusivePrediction => {
            steps.push(step(
                "ReRun",
                RecomputationStepKind::ReRunCampaign,
                &record.prediction.proposed_id,
                "structural uncertainty prevents a smaller lawful reuse set",
            ));
        }
    }
    let plan = RecomputationPlan {
        id: format!("RCP_{}", stable_id(&record.prediction.id)),
        prediction_id: Some(record.prediction.id.clone()),
        diff_id: None,
        reusable_artifacts,
        invalidated_artifacts,
        steps,
        explanation: record.prediction.reasons.clone(),
    };
    persist_store_payload(
        plans_root()?,
        &plan.id,
        LocusPacketKind::FrontierEnvelope,
        LocusOpcode::Proposal,
        "recompute_plan.v1",
        &plan,
        LocusCapabilityMask {
            has_frontier: true,
            ..Default::default()
        },
    )?;
    Ok(plan)
}

pub fn plan_recompute_from_diff(diff: &SemanticDiff) -> Result<RecomputationPlan> {
    let prediction = PredictionRecord {
        prediction: ImpactPrediction {
            id: format!("PRD_{}", stable_id(&diff.id)),
            baseline_id: diff.left_id.clone(),
            proposed_id: diff.right_id.clone(),
            class: map_diff_to_prediction(&diff.class),
            confidence: RiskClassification::Moderate,
            reasons: diff.summary.clone(),
            affected_theorems: diff
                .theorem_drifts
                .iter()
                .map(|item| item.theorem_id.clone())
                .collect(),
            affected_reports: vec![diff.left_id.clone()],
        },
        target: PredictionTarget::PolicyOverride {
            policy_id: "derived-from-diff".into(),
            kind: format!("{:?}", diff.class),
            notes: diff.summary.clone(),
        },
    };
    let mut plan = plan_recompute_from_prediction(&prediction)?;
    plan.diff_id = Some(diff.id.clone());
    persist_store_payload(
        plans_root()?,
        &plan.id,
        LocusPacketKind::FrontierEnvelope,
        LocusOpcode::Proposal,
        "recompute_plan.v1",
        &plan,
        LocusCapabilityMask {
            has_frontier: true,
            ..Default::default()
        },
    )?;
    Ok(plan)
}

pub fn assess_prediction(
    record: &PredictionRecord,
    actual_report: &CertificationReport,
) -> Result<PredictionAssessment> {
    let baseline = replay_report(&record.prediction.baseline_id)
        .map_err(|_| anyhow!("baseline report not available for assessment"))?;
    let actual_diff = semantic_diff_between_reports(&baseline, actual_report);
    persist_diff(&actual_diff)?;
    let predicted = severity(&record.prediction.class);
    let actual = severity(&map_diff_to_prediction(&actual_diff.class));
    let outcome = if matches!(
        record.prediction.class,
        PredictionClass::InconclusivePrediction
    ) {
        PredictionAssessmentOutcome::PredictionUncheckable
    } else if predicted == actual {
        PredictionAssessmentOutcome::PredictionConfirmed
    } else if predicted < actual {
        PredictionAssessmentOutcome::PredictionUnderestimated
    } else {
        PredictionAssessmentOutcome::PredictionOverestimated
    };
    let assessment = PredictionAssessment {
        id: format!(
            "RSK_{}",
            stable_id(&(record.prediction.id.clone() + &actual_diff.id))
        ),
        prediction_id: record.prediction.id.clone(),
        actual_diff_id: actual_diff.id.clone(),
        outcome,
        notes: actual_diff.summary.clone(),
    };
    persist_store_payload(
        assessments_root()?,
        &assessment.id,
        LocusPacketKind::FrontierEnvelope,
        LocusOpcode::Coverage,
        "prediction_assessment.v1",
        &assessment,
        LocusCapabilityMask {
            has_frontier: true,
            ..Default::default()
        },
    )?;
    Ok(assessment)
}

pub fn execute_plan(
    plan: &RecomputationPlan,
    options: &ExecutePlanOptions,
) -> Result<PlanExecutionRecord> {
    let prediction = plan
        .prediction_id
        .as_ref()
        .map(|id| load_prediction(id))
        .transpose()?;
    let root_resolution = runtime_root_report(&[]).map_err(anyhow::Error::msg)?;
    let scheduler_policy = scheduler_policy_for(prediction.as_ref(), options);
    let execution_scope = ExecutionScope {
        id: format!(
            "SCP_{}",
            stable_id(&(plan.id.clone() + &scheduler_policy.id))
        ),
        cache_namespace: l64_core::CacheNamespace {
            id: root_resolution
                .cache_root
                .namespace
                .clone()
                .unwrap_or_else(|| "default".into()),
            root: root_resolution.cache_root.absolute_path.clone(),
        },
    };
    let mut outcomes = Vec::new();
    let mut resulting_reports = Vec::new();
    let mut resulting_report_ids = Vec::new();
    let mut manifest_ids = Vec::new();
    let lock_ids = Vec::new();
    let mut rerun_artifacts = Vec::new();
    let reused_artifacts = plan.reusable_artifacts.clone();
    let mut coherence_receipts = Vec::new();
    let mut obligation_plans = Vec::new();
    let mut obligation_lanes = Vec::new();
    let mut obligation_ordering_receipts = Vec::new();
    let mut obligation_merge_receipts = Vec::new();
    let mut replay_legality_checks = Vec::new();
    let mut replay_barrier_receipts = Vec::new();
    let mut replay_merge_receipts = Vec::new();
    let mut replay_divergence_records = Vec::new();
    let mut obligation_cache_shards = Vec::new();
    let mut obligation_write_sets = Vec::new();
    let mut obligation_collision_reports = Vec::new();
    let mut obligation_namespace_receipts = Vec::new();

    let needs_rerun = plan.steps.iter().any(|step| {
        matches!(
            step.kind,
            RecomputationStepKind::ReEvaluateObligation
                | RecomputationStepKind::ReRoute
                | RecomputationStepKind::ReCertifyTheorem
                | RecomputationStepKind::ReRunCampaign
                | RecomputationStepKind::RebuildManifestLock
        )
    });

    let execution_result = if options.dry_run {
        ExecutedTaskBatch::empty()
    } else if let Some(prediction) = prediction.as_ref() {
        if needs_rerun || !scheduler_policy.replay_allowed {
            run_prediction_target(prediction, &scheduler_policy, options)?
        } else {
            let baseline = replay_report(&prediction.prediction.baseline_id)
                .map_err(|err| anyhow!("unable to replay baseline report: {err}"))?;
            ExecutedTaskBatch {
                reports: vec![baseline],
                lane_records: vec![l64_core::LaneExecutionRecord {
                    lane_id: "LAN_0".into(),
                    step_ids: plan.steps.iter().map(|item| item.id.clone()).collect(),
                    task_ids: vec![prediction.prediction.baseline_id.clone()],
                    serialized_reason: Some("replay/cache policy allowed direct reuse".into()),
                }],
                coherence_receipts: Vec::new(),
                schedule_hash: ExecutionScheduleHash {
                    id: format!("SCHH_{}", stable_id(&plan.id)),
                    hash: stable_id(&(plan.id.clone() + "|replay")),
                },
                ordering_receipt: OrderingReceipt {
                    id: format!("ORD_{}", stable_id(&plan.id)),
                    ordered_step_ids: plan.steps.iter().map(|item| item.id.clone()).collect(),
                    notes: vec!["serialized replay ordering".into()],
                },
                explanation: vec!["scheduler reused cached baseline report".into()],
            }
        }
    } else {
        ExecutedTaskBatch::empty()
    };
    let execution_reports = execution_result.reports.clone();

    if !execution_result.coherence_receipts.is_empty() {
        coherence_receipts.extend(execution_result.coherence_receipts.clone());
    }
    rerun_artifacts.extend(execution_reports.iter().map(report_id));
    for report in &execution_reports {
        if let Some(plan) = &report.obligation_plan {
            obligation_plans.push(plan.clone());
        }
        obligation_lanes.extend(report.obligation_lanes.clone());
        if let Some(receipt) = &report.obligation_ordering_receipt {
            obligation_ordering_receipts.push(receipt.clone());
        }
        if let Some(receipt) = &report.obligation_merge_receipt {
            obligation_merge_receipts.push(receipt.clone());
        }
        replay_legality_checks.extend(report.replay_legality_checks.clone());
        replay_barrier_receipts.extend(report.replay_barrier_receipts.clone());
        if let Some(receipt) = &report.replay_merge_receipt {
            replay_merge_receipts.push(receipt.clone());
        }
        replay_divergence_records.extend(report.replay_divergence_records.clone());
        obligation_cache_shards.extend(report.obligation_cache_shards.clone());
        obligation_write_sets.extend(report.obligation_write_sets.clone());
        obligation_collision_reports.extend(report.obligation_collision_reports.clone());
        if let Some(receipt) = &report.obligation_namespace_receipt {
            obligation_namespace_receipts.push(receipt.clone());
        }
    }

    let manifest = if options.dry_run {
        None
    } else {
        prediction.as_ref().and_then(|record| {
            build_execution_manifest_for_prediction(
                record,
                &execution_reports,
                plan,
                &scheduler_policy,
                &execution_scope,
                &execution_result.lane_records,
                &execution_result.schedule_hash,
                &execution_result.ordering_receipt,
                &coherence_receipts,
                &obligation_plans,
                &obligation_lanes,
                &obligation_ordering_receipts,
                &obligation_merge_receipts,
                &replay_legality_checks,
                &replay_barrier_receipts,
                &replay_merge_receipts,
                &replay_divergence_records,
                &obligation_cache_shards,
                &obligation_write_sets,
                &obligation_collision_reports,
                &obligation_namespace_receipts,
            )
            .ok()
        })
    };
    if let Some(manifest) = &manifest {
        persist_execution_manifest(manifest)?;
        manifest_ids.push(manifest.id.clone());
    }

    let reconciliation = if options.dry_run {
        None
    } else if let (Some(prediction), Some(report)) =
        (prediction.as_ref(), execution_reports.first())
    {
        let assessment = assess_prediction(prediction, report)?;
        let reconciliation = ReconciliationRecord {
            id: format!("REC_{}", stable_id(&(plan.id.clone() + &assessment.id))),
            prediction_id: prediction.prediction.id.clone(),
            plan_id: plan.id.clone(),
            execution_id: format!("RUN_{}", stable_id(&plan.id)),
            assessment: assessment.clone(),
            notes: assessment.notes.clone(),
        };
        persist_reconciliation(&reconciliation)?;
        Some(reconciliation)
    } else {
        None
    };

    for step in &plan.steps {
        let (status, produced, reasons) = if options.dry_run {
            (
                ScheduledStepStatus::DryRun,
                Vec::new(),
                vec!["dry-run requested".into()],
            )
        } else {
            match step.kind {
                RecomputationStepKind::Reuse => (
                    ScheduledStepStatus::Reused,
                    vec![step.target_id.clone()],
                    vec!["semantic identity preserved; scheduler reused baseline artifact".into()],
                ),
                RecomputationStepKind::Replay
                    if scheduler_policy.replay_allowed && !needs_rerun =>
                {
                    (
                        ScheduledStepStatus::Replayed,
                        vec![step.target_id.clone()],
                        vec!["replay/cache policy allowed reuse of cached report".into()],
                    )
                }
                RecomputationStepKind::RebuildManifestLock => (
                    ScheduledStepStatus::Executed,
                    manifest
                        .as_ref()
                        .map(|item| vec![item.id.clone()])
                        .unwrap_or_default(),
                    vec!["manifest rebuilt to reflect executed plan and reconciliation".into()],
                ),
                RecomputationStepKind::ReExportReport => (
                    ScheduledStepStatus::Executed,
                    execution_reports.iter().map(report_id).collect(),
                    vec!["report provenance refreshed after certification lanes completed".into()],
                ),
                _ => (
                    ScheduledStepStatus::Executed,
                    execution_reports.iter().map(report_id).collect(),
                    vec!["scheduler executed plan step against bundle-local target".into()],
                ),
            }
        };
        let receipt = SchedulerDecisionReceipt {
            id: format!("SDR_{}", stable_id(&(plan.id.clone() + &step.id))),
            step_id: step.id.clone(),
            policy_resolution_id: prediction
                .as_ref()
                .and_then(|item| prediction_policy_resolution(item))
                .map(|item| item.id.clone()),
            decision: format!("{status:?}"),
            reasons,
        };
        outcomes.push(StepExecutionOutcome {
            step_id: step.id.clone(),
            kind: step.kind.clone(),
            status,
            produced_artifact_ids: produced,
            receipt,
        });
    }

    for mut report in execution_reports {
        if let Some(envelope) = &mut report.execution_envelope {
            envelope.executed_plan_id = Some(plan.id.clone());
            envelope.reconciliation_id = reconciliation.as_ref().map(|item| item.id.clone());
            if let Some(manifest) = &manifest {
                envelope.manifest_id = Some(manifest.id.clone());
            }
        }
        if let Some(reconciliation) = &reconciliation {
            report.reconciliation_summary = reconciliation.notes.clone();
        }
        resulting_report_ids.push(report_id(&report));
        resulting_reports.push(report);
    }

    let record = PlanExecutionRecord {
        id: format!(
            "RUN_{}",
            stable_id(&(plan.id.clone() + &scheduler_policy.id))
        ),
        plan_id: plan.id.clone(),
        prediction_id: plan.prediction_id.clone(),
        scheduler_policy,
        root_resolution,
        execution_scope: Some(execution_scope),
        lane_records: execution_result.lane_records,
        schedule_hash: Some(execution_result.schedule_hash),
        coherence_receipts,
        ordering_receipt: Some(execution_result.ordering_receipt),
        obligation_plans,
        obligation_lanes,
        obligation_ordering_receipts,
        obligation_merge_receipts,
        replay_legality_checks,
        replay_barrier_receipts,
        replay_merge_receipts,
        replay_divergence_records,
        obligation_cache_shards,
        obligation_write_sets,
        obligation_collision_reports,
        obligation_namespace_receipts,
        outcomes,
        reused_artifacts,
        rerun_artifacts,
        resulting_report_ids,
        resulting_reports,
        manifest_ids,
        lock_ids,
        explanation: execution_result
            .explanation
            .into_iter()
            .chain(plan.explanation.clone())
            .collect(),
    };
    persist_execution(&record)?;
    Ok(record)
}

pub fn reconcile_prediction_to_report(
    prediction: &PredictionRecord,
    actual_report: &CertificationReport,
) -> Result<ReconciliationRecord> {
    let assessment = assess_prediction(prediction, actual_report)?;
    let record = ReconciliationRecord {
        id: format!(
            "REC_{}",
            stable_id(&(prediction.prediction.id.clone() + &assessment.id))
        ),
        prediction_id: prediction.prediction.id.clone(),
        plan_id: "direct".into(),
        execution_id: actual_report
            .execution_envelope
            .as_ref()
            .and_then(|item| item.executed_plan_id.clone())
            .unwrap_or_else(|| report_id(actual_report)),
        assessment: assessment.clone(),
        notes: assessment.notes.clone(),
    };
    persist_reconciliation(&record)?;
    Ok(record)
}

pub fn compare_executions(
    left: &PlanExecutionRecord,
    right: &PlanExecutionRecord,
) -> Result<SemanticDiff> {
    let mut summary = Vec::new();
    let mut changed_routes = Vec::new();
    let mut changed_policies = Vec::new();
    if left.scheduler_policy != right.scheduler_policy {
        summary.push("scheduler policy changed".into());
        changed_policies.push(left.scheduler_policy.id.clone());
        changed_policies.push(right.scheduler_policy.id.clone());
    }
    if left.obligation_lanes != right.obligation_lanes
        || left.obligation_plans != right.obligation_plans
    {
        summary.push("obligation execution schedule changed".into());
    }
    if left.resulting_report_ids != right.resulting_report_ids {
        summary.push("resulting report set changed".into());
    }
    for (l, r) in left
        .resulting_reports
        .iter()
        .zip(right.resulting_reports.iter())
    {
        let nested = semantic_diff_between_reports(l, r);
        if nested.class != SemanticDiffClass::NoSemanticChange {
            summary.extend(nested.summary.clone());
            changed_routes.extend(nested.changed_route_ids.clone());
            changed_policies.extend(nested.changed_policy_ids.clone());
        }
    }
    let diff = SemanticDiff {
        id: format!("DIF_{}", stable_id(&(left.id.clone() + &right.id))),
        left_id: left.id.clone(),
        right_id: right.id.clone(),
        class: classify(
            !changed_policies.is_empty(),
            !changed_routes.is_empty(),
            false,
            false,
            false,
        ),
        summary: if summary.is_empty() {
            vec!["no semantic execution drift".into()]
        } else {
            summary
        },
        theorem_drifts: Vec::new(),
        changed_policy_ids: changed_policies,
        changed_route_ids: changed_routes,
        changed_dependency_ids: Vec::new(),
    };
    persist_diff(&diff)?;
    Ok(diff)
}

fn semantic_diff_between_reports(
    left: &CertificationReport,
    right: &CertificationReport,
) -> SemanticDiff {
    let mut summary = Vec::new();
    let mut changed_policies = Vec::new();
    let mut changed_routes = Vec::new();
    let mut theorem_drifts = Vec::new();
    let mut obligations_changed = false;
    let mut replay_changed = false;

    if left.policy_resolution != right.policy_resolution {
        summary.push("policy resolution changed".into());
        changed_policies = symmetric_diff(
            &left
                .policy_resolution
                .as_ref()
                .map(|item| item.applied_policy_ids.clone())
                .unwrap_or_default(),
            &right
                .policy_resolution
                .as_ref()
                .map(|item| item.applied_policy_ids.clone())
                .unwrap_or_default(),
        );
    }
    if left.selected_atlas_cell != right.selected_atlas_cell
        || left.selected_path != right.selected_path
    {
        summary.push("route winner changed".into());
        if let Some(left_id) = &left.selected_atlas_cell {
            changed_routes.push(left_id.clone());
        }
        if let Some(right_id) = &right.selected_atlas_cell {
            changed_routes.push(right_id.clone());
        }
    }
    let obligation_drifts = diff_obligations(&left.obligations, &right.obligations);
    if !obligation_drifts.is_empty() {
        obligations_changed = true;
        summary.push("obligation outcomes changed".into());
    }
    if left
        .execution_envelope
        .as_ref()
        .map(|item| &item.replay_status)
        != right
            .execution_envelope
            .as_ref()
            .map(|item| &item.replay_status)
    {
        replay_changed = true;
        summary.push("replay/cache behavior changed".into());
    }
    if left.verdict != right.verdict {
        summary.push(format!(
            "certification verdict changed from {:?} to {:?}",
            left.verdict, right.verdict
        ));
    }
    theorem_drifts.push(TheoremDrift {
        theorem_id: left.theorem_id.clone(),
        verdict_before: Some(left.verdict.clone()),
        verdict_after: Some(right.verdict.clone()),
        route_before: left.selected_atlas_cell.clone(),
        route_after: right.selected_atlas_cell.clone(),
        obligation_drifts,
        summary: summary.clone(),
    });
    SemanticDiff {
        id: format!("DIF_{}", stable_id(&(report_id(left) + &report_id(right)))),
        left_id: report_id(left),
        right_id: report_id(right),
        class: classify(
            !changed_policies.is_empty(),
            !changed_routes.is_empty(),
            obligations_changed,
            replay_changed,
            false,
        ),
        summary: if summary.is_empty() {
            vec!["no semantic report drift".into()]
        } else {
            summary
        },
        theorem_drifts,
        changed_policy_ids: changed_policies,
        changed_route_ids: changed_routes,
        changed_dependency_ids: Vec::new(),
    }
}

fn diff_obligations(left: &[ObligationStatus], right: &[ObligationStatus]) -> Vec<ObligationDrift> {
    let left_map = left
        .iter()
        .map(|item| (item.obligation_id.clone(), item))
        .collect::<HashMap<_, _>>();
    let right_map = right
        .iter()
        .map(|item| (item.obligation_id.clone(), item))
        .collect::<HashMap<_, _>>();
    let mut ids = left_map.keys().cloned().collect::<BTreeSet<_>>();
    ids.extend(right_map.keys().cloned());
    ids.into_iter()
        .filter_map(|id| {
            let before = left_map.get(&id).copied();
            let after = right_map.get(&id).copied();
            if before == after {
                return None;
            }
            Some(ObligationDrift {
                obligation_id: id,
                verdict_before: before.map(|item| item.verdict.clone()),
                verdict_after: after.map(|item| item.verdict.clone()),
                mode_before: before.map(|item| item.evaluation_mode.clone()),
                mode_after: after.map(|item| item.evaluation_mode.clone()),
                meaning: format!(
                    "obligation drift from {:?}/{:?}/receipts={} to {:?}/{:?}/receipts={}",
                    before.map(|item| item.verdict.clone()),
                    before.map(|item| item.evaluation_mode.clone()),
                    before
                        .map(|item| receipt_count(&item.receipts))
                        .unwrap_or_default(),
                    after.map(|item| item.verdict.clone()),
                    after.map(|item| item.evaluation_mode.clone()),
                    after
                        .map(|item| receipt_count(&item.receipts))
                        .unwrap_or_default()
                ),
            })
        })
        .collect()
}

fn classify(
    policy_changed: bool,
    route_changed: bool,
    obligation_changed: bool,
    replay_changed: bool,
    dependency_changed: bool,
) -> SemanticDiffClass {
    let count = usize::from(policy_changed)
        + usize::from(route_changed)
        + usize::from(obligation_changed)
        + usize::from(replay_changed)
        + usize::from(dependency_changed);
    match (
        count,
        policy_changed,
        route_changed,
        obligation_changed,
        replay_changed,
        dependency_changed,
    ) {
        (0, ..) => SemanticDiffClass::NoSemanticChange,
        (1, true, false, false, false, false) => SemanticDiffClass::PolicyOnly,
        (1, false, true, false, false, false) => SemanticDiffClass::RouteOnly,
        (1, false, false, true, false, false) => SemanticDiffClass::ObligationOnly,
        (1, false, false, false, true, false) => SemanticDiffClass::ReplayOnly,
        (1, false, false, false, false, true) => SemanticDiffClass::BundleDependencyOnly,
        _ => SemanticDiffClass::Mixed,
    }
}

fn classify_evaluator_prediction(
    baseline_report: &CertificationReport,
    resolution: &PolicyResolution,
    reasons: &mut Vec<String>,
) -> PredictionClass {
    if resolution.evaluator.unsupported_mode == l64_core::UnsupportedHandlingMode::StrictFail
        && baseline_report.obligations.iter().any(|item| {
            matches!(
                item.evaluation_mode,
                l64_core::ObligationEvaluationMode::Unsupported
                    | l64_core::ObligationEvaluationMode::RecomputedApproximate
                    | l64_core::ObligationEvaluationMode::RecomputedPartial
            )
        })
    {
        reasons.push(
            "stricter evaluator policy can flip unsupported or approximate obligations".into(),
        );
        PredictionClass::ObligationOutcomeChangeLikely
    } else {
        reasons.push(
            "evaluator provenance changed without an obvious strictness-triggered flip".into(),
        );
        PredictionClass::ReportHashChangeLikely
    }
}

fn map_diff_to_prediction(class: &SemanticDiffClass) -> PredictionClass {
    match class {
        SemanticDiffClass::NoSemanticChange => PredictionClass::NoImpactPredicted,
        SemanticDiffClass::PolicyOnly => PredictionClass::ReportHashChangeLikely,
        SemanticDiffClass::RouteOnly => PredictionClass::RouteWinnerChangeLikely,
        SemanticDiffClass::ObligationOnly => PredictionClass::ObligationOutcomeChangeLikely,
        SemanticDiffClass::ReplayOnly => PredictionClass::ReplayOnlyImpact,
        SemanticDiffClass::SurfaceOnly => PredictionClass::ReportHashChangeLikely,
        SemanticDiffClass::BundleDependencyOnly => PredictionClass::RouteWinnerChangeLikely,
        SemanticDiffClass::Mixed => PredictionClass::CertificationVerdictChangeLikely,
        SemanticDiffClass::Inconclusive => PredictionClass::InconclusivePrediction,
    }
}

fn severity(class: &PredictionClass) -> usize {
    match class {
        PredictionClass::NoImpactPredicted => 0,
        PredictionClass::ReplayOnlyImpact => 1,
        PredictionClass::ReportHashChangeLikely => 2,
        PredictionClass::RouteWinnerChangeLikely => 3,
        PredictionClass::ObligationOutcomeChangeLikely => 4,
        PredictionClass::CertificationVerdictChangeLikely => 5,
        PredictionClass::BundleConflictRisk => 6,
        PredictionClass::InconclusivePrediction => 7,
    }
}

fn persist_diff(diff: &SemanticDiff) -> Result<()> {
    persist_store_payload(
        diffs_root()?,
        &diff.id,
        LocusPacketKind::FrontierEnvelope,
        LocusOpcode::Coverage,
        "semantic_diff.v1",
        diff,
        LocusCapabilityMask {
            has_frontier: true,
            ..Default::default()
        },
    )?;
    Ok(())
}

fn persist_execution(record: &PlanExecutionRecord) -> Result<()> {
    persist_store_payload(
        executions_root()?,
        &record.id,
        LocusPacketKind::FrontierEnvelope,
        LocusOpcode::ReceiptTable,
        "plan_execution.v1",
        record,
        LocusCapabilityMask {
            has_frontier: true,
            ..Default::default()
        },
    )?;
    Ok(())
}

fn persist_reconciliation(record: &ReconciliationRecord) -> Result<()> {
    persist_store_payload(
        reconciliations_root()?,
        &record.id,
        LocusPacketKind::FrontierEnvelope,
        LocusOpcode::Coverage,
        "reconciliation_record.v1",
        record,
        LocusCapabilityMask {
            has_frontier: true,
            ..Default::default()
        },
    )?;
    Ok(())
}

fn persist_execution_manifest(manifest: &ExecutionManifest) -> Result<()> {
    let root = PathBuf::from(
        resolve_cache_root()
            .map_err(anyhow::Error::msg)?
            .absolute_path,
    )
    .join("manifests");
    fs::create_dir_all(&root)?;
    persist_store_payload(
        root,
        &manifest.id,
        LocusPacketKind::CanonicalTransfer,
        LocusOpcode::CanonicalPayload,
        "execution_manifest.v1",
        manifest,
        LocusCapabilityMask::default(),
    )?;
    Ok(())
}

fn persist_prediction(record: &PredictionRecord) -> Result<()> {
    persist_store_payload(
        predictions_root()?,
        &record.prediction.id,
        LocusPacketKind::FrontierEnvelope,
        LocusOpcode::Frontier,
        "prediction_record.v1",
        record,
        LocusCapabilityMask {
            has_frontier: true,
            ..Default::default()
        },
    )?;
    Ok(())
}

fn route_trace(
    report: &CertificationReport,
    explanation: &RouteExplanation,
) -> l64_core::RouteTraceEvent {
    l64_core::RouteTraceEvent {
        id: format!("RTE_{}", report_id(report)),
        theorem_id: report.theorem_id.clone(),
        winner_id: explanation.winner_atlas_cell_id.clone(),
        optimizer_backend: explanation.optimizer_backend.clone(),
        axes: explanation.axes_used.clone(),
        explanation: explanation.explanation.clone(),
    }
}

fn mk_choice(
    report: &CertificationReport,
    explanation: &RouteExplanation,
) -> l64_core::ChoiceRecord {
    l64_core::ChoiceRecord {
        id: format!("CHC_{}", report_id(report)),
        choice_kind: "route-winner".into(),
        selected_id: explanation
            .winner_atlas_cell_id
            .clone()
            .unwrap_or_else(|| "none".into()),
        alternatives: explanation.dominated_candidates.clone(),
        reasons: explanation.explanation.clone(),
    }
}

fn mk_obligation_trace(
    report: &CertificationReport,
    obligation: &ObligationStatus,
) -> l64_core::ObligationTraceRecord {
    l64_core::ObligationTraceRecord {
        id: format!("OTR_{}_{}", report_id(report), obligation.obligation_id),
        theorem_id: report.theorem_id.clone(),
        obligation_id: obligation.obligation_id.clone(),
        verdict: obligation.verdict.clone(),
        evaluation_mode: obligation.evaluation_mode.clone(),
        detail: obligation.detail.clone(),
        receipts: obligation.receipts.clone(),
    }
}

fn edge(from: &str, to: &str, relation: &str) -> ObservationEdge {
    ObservationEdge {
        id: format!("OBE_{}", stable_id(&(from.to_string() + to + relation))),
        from: from.to_string(),
        to: to.to_string(),
        relation: relation.to_string(),
    }
}

fn step(
    label: &str,
    kind: RecomputationStepKind,
    target_id: &str,
    reason: &str,
) -> RecomputationStep {
    RecomputationStep {
        id: format!(
            "{}_{:}",
            label,
            stable_id(&(target_id.to_string() + reason))
        ),
        kind,
        target_id: target_id.to_string(),
        reason: reason.to_string(),
    }
}

fn diff_dependencies(
    left: &[l64_core::BundleDependency],
    right: &[l64_core::BundleDependency],
) -> Vec<String> {
    let left_set = left
        .iter()
        .map(|item| item.id.clone())
        .collect::<BTreeSet<_>>();
    let right_set = right
        .iter()
        .map(|item| item.id.clone())
        .collect::<BTreeSet<_>>();
    left_set.symmetric_difference(&right_set).cloned().collect()
}

fn symmetric_diff(left: &[String], right: &[String]) -> Vec<String> {
    let left_set = left.iter().cloned().collect::<BTreeSet<_>>();
    let right_set = right.iter().cloned().collect::<BTreeSet<_>>();
    left_set.symmetric_difference(&right_set).cloned().collect()
}

fn btreemap<const N: usize>(
    items: [(impl Into<String>, impl Into<String>); N],
) -> BTreeMap<String, String> {
    items
        .into_iter()
        .map(|(key, value)| (key.into(), value.into()))
        .collect()
}

fn stable_id(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:x}")
}

fn prediction_policy_resolution(record: &PredictionRecord) -> Option<&PolicyResolution> {
    match &record.target {
        PredictionTarget::ProposedBundle {
            policy_resolution, ..
        } => policy_resolution.as_ref(),
        PredictionTarget::PolicyOverride { .. } => None,
    }
}

fn scheduler_policy_for(
    prediction: Option<&PredictionRecord>,
    options: &ExecutePlanOptions,
) -> SchedulerPolicy {
    let mut scheduler_config = prediction
        .and_then(prediction_policy_resolution)
        .map(|resolution| resolution.scheduler.clone())
        .unwrap_or(l64_core::SchedulerPolicyConfig {
            parallelization: ParallelizationPolicy::Serialize,
            max_workers: 1,
            allow_parallel_replay: false,
            allow_parallel_certification: true,
            allow_parallel_exports: true,
            deterministic_ordering: true,
            allow_parallel_obligations: false,
            max_obligation_workers: 1,
            allow_parallel_obligation_replay: false,
            serialize_canonicalization_sensitive: true,
        });
    if options.force_parallel {
        scheduler_config.parallelization = ParallelizationPolicy::ParallelIndependent;
    }
    if options.force_parallel_obligations {
        scheduler_config.allow_parallel_obligations = true;
        if scheduler_config.max_obligation_workers <= 1 {
            scheduler_config.max_obligation_workers = options.max_workers.unwrap_or(2).max(2);
        }
    }
    if options.force_serialized {
        scheduler_config.parallelization = ParallelizationPolicy::Serialize;
        scheduler_config.allow_parallel_obligations = false;
    }
    if let Some(max_workers) = options.max_workers {
        scheduler_config.max_workers = max_workers.max(1);
    } else if scheduler_config.parallelization == ParallelizationPolicy::ParallelIndependent
        && scheduler_config.max_workers == 1
    {
        scheduler_config.max_workers = thread::available_parallelism()
            .map(|v| v.get())
            .unwrap_or(2)
            .max(2);
    }
    if let Some(max_workers) = options.max_obligation_workers {
        scheduler_config.max_obligation_workers = max_workers.max(1);
    } else if scheduler_config.allow_parallel_obligations
        && scheduler_config.max_obligation_workers == 1
    {
        scheduler_config.max_obligation_workers = scheduler_config.max_workers.max(2);
    }
    if options.strict_determinism {
        scheduler_config.deterministic_ordering = true;
    }
    let (replay_allowed, strict_unsupported, reuse_trust) = prediction
        .and_then(prediction_policy_resolution)
        .map(|resolution| {
            (
                resolution.replay_cache.replay_allowed,
                matches!(
                    resolution.evaluator.unsupported_mode,
                    l64_core::UnsupportedHandlingMode::StrictFail
                ),
                match resolution.replay_cache.trust_class {
                    l64_core::ReplayTrustClass::ExactPolicyOnly => ReuseTrustClass::ExactOnly,
                    l64_core::ReplayTrustClass::AllowSurfaceOnlyChanges => {
                        ReuseTrustClass::SurfaceStable
                    }
                    l64_core::ReplayTrustClass::AllowApproximateReuse => {
                        ReuseTrustClass::ApproximateAllowed
                    }
                },
            )
        })
        .unwrap_or((true, options.strict, ReuseTrustClass::ExactOnly));
    SchedulerPolicy {
        id: format!(
            "SCH_{}",
            stable_id(
                prediction
                    .map(|item| item.prediction.id.as_str())
                    .unwrap_or("default-scheduler")
            )
        ),
        replay_allowed,
        strict_recomputation: options.strict,
        strict_unsupported,
        parallelization: scheduler_config.parallelization,
        max_workers: scheduler_config.max_workers.max(1),
        allow_parallel_replay: scheduler_config.allow_parallel_replay,
        allow_parallel_certification: scheduler_config.allow_parallel_certification,
        allow_parallel_exports: scheduler_config.allow_parallel_exports,
        deterministic_ordering: scheduler_config.deterministic_ordering,
        allow_parallel_obligations: scheduler_config.allow_parallel_obligations,
        max_obligation_workers: scheduler_config.max_obligation_workers,
        allow_parallel_obligation_replay: scheduler_config.allow_parallel_obligation_replay,
        serialize_canonicalization_sensitive: scheduler_config.serialize_canonicalization_sensitive,
        reuse_trust,
        notes: vec!["scheduler derived from resolved replay/evaluator/scheduler policy".into()],
    }
}

fn build_execution_manifest_for_prediction(
    record: &PredictionRecord,
    reports: &[CertificationReport],
    plan: &RecomputationPlan,
    scheduler_policy: &SchedulerPolicy,
    execution_scope: &ExecutionScope,
    lane_records: &[l64_core::LaneExecutionRecord],
    schedule_hash: &ExecutionScheduleHash,
    ordering_receipt: &OrderingReceipt,
    coherence_receipts: &[l64_core::ConcurrencyCoherenceReceipt],
    obligation_plans: &[l64_core::ObligationPlan],
    obligation_lanes: &[l64_core::ObligationLaneRecord],
    obligation_ordering_receipts: &[l64_core::ObligationOrderingReceipt],
    obligation_merge_receipts: &[l64_core::ObligationMergeReceipt],
    replay_legality_checks: &[l64_core::ReplayLegalityCheck],
    replay_barrier_receipts: &[l64_core::ReplayBarrierReceipt],
    replay_merge_receipts: &[l64_core::ReplayMergeReceipt],
    replay_divergence_records: &[l64_core::ReplayDivergenceRecord],
    obligation_cache_shards: &[l64_core::ObligationCacheShard],
    obligation_write_sets: &[l64_core::ObligationWriteSet],
    obligation_collision_reports: &[l64_core::ObligationCollisionReport],
    obligation_namespace_receipts: &[l64_core::ObligationNamespaceReceipt],
) -> Result<ExecutionManifest> {
    let bundle_id = match &record.target {
        PredictionTarget::ProposedBundle { bundle_id, .. } => bundle_id.clone(),
        PredictionTarget::PolicyOverride { .. } => reports
            .first()
            .and_then(|item| item.execution_envelope.as_ref())
            .and_then(|item| item.bundle_id.clone())
            .ok_or_else(|| anyhow!("policy override execution needs a bundle-backed report"))?,
    };
    let world = load_bundle_world(&bundle_id)?;
    let resolution = match &record.target {
        PredictionTarget::ProposedBundle {
            policy_resolution, ..
        } => policy_resolution.clone().unwrap_or(
            resolve_policy_graph(
                &world.overlay,
                Some(&world.manifest.id),
                world
                    .overlay
                    .local
                    .theorem_specs
                    .first()
                    .map(|item| item.id.as_str()),
                world
                    .overlay
                    .local
                    .campaigns
                    .first()
                    .map(|item| item.id.as_str()),
                world
                    .overlay
                    .local
                    .target_profiles
                    .first()
                    .map(|item| item.id.as_str()),
                scheduler_policy.strict_recomputation,
                reports
                    .first()
                    .and_then(|item| item.route_explanation.as_ref())
                    .map(|item| item.optimizer_policy.clone())
                    .unwrap_or(l64_core::OptimizerPolicy::Conservative),
            )
            .map_err(anyhow::Error::msg)?
            .resolution,
        ),
        PredictionTarget::PolicyOverride { .. } => {
            resolve_policy_graph(
                &world.overlay,
                Some(&world.manifest.id),
                world
                    .overlay
                    .local
                    .theorem_specs
                    .first()
                    .map(|item| item.id.as_str()),
                world
                    .overlay
                    .local
                    .campaigns
                    .first()
                    .map(|item| item.id.as_str()),
                world
                    .overlay
                    .local
                    .target_profiles
                    .first()
                    .map(|item| item.id.as_str()),
                scheduler_policy.strict_recomputation,
                reports
                    .first()
                    .and_then(|item| item.route_explanation.as_ref())
                    .map(|item| item.optimizer_policy.clone())
                    .unwrap_or(l64_core::OptimizerPolicy::Conservative),
            )
            .map_err(anyhow::Error::msg)?
            .resolution
        }
    };
    let bundle_hash = reports
        .first()
        .and_then(|item| item.execution_envelope.as_ref())
        .map(|item| item.bundle_hash.clone())
        .unwrap_or_else(|| stable_id(&world.manifest.id));
    let mut manifest = l64_policy::build_execution_manifest(
        &world.manifest.id,
        &bundle_hash,
        world.manifest.dependencies.clone(),
        &resolution,
        reports
            .iter()
            .filter_map(|item| item.selected_atlas_cell.clone())
            .collect(),
        vec!["l64-cert-obl-v2".into()],
        vec!["v7-surfaces".into()],
        reports.iter().map(report_id).collect(),
    );
    manifest.executed_plan_hash = Some(stable_id(&serde_json::to_string(plan)?));
    manifest.executed_steps = plan
        .steps
        .iter()
        .map(|step| StepExecutionOutcome {
            step_id: step.id.clone(),
            kind: step.kind.clone(),
            status: ScheduledStepStatus::Executed,
            produced_artifact_ids: Vec::new(),
            receipt: SchedulerDecisionReceipt {
                id: format!("SDR_{}", stable_id(&(manifest.id.clone() + &step.id))),
                step_id: step.id.clone(),
                policy_resolution_id: Some(resolution.id.clone()),
                decision: "manifest-captured".into(),
                reasons: vec![step.reason.clone()],
            },
        })
        .collect();
    manifest.reused_artifacts = plan.reusable_artifacts.clone();
    manifest.rerun_artifacts = plan.invalidated_artifacts.clone();
    manifest.reconciliation_summary = Vec::new();
    manifest.root_resolution = Some(runtime_root_report(&[]).map_err(anyhow::Error::msg)?);
    manifest.scheduler_policy = Some(scheduler_policy.clone());
    manifest.execution_scope = Some(execution_scope.clone());
    manifest.lane_records = lane_records.to_vec();
    manifest.schedule_hash = Some(schedule_hash.clone());
    manifest.coherence_receipts = coherence_receipts.to_vec();
    manifest.ordering_receipt = Some(ordering_receipt.clone());
    manifest.obligation_plans = obligation_plans.to_vec();
    manifest.obligation_lanes = obligation_lanes.to_vec();
    manifest.obligation_ordering_receipts = obligation_ordering_receipts.to_vec();
    manifest.obligation_merge_receipts = obligation_merge_receipts.to_vec();
    manifest.replay_legality_checks = replay_legality_checks.to_vec();
    manifest.replay_barrier_receipts = replay_barrier_receipts.to_vec();
    manifest.replay_merge_receipts = replay_merge_receipts.to_vec();
    manifest.replay_divergence_records = replay_divergence_records.to_vec();
    manifest.obligation_cache_shards = obligation_cache_shards.to_vec();
    manifest.obligation_write_sets = obligation_write_sets.to_vec();
    manifest.obligation_collision_reports = obligation_collision_reports.to_vec();
    manifest.obligation_namespace_receipts = obligation_namespace_receipts.to_vec();
    Ok(manifest)
}

fn run_prediction_target(
    record: &PredictionRecord,
    scheduler_policy: &SchedulerPolicy,
    options: &ExecutePlanOptions,
) -> Result<ExecutedTaskBatch> {
    let (world, options) = match &record.target {
        PredictionTarget::ProposedBundle {
            bundle_id,
            policy_resolution,
            ..
        } => {
            let world = load_bundle_world(bundle_id)?;
            let optimizer = policy_resolution
                .as_ref()
                .map(|item| item.optimizer.optimizer_policy.clone())
                .unwrap_or(l64_core::OptimizerPolicy::Conservative);
            let options = CertificationOptions {
                optimizer_policy: optimizer,
                bundle_hash: stable_id(&serde_json::to_string(&world.manifest)?),
                policy_hash: stable_id(
                    &policy_resolution
                        .as_ref()
                        .map(|item| serde_json::to_string(item).unwrap_or_default())
                        .unwrap_or_default(),
                ),
                bundle_id: Some(world.manifest.id.clone()),
                evaluator_policy: None,
                cache_policy: None,
                no_cache: options.no_cache,
                replay_only: false,
                strict_derived: options.strict,
                strict_policy: scheduler_policy.strict_recomputation,
                force_parallel_obligations: scheduler_policy.allow_parallel_obligations
                    || options.force_parallel_obligations,
                max_obligation_workers: Some(scheduler_policy.max_obligation_workers),
            };
            (world, options)
        }
        PredictionTarget::PolicyOverride {
            policy_id, kind, ..
        } => {
            let baseline = replay_report(&record.prediction.baseline_id).map_err(|err| {
                anyhow!("unable to load baseline report for policy execution: {err}")
            })?;
            let bundle_id = baseline
                .execution_envelope
                .as_ref()
                .and_then(|item| item.bundle_id.clone())
                .ok_or_else(|| anyhow!("baseline report is not bundle-backed"))?;
            let world = load_bundle_world(&bundle_id)?;
            let policy = world.overlay.get_policy_object(policy_id).ok_or_else(|| {
                anyhow!("policy `{policy_id}` not present in overlay or seed registry")
            })?;
            let mut options = CertificationOptions {
                optimizer_policy: baseline
                    .route_explanation
                    .as_ref()
                    .map(|item| item.optimizer_policy.clone())
                    .unwrap_or(l64_core::OptimizerPolicy::Conservative),
                bundle_hash: stable_id(&serde_json::to_string(&world.manifest)?),
                policy_hash: stable_id(&(policy_id.clone() + kind)),
                bundle_id: Some(world.manifest.id.clone()),
                evaluator_policy: None,
                cache_policy: None,
                no_cache: options.no_cache,
                replay_only: false,
                strict_derived: options.strict,
                strict_policy: scheduler_policy.strict_recomputation,
                force_parallel_obligations: scheduler_policy.allow_parallel_obligations
                    || options.force_parallel_obligations,
                max_obligation_workers: Some(scheduler_policy.max_obligation_workers),
            };
            match kind.as_str() {
                "Optimizer" => {
                    if let Some(config) = &policy.optimizer {
                        options.optimizer_policy = config.optimizer_policy.clone();
                    }
                }
                "Evaluator" => options.evaluator_policy = Some(policy.id.clone()),
                "ReplayCache" => options.cache_policy = Some(policy.id.clone()),
                _ => {}
            }
            (world, options)
        }
    };

    let tasks = execution_tasks(&world)?;
    if tasks.is_empty() {
        return Ok(ExecutedTaskBatch::empty());
    }
    let can_parallelize = scheduler_policy.parallelization
        == ParallelizationPolicy::ParallelIndependent
        && scheduler_policy.max_workers > 1
        && tasks.len() > 1
        && scheduler_policy.allow_parallel_certification
        && (!options.replay_only || scheduler_policy.allow_parallel_replay);

    if !can_parallelize {
        let mut reports = Vec::new();
        for task in &tasks {
            reports.push(run_task(&world, &options, task, false)?);
        }
        return Ok(ExecutedTaskBatch {
            reports,
            lane_records: vec![l64_core::LaneExecutionRecord {
                lane_id: "LAN_0".into(),
                step_ids: tasks.iter().map(|task| task.task_id()).collect(),
                task_ids: tasks.iter().map(|task| task.task_id()).collect(),
                serialized_reason: Some(serialization_reason(scheduler_policy, &tasks)),
            }],
            schedule_hash: ExecutionScheduleHash {
                id: format!("SCHH_{}", stable_id(&world.manifest.id)),
                hash: stable_id(&(world.manifest.id.clone() + "|serial")),
            },
            coherence_receipts: Vec::new(),
            ordering_receipt: OrderingReceipt {
                id: format!(
                    "ORD_{}",
                    stable_id(&(world.manifest.id.clone() + "|serial"))
                ),
                ordered_step_ids: tasks.iter().map(|task| task.task_id()).collect(),
                notes: vec![
                    "serialized certification due to scheduler policy or dependency limits".into(),
                ],
            },
            explanation: vec![serialization_reason(scheduler_policy, &tasks)],
        });
    }

    let worker_count = scheduler_policy.max_workers.min(tasks.len()).max(1);
    let lane_records = assign_lanes(&tasks, worker_count);
    let (tx, rx) = mpsc::channel();
    thread::scope(|scope| {
        for (index, task) in tasks.iter().cloned().enumerate() {
            let tx = tx.clone();
            let world = world.clone();
            let mut lane_options = options.clone();
            lane_options.no_cache = true;
            scope.spawn(move || {
                let result = run_task(&world, &lane_options, &task, true);
                let _ = tx.send((index, task, result));
            });
        }
    });
    drop(tx);
    let mut ordered = Vec::new();
    for item in rx {
        ordered.push(item);
    }
    ordered.sort_by_key(|(index, _, _)| *index);
    let mut reports = Vec::new();
    for (_, _, result) in ordered {
        reports.push(result?);
    }
    let schedule_hash = ExecutionScheduleHash {
        id: format!(
            "SCHH_{}",
            stable_id(&(world.manifest.id.clone() + "|parallel"))
        ),
        hash: stable_id(
            &lane_records
                .iter()
                .map(|lane| format!("{}:{}", lane.lane_id, lane.task_ids.join(",")))
                .collect::<Vec<_>>()
                .join("|"),
        ),
    };
    let coherence = l64_core::ConcurrencyCoherenceReceipt {
        id: format!("COH_{}", stable_id(&(world.manifest.id.clone() + &schedule_hash.hash))),
        namespace_id: resolve_cache_root()
            .map_err(anyhow::Error::msg)?
            .namespace
            .unwrap_or_else(|| "default".into()),
        merged_artifact_ids: reports.iter().map(report_id).collect(),
        notes: vec![
            "parallel worker lanes ran with isolated no-cache certification and merged deterministically".into(),
        ],
    };
    Ok(ExecutedTaskBatch {
        reports,
        lane_records,
        schedule_hash,
        coherence_receipts: vec![coherence],
        ordering_receipt: OrderingReceipt {
            id: format!(
                "ORD_{}",
                stable_id(&(world.manifest.id.clone() + "|parallel"))
            ),
            ordered_step_ids: tasks.iter().map(|task| task.task_id()).collect(),
            notes: vec!["deterministic task ordering preserved across parallel lanes".into()],
        },
        explanation: vec!["independent certification tasks executed in parallel lanes".into()],
    })
}

fn report_id(report: &CertificationReport) -> String {
    format!(
        "REPORT_{}_{}",
        report.theorem_id,
        report
            .campaign_id
            .clone()
            .unwrap_or_else(|| "THEOREM".into())
    )
}

#[derive(Debug, Clone)]
enum ExecutionTask {
    Campaign {
        campaign_id: String,
    },
    Theorem {
        theorem_id: String,
        target_profile_id: String,
    },
}

impl ExecutionTask {
    fn task_id(&self) -> String {
        match self {
            Self::Campaign { campaign_id } => campaign_id.clone(),
            Self::Theorem {
                theorem_id,
                target_profile_id,
            } => format!("{theorem_id}@{target_profile_id}"),
        }
    }
}

fn execution_tasks(world: &BundleWorld) -> Result<Vec<ExecutionTask>> {
    if world.overlay.local.campaigns.is_empty() {
        let target = world
            .overlay
            .local
            .target_profiles
            .first()
            .ok_or_else(|| anyhow!("bundle theorem execution requires a target profile"))?;
        let mut tasks = world
            .overlay
            .local
            .theorem_specs
            .iter()
            .map(|item| ExecutionTask::Theorem {
                theorem_id: item.id.clone(),
                target_profile_id: target.id.clone(),
            })
            .collect::<Vec<_>>();
        tasks.sort_by_key(|task| task.task_id());
        Ok(tasks)
    } else {
        let mut tasks = world
            .overlay
            .local
            .campaigns
            .iter()
            .map(|item| ExecutionTask::Campaign {
                campaign_id: item.id.clone(),
            })
            .collect::<Vec<_>>();
        tasks.sort_by_key(|task| task.task_id());
        Ok(tasks)
    }
}

fn run_task(
    world: &BundleWorld,
    options: &CertificationOptions,
    task: &ExecutionTask,
    parallel_lane: bool,
) -> Result<CertificationReport> {
    let atlas = CompiledAtlas::compile(&world.overlay).map_err(anyhow::Error::msg)?;
    let mut report = match task {
        ExecutionTask::Campaign { campaign_id } => {
            certify_derived_campaign_with_options(&world.overlay, &atlas, campaign_id, options)
                .map_err(anyhow::Error::msg)?
        }
        ExecutionTask::Theorem {
            theorem_id,
            target_profile_id,
        } => certify_derived_theorem_with_options(
            &world.overlay,
            &atlas,
            theorem_id,
            target_profile_id,
            None,
            options,
        )
        .map_err(anyhow::Error::msg)?,
    };
    if parallel_lane {
        report
            .diagnostics
            .push("certification computed in parallel lane with isolated cache writes".into());
    }
    Ok(report)
}

fn serialization_reason(scheduler_policy: &SchedulerPolicy, tasks: &[ExecutionTask]) -> String {
    if scheduler_policy.parallelization == ParallelizationPolicy::Serialize {
        "scheduler policy requires serialized execution".into()
    } else if scheduler_policy.max_workers <= 1 {
        "max worker policy reduced execution to a single lane".into()
    } else if tasks.len() <= 1 {
        "only one certification task is available".into()
    } else if !scheduler_policy.allow_parallel_certification {
        "scheduler policy forbids parallel certification".into()
    } else if !scheduler_policy.allow_parallel_replay {
        "replay or strict certification settings block parallel reuse".into()
    } else {
        "lane barriers forced serialization".into()
    }
}

fn assign_lanes(
    tasks: &[ExecutionTask],
    worker_count: usize,
) -> Vec<l64_core::LaneExecutionRecord> {
    let mut lanes = (0..worker_count)
        .map(|index| l64_core::LaneExecutionRecord {
            lane_id: format!("LAN_{index}"),
            step_ids: Vec::new(),
            task_ids: Vec::new(),
            serialized_reason: None,
        })
        .collect::<Vec<_>>();
    for (index, task) in tasks.iter().enumerate() {
        let lane_index = index % worker_count;
        lanes[lane_index].task_ids.push(task.task_id());
        lanes[lane_index].step_ids.push(task.task_id());
    }
    lanes
}

#[cfg(test)]
mod tests {
    use super::*;
    use l64_core::{
        CertificationCandidate, CertificationVerdict, DeterministicExecutionEnvelope,
        EvaluatorPolicyConfig, EvidencePreference, OptimizerBackend, OptimizerPolicy,
        OptimizerPolicyConfig, PolicyResolution, PolicyScope, PolicyTrace, PolicyVerdict,
        ReplayCachePolicyConfig, ReplayTrustClass, ReportPolicyConfig, RouteExplanation,
        RouteScoreVector, UnsupportedHandlingMode,
    };

    fn sample_report(id: &str, winner: &str) -> CertificationReport {
        CertificationReport {
            theorem_id: id.into(),
            campaign_id: Some(format!("CPG_{id}")),
            target_profile_id: "TGT".into(),
            verdict: CertificationVerdict::Benchmarked,
            selected_atlas_cell: Some(winner.into()),
            selected_path: vec!["B1".into()],
            route_class_id: None,
            certificate_id: None,
            candidates: vec![CertificationCandidate {
                atlas_cell_id: winner.into(),
                path: vec!["B1".into()],
                loss_count: 0,
                proof_shapes: vec!["PS".into()],
                route_class_id: None,
                score: vec![0],
                route_score: Some(RouteScoreVector {
                    lawfulness: 0,
                    identity_preservation: 0,
                    loss_compliance: 0,
                    rollback_viability: 0,
                    proof_shape_satisfiability: 0,
                    bundle_resolution: 0,
                    surface_transition_penalty: 0,
                    execution_cost: 0,
                    derived_obligation_depth: 0,
                    maturity_confidence: 0,
                    symbolic_fidelity: 0,
                    receipt_completeness: 0,
                }),
            }],
            obligations: vec![],
            reasons: vec![],
            diagnostics: vec![],
            deficiencies: vec![],
            adequacy_records: vec![],
            checker_receipts: vec![],
            burden_pack_ids: vec![],
            claim_packet_ids: vec![],
            evidence_contract_ids: vec![],
            benchmark_receipt_ids: vec![],
            challenge_receipt_ids: vec![],
            reproducibility_packet_ids: vec![],
            promotion_artifact_ids: vec![],
            reused_artifact_ids: vec![],
            default_selected_artifact_ids: vec![],
            payoff_receipt_ids: vec![],
            policy_resolution: Some(PolicyResolution {
                id: "MPR".into(),
                scope: PolicyScope::Global,
                applied_policy_ids: vec!["MOP_OPT".into()],
                conflicts: vec![],
                trace: PolicyTrace {
                    id: "MPT".into(),
                    steps: vec!["resolved".into()],
                },
                optimizer: OptimizerPolicyConfig {
                    optimizer_policy: OptimizerPolicy::Conservative,
                    backend: OptimizerBackend::Lexicographic,
                    active_axes: vec![l64_core::OptimizationAxis::Lawfulness],
                    route_explanation_verbosity: "standard".into(),
                    symbolic_fidelity_preferred: false,
                    tie_break_rules: vec![],
                },
                evaluator: EvaluatorPolicyConfig {
                    evidence_preference: EvidencePreference::RecomputeIfSupported,
                    allow_approximation: true,
                    unsupported_mode: UnsupportedHandlingMode::Permit,
                    require_symbolic_fidelity_route: false,
                    prefer_comp_replay: true,
                },
                replay_cache: ReplayCachePolicyConfig {
                    replay_allowed: true,
                    exact_policy_match_required: true,
                    survive_surface_only_changes: false,
                    reuse_approximate_results: true,
                    optimizer_change_invalidates: true,
                    surface_pack_change_invalidates: true,
                    trust_class: ReplayTrustClass::ExactPolicyOnly,
                },
                report: ReportPolicyConfig {
                    export_surfaces: vec![l64_core::SurfaceKind::Qc0],
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
                verdict: PolicyVerdict::Applied,
            }),
            route_explanation: Some(RouteExplanation {
                optimizer_policy: OptimizerPolicy::Conservative,
                optimizer_backend: OptimizerBackend::Lexicographic,
                policy_resolution_id: Some("MPR".into()),
                winner_atlas_cell_id: Some(winner.into()),
                winner_score: None,
                dominated_candidates: vec!["A2".into()],
                rejected_candidates: vec![],
                axes_used: vec![l64_core::OptimizationAxis::Lawfulness],
                explanation: vec!["test route".into()],
            }),
            execution_envelope: Some(DeterministicExecutionEnvelope {
                bundle_hash: "bundle".into(),
                bundle_id: Some("BND_TEST".into()),
                policy_hash: "policy".into(),
                policy_resolution_id: Some("MPR".into()),
                manifest_id: Some("EXM".into()),
                lock_id: Some("BLK".into()),
                route_winner_hash: "winner".into(),
                obligation_replay_keys: vec![],
                report_hash: "report".into(),
                replay_status: ReplayStatus::Fresh,
                executed_plan_id: None,
                reconciliation_id: None,
            }),
            reconciliation_summary: Vec::new(),
            obligation_plan: None,
            obligation_lanes: Vec::new(),
            obligation_ordering_receipt: None,
            obligation_merge_receipt: None,
            replay_legality_checks: Vec::new(),
            replay_barrier_receipts: Vec::new(),
            replay_merge_receipt: None,
            replay_divergence_records: Vec::new(),
            obligation_cache_shards: Vec::new(),
            reuse_legality_receipts: Vec::new(),
            reuse_decision_receipts: Vec::new(),
            residual_verification_receipts: Vec::new(),
            obligation_write_sets: Vec::new(),
            obligation_collision_reports: Vec::new(),
            obligation_namespace_receipt: None,
        }
    }

    #[test]
    fn report_diff_detects_route_change() {
        let left = sample_report("THS_A", "A1");
        let right = sample_report("THS_A", "A2");
        let diff = compare_reports(&left, &right).unwrap();
        assert_eq!(diff.class, SemanticDiffClass::RouteOnly);
    }

    #[test]
    fn observation_artifact_is_populated() {
        let report = sample_report("THS_A", "A1");
        let observed = observe_report(&report, None, None).unwrap();
        assert!(!observed.artifact.graph.nodes.is_empty());
        assert!(!observed.artifact.record.events.is_empty());
    }
}
