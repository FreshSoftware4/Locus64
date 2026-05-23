use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand, ValueEnum};
use l64_bundle::{BundleWorld, import_bundle_file, load_bundle_world};
use l64_cert::{
    CertificationOptions, certify_derived_campaign_with_options,
    certify_derived_theorem_with_options, encode_locus_packet_for_report, replay_report,
};
use l64_command::{AdminSurfaceArg as SurfaceArg, BundlePolicyArg, OptimizerPolicyArg};
use l64_core::{
    ArtifactContract, ArtifactKind, ArtifactLocator, BundleLock, CapabilityReadiness,
    CertificationReport, CommandContract, LockDiff, LockReceipt, MechanizationPolicyObject,
    NamespaceScope, OptimizerPolicy, QaDocument, QaEntry, RegistryLookup, locate_artifact,
    runtime_root_report,
};
use l64_observe::{
    ExecutePlanOptions, assess_prediction, compare_executions, compare_locks, compare_manifests,
    compare_report_manifest, compare_reports, drift_report, execute_plan, explain_drift, load_diff,
    load_execution, load_plan, load_prediction, observe_report, plan_recompute_from_diff,
    plan_recompute_from_prediction, predict_from_bundle_change, predict_from_policy_override,
    reconcile_prediction_to_report,
};
use l64_policy::{build_execution_manifest, build_replay_lock_manifest, resolve_policy_graph};
use l64_registry::SeedRegistry;
use l64_surfaces::{
    default_policy_for, document_for_registry_id, export_document, load_bundle_lock,
    load_execution_manifest, load_report_document_with_registry, manifest_cache_root,
    persist_bundle_lock, persist_execution_manifest, report_cache_path, report_cache_root,
    report_id,
};
use std::{fs, path::Path};

#[derive(Debug, Clone, ValueEnum)]
enum CompareKindArg {
    Report,
    Manifest,
    Lock,
}

#[derive(Debug, Parser)]
#[command(name = "l64-admin")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    LockBundle {
        file: String,
        #[arg(long = "as", value_enum)]
        as_kind: Option<SurfaceArg>,
        #[arg(long, value_enum, default_value = "reject")]
        conflict_policy: BundlePolicyArg,
        #[arg(long, value_enum, default_value = "conservative")]
        optimizer_policy: OptimizerPolicyArg,
        #[arg(long)]
        evaluator_policy: Option<String>,
        #[arg(long)]
        cache_policy: Option<String>,
        #[arg(long, default_value_t = false)]
        strict_policy: bool,
    },
    DumpExecutionManifest {
        id: String,
    },
    DumpRuntimeRoots,
    ReplayWithLock {
        id: String,
        #[arg(long, default_value_t = false)]
        parallel_obligations: bool,
        #[arg(long)]
        max_obligation_workers: Option<usize>,
    },
    ObserveRun {
        #[arg(long)]
        report: Option<String>,
        #[arg(long)]
        lock: Option<String>,
    },
    CompareLocks {
        left: String,
        right: String,
    },
    CompareReports {
        left: String,
        right: String,
    },
    CompareManifests {
        left: String,
        right: String,
    },
    CompareReportManifest {
        report_id: String,
        manifest_id: String,
    },
    PredictImpact {
        #[arg(long)]
        report: Option<String>,
        #[arg(long)]
        lock: Option<String>,
        #[arg(long)]
        bundle_file: Option<String>,
        #[arg(long)]
        policy: Option<String>,
        #[arg(long = "as", value_enum)]
        as_kind: Option<SurfaceArg>,
        #[arg(long, value_enum, default_value = "reject")]
        conflict_policy: BundlePolicyArg,
    },
    PlanRecompute {
        #[arg(long)]
        prediction: Option<String>,
        #[arg(long)]
        diff: Option<String>,
    },
    ExecutePlan {
        #[arg(long)]
        prediction: Option<String>,
        #[arg(long)]
        plan: Option<String>,
        #[arg(long, default_value_t = false)]
        dry_run: bool,
        #[arg(long, default_value_t = false)]
        strict: bool,
        #[arg(long, default_value_t = false)]
        no_cache: bool,
        #[arg(long, default_value_t = false)]
        parallel: bool,
        #[arg(long, default_value_t = false)]
        parallel_obligations: bool,
        #[arg(long, default_value_t = false)]
        serialized: bool,
        #[arg(long)]
        max_workers: Option<usize>,
        #[arg(long)]
        max_obligation_workers: Option<usize>,
        #[arg(long, default_value_t = false)]
        strict_determinism: bool,
    },
    ExplainExecution {
        id: String,
    },
    DumpExecutionDag {
        id: String,
    },
    DumpLanePlan {
        id: String,
    },
    ExplainObligationPlan {
        id: String,
    },
    DumpObligationDag {
        id: String,
    },
    DumpObligationLanes {
        id: String,
    },
    CompareObligationExecutions {
        left: String,
        right: String,
    },
    CompareSchedules {
        left: String,
        right: String,
    },
    ExplainPlan {
        id: String,
    },
    ExplainDrift {
        left: String,
        right: String,
        #[arg(long, value_enum, default_value = "report")]
        kind: CompareKindArg,
    },
    CompareExecutions {
        left: String,
        right: String,
    },
    AssessPrediction {
        prediction_id: String,
        actual_report_id: String,
    },
    ReconcileRun {
        #[arg(long)]
        prediction: String,
        #[arg(long = "actual-report")]
        actual_report: String,
    },
    DumpPolicyGraph {
        #[arg(long)]
        bundle: Option<String>,
    },
    ExplainPolicyResolution {
        #[arg(long)]
        bundle: Option<String>,
        #[arg(long)]
        policy: Option<String>,
    },
    ExportArtifact {
        #[arg(long)]
        id: String,
        #[arg(long = "to", value_enum)]
        to_kind: SurfaceArg,
    },
    ResolveArtifactPath {
        id: String,
    },
    ClassifyArtifact {
        id: String,
    },
    CheckCommandInput {
        #[arg(long)]
        command: String,
        #[arg(long)]
        id: String,
    },
    DumpCommandContracts,
    DumpArtifactContracts,
    DumpCapabilityReadiness,
    DumpCacheNamespaces,
}

fn main() -> Result<()> {
    std::thread::Builder::new()
        .name("l64-admin-main".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(real_main)
        .map_err(|err| anyhow!("failed to start l64-admin main thread: {err}"))?
        .join()
        .map_err(|_| anyhow!("l64-admin main thread panicked"))?
}

fn real_main() -> Result<()> {
    let cli = Cli::parse();
    let registry = SeedRegistry::load().context("failed to load seed registry")?;
    match cli.command {
        Command::LockBundle {
            file,
            as_kind,
            conflict_policy,
            optimizer_policy,
            evaluator_policy,
            cache_policy,
            strict_policy,
        } => {
            let world = import_bundle_file(
                Path::new(&file),
                as_kind.map(Into::into),
                conflict_policy.into(),
                None,
            )?;
            let options = build_cert_options(
                optimizer_policy.into(),
                &file,
                &world.manifest.id,
                evaluator_policy,
                cache_policy,
                strict_policy,
            )?;
            let resolved = resolve_for_world(&world, &options)?;
            let reports = certify_bundle_world(&world, &options)?;
            let route_winner_ids = reports
                .iter()
                .filter_map(|report| report.selected_atlas_cell.clone())
                .collect::<Vec<_>>();
            let report_ids = reports.iter().map(report_id).collect::<Vec<_>>();
            let manifest = build_execution_manifest(
                &world.manifest.id,
                &options.bundle_hash,
                world.manifest.dependencies.clone(),
                &resolved.resolution,
                route_winner_ids,
                vec!["l64-cert-obl-v7".into()],
                vec!["v7-surfaces".into()],
                report_ids.clone(),
            );
            persist_execution_manifest(&manifest)?;
            let lock = BundleLock {
                id: format!("BLK_{:x}", fxhash(&manifest.id)),
                bundle_id: world.manifest.id.clone(),
                bundle_hash: options.bundle_hash.clone(),
                policy_resolution_id: resolved.resolution.id.clone(),
                report_ids: report_ids.clone(),
                manifest_id: manifest.id.clone(),
            };
            persist_bundle_lock(&lock)?;
            for report in reports {
                persist_report_document_with_lock(&report, &manifest.id, &lock.id)?;
            }
            let replay_manifest = build_replay_lock_manifest(
                report_ids.first().map(String::as_str).unwrap_or(&lock.id),
                &manifest.policy_manifest.policy_hash,
                &manifest.route_winner_ids.join("|"),
                &manifest.policy_manifest.policy_hash,
                &manifest.bundle_hash,
            );
            let receipt = LockReceipt {
                id: format!("LRC_{:x}", fxhash(&lock.id)),
                lock_id: lock.id.clone(),
                manifest_id: manifest.id.clone(),
                bundle_id: world.manifest.id.clone(),
                receipt_ids: world
                    .overlay
                    .import_receipts
                    .iter()
                    .map(|item| item.id.clone())
                    .collect(),
                verdict: resolved.resolution.verdict.clone(),
            };
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "lock": lock,
                    "manifest": manifest,
                    "replay_lock": replay_manifest,
                    "receipt": receipt
                }))?
            );
        }
        Command::DumpExecutionManifest { id } => {
            let manifest = load_execution_manifest(&id)?;
            println!("{}", serde_json::to_string_pretty(&manifest)?);
        }
        Command::DumpRuntimeRoots => {
            let report = runtime_root_report(&[]).map_err(anyhow::Error::msg)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::ReplayWithLock {
            id,
            parallel_obligations: _parallel_obligations,
            max_obligation_workers: _max_obligation_workers,
        } => {
            let lock = load_bundle_lock(&id)?;
            let manifest = load_execution_manifest(&lock.manifest_id)?;
            let world = load_bundle_world(&lock.bundle_id)?;
            let options = CertificationOptions {
                optimizer_policy: OptimizerPolicy::Conservative,
                bundle_hash: manifest.bundle_hash.clone(),
                policy_hash: manifest.policy_manifest.policy_hash.clone(),
                bundle_id: Some(lock.bundle_id.clone()),
                evaluator_policy: None,
                cache_policy: None,
                no_cache: false,
                replay_only: true,
                strict_derived: false,
                strict_policy: false,
                force_parallel_obligations: _parallel_obligations,
                max_obligation_workers: _max_obligation_workers,
            };
            let reports = certify_bundle_world(&world, &options)?;
            let replayed_ids = reports.iter().map(report_id).collect::<Vec<_>>();
            if replayed_ids != lock.report_ids {
                return Err(anyhow!("locked report set changed during replay"));
            }
            let replayed_winners = reports
                .iter()
                .filter_map(|report| report.selected_atlas_cell.clone())
                .collect::<Vec<_>>();
            if replayed_winners != manifest.route_winner_ids {
                return Err(anyhow!("locked route winner set changed during replay"));
            }
            let reports = reports
                .into_iter()
                .map(|mut report| {
                    if let Some(envelope) = &mut report.execution_envelope {
                        envelope.manifest_id = Some(manifest.id.clone());
                        envelope.lock_id = Some(lock.id.clone());
                    }
                    report
                })
                .collect::<Vec<_>>();
            println!("{}", serde_json::to_string_pretty(&reports)?);
        }
        Command::ObserveRun { report, lock } => {
            let reports = if let Some(report_id) = report {
                vec![replay_report(&report_id).map_err(anyhow::Error::msg)?]
            } else if let Some(lock_id) = lock {
                let lock = load_bundle_lock(&lock_id)?;
                lock.report_ids
                    .iter()
                    .map(|id| replay_report(id).map_err(anyhow::Error::msg))
                    .collect::<Result<Vec<_>>>()?
            } else {
                return Err(anyhow!("use --report <id> or --lock <id>"));
            };
            let observations = reports
                .iter()
                .map(|report| {
                    let manifest = report
                        .execution_envelope
                        .as_ref()
                        .and_then(|item| item.manifest_id.as_ref())
                        .and_then(|id| load_execution_manifest(id).ok());
                    let lock = report
                        .execution_envelope
                        .as_ref()
                        .and_then(|item| item.lock_id.as_ref())
                        .and_then(|id| load_bundle_lock(id).ok());
                    observe_report(report, manifest.as_ref(), lock.as_ref())
                })
                .collect::<Result<Vec<_>>>()?;
            println!("{}", serde_json::to_string_pretty(&observations)?);
        }
        Command::CompareLocks { left, right } => {
            let left_lock = load_bundle_lock(&left)?;
            let right_lock = load_bundle_lock(&right)?;
            let left_manifest = load_execution_manifest(&left_lock.manifest_id)?;
            let right_manifest = load_execution_manifest(&right_lock.manifest_id)?;
            let mut changed_fields = Vec::new();
            let semantic = compare_locks(&left_lock, &left_manifest, &right_lock, &right_manifest)?;
            if left_lock.bundle_hash != right_lock.bundle_hash {
                changed_fields.push("bundle_hash".into());
            }
            if left_manifest.policy_manifest.policy_hash
                != right_manifest.policy_manifest.policy_hash
            {
                changed_fields.push("policy_hash".into());
            }
            if left_manifest.route_winner_ids != right_manifest.route_winner_ids {
                changed_fields.push("route_winner_ids".into());
            }
            if left_manifest.report_ids != right_manifest.report_ids {
                changed_fields.push("report_ids".into());
            }
            let diff = LockDiff {
                id: format!("LDF_{:x}", fxhash(&(left.clone() + &right))),
                left_lock_id: left,
                right_lock_id: right,
                changed_fields,
                semantic_changes: semantic.summary,
            };
            println!("{}", serde_json::to_string_pretty(&diff)?);
        }
        Command::CompareReports { left, right } => {
            let left_report = replay_report(&left).map_err(anyhow::Error::msg)?;
            let right_report = replay_report(&right).map_err(anyhow::Error::msg)?;
            let diff = compare_reports(&left_report, &right_report)?;
            println!("{}", serde_json::to_string_pretty(&diff)?);
        }
        Command::CompareManifests { left, right } => {
            let left_manifest = load_execution_manifest(&left)?;
            let right_manifest = load_execution_manifest(&right)?;
            let diff = compare_manifests(&left_manifest, &right_manifest)?;
            println!("{}", serde_json::to_string_pretty(&diff)?);
        }
        Command::CompareReportManifest {
            report_id,
            manifest_id,
        } => {
            let report = replay_report(&report_id).map_err(anyhow::Error::msg)?;
            let manifest = load_execution_manifest(&manifest_id)?;
            let diff = compare_report_manifest(&report, &manifest)?;
            println!("{}", serde_json::to_string_pretty(&diff)?);
        }
        Command::PredictImpact {
            report,
            lock,
            bundle_file,
            policy,
            as_kind,
            conflict_policy,
        } => {
            if report.is_some() && lock.is_some() {
                return Err(anyhow!(
                    "predict-impact accepts exactly one baseline selector: --report <id> or --lock <id>"
                ));
            }
            if bundle_file.is_some() && policy.is_some() {
                return Err(anyhow!(
                    "predict-impact accepts exactly one proposed-change selector: --bundle-file <file> or --policy <id>"
                ));
            }
            let baseline_report = if let Some(report_id) = report {
                replay_report(&report_id).map_err(anyhow::Error::msg)?
            } else if let Some(lock_id) = lock {
                let lock = load_bundle_lock(&lock_id)?;
                let report_id = lock
                    .report_ids
                    .first()
                    .ok_or_else(|| anyhow!("lock has no report ids"))?;
                replay_report(report_id).map_err(anyhow::Error::msg)?
            } else {
                return Err(anyhow!(
                    "predict-impact requires one baseline selector: --report <id> or --lock <id>; use dump-command-contracts for details"
                ));
            };
            let prediction = if let Some(bundle_file) = bundle_file {
                let proposed = import_bundle_file(
                    Path::new(&bundle_file),
                    as_kind.map(Into::into),
                    conflict_policy.into(),
                    None,
                )?;
                let baseline_bundle = baseline_report
                    .execution_envelope
                    .as_ref()
                    .and_then(|item| item.bundle_id.as_ref())
                    .and_then(|id| load_bundle_world(id).ok());
                let resolved = resolve_policy_graph(
                    &proposed.overlay,
                    Some(&proposed.manifest.id),
                    proposed
                        .overlay
                        .local
                        .theorem_specs
                        .first()
                        .map(|item| item.id.as_str()),
                    proposed
                        .overlay
                        .local
                        .campaigns
                        .first()
                        .map(|item| item.id.as_str()),
                    proposed
                        .overlay
                        .local
                        .target_profiles
                        .first()
                        .map(|item| item.id.as_str()),
                    false,
                    baseline_report
                        .route_explanation
                        .as_ref()
                        .map(|item| item.optimizer_policy.clone())
                        .unwrap_or(OptimizerPolicy::Conservative),
                )
                .map_err(anyhow::Error::msg)?;
                predict_from_bundle_change(
                    &baseline_report,
                    baseline_bundle.as_ref(),
                    &proposed,
                    Some(&resolved.resolution),
                )?
            } else if let Some(policy_id) = policy {
                let policy = registry
                    .get_policy_object(&policy_id)
                    .ok_or_else(|| anyhow!("unknown policy `{policy_id}`"))?;
                predict_from_policy_object(&baseline_report, &policy)?
            } else {
                return Err(anyhow!(
                    "predict-impact requires one proposed-change selector: --bundle-file <file> or --policy <id>; use dump-command-contracts for details"
                ));
            };
            println!("{}", serde_json::to_string_pretty(&prediction)?);
        }
        Command::PlanRecompute { prediction, diff } => {
            let plan = if let Some(prediction_id) = prediction {
                let prediction = load_prediction(&prediction_id)?;
                plan_recompute_from_prediction(&prediction)?
            } else if let Some(diff_id) = diff {
                let diff = load_diff(&diff_id)?;
                plan_recompute_from_diff(&diff)?
            } else {
                return Err(anyhow!("use --prediction <id> or --diff <id>"));
            };
            println!("{}", serde_json::to_string_pretty(&plan)?);
        }
        Command::ExecutePlan {
            prediction,
            plan,
            dry_run,
            strict,
            no_cache,
            parallel,
            parallel_obligations,
            serialized,
            max_workers,
            max_obligation_workers,
            strict_determinism,
        } => {
            let plan = if let Some(prediction_id) = prediction {
                let prediction = load_prediction(&prediction_id)?;
                plan_recompute_from_prediction(&prediction)?
            } else if let Some(plan_id) = plan {
                load_plan(&plan_id)?
            } else {
                return Err(anyhow!("use --prediction <id> or --plan <id>"));
            };
            let execution = execute_plan(
                &plan,
                &ExecutePlanOptions {
                    dry_run,
                    no_cache,
                    strict,
                    force_parallel: parallel,
                    force_parallel_obligations: parallel_obligations,
                    force_serialized: serialized,
                    max_workers,
                    max_obligation_workers,
                    strict_determinism,
                },
            )?;
            println!("{}", serde_json::to_string_pretty(&execution)?);
        }
        Command::ExplainExecution { id } => match resolve_execution_record(&id) {
            Ok(execution) => {
                let explanation = serde_json::json!({
                    "kind": "full-execution-record",
                    "id": execution.id,
                    "scheduler_policy": execution.scheduler_policy,
                    "scope": execution.execution_scope,
                    "lanes": execution.lane_records,
                    "obligation_lanes": execution.obligation_lanes,
                    "obligation_ordering": execution.obligation_ordering_receipts,
                    "ordering": execution.ordering_receipt,
                    "coherence": execution.coherence_receipts,
                    "notes": execution.explanation,
                });
                println!("{}", serde_json::to_string_pretty(&explanation)?);
            }
            Err(primary_err) => {
                let fallback = explain_execution_fallback(&id)?;
                let explanation = serde_json::json!({
                    "kind": "surrogate-execution-context",
                    "id": id,
                    "resolution": fallback,
                    "warning": primary_err.to_string(),
                });
                println!("{}", serde_json::to_string_pretty(&explanation)?);
            }
        },
        Command::DumpExecutionDag { id } => {
            let execution = resolve_execution_record(&id)?;
            let nodes = execution
                .outcomes
                .iter()
                .map(|item| {
                    serde_json::json!({
                        "id": item.step_id,
                        "kind": item.kind,
                        "status": item.status
                    })
                })
                .collect::<Vec<_>>();
            let edges = execution
                .outcomes
                .windows(2)
                .map(|pair| {
                    serde_json::json!({
                        "from": pair[0].step_id,
                        "to": pair[1].step_id,
                        "relation": "ordered-before"
                    })
                })
                .collect::<Vec<_>>();
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({"nodes": nodes, "edges": edges}))?
            );
        }
        Command::DumpLanePlan { id } => {
            let execution = resolve_execution_record(&id)?;
            println!("{}", serde_json::to_string_pretty(&execution.lane_records)?);
        }
        Command::ExplainObligationPlan { id } => {
            let execution = resolve_execution_record(&id)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "plans": execution.obligation_plans,
                    "lanes": execution.obligation_lanes,
                    "ordering": execution.obligation_ordering_receipts,
                    "merge": execution.obligation_merge_receipts,
                    "replay_legality": execution.replay_legality_checks,
                    "barriers": execution.replay_barrier_receipts,
                    "obligation_statuses": execution
                        .resulting_reports
                        .iter()
                        .map(|report| serde_json::json!({
                            "theorem_id": report.theorem_id,
                            "campaign_id": report.campaign_id,
                            "obligations": report.obligations,
                        }))
                        .collect::<Vec<_>>(),
                }))?
            );
        }
        Command::DumpObligationDag { id } => {
            let execution = resolve_execution_record(&id)?;
            let dags = execution
                .obligation_plans
                .iter()
                .map(|plan| {
                    serde_json::json!({
                        "plan_id": plan.id,
                        "theorem_id": plan.theorem_id,
                        "campaign_id": plan.campaign_id,
                        "nodes": plan.nodes,
                        "edges": plan.edges,
                    })
                })
                .collect::<Vec<_>>();
            println!("{}", serde_json::to_string_pretty(&dags)?);
        }
        Command::DumpObligationLanes { id } => {
            let execution = resolve_execution_record(&id)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&execution.obligation_lanes)?
            );
        }
        Command::CompareObligationExecutions { left, right } => {
            let left = resolve_execution_record(&left)?;
            let right = resolve_execution_record(&right)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "left": {
                        "plans": left.obligation_plans,
                        "lanes": left.obligation_lanes,
                        "reports": left.resulting_reports,
                    },
                    "right": {
                        "plans": right.obligation_plans,
                        "lanes": right.obligation_lanes,
                        "reports": right.resulting_reports,
                    },
                    "same": left.obligation_plans == right.obligation_plans && left.obligation_lanes == right.obligation_lanes,
                }))?
            );
        }
        Command::CompareSchedules { left, right } => {
            let left = resolve_execution_record(&left)?;
            let right = resolve_execution_record(&right)?;
            let output = serde_json::json!({
                "left": {
                    "schedule_hash": left.schedule_hash,
                    "lanes": left.lane_records,
                },
                "right": {
                    "schedule_hash": right.schedule_hash,
                    "lanes": right.lane_records,
                },
                "same_schedule": left.schedule_hash == right.schedule_hash && left.lane_records == right.lane_records,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        Command::ExplainPlan { id } => {
            let plan = load_plan(&id)?;
            let explanation = serde_json::json!({
                "id": plan.id,
                "prediction_id": plan.prediction_id,
                "diff_id": plan.diff_id,
                "reusable_artifacts": plan.reusable_artifacts,
                "invalidated_artifacts": plan.invalidated_artifacts,
                "steps": plan.steps,
                "hints": plan.steps.iter().map(|step| serde_json::json!({
                    "step_id": step.id,
                    "kind": step.kind,
                    "target_id": step.target_id,
                    "reason": step.reason
                })).collect::<Vec<_>>(),
                "explanation": plan.explanation,
            });
            println!("{}", serde_json::to_string_pretty(&explanation)?);
        }
        Command::ExplainDrift { left, right, kind } => {
            let diff = match kind {
                CompareKindArg::Report => {
                    let left_report = replay_report(&left).map_err(anyhow::Error::msg)?;
                    let right_report = replay_report(&right).map_err(anyhow::Error::msg)?;
                    compare_reports(&left_report, &right_report)?
                }
                CompareKindArg::Manifest => {
                    let left_manifest = load_execution_manifest(&left)?;
                    let right_manifest = load_execution_manifest(&right)?;
                    compare_manifests(&left_manifest, &right_manifest)?
                }
                CompareKindArg::Lock => {
                    let left_lock = load_bundle_lock(&left)?;
                    let right_lock = load_bundle_lock(&right)?;
                    let left_manifest = load_execution_manifest(&left_lock.manifest_id)?;
                    let right_manifest = load_execution_manifest(&right_lock.manifest_id)?;
                    compare_locks(&left_lock, &left_manifest, &right_lock, &right_manifest)?
                }
            };
            let explanation = explain_drift(&diff)?;
            let drift = drift_report(&diff)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "diff": diff,
                    "drift": drift,
                    "explanation": explanation
                }))?
            );
        }
        Command::CompareExecutions { left, right } => {
            let left = resolve_execution_record(&left)?;
            let right = resolve_execution_record(&right)?;
            let diff = compare_executions(&left, &right)?;
            println!("{}", serde_json::to_string_pretty(&diff)?);
        }
        Command::AssessPrediction {
            prediction_id,
            actual_report_id,
        } => {
            let prediction = load_prediction(&prediction_id)?;
            let actual_report = replay_report(&actual_report_id).map_err(anyhow::Error::msg)?;
            let assessment = assess_prediction(&prediction, &actual_report)?;
            println!("{}", serde_json::to_string_pretty(&assessment)?);
        }
        Command::ReconcileRun {
            prediction,
            actual_report,
        } => {
            let prediction = load_prediction(&prediction)?;
            let actual_report = replay_report(&actual_report).map_err(anyhow::Error::msg)?;
            let reconciliation = reconcile_prediction_to_report(&prediction, &actual_report)?;
            println!("{}", serde_json::to_string_pretty(&reconciliation)?);
        }
        Command::DumpPolicyGraph { bundle } => {
            let resolution = if let Some(bundle_id) = bundle {
                let world = load_bundle_world(&bundle_id)?;
                resolve_policy_graph(
                    &world.overlay,
                    Some(&bundle_id),
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
                    false,
                    OptimizerPolicy::Conservative,
                )
                .map_err(anyhow::Error::msg)?
            } else {
                resolve_policy_graph(
                    &registry,
                    None,
                    None,
                    None,
                    None,
                    false,
                    OptimizerPolicy::Conservative,
                )
                .map_err(anyhow::Error::msg)?
            };
            println!("{}", serde_json::to_string_pretty(&resolution)?);
        }
        Command::ExplainPolicyResolution { bundle, policy } => {
            let resolved = if let Some(bundle_id) = bundle {
                let world = load_bundle_world(&bundle_id)?;
                resolve_policy_graph(
                    &world.overlay,
                    Some(&bundle_id),
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
                    true,
                    OptimizerPolicy::Conservative,
                )
                .map_err(anyhow::Error::msg)?
            } else {
                resolve_policy_graph(
                    &registry,
                    None,
                    None,
                    None,
                    None,
                    true,
                    OptimizerPolicy::Conservative,
                )
                .map_err(anyhow::Error::msg)?
            };
            let output = if let Some(policy_id) = policy {
                serde_json::json!({
                    "policy": policy_id,
                    "applied": resolved.resolution.applied_policy_ids.iter().any(|id| id == &policy_id),
                    "trace": resolved.resolution.trace.steps,
                    "conflicts": resolved.resolution.conflicts,
                })
            } else {
                serde_json::to_value(&resolved.resolution)?
            };
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        Command::ExportArtifact { id, to_kind } => {
            let document = document_for_id(&registry, &id)?;
            let policy = default_policy_for(to_kind.clone().into(), &registry)?;
            let (rendered, receipt) =
                export_document(&document, to_kind.into(), &policy, &registry)?;
            println!("{rendered}");
            eprintln!("{}", serde_json::to_string(&receipt)?);
        }
        Command::ResolveArtifactPath { id } => {
            let locator = locate_known_artifact(&id)?;
            println!("{}", serde_json::to_string_pretty(&locator)?);
        }
        Command::ClassifyArtifact { id } => {
            let info = classify_artifact(&id)?;
            println!("{}", serde_json::to_string_pretty(&info)?);
        }
        Command::CheckCommandInput { command, id } => {
            let info = classify_artifact(&id)?;
            let contract = command_contract_by_name(&command).ok_or_else(|| {
                anyhow!("unknown command `{command}`; use dump-command-contracts")
            })?;
            let accepted = info
                .get("kind")
                .and_then(|v| v.as_str())
                .map(|kind| {
                    contract
                        .accepted_inputs
                        .iter()
                        .any(|item| format!("{:?}", item) == kind)
                })
                .unwrap_or(false);
            let output = serde_json::json!({
                "command": contract.command,
                "artifact": info,
                "accepted": accepted,
                "reason": if accepted {
                    "artifact kind is accepted by command contract"
                } else {
                    "artifact kind is not accepted by command contract"
                },
                "command_contract": contract,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        Command::DumpCommandContracts => {
            println!("{}", serde_json::to_string_pretty(&command_contracts())?);
        }
        Command::DumpArtifactContracts => {
            println!("{}", serde_json::to_string_pretty(&artifact_contracts())?);
        }
        Command::DumpCapabilityReadiness => {
            let mut readiness = std::collections::BTreeMap::new();
            for contract in command_contracts() {
                let bucket = format!("{:?}", contract.readiness);
                readiness
                    .entry(bucket)
                    .or_insert_with(Vec::new)
                    .push(contract.command);
            }
            println!(
                "{}",
                serde_json::to_string_pretty(
                    &serde_json::json!({"readiness": readiness, "count": command_contracts().len()})
                )?
            );
        }
        Command::DumpCacheNamespaces => {
            println!(
                "{}",
                serde_json::to_string_pretty(&dump_cache_namespaces()?)?
            );
        }
    }
    Ok(())
}

fn build_cert_options(
    optimizer_policy: OptimizerPolicy,
    file: &str,
    bundle_id: &str,
    evaluator_policy: Option<String>,
    cache_policy: Option<String>,
    strict_policy: bool,
) -> Result<CertificationOptions> {
    let input = fs::read_to_string(file).with_context(|| format!("failed to read `{file}`"))?;
    Ok(CertificationOptions {
        optimizer_policy: optimizer_policy.clone(),
        bundle_hash: format!("{:x}", fxhash(&input)),
        policy_hash: format!(
            "{:x}",
            fxhash(&format!(
                "{optimizer_policy:?}|evaluator={:?}|cache={:?}|strict={strict_policy}",
                evaluator_policy, cache_policy
            ))
        ),
        bundle_id: Some(bundle_id.to_string()),
        evaluator_policy,
        cache_policy,
        no_cache: false,
        replay_only: false,
        strict_derived: false,
        strict_policy,
        force_parallel_obligations: false,
        max_obligation_workers: None,
    })
}

fn resolve_for_world(
    world: &BundleWorld,
    options: &CertificationOptions,
) -> Result<l64_policy::ResolvedPolicyGraph> {
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
        options.strict_policy,
        options.optimizer_policy.clone(),
    )
    .map_err(anyhow::Error::msg)
}

fn certify_bundle_world(
    world: &BundleWorld,
    options: &CertificationOptions,
) -> Result<Vec<CertificationReport>> {
    let mut reports = Vec::new();
    if world.overlay.local.campaigns.is_empty() {
        for theorem in &world.overlay.local.theorem_specs {
            let target = world.overlay.local.target_profiles.first().ok_or_else(|| {
                anyhow!("bundle theorem execution requires at least one target profile")
            })?;
            let atlas =
                l64_atlas::CompiledAtlas::compile(&world.overlay).map_err(anyhow::Error::msg)?;
            let report = certify_derived_theorem_with_options(
                &world.overlay,
                &atlas,
                &theorem.id,
                &target.id,
                None,
                options,
            )
            .map_err(anyhow::Error::msg)?;
            reports.push(report);
        }
    } else {
        let atlas =
            l64_atlas::CompiledAtlas::compile(&world.overlay).map_err(anyhow::Error::msg)?;
        for campaign in &world.overlay.local.campaigns {
            let report = certify_derived_campaign_with_options(
                &world.overlay,
                &atlas,
                &campaign.id,
                options,
            )
            .map_err(anyhow::Error::msg)?;
            reports.push(report);
        }
    }
    Ok(reports)
}

fn persist_report_document_with_lock(
    report: &CertificationReport,
    manifest_id: &str,
    lock_id: &str,
) -> Result<()> {
    let path = report_cache_path(&report_id(report))?;
    let mut report = report.clone();
    if let Some(envelope) = &mut report.execution_envelope {
        envelope.manifest_id = Some(manifest_id.to_string());
        envelope.lock_id = Some(lock_id.to_string());
    }
    let bytes = encode_locus_packet_for_report(&report).map_err(anyhow::Error::msg)?;
    fs::write(path, bytes)?;
    Ok(())
}

fn predict_from_policy_object(
    baseline_report: &CertificationReport,
    policy: &MechanizationPolicyObject,
) -> Result<l64_observe::PredictionRecord> {
    let kind = match policy.kind {
        l64_core::PolicyKind::Optimizer => "Optimizer",
        l64_core::PolicyKind::Evaluator => "Evaluator",
        l64_core::PolicyKind::ReplayCache => "ReplayCache",
        _ => "Other",
    };
    predict_from_policy_override(baseline_report, &policy.id, kind, policy.notes.clone())
}

fn document_for_id(registry: &SeedRegistry, id: &str) -> Result<QaDocument> {
    if let Some(document) = document_for_registry_id(registry, id) {
        return Ok(document);
    }
    if let Ok(manifest) = load_execution_manifest(id) {
        return Ok(QaDocument {
            entries: vec![QaEntry::ExecutionManifest(manifest)],
        });
    }
    if let Ok(lock) = load_bundle_lock(id) {
        return Ok(QaDocument {
            entries: vec![QaEntry::BundleLock(lock)],
        });
    }
    if let Ok(document) = load_report_document_with_registry(id, registry) {
        return Ok(document);
    }
    Err(anyhow!("no lock/manifest/report artifact found for `{id}`"))
}

fn locate_known_artifact(id: &str) -> Result<ArtifactLocator> {
    let manifest = manifest_cache_root()?.join(format!("{id}.locus"));
    if manifest.exists() {
        return Ok(locate_artifact(id, "manifest", &manifest).map_err(anyhow::Error::msg)?);
    }
    let legacy_manifest = manifest_cache_root()?.join(format!("{id}.json"));
    if legacy_manifest.exists() {
        return Ok(locate_artifact(id, "manifest", &legacy_manifest).map_err(anyhow::Error::msg)?);
    }
    let lock = manifest_cache_root()?.join(format!("{id}.lock.locus"));
    if lock.exists() {
        return Ok(locate_artifact(id, "lock", &lock).map_err(anyhow::Error::msg)?);
    }
    let legacy_lock = manifest_cache_root()?.join(format!("{id}.lock.json"));
    if legacy_lock.exists() {
        return Ok(locate_artifact(id, "lock", &legacy_lock).map_err(anyhow::Error::msg)?);
    }
    let report = report_cache_path(id)?;
    if report.exists() {
        return Ok(locate_artifact(id, "report", &report).map_err(anyhow::Error::msg)?);
    }
    let legacy_report = report_cache_root()?.join(format!("{id}.json"));
    if legacy_report.exists() {
        return Ok(locate_artifact(id, "report", &legacy_report).map_err(anyhow::Error::msg)?);
    }
    let roots = runtime_root_report(&[]).map_err(anyhow::Error::msg)?;
    Err(anyhow!(
        "artifact `{id}` not found under {} or {}",
        roots.project_root.absolute_path,
        roots.cache_root.absolute_path
    ))
}

fn command_contracts() -> Vec<CommandContract> {
    vec![
        CommandContract {
            command: "explain-execution".into(),
            accepted_inputs: vec![ArtifactKind::PlanExecution, ArtifactKind::Report, ArtifactKind::ExecutionManifest, ArtifactKind::BundleLock, ArtifactKind::ObservationRecord],
            produces: vec![ArtifactKind::ForensicBundle],
            readiness: CapabilityReadiness::ContractKnown,
            namespace_scope: NamespaceScope::SameNamespace,
            notes: vec!["execution-centric admin command; resolves through report/manifest/lock/observe references before scanning executions".into()],
        },
        CommandContract {
            command: "predict-impact".into(),
            accepted_inputs: vec![ArtifactKind::Report, ArtifactKind::BundleLock],
            produces: vec![ArtifactKind::PredictionRecord],
            readiness: CapabilityReadiness::ContractKnown,
            namespace_scope: NamespaceScope::SameNamespace,
            notes: vec!["requires exactly one baseline selector (--report or --lock) and exactly one proposed change selector (--bundle-file or --policy)".into()],
        },
        CommandContract {
            command: "dump-execution-manifest".into(),
            accepted_inputs: vec![ArtifactKind::ExecutionManifest],
            produces: vec![ArtifactKind::ExecutionManifest],
            readiness: CapabilityReadiness::FullyExercised,
            namespace_scope: NamespaceScope::SameNamespace,
            notes: vec!["manifest id only".into()],
        },
        CommandContract {
            command: "compare-executions".into(),
            accepted_inputs: vec![ArtifactKind::PlanExecution],
            produces: vec![ArtifactKind::ForensicBundle],
            readiness: CapabilityReadiness::SmokeExecuted,
            namespace_scope: NamespaceScope::SameNamespace,
            notes: vec!["compare realized execution records, not plan ids".into()],
        },
        CommandContract {
            command: "classify-artifact".into(),
            accepted_inputs: vec![ArtifactKind::Report, ArtifactKind::ExecutionManifest, ArtifactKind::BundleLock, ArtifactKind::ObservationRecord, ArtifactKind::PlanExecution],
            produces: vec![ArtifactKind::ForensicBundle],
            readiness: CapabilityReadiness::SmokeExecuted,
            namespace_scope: NamespaceScope::Any,
            notes: vec!["artifact self-inspection surface for operational truth".into()],
        },
        CommandContract {
            command: "check-command-input".into(),
            accepted_inputs: vec![ArtifactKind::Report, ArtifactKind::ExecutionManifest, ArtifactKind::BundleLock, ArtifactKind::ObservationRecord, ArtifactKind::PlanExecution, ArtifactKind::PredictionRecord, ArtifactKind::RecomputationPlan],
            produces: vec![ArtifactKind::ForensicBundle],
            readiness: CapabilityReadiness::SmokeExecuted,
            namespace_scope: NamespaceScope::Any,
            notes: vec!["checks whether a specific artifact id is lawful input for a command contract".into()],
        },
    ]
}

fn artifact_contracts() -> Vec<ArtifactContract> {
    vec![
        ArtifactContract {
            kind: ArtifactKind::InspectionReport,
            class: "inspection".into(),
            standalone_validation_complete: false,
            namespace_scope: NamespaceScope::Any,
            notes: vec!["optimized for inspection/export; may omit theorem/route context needed for full standalone validation".into()],
        },
        ArtifactContract {
            kind: ArtifactKind::ValidationBundle,
            class: "validation".into(),
            standalone_validation_complete: true,
            namespace_scope: NamespaceScope::Any,
            notes: vec!["self-contained export intended for validate/replay-style standalone checking".into()],
        },
        ArtifactContract {
            kind: ArtifactKind::PlanExecution,
            class: "execution".into(),
            standalone_validation_complete: false,
            namespace_scope: NamespaceScope::SameNamespace,
            notes: vec!["execution records are namespace-local operational artifacts".into()],
        },
        ArtifactContract {
            kind: ArtifactKind::Report,
            class: "report".into(),
            standalone_validation_complete: false,
            namespace_scope: NamespaceScope::SameNamespace,
            notes: vec!["live report ids may resolve into execution, manifest, and observation context inside the same namespace".into()],
        },
        ArtifactContract {
            kind: ArtifactKind::ExecutionManifest,
            class: "manifest".into(),
            standalone_validation_complete: false,
            namespace_scope: NamespaceScope::SameNamespace,
            notes: vec!["execution manifests are operational context artifacts and may lead to execution reports but are not themselves validation bundles".into()],
        },
        ArtifactContract {
            kind: ArtifactKind::BundleLock,
            class: "lock".into(),
            standalone_validation_complete: false,
            namespace_scope: NamespaceScope::SameNamespace,
            notes: vec!["bundle locks pin replay and report sets inside one namespace".into()],
        },
        ArtifactContract {
            kind: ArtifactKind::ObservationRecord,
            class: "observation".into(),
            standalone_validation_complete: false,
            namespace_scope: NamespaceScope::SameNamespace,
            notes: vec!["observation records expose execution context indirectly through report linkage".into()],
        },
        ArtifactContract {
            kind: ArtifactKind::PredictionRecord,
            class: "prediction".into(),
            standalone_validation_complete: false,
            namespace_scope: NamespaceScope::SameNamespace,
            notes: vec!["prediction records are advisory frontier artifacts, not validation-complete truth".into()],
        },
        ArtifactContract {
            kind: ArtifactKind::RecomputationPlan,
            class: "plan".into(),
            standalone_validation_complete: false,
            namespace_scope: NamespaceScope::SameNamespace,
            notes: vec!["recomputation plans are executable within one cache/runtime context".into()],
        },
    ]
}

fn command_contract_by_name(command: &str) -> Option<CommandContract> {
    command_contracts()
        .into_iter()
        .find(|item| item.command == command)
}

fn artifact_contract_for_kind(kind: ArtifactKind) -> Option<ArtifactContract> {
    artifact_contracts()
        .into_iter()
        .find(|item| item.kind == kind)
}

fn classify_artifact(id: &str) -> Result<serde_json::Value> {
    if let Ok(record) = load_execution(id) {
        let locator = locate_known_artifact(id).ok();
        let contract = artifact_contract_for_kind(ArtifactKind::PlanExecution);
        return Ok(serde_json::json!({
            "id": id,
            "kind": format!("{:?}", ArtifactKind::PlanExecution),
            "locator": locator,
            "contract": contract,
            "linked_reports": record.resulting_report_ids,
            "linked_manifests": record.manifest_ids,
            "linked_locks": record.lock_ids,
        }));
    }
    if let Ok(observation) = l64_observe::load_observation(id) {
        let locator = locate_known_artifact(id).ok();
        let contract = artifact_contract_for_kind(ArtifactKind::ObservationRecord);
        return Ok(serde_json::json!({
            "id": id,
            "kind": format!("{:?}", ArtifactKind::ObservationRecord),
            "locator": locator,
            "contract": contract,
            "linked_report": observation.record.report_id,
        }));
    }
    if let Ok(report) = replay_report(id).map_err(anyhow::Error::msg) {
        let locator = locate_known_artifact(id).ok();
        let contract = artifact_contract_for_kind(ArtifactKind::Report);
        return Ok(serde_json::json!({
            "id": id,
            "kind": format!("{:?}", ArtifactKind::Report),
            "locator": locator,
            "contract": contract,
            "execution_envelope": report.execution_envelope,
            "selected_atlas_cell": report.selected_atlas_cell,
        }));
    }
    if let Ok(manifest) = load_execution_manifest(id) {
        let locator = locate_known_artifact(id).ok();
        let contract = artifact_contract_for_kind(ArtifactKind::ExecutionManifest);
        return Ok(serde_json::json!({
            "id": id,
            "kind": format!("{:?}", ArtifactKind::ExecutionManifest),
            "locator": locator,
            "contract": contract,
            "report_ids": manifest.report_ids,
            "rerun_artifacts": manifest.rerun_artifacts,
        }));
    }
    if let Ok(lock) = load_bundle_lock(id) {
        let locator = locate_known_artifact(id).ok();
        let contract = artifact_contract_for_kind(ArtifactKind::BundleLock);
        return Ok(serde_json::json!({
            "id": id,
            "kind": format!("{:?}", ArtifactKind::BundleLock),
            "locator": locator,
            "contract": contract,
            "manifest_id": lock.manifest_id,
            "report_ids": lock.report_ids,
        }));
    }
    if let Ok(locator) = locate_known_artifact(id) {
        return Ok(serde_json::json!({
            "id": id,
            "kind": "UnclassifiedLocatedArtifact",
            "locator": locator,
            "warning": "artifact path exists but no typed loader classified it"
        }));
    }
    Err(anyhow!("artifact `{id}` could not be classified"))
}

fn resolve_execution_record(id: &str) -> Result<l64_core::PlanExecutionRecord> {
    if let Ok(record) = load_execution(id) {
        return Ok(record);
    }
    if let Ok(observation) = l64_observe::load_observation(id) {
        return resolve_execution_record_from_report_id(&observation.record.report_id);
    }
    if let Ok(report) = replay_report(id).map_err(anyhow::Error::msg) {
        return resolve_execution_record_from_report(&report);
    }
    if let Ok(manifest) = load_execution_manifest(id) {
        if let Some(record) = scan_execution_records(|item| {
            item.manifest_ids
                .iter()
                .any(|manifest_id| manifest_id == &manifest.id)
        })? {
            return Ok(record);
        }
    }
    if let Ok(lock) = load_bundle_lock(id) {
        if let Some(record) =
            scan_execution_records(|item| item.lock_ids.iter().any(|lock_id| lock_id == &lock.id))?
        {
            return Ok(record);
        }
        for report_id in &lock.report_ids {
            if let Ok(record) = resolve_execution_record_from_report_id(report_id) {
                return Ok(record);
            }
        }
    }
    Err(anyhow!(
        "no execution record could be resolved from `{id}`; use dump-command-contracts for accepted inputs"
    ))
}

fn resolve_execution_record_from_report_id(
    report_id: &str,
) -> Result<l64_core::PlanExecutionRecord> {
    let report = replay_report(report_id).map_err(anyhow::Error::msg)?;
    resolve_execution_record_from_report(&report)
}

fn resolve_execution_record_from_report(
    report: &CertificationReport,
) -> Result<l64_core::PlanExecutionRecord> {
    if let Some(execution_id) = report
        .execution_envelope
        .as_ref()
        .and_then(|item| item.executed_plan_id.as_ref())
    {
        if let Ok(record) = load_execution(execution_id) {
            return Ok(record);
        }
    }
    if let Some(record) = scan_execution_records(|item| {
        item.resulting_report_ids
            .iter()
            .any(|id| id == &report_id(report))
    })? {
        return Ok(record);
    }
    if let Some(manifest_id) = report
        .execution_envelope
        .as_ref()
        .and_then(|item| item.manifest_id.as_ref())
    {
        if let Some(record) =
            scan_execution_records(|item| item.manifest_ids.iter().any(|id| id == manifest_id))?
        {
            return Ok(record);
        }
    }
    if let Some(lock_id) = report
        .execution_envelope
        .as_ref()
        .and_then(|item| item.lock_id.as_ref())
    {
        if let Some(record) =
            scan_execution_records(|item| item.lock_ids.iter().any(|id| id == lock_id))?
        {
            return Ok(record);
        }
    }
    Err(anyhow!(
        "no execution record linked to report `{}`",
        report_id(report)
    ))
}

fn explain_execution_fallback(id: &str) -> Result<serde_json::Value> {
    let classified = classify_artifact(id).ok();
    if let Ok(observation) = l64_observe::load_observation(id) {
        let route_events = observation
            .record
            .events
            .iter()
            .filter(|event| matches!(event.kind, l64_core::ExecutionEventKind::RouteSelected))
            .count();
        let obligation_traces = observation
            .record
            .events
            .iter()
            .filter(|event| {
                matches!(
                    event.kind,
                    l64_core::ExecutionEventKind::ObligationEvaluated
                )
            })
            .count();
        return Ok(serde_json::json!({
            "artifact_class": "observation-record",
            "classification": classified,
            "report_id": observation.record.report_id,
            "record_id": observation.record.id,
            "available": {
                "decision_nodes": observation.graph.nodes.len(),
                "route_events": route_events,
                "obligation_traces": obligation_traces,
            },
            "next_best_inputs": [observation.record.report_id.clone()],
            "notes": ["full execution record unavailable; observation-derived execution context returned"]
        }));
    }
    if let Ok(report) = replay_report(id).map_err(anyhow::Error::msg) {
        let mut next = Vec::new();
        if let Some(env) = &report.execution_envelope {
            if let Some(mid) = &env.manifest_id {
                next.push(mid.clone());
            }
            if let Some(lid) = &env.lock_id {
                next.push(lid.clone());
            }
            if let Some(eid) = &env.executed_plan_id {
                next.push(eid.clone());
            }
        }
        return Ok(serde_json::json!({
            "artifact_class": "report",
            "classification": classified,
            "report_id": report_id(&report),
            "execution_envelope": report.execution_envelope,
            "selected_path": report.selected_path,
            "next_best_inputs": next,
            "notes": ["full execution record unavailable; report-linked execution envelope returned"]
        }));
    }
    if let Ok(manifest) = load_execution_manifest(id) {
        return Ok(serde_json::json!({
            "artifact_class": "execution-manifest",
            "classification": classified,
            "manifest_id": manifest.id,
            "report_ids": manifest.report_ids,
            "rerun_artifacts": manifest.rerun_artifacts,
            "notes": ["full execution record unavailable; manifest-derived execution context returned"]
        }));
    }
    if let Ok(lock) = load_bundle_lock(id) {
        return Ok(serde_json::json!({
            "artifact_class": "bundle-lock",
            "classification": classified,
            "lock_id": lock.id,
            "report_ids": lock.report_ids,
            "manifest_id": lock.manifest_id,
            "notes": ["full execution record unavailable; lock-derived execution context returned"]
        }));
    }
    Err(anyhow!(
        "no explainable execution context could be resolved from `{id}`; use dump-command-contracts for accepted inputs"
    ))
}

fn scan_execution_records(
    mut predicate: impl FnMut(&l64_core::PlanExecutionRecord) -> bool,
) -> Result<Option<l64_core::PlanExecutionRecord>> {
    let root = l64_core::resolve_cache_root().map_err(anyhow::Error::msg)?;
    let dir = Path::new(&root.absolute_path).join("executions");
    if !dir.exists() {
        return Ok(None);
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|item| item.to_str()) != Some("locus")
            && path.extension().and_then(|item| item.to_str()) != Some("json")
        {
            continue;
        }
        let stem = match path.file_stem().and_then(|item| item.to_str()) {
            Some(stem) => stem.to_string(),
            None => continue,
        };
        if let Ok(record) = load_execution(&stem) {
            if predicate(&record) {
                return Ok(Some(record));
            }
        }
    }
    Ok(None)
}

fn dump_cache_namespaces() -> Result<Vec<String>> {
    let project = l64_core::resolve_project_root().map_err(anyhow::Error::msg)?;
    let root = Path::new(&project.absolute_path)
        .join(".l64-cache")
        .join("namespaces");
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut items = fs::read_dir(root)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    items.sort();
    Ok(items)
}

fn fxhash(input: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
