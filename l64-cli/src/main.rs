use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use l64_atlas::{CompiledAtlas, run_seed_theorem};
use l64_bundle::{
    BundleWorld, import_bundle_file, load_bundle_world, overlay_registry_from_document,
};
use l64_canon::CanonEngine;
use l64_cert::{
    CertificationOptions, build_vertical_compounding_bundle, cache_stats as cert_cache_stats,
    certify_derived_campaign_with_options, certify_derived_theorem_with_options,
    clear_cache as clear_cert_cache, decode_locus_packet_report, decode_locus_packet_summary,
    derive_distress_vector, derive_help_request, dispatch_proof_coverage,
    encode_locus_packet_for_report, explain_invalidation as cert_explain_invalidation,
    replay_report as cert_replay_report,
};
use l64_command::{BundlePolicyArg, OptimizerPolicyArg, SurfaceArg};
use l64_core::{
    Budget, BundleConflictPolicy, Canonicalize, CheckProofShape, GenomeArtifactClass,
    GenomeSurface, OptimizerPolicy, Promote, QaDocument, QaEntry, RegistryLookup, ReplayStatus,
    ResearchLineageRecord, SelectRoute, SurfaceKind, decode_locus_packet, locus_packet_summary,
};
use l64_kernel::ConstitutionKernel;
use l64_locus::{compile_rna_to_dna_packet, sequence_dna_to_rna};
use l64_qa0::{normalize_document as normalize_qa0_document, parse_document as parse_qa0_document};
use l64_registry::SeedRegistry;
use l64_research::{
    derive_governed_complete_research_bundle_from_report,
    derive_governed_promotion_readiness_from_report, list_benchmark_run_records,
    list_benchmark_schemas, list_challenge_records, list_coverage_dispatches,
    list_derivation_signatures, list_framework_registry_entries, list_handoff_packets,
    list_lineage_records, list_math_claim_packets, list_operator_records, list_producer_host_specs,
    list_projection_map_records, list_promotion_queue_entries, list_promotion_readiness_reports,
    list_reduction_map_records, list_remediation_entries, list_reproducibility_packets,
    list_review_receipts, list_route_assignments, list_strengthening_artifacts,
    list_task_envelopes, list_vertical_bundles, load_benchmark_run_record, load_benchmark_schema,
    load_challenge_record, load_coverage_dispatch, load_derivation_signature,
    load_framework_registry_entry, load_handoff_packet, load_lineage_record,
    load_math_claim_packet, load_operator_record, load_producer_host_spec,
    load_projection_map_record, load_promotion_queue_entry, load_promotion_readiness_report,
    load_reduction_map_record, load_remediation_entry, load_reproducibility_packet,
    load_review_receipt, load_route_assignment, load_strengthening_artifact, load_task_envelope,
    load_vertical_bundle, persist_benchmark_run_record, persist_benchmark_schema,
    persist_challenge_record, persist_coverage_dispatch, persist_derivation_signature,
    persist_framework_registry_entry, persist_handoff_packet, persist_lineage_record,
    persist_math_claim_packet, persist_operator_record, persist_producer_host_spec,
    persist_projection_map_record, persist_promotion_queue_entry,
    persist_promotion_readiness_report, persist_reduction_map_record,
    persist_reproducibility_packet, persist_research_import, persist_review_receipt,
    persist_route_assignment, persist_seeded_export_remediation_entries,
    persist_strengthening_artifact, persist_task_envelope, persist_vertical_bundle,
    research_status_summary, score_routes_governed, summarize_remediation_entries,
};
use l64_runtime::{RuntimeWorld, exec_host};
use l64_selector::{AtlasSelector, CampaignCertifier};
use l64_surfaces::{
    default_policy_for, document_for_registry_id, dump_transform_receipt, expand_qk0_file,
    export_document, import_file, load_bundle_lock, load_execution_manifest,
    load_report_document_with_registry, normalize_surface, persist_report_document,
    report_cache_path, report_cache_root, report_id, report_to_document_with_registry,
    report_to_validation_bundle_with_registry, roundtrip_check, surface_capabilities,
    surface_extension, transcode_text,
};
use std::{fs, path::Path};

#[derive(Debug, Parser)]
#[command(name = "l64-cli")]
#[command(about = "Mathematical framework constitution kernel CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum ResearchKindArg {
    Task,
    Signature,
    Review,
    Challenge,
    Registry,
    Operator,
    Benchmark,
    BenchmarkRun,
    Claim,
    Reduction,
    Projection,
    Handoff,
    ProducerHost,
    Promotion,
    PromotionReport,
    RouteAssignment,
    Strengthening,
    Repro,
    Remediation,
    Coverage,
    Tower,
    Lineage,
}

impl ResearchKindArg {
    fn as_str(self) -> &'static str {
        match self {
            ResearchKindArg::Task => "task",
            ResearchKindArg::Signature => "signature",
            ResearchKindArg::Review => "review",
            ResearchKindArg::Challenge => "challenge",
            ResearchKindArg::Registry => "registry",
            ResearchKindArg::Operator => "operator",
            ResearchKindArg::Benchmark => "benchmark",
            ResearchKindArg::BenchmarkRun => "benchmark-run",
            ResearchKindArg::Claim => "claim",
            ResearchKindArg::Reduction => "reduction",
            ResearchKindArg::Projection => "projection",
            ResearchKindArg::Handoff => "handoff",
            ResearchKindArg::ProducerHost => "producer-host",
            ResearchKindArg::Promotion => "promotion",
            ResearchKindArg::PromotionReport => "promotion-report",
            ResearchKindArg::RouteAssignment => "route-assignment",
            ResearchKindArg::Strengthening => "strengthening",
            ResearchKindArg::Repro => "repro",
            ResearchKindArg::Remediation => "remediation",
            ResearchKindArg::Coverage => "coverage",
            ResearchKindArg::Tower => "tower",
            ResearchKindArg::Lineage => "lineage",
        }
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    Parse {
        file: String,
        #[arg(long = "as", value_enum)]
        as_kind: Option<SurfaceArg>,
    },
    Normalize {
        file: String,
        #[arg(long = "as", value_enum)]
        as_kind: Option<SurfaceArg>,
    },
    Validate {
        file: String,
        #[arg(long = "as", value_enum)]
        as_kind: Option<SurfaceArg>,
    },
    Import {
        file: String,
        #[arg(long = "as", value_enum)]
        as_kind: Option<SurfaceArg>,
    },
    ImportBundle {
        file: String,
        #[arg(long = "as", value_enum)]
        as_kind: Option<SurfaceArg>,
        #[arg(long, value_enum, default_value = "reject")]
        conflict_policy: BundlePolicyArg,
        #[arg(long)]
        namespace: Option<String>,
    },
    Export {
        #[arg(long)]
        id: String,
        #[arg(long = "to", value_enum)]
        to_kind: SurfaceArg,
    },
    Transcode {
        input_file: String,
        #[arg(long = "to", value_enum)]
        to_kind: SurfaceArg,
        #[arg(long = "as", value_enum)]
        as_kind: Option<SurfaceArg>,
    },
    NormalizeSurface {
        file: String,
        #[arg(long = "as", value_enum)]
        as_kind: Option<SurfaceArg>,
    },
    RoundtripCheck {
        file: String,
        #[arg(long = "as", value_enum)]
        as_kind: Option<SurfaceArg>,
    },
    ExpandQk0 {
        file: String,
    },
    SurfaceCapabilities,
    DumpTransformReceipt {
        id: String,
    },
    ReplayReport {
        report_id: String,
    },
    CacheStats,
    ClearCache {
        #[arg(long)]
        scope: Option<String>,
    },
    ExplainInvalidation {
        report_id: String,
        #[arg(long, default_value = "seed")]
        bundle_hash: String,
        #[arg(long, default_value = "default")]
        policy_hash: String,
    },
    DumpBundleGraph {
        bundle_id: String,
    },
    DumpOverlayWorld {
        bundle_id: String,
    },
    SelectRoute {
        #[arg(long)]
        src: String,
        #[arg(long)]
        tgt: String,
        #[arg(long)]
        proof_target: Option<String>,
        #[arg(long, default_value_t = 8)]
        max_loss: usize,
        #[arg(long, default_value_t = true)]
        allow_lossy_supported: bool,
        #[arg(long, default_value_t = false)]
        require_proof: bool,
    },
    Certify {
        #[arg(long)]
        campaign: Option<String>,
        #[arg(long)]
        theorem: Option<String>,
        #[arg(long)]
        target_profile: Option<String>,
        #[arg(long)]
        file: Option<String>,
        #[arg(long = "as", value_enum)]
        as_kind: Option<SurfaceArg>,
    },
    CertifyDerived {
        #[arg(long)]
        campaign: Option<String>,
        #[arg(long)]
        theorem: Option<String>,
        #[arg(long)]
        target_profile: Option<String>,
        #[arg(long)]
        file: Option<String>,
        #[arg(long)]
        bundle: Option<String>,
        #[arg(long, default_value_t = false)]
        overlay_only: bool,
        #[arg(long, value_enum, default_value = "conservative")]
        optimizer_policy: OptimizerPolicyArg,
        #[arg(long)]
        surface_policy: Option<String>,
        #[arg(long, default_value_t = false)]
        strict_surface: bool,
        #[arg(long, default_value_t = false)]
        strict_derived: bool,
        #[arg(long, default_value_t = false)]
        replay_only: bool,
        #[arg(long, default_value_t = false)]
        no_cache: bool,
        #[arg(long)]
        evaluator_policy: Option<String>,
        #[arg(long)]
        cache_policy: Option<String>,
        #[arg(long, default_value_t = false)]
        strict_policy: bool,
        #[arg(long = "report-surface", value_enum)]
        report_surface: Option<SurfaceArg>,
        #[arg(long = "as", value_enum)]
        as_kind: Option<SurfaceArg>,
    },
    RunBundle {
        #[arg(long)]
        file: String,
        #[arg(long = "as", value_enum)]
        as_kind: Option<SurfaceArg>,
        #[arg(long, default_value_t = false)]
        overlay_only: bool,
        #[arg(long, default_value_t = false)]
        explain_route: bool,
        #[arg(long, value_enum, default_value = "conservative")]
        optimizer_policy: OptimizerPolicyArg,
        #[arg(long = "report-surface", value_enum)]
        report_surface: Option<SurfaceArg>,
        #[arg(long, default_value_t = false)]
        replay_only: bool,
        #[arg(long, default_value_t = false)]
        no_cache: bool,
        #[arg(long)]
        evaluator_policy: Option<String>,
        #[arg(long)]
        cache_policy: Option<String>,
        #[arg(long, default_value_t = false)]
        strict_policy: bool,
        #[arg(long, value_enum, default_value = "reject")]
        conflict_policy: BundlePolicyArg,
    },
    CertifyBundle {
        #[arg(long)]
        file: String,
        #[arg(long = "as", value_enum)]
        as_kind: Option<SurfaceArg>,
        #[arg(long, default_value_t = false)]
        overlay_only: bool,
        #[arg(long, default_value_t = false)]
        explain_route: bool,
        #[arg(long, value_enum, default_value = "conservative")]
        optimizer_policy: OptimizerPolicyArg,
        #[arg(long = "report-surface", value_enum)]
        report_surface: Option<SurfaceArg>,
        #[arg(long, default_value_t = false)]
        strict_derived: bool,
        #[arg(long, default_value_t = false)]
        replay_only: bool,
        #[arg(long, default_value_t = false)]
        no_cache: bool,
        #[arg(long)]
        evaluator_policy: Option<String>,
        #[arg(long)]
        cache_policy: Option<String>,
        #[arg(long, default_value_t = false)]
        strict_policy: bool,
        #[arg(long, value_enum, default_value = "reject")]
        conflict_policy: BundlePolicyArg,
        #[arg(long, default_value_t = false)]
        strict_surface: bool,
    },
    ExportReport {
        #[arg(long)]
        id: String,
        #[arg(long = "to", value_enum)]
        to_kind: SurfaceArg,
    },
    ExportValidationBundle {
        #[arg(long)]
        id: String,
        #[arg(long = "to", value_enum)]
        to_kind: SurfaceArg,
    },
    ExportBundleReport {
        #[arg(long)]
        bundle: String,
        #[arg(long = "to", value_enum)]
        to_kind: SurfaceArg,
    },
    CompileAtlas,
    Canonize {
        object_id: String,
    },
    ExecHost {
        #[arg(long)]
        regime: String,
        #[arg(long)]
        object: Option<String>,
    },
    RunTheorem {
        #[arg(long)]
        id: Option<String>,
        #[arg(long)]
        file: Option<String>,
        #[arg(long)]
        bundle: Option<String>,
        #[arg(long, default_value_t = false)]
        overlay_only: bool,
        #[arg(long, default_value_t = false)]
        explain_route: bool,
        #[arg(long, value_enum, default_value = "conservative")]
        optimizer_policy: OptimizerPolicyArg,
        #[arg(long)]
        surface_policy: Option<String>,
        #[arg(long, default_value_t = false)]
        strict_surface: bool,
        #[arg(long, default_value_t = false)]
        replay_only: bool,
        #[arg(long, default_value_t = false)]
        no_cache: bool,
        #[arg(long = "report-surface", value_enum)]
        report_surface: Option<SurfaceArg>,
        #[arg(long = "as", value_enum)]
        as_kind: Option<SurfaceArg>,
    },
    DumpCanonical {
        id: String,
    },
    ExportLocusPacket {
        #[arg(long)]
        report_id: String,
        #[arg(long)]
        out: Option<String>,
    },
    ImportLocusPacket {
        file: String,
    },
    NormalizeRna {
        file: String,
    },
    CompileRna {
        file: String,
        #[arg(long)]
        out: Option<String>,
        #[arg(long, default_value = "gene")]
        artifact_class: String,
        #[arg(long, default_value_t = false)]
        persist_lineage: bool,
    },
    SequenceDna {
        file: String,
    },
    DeriveFrontier {
        #[arg(long)]
        report_id: String,
    },
    TowerStep {
        #[arg(long)]
        report_id: String,
    },
    DispatchCoverage {
        #[arg(long)]
        report_id: String,
    },
    DeriveDistress {
        #[arg(long)]
        report_id: String,
    },
    ResearchImport {
        #[arg(long, value_enum)]
        kind: ResearchKindArg,
        file: String,
    },
    ResearchExport {
        #[arg(long, value_enum)]
        kind: ResearchKindArg,
        id: String,
    },
    ResearchList {
        #[arg(long, value_enum)]
        kind: ResearchKindArg,
    },
    ResearchRoute {
        #[arg(long)]
        task_id: String,
        #[arg(long)]
        signature_id: String,
    },
    ResearchDeriveFromReport {
        #[arg(long)]
        report_id: String,
        #[arg(long, default_value_t = false)]
        persist: bool,
    },
    ResearchGovernReport {
        #[arg(long)]
        report_id: String,
        #[arg(long, default_value_t = false)]
        persist: bool,
    },
    ResearchSeedExportRemediation {
        #[arg(long, default_value_t = false)]
        persist: bool,
    },
    ResearchRemediationSummary,
    ResearchStatus,
    ResearchPromotionReadiness {
        report_id: String,
    },
}

fn main() -> Result<()> {
    std::thread::Builder::new()
        .name("l64-cli-main".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(real_main)
        .map_err(|err| anyhow!("failed to start l64-cli main thread: {err}"))?
        .join()
        .map_err(|_| anyhow!("l64-cli main thread panicked"))?
}

fn real_main() -> Result<()> {
    let cli = Cli::parse();
    let registry = SeedRegistry::load().context("failed to load seed registry")?;
    let kernel = ConstitutionKernel;

    match cli.command {
        Command::Parse { file, as_kind } => {
            let (document, receipt) = load_document(&file, as_kind, &registry)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "document": describe_document(&document),
                    "receipt": receipt
                }))?
            );
        }
        Command::Normalize { file, as_kind } => {
            let (text, receipt) = normalize_file(&file, as_kind, &registry)?;
            println!("{text}");
            eprintln!("{}", serde_json::to_string(&receipt)?);
        }
        Command::Validate { file, as_kind } => {
            let (document, _) = load_document(&file, as_kind, &registry)?;
            let overlay = overlay_registry_from_document(registry.clone(), &document);
            validate_document(&kernel, &overlay, &document)
                .with_context(|| "validation failed; if this surface came from export-report, use export-validation-bundle for a self-contained validation/replay artifact")?;
            println!("validation passed");
        }
        Command::Import { file, as_kind } => {
            let (artifact, receipt) =
                import_file(Path::new(&file), as_kind.map(Into::into), &registry)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "header": artifact.header,
                    "document": describe_document(&artifact.document),
                    "receipt": receipt
                }))?
            );
        }
        Command::ImportBundle {
            file,
            as_kind,
            conflict_policy,
            namespace,
        } => {
            let world = import_bundle_file(
                Path::new(&file),
                as_kind.map(Into::into),
                conflict_policy.into(),
                namespace.as_deref(),
            )?;
            println!("{}", serde_json::to_string_pretty(&world.manifest)?);
        }
        Command::Export { id, to_kind } => {
            let document = document_for_id(&registry, &id)?;
            let policy = default_policy_for(to_kind.clone().into(), &registry)?;
            let (rendered, receipt) =
                export_document(&document, to_kind.into(), &policy, &registry)?;
            println!("{rendered}");
            eprintln!("{}", serde_json::to_string(&receipt)?);
            eprintln!(
                "{}",
                serde_json::to_string(
                    &serde_json::json!({"artifact_class":"InspectionReport","standalone_validation_complete":false,"hint":"use export-validation-bundle for self-contained validation"})
                )?
            );
        }
        Command::Transcode {
            input_file,
            to_kind,
            as_kind,
        } => {
            let source_kind = infer_surface_kind(&input_file, as_kind)?;
            let input = fs::read_to_string(&input_file)
                .with_context(|| format!("failed to read `{input_file}`"))?;
            let (rendered, receipt) =
                transcode_text(&input, source_kind, to_kind.into(), &registry)?;
            println!("{rendered}");
            eprintln!("{}", serde_json::to_string(&receipt)?);
        }
        Command::NormalizeSurface { file, as_kind } => {
            let (text, receipt) = normalize_file(&file, as_kind, &registry)?;
            println!("{text}");
            eprintln!("{}", serde_json::to_string(&receipt)?);
        }
        Command::RoundtripCheck { file, as_kind } => {
            let source_kind = infer_surface_kind(&file, as_kind)?;
            let input =
                fs::read_to_string(&file).with_context(|| format!("failed to read `{file}`"))?;
            let (report, receipts) = roundtrip_check(&input, source_kind, &registry)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "report": report,
                    "receipts": receipts
                }))?
            );
        }
        Command::ExpandQk0 { file } => {
            let expanded = expand_qk0_file(Path::new(&file), &registry)?;
            println!("{expanded}");
        }
        Command::SurfaceCapabilities => {
            println!(
                "{}",
                serde_json::to_string_pretty(&surface_capabilities(&registry))?
            );
        }
        Command::DumpTransformReceipt { id } => {
            let receipt = dump_transform_receipt(&id)?;
            println!("{}", serde_json::to_string_pretty(&receipt)?);
        }
        Command::ReplayReport { report_id } => {
            let report = cert_replay_report(&report_id).map_err(anyhow::Error::msg)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::CacheStats => {
            let stats = cert_cache_stats().map_err(anyhow::Error::msg)?;
            println!("{}", serde_json::to_string_pretty(&stats)?);
        }
        Command::ClearCache { scope } => {
            clear_cert_cache(scope.as_deref()).map_err(anyhow::Error::msg)?;
            println!("cache cleared");
        }
        Command::ExplainInvalidation {
            report_id,
            bundle_hash,
            policy_hash,
        } => {
            let explanation = cert_explain_invalidation(&report_id, &bundle_hash, &policy_hash)
                .map_err(anyhow::Error::msg)?;
            println!("{}", serde_json::to_string_pretty(&explanation)?);
        }
        Command::DumpBundleGraph { bundle_id } => {
            let world = load_bundle_world(&bundle_id)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&world.manifest.dependencies)?
            );
        }
        Command::DumpOverlayWorld { bundle_id } => {
            let world = load_bundle_world(&bundle_id)?;
            println!("{}", serde_json::to_string_pretty(&world.manifest.overlay)?);
        }
        Command::SelectRoute {
            src,
            tgt,
            proof_target,
            max_loss,
            allow_lossy_supported,
            require_proof,
        } => {
            let selector = AtlasSelector::new(registry.clone());
            let selection = selector
                .select_route(
                    &src,
                    &tgt,
                    proof_target.as_deref(),
                    Some(&Budget {
                        max_loss,
                        allow_lossy_supported,
                        require_proof,
                    }),
                )
                .map_err(anyhow::Error::msg)?;
            println!("{}", serde_json::to_string_pretty(&selection)?);
        }
        Command::Certify {
            campaign,
            theorem,
            target_profile,
            file,
            as_kind,
        } => {
            let certifier = CampaignCertifier::new(registry.clone());
            let (campaign, theorem, target_profile) = resolve_surface_cert_args(
                &registry,
                campaign,
                theorem,
                target_profile,
                file,
                as_kind,
            )?;
            let report = match (campaign.as_deref(), theorem.as_deref(), target_profile.as_deref()) {
                (Some(campaign_id), _, _) => certifier.certify_campaign(campaign_id),
                (None, Some(theorem_id), Some(target_id)) => {
                    certifier.certify_theorem_with_target(theorem_id, target_id, None)
                }
                _ => Err("use either --campaign <id>, --file <surface>, or --theorem <id> --target-profile <id>".into()),
            }
            .map_err(anyhow::Error::msg)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::CertifyDerived {
            campaign,
            theorem,
            target_profile,
            file,
            bundle,
            overlay_only,
            optimizer_policy,
            surface_policy: _surface_policy,
            strict_surface,
            strict_derived,
            replay_only,
            no_cache,
            evaluator_policy,
            cache_policy,
            strict_policy,
            report_surface,
            as_kind,
        } => {
            let options = build_cert_options(
                optimizer_policy.into(),
                file.as_deref(),
                bundle.as_deref(),
                replay_only,
                no_cache,
                strict_derived,
                evaluator_policy,
                cache_policy,
                strict_policy,
            )?;
            let report = if let Some(bundle_id) = bundle {
                let world = load_bundle_world(&bundle_id)?;
                certify_with_registry(
                    &world.overlay,
                    campaign.or_else(|| {
                        world
                            .overlay
                            .local
                            .campaigns
                            .first()
                            .map(|item| item.id.clone())
                    }),
                    theorem.or_else(|| {
                        world
                            .overlay
                            .local
                            .theorem_specs
                            .first()
                            .map(|item| item.id.clone())
                    }),
                    target_profile.or_else(|| {
                        world
                            .overlay
                            .local
                            .target_profiles
                            .first()
                            .map(|item| item.id.clone())
                    }),
                    None,
                    overlay_only,
                    &options,
                )?
            } else if let Some(file) = file {
                let world = import_bundle_file(
                    Path::new(&file),
                    as_kind.map(Into::into),
                    BundleConflictPolicy::ExactMatch,
                    None,
                )?;
                let mut file_options = options.clone();
                file_options.bundle_id = Some(world.manifest.id.clone());
                certify_with_registry(
                    &world.overlay,
                    campaign.or_else(|| {
                        world
                            .overlay
                            .local
                            .campaigns
                            .first()
                            .map(|item| item.id.clone())
                    }),
                    theorem.or_else(|| {
                        world
                            .overlay
                            .local
                            .theorem_specs
                            .first()
                            .map(|item| item.id.clone())
                    }),
                    target_profile.or_else(|| {
                        world
                            .overlay
                            .local
                            .target_profiles
                            .first()
                            .map(|item| item.id.clone())
                    }),
                    None,
                    overlay_only,
                    &file_options,
                )?
            } else {
                certify_with_registry(
                    &registry,
                    campaign,
                    theorem,
                    target_profile,
                    None,
                    false,
                    &options,
                )?
            };
            if strict_surface && !report.diagnostics.is_empty() {
                return Err(anyhow!(
                    "strict-surface rejected certification due to diagnostics"
                ));
            }
            persist_report_document(&report)?;
            if let Some(surface) = report_surface {
                export_report_sidecar(&report, surface.into(), &registry)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::RunBundle {
            file,
            as_kind,
            overlay_only,
            explain_route,
            optimizer_policy,
            report_surface,
            replay_only,
            no_cache,
            evaluator_policy,
            cache_policy,
            strict_policy,
            conflict_policy,
        } => {
            let world = import_bundle_file(
                Path::new(&file),
                as_kind.map(Into::into),
                conflict_policy.into(),
                None,
            )?;
            let mut options = build_cert_options(
                optimizer_policy.into(),
                Some(&file),
                None,
                replay_only,
                no_cache,
                false,
                evaluator_policy,
                cache_policy,
                strict_policy,
            )?;
            options.bundle_id = Some(world.manifest.id.clone());
            let result = run_bundle_world(&world, overlay_only, &options)?;
            if explain_route {
                eprintln!("route explanation requested for bundle execution");
            }
            if let Some(surface) = report_surface {
                for value in &result {
                    if let Some(report) = value.get("theorem_id").and_then(|_| {
                        serde_json::from_value::<l64_core::CertificationReport>(value.clone()).ok()
                    }) {
                        export_report_sidecar(&report, surface.clone().into(), &registry)?;
                    }
                }
            }
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Command::CertifyBundle {
            file,
            as_kind,
            overlay_only,
            explain_route,
            optimizer_policy,
            report_surface,
            strict_derived,
            replay_only,
            no_cache,
            evaluator_policy,
            cache_policy,
            strict_policy,
            conflict_policy,
            strict_surface,
        } => {
            let world = import_bundle_file(
                Path::new(&file),
                as_kind.map(Into::into),
                conflict_policy.into(),
                None,
            )?;
            let mut options = build_cert_options(
                optimizer_policy.into(),
                Some(&file),
                None,
                replay_only,
                no_cache,
                strict_derived,
                evaluator_policy,
                cache_policy,
                strict_policy,
            )?;
            options.bundle_id = Some(world.manifest.id.clone());
            let reports = certify_bundle_world(&world, overlay_only, &options)?;
            if strict_surface && reports.iter().any(|report| !report.diagnostics.is_empty()) {
                return Err(anyhow!(
                    "strict-surface rejected certify-bundle due to diagnostics"
                ));
            }
            for report in &reports {
                persist_report_document(report)?;
                if let Some(surface) = report_surface.clone() {
                    export_report_sidecar(report, surface.into(), &registry)?;
                }
            }
            if explain_route {
                eprintln!("route explanation requested for certify-bundle");
            }
            println!("{}", serde_json::to_string_pretty(&reports)?);
        }
        Command::ExportReport { id, to_kind } => {
            let registry = SeedRegistry::load()?;
            let document = load_report_document_for_export(&id, &registry)?;
            let policy = default_policy_for(to_kind.clone().into(), &registry)?;
            let (rendered, receipt) =
                export_document(&document, to_kind.into(), &policy, &registry)?;
            println!("{rendered}");
            eprintln!("{}", serde_json::to_string(&receipt)?);
            eprintln!(
                "{}",
                serde_json::to_string(&serde_json::json!({
                    "artifact_class": "InspectionReport",
                    "standalone_validation_complete": false,
                    "hint": "use export-validation-bundle for self-contained validation"
                }))?
            );
        }
        Command::ExportValidationBundle { id, to_kind } => {
            let registry = SeedRegistry::load()?;
            let report = cert_replay_report(&id).map_err(anyhow::Error::msg)?;
            let document = if let Some(bundle_id) = report
                .execution_envelope
                .as_ref()
                .and_then(|item| item.bundle_id.clone())
            {
                if let Ok(world) = load_bundle_world(&bundle_id) {
                    report_to_validation_bundle_with_registry(&report, &world.overlay)?
                } else {
                    report_to_validation_bundle_with_registry(&report, &registry)?
                }
            } else {
                report_to_validation_bundle_with_registry(&report, &registry)?
            };
            let policy = default_policy_for(to_kind.clone().into(), &registry)?;
            let (rendered, receipt) =
                export_document(&document, to_kind.into(), &policy, &registry)?;
            println!("{rendered}");
            eprintln!("{}", serde_json::to_string(&receipt)?);
        }
        Command::ExportBundleReport { bundle, to_kind } => {
            let world = load_bundle_world(&bundle)?;
            let report_id = world
                .overlay
                .local
                .campaigns
                .first()
                .map(|item| format!("REPORT_{}_{}", item.theorem, item.id))
                .or_else(|| {
                    world
                        .overlay
                        .local
                        .theorem_specs
                        .first()
                        .map(|item| format!("REPORT_{}_THEOREM", item.id))
                })
                .ok_or_else(|| anyhow!("bundle has no reportable theorem/campaign"))?;
            let registry = SeedRegistry::load()?;
            let document = load_report_document_for_export(&report_id, &registry)?;
            let policy = default_policy_for(to_kind.clone().into(), &registry)?;
            let (rendered, receipt) =
                export_document(&document, to_kind.into(), &policy, &registry)?;
            println!("{rendered}");
            eprintln!("{}", serde_json::to_string(&receipt)?);
        }
        Command::CompileAtlas => {
            let atlas = CompiledAtlas::compile(&registry).map_err(anyhow::Error::msg)?;
            let summary = atlas.compile_summary();
            let encoded = bincode::serialize(&summary)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "summary": summary,
                    "binary_cache_bytes": encoded.len()
                }))?
            );
        }
        Command::Canonize { object_id } => {
            let mut canon = CanonEngine::compile(&registry);
            let runtime = RuntimeWorld::compile_seed(&registry);
            let object = runtime
                .lookup_object(&object_id)
                .ok_or_else(|| anyhow!("unknown runtime object `{object_id}`"))?;
            let canonical = canon.canonicalize(object).map_err(anyhow::Error::msg)?;
            let eq_class = canon.eq_class(object).map_err(anyhow::Error::msg)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "object_id": object_id,
                    "runtime_object_id": object.0,
                    "canonical_id": canonical,
                    "eq_class_id": eq_class
                }))?
            );
        }
        Command::ExecHost { regime, object } => {
            let result = exec_host(&regime, object.as_deref()).map_err(anyhow::Error::msg)?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Command::RunTheorem {
            id,
            file,
            bundle,
            overlay_only,
            explain_route,
            optimizer_policy,
            surface_policy: _surface_policy,
            strict_surface,
            replay_only,
            no_cache,
            report_surface,
            as_kind,
        } => {
            let options = build_cert_options(
                optimizer_policy.into(),
                file.as_deref(),
                bundle.as_deref(),
                replay_only,
                no_cache,
                false,
                None,
                None,
                false,
            )?;
            let result = if let Some(bundle_id) = bundle {
                let world = load_bundle_world(&bundle_id)?;
                let theorem_id = id.unwrap_or_else(|| {
                    world
                        .overlay
                        .local
                        .theorem_specs
                        .first()
                        .map(|item| item.id.clone())
                        .unwrap_or_default()
                });
                if overlay_only {
                    let local = world.overlay.local_only();
                    let atlas = CompiledAtlas::compile(&local).map_err(anyhow::Error::msg)?;
                    run_seed_theorem(&local, &theorem_id, &atlas).map_err(anyhow::Error::msg)?
                } else {
                    let atlas =
                        CompiledAtlas::compile(&world.overlay).map_err(anyhow::Error::msg)?;
                    run_seed_theorem(&world.overlay, &theorem_id, &atlas)
                        .map_err(anyhow::Error::msg)?
                }
            } else if let Some(file) = file {
                let world = import_bundle_file(
                    Path::new(&file),
                    as_kind.map(Into::into),
                    BundleConflictPolicy::ExactMatch,
                    None,
                )?;
                let theorem_id = id.unwrap_or_else(|| {
                    world
                        .overlay
                        .local
                        .theorem_specs
                        .first()
                        .map(|item| item.id.clone())
                        .unwrap_or_default()
                });
                if overlay_only {
                    let local = world.overlay.local_only();
                    let atlas = CompiledAtlas::compile(&local).map_err(anyhow::Error::msg)?;
                    run_seed_theorem(&local, &theorem_id, &atlas).map_err(anyhow::Error::msg)?
                } else {
                    let atlas =
                        CompiledAtlas::compile(&world.overlay).map_err(anyhow::Error::msg)?;
                    run_seed_theorem(&world.overlay, &theorem_id, &atlas)
                        .map_err(anyhow::Error::msg)?
                }
            } else {
                let theorem_id = id.ok_or_else(|| {
                    anyhow!(
                        "use either --id <theorem-id>, --file <surface>, or --bundle <bundle-id>"
                    )
                })?;
                let atlas = CompiledAtlas::compile(&registry).map_err(anyhow::Error::msg)?;
                run_seed_theorem(&registry, &theorem_id, &atlas).map_err(anyhow::Error::msg)?
            };
            if strict_surface && result.selected_route.is_empty() {
                return Err(anyhow!("strict-surface rejected empty surfaced route"));
            }
            if explain_route {
                eprintln!("optimizer policy: {:?}", options.optimizer_policy);
            }
            if let Some(surface) = report_surface {
                let report_id = format!("REPORT_{}_THEOREM", result.theorem_id);
                let theorem_report = l64_core::CertificationReport {
                    theorem_id: result.theorem_id.clone(),
                    campaign_id: None,
                    target_profile_id: "THEOREM_RUN".into(),
                    verdict: result.verdict.clone(),
                    selected_atlas_cell: None,
                    selected_path: result.selected_route.clone(),
                    route_class_id: None,
                    certificate_id: None,
                    candidates: Vec::new(),
                    obligations: Vec::new(),
                    reasons: vec!["theorem execution export".into()],
                    diagnostics: Vec::new(),
                    deficiencies: Vec::new(),
                    adequacy_records: Vec::new(),
                    checker_receipts: Vec::new(),
                    burden_pack_ids: Vec::new(),
                    claim_packet_ids: Vec::new(),
                    evidence_contract_ids: Vec::new(),
                    benchmark_receipt_ids: Vec::new(),
                    challenge_receipt_ids: Vec::new(),
                    reproducibility_packet_ids: Vec::new(),
                    promotion_artifact_ids: Vec::new(),
                    reused_artifact_ids: Vec::new(),
                    default_selected_artifact_ids: Vec::new(),
                    payoff_receipt_ids: Vec::new(),
                    policy_resolution: None,
                    route_explanation: None,
                    execution_envelope: Some(l64_core::DeterministicExecutionEnvelope {
                        bundle_hash: options.bundle_hash.clone(),
                        bundle_id: options.bundle_id.clone(),
                        policy_hash: options.policy_hash.clone(),
                        policy_resolution_id: None,
                        manifest_id: None,
                        lock_id: None,
                        route_winner_hash: fxhash(&result.selected_route.join("->")).to_string(),
                        obligation_replay_keys: Vec::new(),
                        report_hash: fxhash(&report_id).to_string(),
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
                };
                persist_report_document(&theorem_report)?;
                export_report_sidecar(&theorem_report, surface.into(), &registry)?;
            }
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Command::ExportLocusPacket { report_id, out } => {
            let report = cert_replay_report(&report_id).map_err(anyhow::Error::msg)?;
            let bytes = encode_locus_packet_for_report(&report).map_err(anyhow::Error::msg)?;
            if let Some(path) = out {
                fs::write(&path, &bytes).with_context(|| format!("failed to write `{path}`"))?;
                let packet = decode_locus_packet(&bytes).map_err(anyhow::Error::msg)?;
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "report_id": report_id,
                        "bytes": bytes.len(),
                        "summary": locus_packet_summary(&packet)
                    }))?
                );
            } else {
                println!("{}", hex::encode(bytes));
            }
        }
        Command::ImportLocusPacket { file } => {
            let bytes = fs::read(&file).with_context(|| format!("failed to read `{file}`"))?;
            let summary = decode_locus_packet_summary(&bytes).map_err(anyhow::Error::msg)?;
            println!("{}", serde_json::to_string_pretty(&summary)?);
        }
        Command::NormalizeRna { file } => {
            let input =
                fs::read_to_string(&file).with_context(|| format!("failed to read `{file}`"))?;
            let (normalized, receipt) =
                l64_core::normalize_rna(&input).map_err(anyhow::Error::msg)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "normalized": normalized,
                    "receipt": receipt
                }))?
            );
        }
        Command::CompileRna {
            file,
            out,
            artifact_class,
            persist_lineage,
        } => {
            let input =
                fs::read_to_string(&file).with_context(|| format!("failed to read `{file}`"))?;
            let artifact_class = parse_genome_artifact_class(&artifact_class)?;
            let subject_id = Path::new(&file)
                .file_stem()
                .and_then(|item| item.to_str())
                .unwrap_or("RNA_ARTIFACT");
            let (bytes, artifact) =
                compile_rna_to_dna_packet(subject_id, &input, artifact_class, vec!["core".into()])
                    .map_err(anyhow::Error::msg)?;
            if let Some(path) = out {
                fs::write(&path, &bytes).with_context(|| format!("failed to write `{path}`"))?;
            }
            let packet = decode_locus_packet(&bytes).map_err(anyhow::Error::msg)?;
            let lineage_id = format!("LIN_{}", subject_id);
            let lineage = ResearchLineageRecord {
                id: lineage_id.clone(),
                subject_id: subject_id.into(),
                artifact_class,
                source_surface: GenomeSurface::Rna,
                target_surface: GenomeSurface::Dna,
                grammar_id: packet.header.grammar_id.clone(),
                canonical_hash: artifact.lowering_receipt.canonical_hash.clone(),
                lowering_receipt_id: artifact.lowering_receipt.id.clone(),
                phase_ids: artifact
                    .phase_ledger
                    .iter()
                    .map(|entry| entry.phase_id)
                    .collect(),
                phase_ledger: artifact.phase_ledger.clone(),
                notes: vec![
                    format!("root_subject_id={}", packet.header.root_subject_id),
                    format!("integrity_hash={}", packet.header.integrity_hash),
                ],
            };
            if persist_lineage {
                persist_lineage_record(&lineage)?;
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "bytes": bytes.len(),
                    "packet_summary": locus_packet_summary(&packet),
                    "artifact": artifact,
                    "lineage": lineage,
                    "persisted_lineage": persist_lineage
                }))?
            );
        }
        Command::SequenceDna { file } => {
            let bytes = fs::read(&file).with_context(|| format!("failed to read `{file}`"))?;
            let artifact = sequence_dna_to_rna(&bytes).map_err(anyhow::Error::msg)?;
            println!("{}", serde_json::to_string_pretty(&artifact)?);
        }
        Command::DeriveFrontier { report_id } => {
            let report = cert_replay_report(&report_id).map_err(anyhow::Error::msg)?;
            let bundle = build_vertical_compounding_bundle(&registry, &report);
            println!("{}", serde_json::to_string_pretty(&bundle)?);
        }
        Command::TowerStep { report_id } => {
            let report = cert_replay_report(&report_id).map_err(anyhow::Error::msg)?;
            let bundle = build_vertical_compounding_bundle(&registry, &report);
            let bytes = encode_locus_packet_for_report(&report).map_err(anyhow::Error::msg)?;
            let summary = decode_locus_packet_summary(&bytes).map_err(anyhow::Error::msg)?;
            let coverage = dispatch_proof_coverage(&report);
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "frontier": bundle.frontier_ledger,
                    "generated_counts": {
                        "semantic_leaves": bundle.semantic_leaves.len(),
                        "adequacy_clauses": bundle.adequacy_clauses.len(),
                        "checker_extensions": bundle.checker_extensions.len(),
                        "campaigns": bundle.campaigns.len(),
                        "atlas_cells": bundle.atlas_cells.len(),
                        "payoff_tasks": bundle.payoff_tasks.len(),
                        "compartments": bundle.compartments.len(),
                        "help_requests": bundle.help_requests.len(),
                        "recipes": bundle.recipes.len(),
                        "promotion_candidates": bundle.promotion_candidates.len()
                    },
                    "distress": bundle.distress,
                    "help_requests": bundle.help_requests,
                    "calibration_pressure": bundle.calibration_pressure,
                    "coverage_dispatch": coverage,
                    "packet_summary": summary
                }))?
            );
        }
        Command::DispatchCoverage { report_id } => {
            let report = cert_replay_report(&report_id).map_err(anyhow::Error::msg)?;
            let dispatch = dispatch_proof_coverage(&report);
            println!("{}", serde_json::to_string_pretty(&dispatch)?);
        }
        Command::DeriveDistress { report_id } => {
            let report = cert_replay_report(&report_id).map_err(anyhow::Error::msg)?;
            let bundle = build_vertical_compounding_bundle(&registry, &report);
            let distress = derive_distress_vector(&report, &bundle);
            let help_request = derive_help_request(&report, &distress);
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "distress": distress,
                    "help_request": help_request,
                    "frontier": bundle.frontier_ledger
                }))?
            );
        }
        Command::ResearchImport { kind, file } => {
            let id = persist_research_import(kind.as_str(), Path::new(&file))?;
            println!(
                "{}",
                serde_json::to_string_pretty(
                    &serde_json::json!({"kind": kind.as_str(), "id": id, "file": file})
                )?
            );
        }
        Command::ResearchExport { kind, id } => match kind {
            ResearchKindArg::Task => println!(
                "{}",
                serde_json::to_string_pretty(&load_task_envelope(&id)?)?
            ),
            ResearchKindArg::Signature => println!(
                "{}",
                serde_json::to_string_pretty(&load_derivation_signature(&id)?)?
            ),
            ResearchKindArg::Review => println!(
                "{}",
                serde_json::to_string_pretty(&load_review_receipt(&id)?)?
            ),
            ResearchKindArg::Challenge => println!(
                "{}",
                serde_json::to_string_pretty(&load_challenge_record(&id)?)?
            ),
            ResearchKindArg::Registry => println!(
                "{}",
                serde_json::to_string_pretty(&load_framework_registry_entry(&id)?)?
            ),
            ResearchKindArg::Operator => println!(
                "{}",
                serde_json::to_string_pretty(&load_operator_record(&id)?)?
            ),
            ResearchKindArg::Benchmark => println!(
                "{}",
                serde_json::to_string_pretty(&load_benchmark_schema(&id)?)?
            ),
            ResearchKindArg::BenchmarkRun => println!(
                "{}",
                serde_json::to_string_pretty(&load_benchmark_run_record(&id)?)?
            ),
            ResearchKindArg::Claim => println!(
                "{}",
                serde_json::to_string_pretty(&load_math_claim_packet(&id)?)?
            ),
            ResearchKindArg::Reduction => println!(
                "{}",
                serde_json::to_string_pretty(&load_reduction_map_record(&id)?)?
            ),
            ResearchKindArg::Projection => println!(
                "{}",
                serde_json::to_string_pretty(&load_projection_map_record(&id)?)?
            ),
            ResearchKindArg::Handoff => println!(
                "{}",
                serde_json::to_string_pretty(&load_handoff_packet(&id)?)?
            ),
            ResearchKindArg::ProducerHost => println!(
                "{}",
                serde_json::to_string_pretty(&load_producer_host_spec(&id)?)?
            ),
            ResearchKindArg::Promotion => println!(
                "{}",
                serde_json::to_string_pretty(&load_promotion_queue_entry(&id)?)?
            ),
            ResearchKindArg::PromotionReport => println!(
                "{}",
                serde_json::to_string_pretty(&load_promotion_readiness_report(&id)?)?
            ),
            ResearchKindArg::RouteAssignment => println!(
                "{}",
                serde_json::to_string_pretty(&load_route_assignment(&id)?)?
            ),
            ResearchKindArg::Strengthening => println!(
                "{}",
                serde_json::to_string_pretty(&load_strengthening_artifact(&id)?)?
            ),
            ResearchKindArg::Repro => println!(
                "{}",
                serde_json::to_string_pretty(&load_reproducibility_packet(&id)?)?
            ),
            ResearchKindArg::Remediation => println!(
                "{}",
                serde_json::to_string_pretty(&load_remediation_entry(&id)?)?
            ),
            ResearchKindArg::Coverage => println!(
                "{}",
                serde_json::to_string_pretty(&load_coverage_dispatch(&id)?)?
            ),
            ResearchKindArg::Tower => println!(
                "{}",
                serde_json::to_string_pretty(&load_vertical_bundle(&id)?)?
            ),
            ResearchKindArg::Lineage => println!(
                "{}",
                serde_json::to_string_pretty(&load_lineage_record(&id)?)?
            ),
        },
        Command::ResearchList { kind } => {
            let ids = match kind {
                ResearchKindArg::Task => list_task_envelopes()?,
                ResearchKindArg::Signature => list_derivation_signatures()?,
                ResearchKindArg::Review => list_review_receipts()?,
                ResearchKindArg::Challenge => list_challenge_records()?,
                ResearchKindArg::Registry => list_framework_registry_entries()?,
                ResearchKindArg::Operator => list_operator_records()?,
                ResearchKindArg::Benchmark => list_benchmark_schemas()?,
                ResearchKindArg::BenchmarkRun => list_benchmark_run_records()?,
                ResearchKindArg::Claim => list_math_claim_packets()?,
                ResearchKindArg::Reduction => list_reduction_map_records()?,
                ResearchKindArg::Projection => list_projection_map_records()?,
                ResearchKindArg::Handoff => list_handoff_packets()?,
                ResearchKindArg::ProducerHost => list_producer_host_specs()?,
                ResearchKindArg::Promotion => list_promotion_queue_entries()?,
                ResearchKindArg::PromotionReport => list_promotion_readiness_reports()?,
                ResearchKindArg::RouteAssignment => list_route_assignments()?,
                ResearchKindArg::Strengthening => list_strengthening_artifacts()?,
                ResearchKindArg::Repro => list_reproducibility_packets()?,
                ResearchKindArg::Remediation => list_remediation_entries()?,
                ResearchKindArg::Coverage => list_coverage_dispatches()?,
                ResearchKindArg::Tower => list_vertical_bundles()?,
                ResearchKindArg::Lineage => list_lineage_records()?,
            };
            println!(
                "{}",
                serde_json::to_string_pretty(
                    &serde_json::json!({"kind": kind.as_str(), "ids": ids})
                )?
            );
        }
        Command::ResearchRoute {
            task_id,
            signature_id,
        } => {
            let task = load_task_envelope(&task_id)?;
            let signature = load_derivation_signature(&signature_id)?;
            let assignment = score_routes_governed(&task, &signature)?;
            println!("{}", serde_json::to_string_pretty(&assignment)?);
        }
        Command::ResearchDeriveFromReport { report_id, persist } => {
            let report = cert_replay_report(&report_id).map_err(anyhow::Error::msg)?;
            let (
                bundle,
                entry,
                operator,
                strengthening,
                repro,
                review,
                challenge,
                readiness,
                handoff,
                route_assignment,
            ) = derive_governed_complete_research_bundle_from_report(&report)?;
            let benchmark_schemas = l64_research::derive_benchmark_schemas_from_report(&report);
            let coverage = dispatch_proof_coverage(&report);
            let tower_bundle = build_vertical_compounding_bundle(&registry, &report);
            if persist {
                persist_framework_registry_entry(&entry)?;
                persist_operator_record(&operator)?;
                persist_reproducibility_packet(&repro)?;
                persist_review_receipt(&review)?;
                if let Some(item) = &challenge {
                    persist_challenge_record(item)?;
                }
                for item in &strengthening {
                    persist_strengthening_artifact(item)?;
                }
                for item in &benchmark_schemas {
                    persist_benchmark_schema(item)?;
                }
                for item in &bundle.tasks {
                    persist_task_envelope(item)?;
                }
                for item in &bundle.signatures {
                    persist_derivation_signature(item)?;
                }
                for item in &bundle.route_assignments {
                    persist_route_assignment(item)?;
                }
                for item in &bundle.claims {
                    persist_math_claim_packet(item)?;
                }
                for item in &bundle.reduction_maps {
                    persist_reduction_map_record(item)?;
                }
                for item in &bundle.projection_maps {
                    persist_projection_map_record(item)?;
                }
                for item in &bundle.benchmark_runs {
                    persist_benchmark_run_record(item)?;
                }
                for item in &bundle.handoff_packets {
                    persist_handoff_packet(item)?;
                }
                for item in &bundle.producer_hosts {
                    persist_producer_host_spec(item)?;
                }
                for item in &bundle.promotion_queue {
                    persist_promotion_queue_entry(item)?;
                }
                for item in &bundle.promotion_reports {
                    persist_promotion_readiness_report(item)?;
                }
                for item in &bundle.lineage_records {
                    persist_lineage_record(item)?;
                }
                persist_coverage_dispatch(&coverage)?;
                persist_vertical_bundle(&report.theorem_id, &tower_bundle)?;
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "task": bundle.tasks.first(),
                    "signature": bundle.signatures.first(),
                    "route_assignment": route_assignment,
                    "registry_entry": entry,
                    "operator_record": operator,
                    "reproducibility_packet": repro,
                    "review_receipt": review,
                    "challenge_record": challenge,
                    "benchmark_schemas": benchmark_schemas,
                    "producer_hosts": bundle.producer_hosts.clone(),
                    "lineage_records": bundle.lineage_records.clone(),
                    "strengthening": strengthening,
                    "handoff_packet": handoff,
                    "bundle": bundle,
                    "promotion_readiness": readiness,
                    "coverage_dispatch": coverage,
                    "tower_bundle": tower_bundle,
                    "persisted": persist,
                }))?
            );
        }
        Command::ResearchGovernReport { report_id, persist } => {
            let report = cert_replay_report(&report_id).map_err(anyhow::Error::msg)?;
            let (task, signature, route_assignment, readiness, handoff) =
                l64_research::govern_report_from_cold_result(&report)?;
            if persist {
                persist_task_envelope(&task)?;
                persist_derivation_signature(&signature)?;
                persist_route_assignment(&route_assignment)?;
                persist_promotion_readiness_report(&readiness)?;
                persist_handoff_packet(&handoff)?;
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "task": task,
                    "signature": signature,
                    "route_assignment": route_assignment,
                    "promotion_readiness": readiness,
                    "handoff_packet": handoff,
                    "persisted": persist,
                }))?
            );
        }
        Command::ResearchSeedExportRemediation { persist } => {
            let entries = if persist {
                persist_seeded_export_remediation_entries()?
            } else {
                l64_research::seed_export_remediation_entries()
            };
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "persisted": persist,
                    "summary": summarize_remediation_entries(&entries),
                    "entries": entries,
                }))?
            );
        }
        Command::ResearchRemediationSummary => {
            let ids = list_remediation_entries()?;
            let entries = ids
                .into_iter()
                .filter_map(|id| load_remediation_entry(&id).ok())
                .collect::<Vec<_>>();
            println!(
                "{}",
                serde_json::to_string_pretty(&summarize_remediation_entries(&entries))?
            );
        }
        Command::ResearchStatus => {
            println!(
                "{}",
                serde_json::to_string_pretty(&research_status_summary()?)?
            );
        }
        Command::ResearchPromotionReadiness { report_id } => {
            let report = cert_replay_report(&report_id).map_err(anyhow::Error::msg)?;
            let readiness = derive_governed_promotion_readiness_from_report(&report)?;
            println!("{}", serde_json::to_string_pretty(&readiness)?);
        }
        Command::DumpCanonical { id } => {
            dump_canonical(&kernel, &registry, &id)?;
        }
    }

    Ok(())
}

fn infer_surface_kind(file: &str, as_kind: Option<SurfaceArg>) -> Result<SurfaceKind> {
    if let Some(kind) = as_kind {
        return Ok(kind.into());
    }
    match Path::new(file)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
    {
        "qc0" => Ok(SurfaceKind::Qc0),
        "qm0" => Ok(SurfaceKind::Qm0),
        "qk0" => Ok(SurfaceKind::Qk0),
        "qa0" => Ok(SurfaceKind::Qa0),
        other => Err(anyhow!(
            "unsupported or missing surface extension `{other}`"
        )),
    }
}

fn normalize_file(
    file: &str,
    as_kind: Option<SurfaceArg>,
    registry: &SeedRegistry,
) -> Result<(String, l64_core::FormatTransformReceipt)> {
    let source_kind = infer_surface_kind(file, as_kind)?;
    let input = fs::read_to_string(file).with_context(|| format!("failed to read `{file}`"))?;
    if source_kind == SurfaceKind::Qa0 && !input.trim_start().starts_with("!qa0 ") {
        let document = parse_qa0_document(&input)?;
        let policy = default_policy_for(SurfaceKind::Qa0, registry)?;
        let normalized = normalize_qa0_document(&document)?;
        let receipt = l64_core::FormatTransformReceipt {
            id: format!("XFR-LEGACY-QA0-{:x}", fxhash(&normalized)),
            src_surface: SurfaceKind::Qa0,
            dst_surface: SurfaceKind::Qa0,
            object_ids: document
                .entries
                .iter()
                .filter_map(|entry| match entry {
                    QaEntry::Object(item) => Some(item.id.clone()),
                    _ => None,
                })
                .collect(),
            transform_kind: l64_core::TransformKind::Normalize,
            policy_id: policy.id,
            defaults_used: Vec::new(),
            alias_expansions: Vec::new(),
            loss_classes: Vec::new(),
            hash_before: format!("{:x}", fxhash(&input)),
            hash_after: format!("{:x}", fxhash(&normalized)),
            verdict: l64_core::TransformVerdict::Lossless,
            rollback_ref: None,
            replay_ref: Some("legacy-qa0".into()),
        };
        return Ok((normalized, receipt));
    }
    normalize_surface(&input, source_kind, registry)
}

fn load_document(
    file: &str,
    as_kind: Option<SurfaceArg>,
    registry: &SeedRegistry,
) -> Result<(QaDocument, l64_core::FormatTransformReceipt)> {
    let source_kind = infer_surface_kind(file, as_kind)?;
    let input = fs::read_to_string(file).with_context(|| format!("failed to read `{file}`"))?;
    if source_kind == SurfaceKind::Qa0 && !input.trim_start().starts_with("!qa0 ") {
        let document = parse_qa0_document(&input)?;
        let policy = default_policy_for(SurfaceKind::Qa0, registry)?;
        let receipt = l64_core::FormatTransformReceipt {
            id: format!("XFR-LEGACY-QA0-IMPORT-{:x}", fxhash(&input)),
            src_surface: SurfaceKind::Qa0,
            dst_surface: SurfaceKind::Qc0,
            object_ids: document
                .entries
                .iter()
                .filter_map(|entry| match entry {
                    QaEntry::Object(item) => Some(item.id.clone()),
                    _ => None,
                })
                .collect(),
            transform_kind: l64_core::TransformKind::Import,
            policy_id: policy.id,
            defaults_used: vec!["legacy-qa0-header".into()],
            alias_expansions: Vec::new(),
            loss_classes: vec!["ascii-mirror".into()],
            hash_before: format!("{:x}", fxhash(&input)),
            hash_after: format!("{:x}", fxhash(&serde_json::to_string(&document)?)),
            verdict: l64_core::TransformVerdict::ReceiptedLoss,
            rollback_ref: None,
            replay_ref: Some("legacy-qa0".into()),
        };
        return Ok((document, receipt));
    }
    let (artifact, receipt) = import_file(Path::new(file), Some(source_kind), registry)?;
    Ok((artifact.document, receipt))
}

fn describe_document(document: &QaDocument) -> String {
    let mut counts = [0usize; 27];
    for entry in &document.entries {
        match entry {
            QaEntry::Object(_) => counts[0] += 1,
            QaEntry::Regime(_) => counts[1] += 1,
            QaEntry::Bridge(_) => counts[2] += 1,
            QaEntry::ProofShape(_) => counts[3] += 1,
            QaEntry::AtlasCell(_) => counts[4] += 1,
            QaEntry::MechanizationPackage(_) => counts[5] += 1,
            QaEntry::TheoremSpec(_) => counts[6] += 1,
            QaEntry::Obligation(_) => counts[7] += 1,
            QaEntry::TargetProfile(_) => counts[8] += 1,
            QaEntry::RouteLedger(_) => counts[9] += 1,
            QaEntry::Certificate(_) => counts[10] += 1,
            QaEntry::Campaign(_) => counts[11] += 1,
            QaEntry::CampaignPortfolio(_) => counts[12] += 1,
            QaEntry::RouteClass(_) => counts[13] += 1,
            QaEntry::AtlasDeficiency(_) => counts[14] += 1,
            QaEntry::AdequacyClause(_) => counts[15] += 1,
            QaEntry::BurdenPack(_) => counts[16] += 1,
            QaEntry::ClaimPacket(_) => counts[17] += 1,
            QaEntry::EvidenceContract(_) => counts[18] += 1,
            QaEntry::BenchmarkReceipt(_) => counts[19] += 1,
            QaEntry::ChallengeReceipt(_) => counts[20] += 1,
            QaEntry::ReproducibilityPacket(_) => counts[21] += 1,
            QaEntry::SurfacePolicy(_) => counts[22] += 1,
            QaEntry::TransformReceipt(_) => counts[23] += 1,
            QaEntry::RoundTripReport(_) => counts[24] += 1,
            QaEntry::CapabilityMatrix(_) => counts[25] += 1,
            QaEntry::SurfaceBudget(_) => counts[26] += 1,
            QaEntry::PolicyObject(_)
            | QaEntry::PolicyBinding(_)
            | QaEntry::PolicyResolution(_)
            | QaEntry::BundleLock(_)
            | QaEntry::ExecutionManifest(_)
            | QaEntry::ReplayLockManifest(_)
            | QaEntry::LockReceipt(_)
            | QaEntry::LockDiff(_)
            | QaEntry::RecomputationPlan(_)
            | QaEntry::PlanExecution(_)
            | QaEntry::PredictionAssessment(_)
            | QaEntry::Reconciliation(_)
            | QaEntry::RootResolution(_) => {}
        }
    }
    format!(
        "objects={} regimes={} bridges={} proofs={} atlas_cells={} mechanization_packages={} theorem_specs={} obligations={} target_profiles={} route_ledgers={} certificates={} campaigns={} portfolios={} route_classes={} diagnostics={} adequacy_clauses={} burden_packs={} claim_packets={} evidence_contracts={} benchmark_receipts={} challenge_receipts={} reproducibility_packets={} surface_policies={} transform_receipts={} roundtrip_reports={} capability_matrices={} surface_budgets={}",
        counts[0],
        counts[1],
        counts[2],
        counts[3],
        counts[4],
        counts[5],
        counts[6],
        counts[7],
        counts[8],
        counts[9],
        counts[10],
        counts[11],
        counts[12],
        counts[13],
        counts[14],
        counts[15],
        counts[16],
        counts[17],
        counts[18],
        counts[19],
        counts[20],
        counts[21],
        counts[22],
        counts[23],
        counts[24],
        counts[25],
        counts[26]
    )
}

fn validate_document(
    kernel: &ConstitutionKernel,
    registry: &impl RegistryLookup,
    document: &QaDocument,
) -> Result<()> {
    for entry in &document.entries {
        match entry {
            QaEntry::Object(object) => {
                kernel.validate_object(object, registry)?;
                kernel
                    .promote(object, registry)
                    .map_err(anyhow::Error::msg)?;
            }
            QaEntry::Regime(regime) => kernel.validate_regime(regime)?,
            QaEntry::Bridge(bridge) => kernel.validate_bridge(bridge, registry)?,
            QaEntry::ProofShape(shape) => {
                kernel
                    .check_proof_shape(shape, registry)
                    .map_err(anyhow::Error::msg)?;
            }
            QaEntry::AtlasCell(cell) => {
                if registry.get_regime(&cell.source_regime).is_none()
                    || registry.get_regime(&cell.target_regime).is_none()
                {
                    anyhow::bail!("atlas cell `{}` references an unknown regime", cell.id);
                }
            }
            QaEntry::MechanizationPackage(package) => {
                if package
                    .parser
                    .supported_surfaces
                    .iter()
                    .all(|surface| surface != "QA-0")
                {
                    anyhow::bail!(
                        "mechanization package `{}` does not support QA-0",
                        package.id
                    );
                }
            }
            QaEntry::TheoremSpec(item) => kernel.validate_theorem_spec(item, registry)?,
            QaEntry::Obligation(item) => {
                if item.description.trim().is_empty() {
                    anyhow::bail!("obligation `{}` is missing a description", item.id);
                }
            }
            QaEntry::TargetProfile(item) => kernel.validate_target_profile(item)?,
            QaEntry::RouteLedger(item) => kernel.validate_route_ledger(item, registry)?,
            QaEntry::Certificate(item) => kernel.validate_certificate(item, registry)?,
            QaEntry::Campaign(item) => kernel.validate_campaign(item, registry)?,
            QaEntry::CampaignPortfolio(item) => {
                for campaign in &item.campaigns {
                    if registry.get_campaign(campaign).is_none() {
                        anyhow::bail!(
                            "portfolio `{}` references unknown campaign `{campaign}`",
                            item.id
                        );
                    }
                }
            }
            QaEntry::RouteClass(item) => {
                if registry.get_theorem_spec(&item.theorem).is_none() {
                    anyhow::bail!("route class `{}` references unknown theorem", item.id);
                }
            }
            QaEntry::AtlasDeficiency(item) => {
                if item.message.trim().is_empty() {
                    anyhow::bail!("diagnostic `{}` is missing a message", item.id);
                }
            }
            QaEntry::AdequacyClause(item) => kernel.validate_adequacy_clause(item, registry)?,
            QaEntry::BurdenPack(item) => {
                if item.obligation_ids.is_empty() {
                    anyhow::bail!("burden pack `{}` is missing obligations", item.id);
                }
            }
            QaEntry::ClaimPacket(item) => {
                if item.statement.trim().is_empty() {
                    anyhow::bail!("claim packet `{}` is missing a statement", item.id);
                }
            }
            QaEntry::EvidenceContract(item) => {
                if item.required_evidence_kinds.is_empty() {
                    anyhow::bail!(
                        "evidence contract `{}` is missing required evidence kinds",
                        item.id
                    );
                }
            }
            QaEntry::BenchmarkReceipt(item) => {
                if registry.get_claim_packet(&item.claim_packet_id).is_none() {
                    anyhow::bail!(
                        "benchmark receipt `{}` references unknown claim packet",
                        item.id
                    );
                }
            }
            QaEntry::ChallengeReceipt(item) => {
                if registry.get_claim_packet(&item.claim_packet_id).is_none() {
                    anyhow::bail!(
                        "challenge receipt `{}` references unknown claim packet",
                        item.id
                    );
                }
            }
            QaEntry::ReproducibilityPacket(item) => {
                if registry.get_claim_packet(&item.claim_packet_id).is_none() {
                    anyhow::bail!(
                        "reproducibility packet `{}` references unknown claim packet",
                        item.id
                    );
                }
            }
            QaEntry::SurfacePolicy(item) => {
                if registry
                    .get_projection_policy(&item.projection_policy)
                    .is_none()
                {
                    anyhow::bail!(
                        "surface policy `{}` references unknown projection policy",
                        item.id
                    );
                }
            }
            QaEntry::TransformReceipt(item) => {
                if item.object_ids.is_empty() {
                    anyhow::bail!("transform receipt `{}` is missing object ids", item.id);
                }
            }
            QaEntry::RoundTripReport(item) => {
                if item.receipt_ids.is_empty() {
                    anyhow::bail!("roundtrip report `{}` is missing receipt ids", item.id);
                }
            }
            QaEntry::CapabilityMatrix(item) => {
                if !item.import_support && !item.export_support {
                    anyhow::bail!(
                        "capability matrix `{}` declares no active surface support",
                        item.id
                    );
                }
            }
            QaEntry::SurfaceBudget(item) => {
                if item.forbid_silent_defaulting && item.max_loss_classes == 0 {
                    continue;
                }
            }
            QaEntry::PolicyObject(_)
            | QaEntry::PolicyBinding(_)
            | QaEntry::PolicyResolution(_)
            | QaEntry::BundleLock(_)
            | QaEntry::ExecutionManifest(_)
            | QaEntry::ReplayLockManifest(_)
            | QaEntry::LockReceipt(_)
            | QaEntry::LockDiff(_)
            | QaEntry::RecomputationPlan(_)
            | QaEntry::PlanExecution(_)
            | QaEntry::PredictionAssessment(_)
            | QaEntry::Reconciliation(_)
            | QaEntry::RootResolution(_) => {}
        }
    }
    Ok(())
}

fn certify_with_registry(
    registry: &(impl RegistryLookup + Sync),
    campaign: Option<String>,
    theorem: Option<String>,
    target_profile: Option<String>,
    file: Option<String>,
    overlay_only: bool,
    options: &CertificationOptions,
) -> Result<l64_core::CertificationReport> {
    let atlas = CompiledAtlas::compile(registry).map_err(anyhow::Error::msg)?;
    let (campaign, theorem, target_profile) =
        resolve_surface_cert_args_registry(registry, campaign, theorem, target_profile, file)?;
    let report = match (
        campaign.as_deref(),
        theorem.as_deref(),
        target_profile.as_deref(),
    ) {
        (Some(campaign_id), _, _) => {
            certify_derived_campaign_with_options(registry, &atlas, campaign_id, options)
        }
        (None, Some(theorem_id), Some(target_id)) => certify_derived_theorem_with_options(
            registry, &atlas, theorem_id, target_id, None, options,
        ),
        _ => Err(l64_cert::CertError::Message(
            "use either --campaign <id>, --file <surface>, or --theorem <id> --target-profile <id>"
                .into(),
        )),
    }
    .map_err(anyhow::Error::msg)?;
    let mut report = report;
    if overlay_only {
        report
            .reasons
            .push("executed against overlay-only bundle world".into());
    }
    Ok(report)
}

fn resolve_surface_cert_args_registry(
    registry: &(impl RegistryLookup + Sync),
    campaign: Option<String>,
    theorem: Option<String>,
    target_profile: Option<String>,
    file: Option<String>,
) -> Result<(Option<String>, Option<String>, Option<String>)> {
    if file.is_none() {
        return Ok((campaign, theorem, target_profile));
    }
    let world = import_bundle_file(
        Path::new(file.as_deref().unwrap_or_default()),
        None,
        BundleConflictPolicy::ExactMatch,
        None,
    )?;
    let document = bundle_document(&world);
    let _ = registry;
    Ok((
        campaign.or_else(|| {
            document.entries.iter().find_map(|entry| match entry {
                QaEntry::Campaign(item) => Some(item.id.clone()),
                _ => None,
            })
        }),
        theorem.or_else(|| {
            document.entries.iter().find_map(|entry| match entry {
                QaEntry::TheoremSpec(item) => Some(item.id.clone()),
                QaEntry::Campaign(item) => Some(item.theorem.clone()),
                _ => None,
            })
        }),
        target_profile.or_else(|| {
            document.entries.iter().find_map(|entry| match entry {
                QaEntry::TargetProfile(item) => Some(item.id.clone()),
                QaEntry::Campaign(item) => Some(item.target_profile.clone()),
                _ => None,
            })
        }),
    ))
}

fn bundle_document(world: &BundleWorld) -> QaDocument {
    let mut entries = Vec::new();
    entries.extend(
        world
            .overlay
            .local
            .objects
            .iter()
            .cloned()
            .map(QaEntry::Object),
    );
    entries.extend(
        world
            .overlay
            .local
            .regimes
            .iter()
            .cloned()
            .map(QaEntry::Regime),
    );
    entries.extend(
        world
            .overlay
            .local
            .bridges
            .iter()
            .cloned()
            .map(QaEntry::Bridge),
    );
    entries.extend(
        world
            .overlay
            .local
            .proof_shapes
            .iter()
            .cloned()
            .map(QaEntry::ProofShape),
    );
    entries.extend(
        world
            .overlay
            .local
            .atlas_cells
            .iter()
            .cloned()
            .map(QaEntry::AtlasCell),
    );
    entries.extend(
        world
            .overlay
            .local
            .theorem_specs
            .iter()
            .cloned()
            .map(QaEntry::TheoremSpec),
    );
    entries.extend(
        world
            .overlay
            .local
            .obligations
            .iter()
            .cloned()
            .map(QaEntry::Obligation),
    );
    entries.extend(
        world
            .overlay
            .local
            .target_profiles
            .iter()
            .cloned()
            .map(QaEntry::TargetProfile),
    );
    entries.extend(
        world
            .overlay
            .local
            .route_ledgers
            .iter()
            .cloned()
            .map(QaEntry::RouteLedger),
    );
    entries.extend(
        world
            .overlay
            .local
            .certificates
            .iter()
            .cloned()
            .map(QaEntry::Certificate),
    );
    entries.extend(
        world
            .overlay
            .local
            .campaigns
            .iter()
            .cloned()
            .map(QaEntry::Campaign),
    );
    QaDocument { entries }
}

fn run_bundle_world(
    world: &BundleWorld,
    overlay_only: bool,
    options: &CertificationOptions,
) -> Result<Vec<serde_json::Value>> {
    if overlay_only {
        run_bundle_with_registry(&world.overlay.local_only(), world, options)
    } else {
        run_bundle_with_registry(&world.overlay, world, options)
    }
}

fn certify_bundle_world(
    world: &BundleWorld,
    overlay_only: bool,
    options: &CertificationOptions,
) -> Result<Vec<l64_core::CertificationReport>> {
    if overlay_only {
        certify_bundle_with_registry(&world.overlay.local_only(), world, options)
    } else {
        certify_bundle_with_registry(&world.overlay, world, options)
    }
}

fn run_bundle_with_registry<R: RegistryLookup + Sync>(
    registry: &R,
    world: &BundleWorld,
    options: &CertificationOptions,
) -> Result<Vec<serde_json::Value>> {
    let atlas = CompiledAtlas::compile(registry).map_err(anyhow::Error::msg)?;
    let mut results = Vec::new();
    for campaign in &world.overlay.local.campaigns {
        let report = certify_derived_campaign_with_options(registry, &atlas, &campaign.id, options)
            .map_err(anyhow::Error::msg)?;
        persist_report_document(&report)?;
        results.push(serde_json::to_value(report)?);
    }
    if results.is_empty() {
        for theorem in &world.overlay.local.theorem_specs {
            let result =
                run_seed_theorem(registry, &theorem.id, &atlas).map_err(anyhow::Error::msg)?;
            results.push(serde_json::to_value(result)?);
        }
    }
    Ok(results)
}

fn certify_bundle_with_registry<R: RegistryLookup + Sync>(
    registry: &R,
    world: &BundleWorld,
    options: &CertificationOptions,
) -> Result<Vec<l64_core::CertificationReport>> {
    let atlas = CompiledAtlas::compile(registry).map_err(anyhow::Error::msg)?;
    let mut reports = Vec::new();
    for campaign in &world.overlay.local.campaigns {
        reports.push(
            certify_derived_campaign_with_options(registry, &atlas, &campaign.id, options)
                .map_err(anyhow::Error::msg)?,
        );
    }
    if reports.is_empty() {
        for theorem in &world.overlay.local.theorem_specs {
            let target = world
                .overlay
                .local
                .target_profiles
                .first()
                .map(|item| item.id.clone())
                .ok_or_else(|| {
                    anyhow!("bundle-local theorem certification requires a target profile")
                })?;
            reports.push(
                certify_derived_theorem_with_options(
                    registry,
                    &atlas,
                    &theorem.id,
                    &target,
                    None,
                    options,
                )
                .map_err(anyhow::Error::msg)?,
            );
        }
    }
    Ok(reports)
}

fn build_cert_options(
    optimizer_policy: OptimizerPolicy,
    file: Option<&str>,
    bundle: Option<&str>,
    replay_only: bool,
    no_cache: bool,
    strict_derived: bool,
    evaluator_policy: Option<String>,
    cache_policy: Option<String>,
    strict_policy: bool,
) -> Result<CertificationOptions> {
    let bundle_hash = if let Some(file) = file {
        let input = fs::read_to_string(file).with_context(|| format!("failed to read `{file}`"))?;
        format!("{:x}", fxhash(&input))
    } else if let Some(bundle) = bundle {
        format!("{:x}", fxhash(bundle))
    } else {
        "seed".into()
    };
    Ok(CertificationOptions {
        optimizer_policy: optimizer_policy.clone(),
        bundle_hash,
        policy_hash: format!(
            "{:x}",
            fxhash(&format!(
                "{optimizer_policy:?}|evaluator={:?}|cache={:?}|strict={strict_policy}",
                evaluator_policy, cache_policy
            ))
        ),
        bundle_id: bundle.map(ToString::to_string),
        evaluator_policy,
        cache_policy,
        no_cache,
        replay_only,
        strict_derived,
        strict_policy,
        force_parallel_obligations: false,
        max_obligation_workers: None,
    })
}

fn export_report_sidecar(
    report: &l64_core::CertificationReport,
    surface: SurfaceKind,
    _registry: &SeedRegistry,
) -> Result<()> {
    let registry = SeedRegistry::load()?;
    let document = if let Some(bundle_id) = report
        .execution_envelope
        .as_ref()
        .and_then(|item| item.bundle_id.clone())
    {
        if let Ok(world) = load_bundle_world(&bundle_id) {
            report_to_document_with_registry(report, &world.overlay)?
        } else {
            report_to_document_with_registry(report, &registry)?
        }
    } else {
        report_to_document_with_registry(report, &registry)?
    };
    let policy = default_policy_for(surface.clone(), &registry)?;
    let (rendered, _) = export_document(&document, surface.clone(), &policy, &registry)?;
    let target = report_cache_root()?.join(format!(
        "{}.{}",
        report_id(report),
        surface_extension(&surface)
    ));
    fs::write(target, rendered)?;
    Ok(())
}

fn load_report_document_for_export(
    registry_id: &str,
    registry: &SeedRegistry,
) -> Result<QaDocument> {
    let path = report_cache_path(registry_id)?;
    if path.exists() {
        let bytes = fs::read(path)?;
        let report = decode_locus_packet_report(&bytes).map_err(anyhow::Error::msg)?;
        if let Some(bundle_id) = report
            .execution_envelope
            .as_ref()
            .and_then(|item| item.bundle_id.clone())
        {
            if let Ok(world) = load_bundle_world(&bundle_id) {
                return report_to_document_with_registry(&report, &world.overlay);
            }
        }
        return report_to_document_with_registry(&report, registry);
    }
    let text = fs::read_to_string(report_cache_root()?.join(format!("{registry_id}.json")))?;
    if let Ok(report) = serde_json::from_str::<l64_core::CertificationReport>(&text) {
        if let Some(bundle_id) = report
            .execution_envelope
            .as_ref()
            .and_then(|item| item.bundle_id.clone())
        {
            if let Ok(world) = load_bundle_world(&bundle_id) {
                return report_to_document_with_registry(&report, &world.overlay);
            }
        }
        return report_to_document_with_registry(&report, registry);
    }
    Ok(serde_json::from_str(&text)?)
}

fn document_for_id(registry: &SeedRegistry, id: &str) -> Result<QaDocument> {
    if let Some(document) = document_for_registry_id(registry, id) {
        return Ok(document);
    }
    let entry = if let Ok(manifest) = load_execution_manifest(id) {
        QaEntry::ExecutionManifest(manifest)
    } else if let Ok(lock) = load_bundle_lock(id) {
        QaEntry::BundleLock(lock)
    } else if let Ok(document) = load_report_document_with_registry(id, registry) {
        return Ok(document);
    } else {
        return Err(anyhow!("no registry entity found for `{id}`"));
    };
    Ok(QaDocument {
        entries: vec![entry],
    })
}

fn resolve_surface_cert_args(
    registry: &SeedRegistry,
    campaign: Option<String>,
    theorem: Option<String>,
    target_profile: Option<String>,
    file: Option<String>,
    as_kind: Option<SurfaceArg>,
) -> Result<(Option<String>, Option<String>, Option<String>)> {
    if file.is_none() {
        return Ok((campaign, theorem, target_profile));
    }
    let (document, _) = load_document(file.as_deref().unwrap_or_default(), as_kind, registry)?;
    Ok((
        campaign.or_else(|| {
            document.entries.iter().find_map(|entry| match entry {
                QaEntry::Campaign(item) => Some(item.id.clone()),
                _ => None,
            })
        }),
        theorem.or_else(|| {
            document.entries.iter().find_map(|entry| match entry {
                QaEntry::TheoremSpec(item) => Some(item.id.clone()),
                QaEntry::Campaign(item) => Some(item.theorem.clone()),
                _ => None,
            })
        }),
        target_profile.or_else(|| {
            document.entries.iter().find_map(|entry| match entry {
                QaEntry::TargetProfile(item) => Some(item.id.clone()),
                QaEntry::Campaign(item) => Some(item.target_profile.clone()),
                _ => None,
            })
        }),
    ))
}

fn parse_genome_artifact_class(value: &str) -> Result<GenomeArtifactClass> {
    match value.to_ascii_lowercase().as_str() {
        "gene" => Ok(GenomeArtifactClass::Gene),
        "hap" | "haplotype" => Ok(GenomeArtifactClass::Haplotype),
        "chrom" | "chromosome" => Ok(GenomeArtifactClass::Chromosome),
        "genome" => Ok(GenomeArtifactClass::Genome),
        _ => Err(anyhow!("unknown genome artifact class `{value}`")),
    }
}

fn dump_canonical(kernel: &ConstitutionKernel, registry: &SeedRegistry, id: &str) -> Result<()> {
    if let Some(object) = registry.get_object(id) {
        let canonical = kernel
            .canonicalize_object(&object, registry)
            .map_err(anyhow::Error::msg)?;
        println!("{}", serde_json::to_string_pretty(&canonical)?);
        return Ok(());
    }
    let document = document_for_id(registry, id)?;
    println!("{}", serde_json::to_string_pretty(&document.entries[0])?);
    Ok(())
}

fn fxhash(input: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}
