use anyhow::{Result, anyhow};
use l64_core::{
    AuthorityState, BenchmarkRunRecord, BenchmarkSchema, CertificationReport, ChallengeGrounds,
    ChallengeRecord, ChallengeSeverity, ChallengeStatus, ClaimClass, DerivationSignature,
    DesiredOutputKind, FrameworkRegistryEntry, HandoffPacket, LocusCapabilityMask, LocusOpcode,
    LocusPacketKind, MathClaimPacket, ObjectiveKind, OperatorRecord, ProducerHostKind,
    ProducerHostSpec, ProjectionMapRecord, PromotionQueueEntry, PromotionQueueStatus,
    PromotionReadinessReport, ProofCoverageDispatch, ReductionMapRecord, RegistryEntryKind,
    RegistryEntryStatus, RemediationClass, RemediationLedgerEntry, RemediationStatus,
    ReproducibilityPacket, ResearchBundle, ResearchLineageRecord, ResearchRouteClass,
    ResponseClass, ReviewDecision, ReviewReceipt, ReviewStatus, RouteAssignment, RouteScore,
    StrengtheningArtifact, TaskEnvelope, VerticalCompoundingBundle,
};
use l64_locus::{read_section_packet_or_json, write_section_packet};
use serde::{Serialize, de::DeserializeOwned};
use std::fs;
use std::path::{Path, PathBuf};

fn research_root() -> Result<PathBuf> {
    let root = PathBuf::from(
        l64_core::resolve_cache_root()
            .map_err(anyhow::Error::msg)?
            .absolute_path,
    )
    .join("research");
    fs::create_dir_all(&root)?;
    Ok(root)
}

fn kind_dir(kind: &str) -> Result<PathBuf> {
    let root = research_root()?.join(kind);
    fs::create_dir_all(&root)?;
    Ok(root)
}

fn dna_path(kind: &str, id: &str) -> Result<PathBuf> {
    Ok(kind_dir(kind)?.join(format!("{id}.dna")))
}
fn locus_path(kind: &str, id: &str) -> Result<PathBuf> {
    Ok(kind_dir(kind)?.join(format!("{id}.locus")))
}
fn legacy_path(kind: &str, id: &str) -> Result<PathBuf> {
    Ok(kind_dir(kind)?.join(format!("{id}.json")))
}

fn store_payload<T: Serialize>(
    kind: &str,
    id: &str,
    schema_hash: &str,
    payload: &T,
    opcode: LocusOpcode,
) -> Result<()> {
    let path = dna_path(kind, id)?;
    write_section_packet(
        &path,
        LocusPacketKind::CanonicalTransfer,
        opcode,
        id,
        schema_hash,
        payload,
        LocusCapabilityMask {
            has_frontier: true,
            ..Default::default()
        },
        1,
    )
    .map_err(anyhow::Error::msg)?;
    fs::copy(&path, locus_path(kind, id)?).map_err(anyhow::Error::msg)?;
    Ok(())
}

fn load_payload<T: DeserializeOwned>(kind: &str, id: &str, opcode: LocusOpcode) -> Result<T> {
    let packet = dna_path(kind, id)?;
    let legacy = legacy_path(kind, id)?;
    if packet.exists() {
        return read_section_packet_or_json(&packet, &legacy, opcode).map_err(anyhow::Error::msg);
    }
    let locus = locus_path(kind, id)?;
    read_section_packet_or_json(&locus, &legacy, opcode).map_err(anyhow::Error::msg)
}

fn list_ids(kind: &str) -> Result<Vec<String>> {
    let root = kind_dir(kind)?;
    let mut out = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if let (Some(stem), Some(ext)) = (
            path.file_stem().and_then(|s| s.to_str()),
            path.extension().and_then(|s| s.to_str()),
        ) {
            if ext == "locus" || ext == "dna" || ext == "json" {
                out.push(stem.to_string());
            }
        }
    }
    out.sort();
    out.dedup();
    Ok(out)
}

pub fn persist_task_envelope(item: &TaskEnvelope) -> Result<()> {
    store_payload(
        "tasks",
        &item.id,
        "task_envelope.v1",
        item,
        LocusOpcode::Proposal,
    )
}
pub fn load_task_envelope(id: &str) -> Result<TaskEnvelope> {
    load_payload("tasks", id, LocusOpcode::Proposal)
}
pub fn list_task_envelopes() -> Result<Vec<String>> {
    list_ids("tasks")
}

pub fn persist_derivation_signature(item: &DerivationSignature) -> Result<()> {
    store_payload(
        "signatures",
        &item.id,
        "derivation_signature.v1",
        item,
        LocusOpcode::Proposal,
    )
}
pub fn load_derivation_signature(id: &str) -> Result<DerivationSignature> {
    load_payload("signatures", id, LocusOpcode::Proposal)
}
pub fn list_derivation_signatures() -> Result<Vec<String>> {
    list_ids("signatures")
}

pub fn persist_review_receipt(item: &ReviewReceipt) -> Result<()> {
    store_payload(
        "reviews",
        &item.id,
        "review_receipt.v1",
        item,
        LocusOpcode::ReceiptTable,
    )
}
pub fn load_review_receipt(id: &str) -> Result<ReviewReceipt> {
    load_payload("reviews", id, LocusOpcode::ReceiptTable)
}
pub fn list_review_receipts() -> Result<Vec<String>> {
    list_ids("reviews")
}

pub fn persist_challenge_record(item: &ChallengeRecord) -> Result<()> {
    store_payload(
        "challenges",
        &item.id,
        "challenge_record.v1",
        item,
        LocusOpcode::ReceiptTable,
    )
}
pub fn load_challenge_record(id: &str) -> Result<ChallengeRecord> {
    load_payload("challenges", id, LocusOpcode::ReceiptTable)
}
pub fn list_challenge_records() -> Result<Vec<String>> {
    list_ids("challenges")
}

pub fn persist_framework_registry_entry(item: &FrameworkRegistryEntry) -> Result<()> {
    store_payload(
        "registry",
        &item.id,
        "framework_registry_entry.v1",
        item,
        LocusOpcode::Coverage,
    )
}
pub fn load_framework_registry_entry(id: &str) -> Result<FrameworkRegistryEntry> {
    load_payload("registry", id, LocusOpcode::Coverage)
}
pub fn list_framework_registry_entries() -> Result<Vec<String>> {
    list_ids("registry")
}

pub fn persist_operator_record(item: &OperatorRecord) -> Result<()> {
    store_payload(
        "operators",
        &item.id,
        "operator_record.v1",
        item,
        LocusOpcode::CanonicalPayload,
    )
}
pub fn load_operator_record(id: &str) -> Result<OperatorRecord> {
    load_payload("operators", id, LocusOpcode::CanonicalPayload)
}
pub fn list_operator_records() -> Result<Vec<String>> {
    list_ids("operators")
}

pub fn persist_benchmark_schema(item: &BenchmarkSchema) -> Result<()> {
    store_payload(
        "benchmarks",
        &item.id,
        "benchmark_schema.v1",
        item,
        LocusOpcode::CanonicalPayload,
    )
}
pub fn load_benchmark_schema(id: &str) -> Result<BenchmarkSchema> {
    load_payload("benchmarks", id, LocusOpcode::CanonicalPayload)
}
pub fn list_benchmark_schemas() -> Result<Vec<String>> {
    list_ids("benchmarks")
}

pub fn persist_math_claim_packet(item: &MathClaimPacket) -> Result<()> {
    store_payload(
        "claims",
        &item.id,
        "math_claim_packet.v1",
        item,
        LocusOpcode::CanonicalPayload,
    )
}
pub fn load_math_claim_packet(id: &str) -> Result<MathClaimPacket> {
    load_payload("claims", id, LocusOpcode::CanonicalPayload)
}
pub fn list_math_claim_packets() -> Result<Vec<String>> {
    list_ids("claims")
}

pub fn persist_reduction_map_record(item: &ReductionMapRecord) -> Result<()> {
    store_payload(
        "reductions",
        &item.id,
        "reduction_map_record.v1",
        item,
        LocusOpcode::RouteLedger,
    )
}
pub fn load_reduction_map_record(id: &str) -> Result<ReductionMapRecord> {
    load_payload("reductions", id, LocusOpcode::RouteLedger)
}
pub fn list_reduction_map_records() -> Result<Vec<String>> {
    list_ids("reductions")
}

pub fn persist_projection_map_record(item: &ProjectionMapRecord) -> Result<()> {
    store_payload(
        "projections",
        &item.id,
        "projection_map_record.v1",
        item,
        LocusOpcode::RouteLedger,
    )
}
pub fn load_projection_map_record(id: &str) -> Result<ProjectionMapRecord> {
    load_payload("projections", id, LocusOpcode::RouteLedger)
}
pub fn list_projection_map_records() -> Result<Vec<String>> {
    list_ids("projections")
}

pub fn persist_benchmark_run_record(item: &BenchmarkRunRecord) -> Result<()> {
    store_payload(
        "benchmark_runs",
        &item.id,
        "benchmark_run_record.v1",
        item,
        LocusOpcode::ReceiptTable,
    )
}
pub fn load_benchmark_run_record(id: &str) -> Result<BenchmarkRunRecord> {
    load_payload("benchmark_runs", id, LocusOpcode::ReceiptTable)
}
pub fn list_benchmark_run_records() -> Result<Vec<String>> {
    list_ids("benchmark_runs")
}

pub fn persist_handoff_packet(item: &HandoffPacket) -> Result<()> {
    store_payload(
        "handoff",
        &item.id,
        "handoff_packet.v1",
        item,
        LocusOpcode::CanonicalPayload,
    )
}
pub fn load_handoff_packet(id: &str) -> Result<HandoffPacket> {
    load_payload("handoff", id, LocusOpcode::CanonicalPayload)
}
pub fn list_handoff_packets() -> Result<Vec<String>> {
    list_ids("handoff")
}

pub fn persist_producer_host_spec(item: &ProducerHostSpec) -> Result<()> {
    store_payload(
        "producer_hosts",
        &item.id,
        "producer_host_spec.v1",
        item,
        LocusOpcode::CanonicalPayload,
    )
}
pub fn load_producer_host_spec(id: &str) -> Result<ProducerHostSpec> {
    load_payload("producer_hosts", id, LocusOpcode::CanonicalPayload)
}
pub fn list_producer_host_specs() -> Result<Vec<String>> {
    list_ids("producer_hosts")
}

pub fn persist_promotion_queue_entry(item: &PromotionQueueEntry) -> Result<()> {
    store_payload(
        "promotion_queue",
        &item.id,
        "promotion_queue_entry.v1",
        item,
        LocusOpcode::Coverage,
    )
}
pub fn load_promotion_queue_entry(id: &str) -> Result<PromotionQueueEntry> {
    load_payload("promotion_queue", id, LocusOpcode::Coverage)
}
pub fn list_promotion_queue_entries() -> Result<Vec<String>> {
    list_ids("promotion_queue")
}

pub fn persist_promotion_readiness_report(item: &PromotionReadinessReport) -> Result<()> {
    store_payload(
        "promotion_reports",
        &item.subject_id,
        "promotion_readiness_report.v1",
        item,
        LocusOpcode::Coverage,
    )
}
pub fn load_promotion_readiness_report(id: &str) -> Result<PromotionReadinessReport> {
    load_payload("promotion_reports", id, LocusOpcode::Coverage)
}
pub fn list_promotion_readiness_reports() -> Result<Vec<String>> {
    list_ids("promotion_reports")
}

pub fn persist_route_assignment(item: &RouteAssignment) -> Result<()> {
    store_payload(
        "route_assignments",
        &item.task_id,
        "route_assignment.v1",
        item,
        LocusOpcode::Coverage,
    )
}
pub fn load_route_assignment(id: &str) -> Result<RouteAssignment> {
    load_payload("route_assignments", id, LocusOpcode::Coverage)
}
pub fn list_route_assignments() -> Result<Vec<String>> {
    list_ids("route_assignments")
}

pub fn persist_strengthening_artifact(item: &StrengtheningArtifact) -> Result<()> {
    store_payload(
        "strengthening",
        &item.id,
        "strengthening_artifact.v1",
        item,
        LocusOpcode::CanonicalPayload,
    )
}
pub fn load_strengthening_artifact(id: &str) -> Result<StrengtheningArtifact> {
    load_payload("strengthening", id, LocusOpcode::CanonicalPayload)
}
pub fn list_strengthening_artifacts() -> Result<Vec<String>> {
    list_ids("strengthening")
}

pub fn persist_reproducibility_packet(item: &ReproducibilityPacket) -> Result<()> {
    store_payload(
        "repro",
        &item.id,
        "reproducibility_packet.v1",
        item,
        LocusOpcode::ReceiptTable,
    )
}
pub fn load_reproducibility_packet(id: &str) -> Result<ReproducibilityPacket> {
    load_payload("repro", id, LocusOpcode::ReceiptTable)
}
pub fn list_reproducibility_packets() -> Result<Vec<String>> {
    list_ids("repro")
}

pub fn persist_remediation_entry(item: &RemediationLedgerEntry) -> Result<()> {
    store_payload(
        "remediation",
        &item.id,
        "remediation_ledger_entry.v1",
        item,
        LocusOpcode::Coverage,
    )
}
pub fn load_remediation_entry(id: &str) -> Result<RemediationLedgerEntry> {
    load_payload("remediation", id, LocusOpcode::Coverage)
}
pub fn list_remediation_entries() -> Result<Vec<String>> {
    list_ids("remediation")
}

pub fn persist_coverage_dispatch(item: &ProofCoverageDispatch) -> Result<()> {
    store_payload(
        "coverage_dispatch",
        &item.subject_id,
        "proof_coverage_dispatch.v1",
        item,
        LocusOpcode::Coverage,
    )
}
pub fn load_coverage_dispatch(id: &str) -> Result<ProofCoverageDispatch> {
    load_payload("coverage_dispatch", id, LocusOpcode::Coverage)
}
pub fn list_coverage_dispatches() -> Result<Vec<String>> {
    list_ids("coverage_dispatch")
}

pub fn persist_vertical_bundle(subject_id: &str, item: &VerticalCompoundingBundle) -> Result<()> {
    store_payload(
        "tower",
        subject_id,
        "vertical_compounding_bundle.v1",
        item,
        LocusOpcode::Frontier,
    )
}
pub fn load_vertical_bundle(id: &str) -> Result<VerticalCompoundingBundle> {
    load_payload("tower", id, LocusOpcode::Frontier)
}
pub fn list_vertical_bundles() -> Result<Vec<String>> {
    list_ids("tower")
}

pub fn persist_lineage_record(item: &ResearchLineageRecord) -> Result<()> {
    store_payload(
        "lineage",
        &item.id,
        "research_lineage_record.v1",
        item,
        LocusOpcode::Forensic,
    )
}
pub fn load_lineage_record(id: &str) -> Result<ResearchLineageRecord> {
    load_payload("lineage", id, LocusOpcode::Forensic)
}
pub fn list_lineage_records() -> Result<Vec<String>> {
    list_ids("lineage")
}

fn subject_like_matches(value: &str, subject: &str) -> bool {
    value == subject || value.contains(subject) || subject.contains(value)
}

fn subject_variants(seed: &str) -> Vec<String> {
    let mut out = vec![seed.to_string()];
    if let Some((_, suffix)) = seed.split_once('_') {
        out.push(suffix.to_string());
    }
    out.sort();
    out.dedup();
    out
}

fn any_subject_match<'a>(value: &str, subjects: impl IntoIterator<Item = &'a String>) -> bool {
    subjects
        .into_iter()
        .any(|subject| subject_like_matches(value, subject))
}

fn lineage_ref_for_subject(subject_id: &str) -> String {
    format!("lineage:LIN_{subject_id}")
}

struct GovernanceState {
    registry_entries: Vec<FrameworkRegistryEntry>,
    challenges: Vec<ChallengeRecord>,
    reviews: Vec<ReviewReceipt>,
    repro: Vec<ReproducibilityPacket>,
    strengthening: Vec<StrengtheningArtifact>,
    remediation: Vec<RemediationLedgerEntry>,
    benchmark_runs: Vec<BenchmarkRunRecord>,
    claims: Vec<MathClaimPacket>,
    reductions: Vec<ReductionMapRecord>,
    projections: Vec<ProjectionMapRecord>,
    handoffs: Vec<HandoffPacket>,
    producer_hosts: Vec<ProducerHostSpec>,
    promotion_queue: Vec<PromotionQueueEntry>,
    _promotion_reports: Vec<PromotionReadinessReport>,
    coverage_dispatches: Vec<ProofCoverageDispatch>,
    tower_bundles: Vec<(String, VerticalCompoundingBundle)>,
    _lineage_records: Vec<ResearchLineageRecord>,
}

fn load_governance_state() -> Result<GovernanceState> {
    Ok(GovernanceState {
        registry_entries: list_framework_registry_entries()?
            .into_iter()
            .filter_map(|id| load_framework_registry_entry(&id).ok())
            .collect(),
        challenges: list_challenge_records()?
            .into_iter()
            .filter_map(|id| load_challenge_record(&id).ok())
            .collect(),
        reviews: list_review_receipts()?
            .into_iter()
            .filter_map(|id| load_review_receipt(&id).ok())
            .collect(),
        repro: list_reproducibility_packets()?
            .into_iter()
            .filter_map(|id| load_reproducibility_packet(&id).ok())
            .collect(),
        strengthening: list_strengthening_artifacts()?
            .into_iter()
            .filter_map(|id| load_strengthening_artifact(&id).ok())
            .collect(),
        remediation: list_remediation_entries()?
            .into_iter()
            .filter_map(|id| load_remediation_entry(&id).ok())
            .collect(),
        benchmark_runs: list_benchmark_run_records()?
            .into_iter()
            .filter_map(|id| load_benchmark_run_record(&id).ok())
            .collect(),
        claims: list_math_claim_packets()?
            .into_iter()
            .filter_map(|id| load_math_claim_packet(&id).ok())
            .collect(),
        reductions: list_reduction_map_records()?
            .into_iter()
            .filter_map(|id| load_reduction_map_record(&id).ok())
            .collect(),
        projections: list_projection_map_records()?
            .into_iter()
            .filter_map(|id| load_projection_map_record(&id).ok())
            .collect(),
        handoffs: list_handoff_packets()?
            .into_iter()
            .filter_map(|id| load_handoff_packet(&id).ok())
            .collect(),
        producer_hosts: list_producer_host_specs()?
            .into_iter()
            .filter_map(|id| load_producer_host_spec(&id).ok())
            .collect(),
        promotion_queue: list_promotion_queue_entries()?
            .into_iter()
            .filter_map(|id| load_promotion_queue_entry(&id).ok())
            .collect(),
        _promotion_reports: list_promotion_readiness_reports()?
            .into_iter()
            .filter_map(|id| load_promotion_readiness_report(&id).ok())
            .collect(),
        coverage_dispatches: list_coverage_dispatches()?
            .into_iter()
            .filter_map(|id| load_coverage_dispatch(&id).ok())
            .collect(),
        tower_bundles: list_vertical_bundles()?
            .into_iter()
            .filter_map(|id| {
                load_vertical_bundle(&id)
                    .ok()
                    .map(|bundle| (id.clone(), bundle))
            })
            .collect(),
        _lineage_records: list_lineage_records()?
            .into_iter()
            .filter_map(|id| load_lineage_record(&id).ok())
            .collect(),
    })
}

pub fn score_routes(
    task: &TaskEnvelope,
    signature: &DerivationSignature,
    open_challenges: &[ChallengeRecord],
    registry_entries: &[FrameworkRegistryEntry],
    reviews: &[ReviewReceipt],
    repro_packets: &[ReproducibilityPacket],
    strengthening: &[StrengtheningArtifact],
    remediation: &[RemediationLedgerEntry],
) -> RouteAssignment {
    score_routes_internal(
        task,
        signature,
        open_challenges,
        registry_entries,
        reviews,
        repro_packets,
        strengthening,
        remediation,
        &[],
        &[],
        &[],
        &[],
        &[],
        &[],
        &[],
        &[],
        &[],
    )
}

fn score_routes_internal(
    task: &TaskEnvelope,
    signature: &DerivationSignature,
    open_challenges: &[ChallengeRecord],
    registry_entries: &[FrameworkRegistryEntry],
    reviews: &[ReviewReceipt],
    repro_packets: &[ReproducibilityPacket],
    strengthening: &[StrengtheningArtifact],
    remediation: &[RemediationLedgerEntry],
    benchmark_runs: &[BenchmarkRunRecord],
    claims: &[MathClaimPacket],
    reductions: &[ReductionMapRecord],
    projections: &[ProjectionMapRecord],
    handoffs: &[HandoffPacket],
    producer_hosts: &[ProducerHostSpec],
    promotion_queue: &[PromotionQueueEntry],
    coverage_dispatches: &[ProofCoverageDispatch],
    tower_bundles: &[(String, VerticalCompoundingBundle)],
) -> RouteAssignment {
    let challenge_penalty = open_challenges
        .iter()
        .filter(|item| {
            matches!(
                item.severity,
                ChallengeSeverity::Blocking | ChallengeSeverity::High
            ) && (item.target_ref == task.id || item.target_ref == signature.task_ref)
        })
        .count() as f64;
    let open_risk_penalty = registry_entries
        .iter()
        .find(|item| item.id == task.id || item.id == signature.task_ref)
        .map(|item| item.open_risks.len() as f64)
        .unwrap_or(0.0);
    let review_bonus = reviews
        .iter()
        .filter(|item| item.subject_ref == signature.task_ref || item.subject_ref == task.id)
        .count() as f64;
    let repro_bonus = repro_packets
        .iter()
        .filter(|item| {
            item.claim_packet_id == signature.task_ref || item.claim_packet_id == task.id
        })
        .count() as f64;
    let strengthening_bonus = strengthening
        .iter()
        .filter(|item| item.source_ref == signature.task_ref || item.source_ref == task.id)
        .count() as f64;
    let remediation_pressure = remediation
        .iter()
        .filter(|item| {
            !matches!(
                item.status,
                RemediationStatus::Verified | RemediationStatus::Deferred
            )
        })
        .count() as f64;
    let mut subject_keys = Vec::new();
    subject_keys.extend(subject_variants(&task.id));
    subject_keys.extend(subject_variants(&signature.task_ref));
    subject_keys.sort();
    subject_keys.dedup();
    let benchmark_support = benchmark_runs
        .iter()
        .filter(|item| any_subject_match(&item.theorem_id, subject_keys.iter()))
        .count() as f64;
    let claim_support = claims
        .iter()
        .filter(|item| {
            any_subject_match(&item.id, subject_keys.iter())
                || item
                    .report_refs
                    .iter()
                    .any(|r| any_subject_match(r, subject_keys.iter()))
        })
        .count() as f64;
    let reduction_support = reductions
        .iter()
        .filter(|item| any_subject_match(&item.theorem_id, subject_keys.iter()))
        .count() as f64;
    let projection_support = projections
        .iter()
        .filter(|item| any_subject_match(&item.theorem_id, subject_keys.iter()))
        .count() as f64;
    let handoff_support = handoffs
        .iter()
        .filter(|item| any_subject_match(&item.subject_id, subject_keys.iter()))
        .count() as f64;
    let producer_support = producer_hosts
        .iter()
        .filter(|item| {
            item.feeds
                .iter()
                .any(|f| any_subject_match(f, subject_keys.iter()))
                || item
                    .owned_objects
                    .iter()
                    .any(|f| any_subject_match(f, subject_keys.iter()))
                || any_subject_match(&item.purpose, subject_keys.iter())
        })
        .count() as f64;
    let ready_promotion = promotion_queue
        .iter()
        .filter(|item| {
            matches!(item.status, PromotionQueueStatus::Ready)
                && any_subject_match(&item.subject_id, subject_keys.iter())
        })
        .count() as f64;
    let blocked_promotion = promotion_queue
        .iter()
        .filter(|item| {
            matches!(item.status, PromotionQueueStatus::Blocked)
                && any_subject_match(&item.subject_id, subject_keys.iter())
        })
        .count() as f64;
    let exact_coverage = coverage_dispatches
        .iter()
        .filter(|item| {
            item.route_fast_path && any_subject_match(&item.subject_id, subject_keys.iter())
        })
        .count() as f64;
    let unsupported_coverage = coverage_dispatches
        .iter()
        .filter(|item| {
            matches!(item.decision, l64_core::CoverageDecision::Unsupported)
                && any_subject_match(&item.subject_id, subject_keys.iter())
        })
        .count() as f64;
    let tower_pressure = tower_bundles
        .iter()
        .filter(|(id, _)| any_subject_match(id, subject_keys.iter()))
        .count() as f64;

    let mut routes = vec![
        ResearchRouteClass::Derive,
        ResearchRouteClass::Benchmark,
        ResearchRouteClass::Stress,
        ResearchRouteClass::ChallengeResponse,
        ResearchRouteClass::Crystallize,
        ResearchRouteClass::Integrate,
        ResearchRouteClass::OperationalHardening,
        ResearchRouteClass::Retire,
    ];

    let mut scores = Vec::new();
    for route in routes.drain(..) {
        let mut fit = 0.0;
        let mut leverage = 0.0;
        let mut risk = 0.0;
        let mut cost = 0.0;
        let mut debt = 0.0;
        let mut reasons = Vec::new();

        match route {
            ResearchRouteClass::Derive => {
                if matches!(
                    task.objective_kind,
                    ObjectiveKind::Derive | ObjectiveKind::Reduce
                ) {
                    fit += 4.0;
                    reasons.push("objective directly requests derivation/reduction".into());
                }
                if matches!(
                    signature.structural_type,
                    l64_core::DerivationStructuralType::Projection
                        | l64_core::DerivationStructuralType::Closure
                        | l64_core::DerivationStructuralType::Reduction
                ) {
                    fit += 3.0;
                    reasons.push("signature is derivation-native".into());
                }
                if matches!(
                    signature.expected_distillate,
                    DesiredOutputKind::Theorem | DesiredOutputKind::ReductionMap
                ) {
                    leverage += 2.5;
                }
                leverage += 0.5 * strengthening_bonus;
                risk += challenge_penalty + open_risk_penalty;
                cost += signature.dependency_depth as f64 * 0.5;
            }
            ResearchRouteClass::Benchmark => {
                if matches!(task.objective_kind, ObjectiveKind::Benchmark) {
                    fit += 5.0;
                }
                if matches!(
                    signature.structural_type,
                    l64_core::DerivationStructuralType::BenchmarkFit
                ) {
                    fit += 3.0;
                }
                leverage += 1.5 + repro_bonus;
                debt += open_risk_penalty * 0.25;
                cost += 1.5;
            }
            ResearchRouteClass::Stress => {
                if matches!(task.objective_kind, ObjectiveKind::Stress) {
                    fit += 5.0;
                }
                leverage += 2.0 + 0.5 * repro_bonus;
                risk += 0.5 * challenge_penalty;
                cost += 2.0;
                reasons.push("stress lowers future promotion error".into());
            }
            ResearchRouteClass::ChallengeResponse => {
                if challenge_penalty > 0.0 {
                    fit += 6.0;
                }
                if matches!(
                    signature.structural_type,
                    l64_core::DerivationStructuralType::ChallengeResponse
                        | l64_core::DerivationStructuralType::OperationalHardening
                ) {
                    fit += 3.0;
                }
                leverage += 2.0;
                debt += open_risk_penalty + remediation_pressure * 0.2;
                cost += 1.0;
            }
            ResearchRouteClass::Crystallize => {
                if matches!(
                    task.objective_kind,
                    ObjectiveKind::Crystallize | ObjectiveKind::Integrate
                ) {
                    fit += 4.0;
                }
                if matches!(
                    signature.expected_distillate,
                    DesiredOutputKind::Operator
                        | DesiredOutputKind::BenchmarkSchema
                        | DesiredOutputKind::Battery
                ) {
                    leverage += 4.0;
                }
                leverage += review_bonus + strengthening_bonus;
                cost += 1.5;
                reasons.push("crystallization converts success into reusable law".into());
            }
            ResearchRouteClass::Integrate => {
                if matches!(
                    task.objective_kind,
                    ObjectiveKind::Integrate | ObjectiveKind::Promote
                ) {
                    fit += 4.0;
                }
                leverage += 3.0 + review_bonus + repro_bonus;
                debt += open_risk_penalty * 0.5;
                cost += 2.0;
            }
            ResearchRouteClass::OperationalHardening => {
                if matches!(
                    signature.structural_type,
                    l64_core::DerivationStructuralType::OperationalHardening
                        | l64_core::DerivationStructuralType::RegistryHardening
                ) {
                    fit += 5.0;
                }
                leverage += 2.5;
                debt += remediation_pressure + challenge_penalty * 0.5;
                cost += 1.0;
                reasons.push(
                    "operational hardening lowers false blocker and artifact ambiguity costs"
                        .into(),
                );
            }
            ResearchRouteClass::Retire => {
                if matches!(task.objective_kind, ObjectiveKind::Refute) {
                    fit += 4.0;
                }
                risk += 1.0;
                cost += 0.5;
                reasons.push(
                    "retire only when evidence says a route should stop consuming budget".into(),
                );
            }
        }

        leverage += 0.25 * benchmark_support
            + 0.2 * claim_support
            + 0.2 * reduction_support
            + 0.15 * projection_support;
        leverage += 0.2 * handoff_support
            + 0.15 * producer_support
            + 0.3 * ready_promotion
            + 0.2 * exact_coverage;
        debt += 0.25 * blocked_promotion + 0.2 * unsupported_coverage + 0.1 * tower_pressure;
        if benchmark_support > 0.0 {
            reasons.push(format!("benchmark support={benchmark_support}"));
        }
        if exact_coverage > 0.0 {
            reasons.push("coverage fast-path exists".into());
        }
        if blocked_promotion > 0.0 {
            reasons.push("blocked promotion is still present in warm state".into());
        }
        if review_bonus > 0.0 {
            leverage += review_bonus * 0.25;
            reasons.push(format!("review-support x{review_bonus}"));
        }
        if repro_bonus > 0.0 {
            leverage += repro_bonus * 0.25;
            reasons.push(format!("repro-support x{repro_bonus}"));
        }
        if strengthening_bonus > 0.0 {
            leverage += strengthening_bonus * 0.25;
            reasons.push(format!("strengthening-support x{strengthening_bonus}"));
        }

        let total = fit + leverage - risk - cost - debt;
        scores.push(RouteScore {
            route,
            fit,
            leverage,
            risk,
            cost,
            debt,
            total,
            reasons,
        });
    }

    scores.sort_by(|a, b| {
        b.total
            .partial_cmp(&a.total)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let recommended_route = scores
        .first()
        .map(|item| item.route)
        .unwrap_or(ResearchRouteClass::Derive);
    RouteAssignment {
        task_id: task.id.clone(),
        signature_id: signature.id.clone(),
        recommended_route,
        scores,
    }
}

pub fn score_routes_governed(
    task: &TaskEnvelope,
    signature: &DerivationSignature,
) -> Result<RouteAssignment> {
    let state = load_governance_state()?;
    Ok(score_routes_internal(
        task,
        signature,
        &state.challenges,
        &state.registry_entries,
        &state.reviews,
        &state.repro,
        &state.strengthening,
        &state.remediation,
        &state.benchmark_runs,
        &state.claims,
        &state.reductions,
        &state.projections,
        &state.handoffs,
        &state.producer_hosts,
        &state.promotion_queue,
        &state.coverage_dispatches,
        &state.tower_bundles,
    ))
}

pub fn derive_task_envelope_from_report(report: &CertificationReport) -> TaskEnvelope {
    let objective_kind = if !report.deficiencies.is_empty() {
        ObjectiveKind::Reduce
    } else if !report.promotion_artifact_ids.is_empty()
        || matches!(
            report.verdict,
            l64_core::CertificationVerdict::Integrated | l64_core::CertificationVerdict::Certified
        )
    {
        ObjectiveKind::Promote
    } else if !report.benchmark_receipt_ids.is_empty() {
        ObjectiveKind::Integrate
    } else {
        ObjectiveKind::Derive
    };
    let desired_output = if !report.promotion_artifact_ids.is_empty() {
        DesiredOutputKind::Operator
    } else if !report.benchmark_receipt_ids.is_empty() {
        DesiredOutputKind::BenchmarkSchema
    } else {
        DesiredOutputKind::Theorem
    };
    TaskEnvelope {
        id: format!("TSK_{}", report.theorem_id),
        title: format!("govern {}", report.theorem_id),
        target_sector: if report
            .selected_path
            .iter()
            .any(|h| h.contains("PROB") || h.contains("COMP") || h.contains("LOG"))
        {
            l64_core::ResearchSector::Crosscut
        } else {
            l64_core::ResearchSector::Control
        },
        objective_kind,
        desired_output,
        hard_constraints: vec![format!("verdict:{:?}", report.verdict)],
        soft_constraints: vec![format!(
            "benchmark_receipts:{}",
            report.benchmark_receipt_ids.len()
        )],
        promotion_target: report.verdict.clone(),
        rollback_path: format!("report:{}", report.theorem_id),
        success_metrics: vec![
            "warm-host governance closure".into(),
            "promotion readiness".into(),
        ],
    }
}

pub fn derive_derivation_signature_from_report(
    report: &CertificationReport,
    task: &TaskEnvelope,
) -> DerivationSignature {
    let structural_type = if !report.deficiencies.is_empty() {
        l64_core::DerivationStructuralType::OperationalHardening
    } else if !report.challenge_receipt_ids.is_empty() {
        l64_core::DerivationStructuralType::ChallengeResponse
    } else if !report.benchmark_receipt_ids.is_empty() {
        l64_core::DerivationStructuralType::BenchmarkFit
    } else if report.route_class_id.is_some() || !report.selected_path.is_empty() {
        l64_core::DerivationStructuralType::Projection
    } else {
        l64_core::DerivationStructuralType::Closure
    };
    let risk_class = if !report.deficiencies.is_empty() {
        l64_core::RiskClass::High
    } else {
        l64_core::RiskClass::Medium
    };
    DerivationSignature {
        id: format!("SIG_{}", report.theorem_id),
        task_ref: task.id.clone(),
        structural_type,
        sector: task.target_sector,
        risk_class,
        dependency_depth: report.deficiencies.len() + report.obligations.len(),
        likely_failure_modes: report
            .deficiencies
            .iter()
            .map(|d| format!("{:?}", d.class))
            .collect(),
        expected_distillate: task.desired_output,
        evidence_gap: vec![
            (!report.checker_receipts.is_empty() || !report.obligations.is_empty())
                .then(|| "analytic".to_string()),
            report
                .benchmark_receipt_ids
                .is_empty()
                .then(|| "numeric".to_string()),
            report
                .challenge_receipt_ids
                .is_empty()
                .then(|| "stress".to_string()),
            Some("empirical".to_string()),
        ]
        .into_iter()
        .flatten()
        .collect(),
    }
}

pub fn derive_framework_registry_entry_from_report(
    report: &CertificationReport,
) -> FrameworkRegistryEntry {
    let status = match report.verdict {
        l64_core::CertificationVerdict::Integrated => RegistryEntryStatus::Integrated,
        l64_core::CertificationVerdict::Certified => RegistryEntryStatus::Certified,
        l64_core::CertificationVerdict::BlockedOpen => RegistryEntryStatus::Blocked,
        _ => RegistryEntryStatus::Derived,
    };
    FrameworkRegistryEntry {
        id: report.theorem_id.clone(),
        kind: RegistryEntryKind::MathClaim,
        host: "Locus Kernel".into(),
        status,
        retained_value: format!("campaign/theorem truth for {}", report.theorem_id),
        open_risks: report
            .deficiencies
            .iter()
            .map(|d| d.message.clone())
            .collect(),
        dependencies: report
            .obligations
            .iter()
            .map(|o| o.obligation_id.clone())
            .collect(),
        last_receipt: report.checker_receipts.last().map(|r| r.id.clone()),
    }
}

pub fn derive_operator_record_from_report(report: &CertificationReport) -> OperatorRecord {
    let semantics = if report.selected_path.is_empty() {
        "report has no selected path; operator remains report-local".to_string()
    } else {
        format!(
            "selected normalized path {}",
            report.selected_path.join(" -> ")
        )
    };
    OperatorRecord {
        id: format!("OPR_DERIVED_{}", report.theorem_id),
        name: format!("Derived operator from {}", report.theorem_id),
        semantics,
        valid_regimes: report.selected_atlas_cell.iter().cloned().collect(),
        evidence_level: report.verdict.clone(),
        dependencies: report.burden_pack_ids.clone(),
        failure_modes: report
            .deficiencies
            .iter()
            .map(|d| d.message.clone())
            .collect(),
        reuse_count: report.promotion_artifact_ids.len(),
        implementation_refs: report.promotion_artifact_ids.clone(),
    }
}

pub fn derive_review_receipt_from_report(report: &CertificationReport) -> ReviewReceipt {
    let verdict = match report.verdict {
        l64_core::CertificationVerdict::Integrated | l64_core::CertificationVerdict::Certified => {
            ReviewDecision::Proceed
        }
        l64_core::CertificationVerdict::BlockedOpen => ReviewDecision::Hold,
        _ => ReviewDecision::Revise,
    };
    let mut notes = Vec::new();
    notes.extend(report.reasons.clone());
    notes.extend(report.diagnostics.clone());
    if notes.is_empty() {
        notes.push("synthetic review receipt derived from certification report".into());
    }
    let (fit, risk, capture, continuity) = match report.verdict {
        l64_core::CertificationVerdict::Integrated | l64_core::CertificationVerdict::Certified => (
            ReviewStatus::Pass,
            if report.deficiencies.is_empty() {
                ReviewStatus::Pass
            } else {
                ReviewStatus::Revise
            },
            ReviewStatus::Pass,
            if report.replay_divergence_records.is_empty() {
                ReviewStatus::Pass
            } else {
                ReviewStatus::Revise
            },
        ),
        l64_core::CertificationVerdict::BlockedOpen => (
            ReviewStatus::Revise,
            ReviewStatus::Revise,
            ReviewStatus::Pass,
            ReviewStatus::Revise,
        ),
        _ => (
            ReviewStatus::Revise,
            ReviewStatus::Revise,
            ReviewStatus::Revise,
            ReviewStatus::Revise,
        ),
    };
    ReviewReceipt {
        id: format!("REV_{}", report.theorem_id),
        subject_ref: report.theorem_id.clone(),
        fit_review: fit,
        risk_review: risk,
        capture_review: capture,
        continuity_review: continuity,
        notes,
        reviewers: vec!["Locus Kernel synthetic reviewer".into()],
        verdict,
    }
}

pub fn derive_reproducibility_packet_from_report(
    report: &CertificationReport,
) -> ReproducibilityPacket {
    let mut artifact_refs = Vec::new();
    artifact_refs.extend(report.promotion_artifact_ids.clone());
    artifact_refs.extend(report.reused_artifact_ids.clone());
    artifact_refs.extend(report.default_selected_artifact_ids.clone());
    artifact_refs.extend(report.payoff_receipt_ids.clone());
    if let Some(id) = &report.certificate_id {
        artifact_refs.push(id.clone());
    }
    if let Some(id) = &report.route_class_id {
        artifact_refs.push(id.clone());
    }
    artifact_refs.push(report.target_profile_id.clone());
    if let Some(id) = &report.campaign_id {
        artifact_refs.push(id.clone());
    }
    ReproducibilityPacket {
        id: format!("RPR_{}", report.theorem_id),
        claim_packet_id: report.theorem_id.clone(),
        derivation_path: report.selected_path.clone(),
        code_refs: report
            .route_class_id
            .iter()
            .cloned()
            .chain(report.certificate_id.iter().cloned())
            .collect(),
        benchmark_refs: report.benchmark_receipt_ids.clone(),
        artifact_refs,
    }
}

pub fn derive_challenge_record_from_report(
    report: &CertificationReport,
) -> Option<ChallengeRecord> {
    let grounds = if !report.deficiencies.is_empty() {
        Some(ChallengeGrounds::ProjectionAmbiguity)
    } else if !report.diagnostics.is_empty() {
        Some(ChallengeGrounds::UnsupportedGeneralization)
    } else if matches!(
        report.verdict,
        l64_core::CertificationVerdict::BlockedOpen
            | l64_core::CertificationVerdict::BlockedContradiction
    ) {
        Some(ChallengeGrounds::PromotionMismatch)
    } else {
        None
    }?;
    let severity = match report.verdict {
        l64_core::CertificationVerdict::BlockedContradiction => ChallengeSeverity::Blocking,
        l64_core::CertificationVerdict::BlockedOpen => ChallengeSeverity::High,
        _ if !report.deficiencies.is_empty() => ChallengeSeverity::Medium,
        _ => ChallengeSeverity::Low,
    };
    let mut evidence_refs = Vec::new();
    evidence_refs.extend(report.challenge_receipt_ids.clone());
    evidence_refs.extend(report.benchmark_receipt_ids.clone());
    evidence_refs.extend(report.reproducibility_packet_ids.clone());
    evidence_refs.extend(report.payoff_receipt_ids.clone());
    Some(ChallengeRecord {
        id: format!("CHG_{}", report.theorem_id),
        target_ref: report.theorem_id.clone(),
        grounds,
        severity,
        evidence_refs,
        required_response: if matches!(
            report.verdict,
            l64_core::CertificationVerdict::BlockedContradiction
        ) {
            ResponseClass::Rollback
        } else if !report.deficiencies.is_empty() {
            ResponseClass::Patch
        } else {
            ResponseClass::Benchmark
        },
        status: ChallengeStatus::Open,
    })
}

pub fn derive_strengthening_artifacts_from_report(
    report: &CertificationReport,
) -> Vec<StrengtheningArtifact> {
    let mut out = Vec::new();
    out.push(StrengtheningArtifact {
        id: format!("STR_ROUTE_{}", report.theorem_id),
        source_ref: report.theorem_id.clone(),
        kind: DesiredOutputKind::ReductionMap,
        inheritance_targets: report.selected_atlas_cell.iter().cloned().collect(),
        redistribution_notes: "route/result residue should remain reusable above one report".into(),
    });
    if !report.checker_receipts.is_empty() {
        out.push(StrengtheningArtifact {
            id: format!("STR_BATTERY_{}", report.theorem_id),
            source_ref: report.theorem_id.clone(),
            kind: DesiredOutputKind::Battery,
            inheritance_targets: report
                .checker_receipts
                .iter()
                .map(|r| r.id.clone())
                .collect(),
            redistribution_notes: "checker-side success should remain as reusable validation law"
                .into(),
        });
    }
    out
}

pub fn seed_export_remediation_entries() -> Vec<RemediationLedgerEntry> {
    vec![
        RemediationLedgerEntry {
            id: "REM_EXPORT_EXPLAIN_EXECUTION".into(),
            source_ref: "Chat export stress test".into(),
            class: RemediationClass::RuntimeBug,
            summary: "explain-execution should resolve execution-linked context or degrade with exact contract truth instead of generic missing-file failure".into(),
            affected_surfaces: vec!["l64-admin explain-execution".into(), "execution cache".into(), "observation/report/manifest/lock resolution".into()],
            expected_fix_chain: vec!["export_remediation_ledger".into(), "operational_truth_hardening".into(), "execution_resolution_truth".into()],
            status: RemediationStatus::InProgress,
            notes: vec!["export reported repeated os error 2 across report/manifest/lock/observe inputs".into()],
        },
        RemediationLedgerEntry {
            id: "REM_EXPORT_VALIDATE_INSPECTION_REPORT".into(),
            source_ref: "Chat export stress test".into(),
            class: RemediationClass::ArtifactTypingBug,
            summary: "export-report surfaces should stay inspection-only unless explicitly widened into validation-complete bundles".into(),
            affected_surfaces: vec!["l64-cli export-report".into(), "l64-cli export-validation-bundle".into(), "l64-cli validate".into()],
            expected_fix_chain: vec!["artifact_class_law".into(), "validation_bundle_rollout".into(), "overlay_validation_truth".into()],
            status: RemediationStatus::InProgress,
            notes: vec!["export reported validate failure because theorem/route context was not fully embedded in inspection exports".into()],
        },
        RemediationLedgerEntry {
            id: "REM_EXPORT_PREDICT_IMPACT_CONTRACT".into(),
            source_ref: "Chat export stress test".into(),
            class: RemediationClass::CommandContractBug,
            summary: "predict-impact must expose a live argument contract stronger than help text and reject ambiguous selector combinations cleanly".into(),
            affected_surfaces: vec!["l64-admin predict-impact".into(), "command contract surfaces".into()],
            expected_fix_chain: vec!["command_contract_truth".into(), "capability_readiness_truth".into()],
            status: RemediationStatus::InProgress,
            notes: vec!["export said command was help-routable but live argument contract was unresolved".into()],
        },
        RemediationLedgerEntry {
            id: "REM_EXPORT_CAPABILITY_TRUTH".into(),
            source_ref: "Chat export stress test".into(),
            class: RemediationClass::CapabilityTruthGap,
            summary: "commands should distinguish declared, help-routable, contract-known, smoke-executed, and fully-exercised support instead of one flat supported flag".into(),
            affected_surfaces: vec!["capability matrices".into(), "command contracts".into(), "stress harness summaries".into()],
            expected_fix_chain: vec!["capability_readiness_truth".into(), "research_registry_visibility".into()],
            status: RemediationStatus::Open,
            notes: vec!["export corrected earlier false failures caused by help-only or misused commands".into()],
        },
        RemediationLedgerEntry {
            id: "REM_EXPORT_NAMESPACE_CONTRACT".into(),
            source_ref: "Chat export stress test".into(),
            class: RemediationClass::RuntimeBug,
            summary: "namespace-local execution, report, and observation artifacts need explicit same-namespace truth to prevent false blocker signals and harness misuse".into(),
            affected_surfaces: vec!["observe-run".into(), "admin compare/export flows".into(), "execution record lookup".into()],
            expected_fix_chain: vec!["namespace_scope_truth".into(), "execution_contracts".into()],
            status: RemediationStatus::Open,
            notes: vec!["export reported same-namespace constraints and one false race-style blocker".into()],
        },
        RemediationLedgerEntry {
            id: "REM_EXPORT_TRUST_MARGIN_ROLES".into(),
            source_ref: "Kineton calibration export".into(),
            class: RemediationClass::SemanticBug,
            summary: "trust parsing must distinguish error-like residuals from success margins, physical scales, coverage-like measures, and control roles".into(),
            affected_surfaces: vec!["benchmark receipts".into(), "trust vectors".into(), "calibration reports".into()],
            expected_fix_chain: vec!["metric_role_schema".into(), "benchmark_schema_normalization".into()],
            status: RemediationStatus::Open,
            notes: vec!["export specifically called out gap vs separation_margin, negative controls, physical scale, and coverage semantics".into()],
        },
        RemediationLedgerEntry {
            id: "REM_EXPORT_MATH_NATIVE_CLAIMS".into(),
            source_ref: "Kineton framework export".into(),
            class: RemediationClass::MissingFunctionality,
            summary: "claim objects need first-class mathematical bodies, assumptions, projection/reduction maps, and blocker leaves so orchestration does not outrun the actual math".into(),
            affected_surfaces: vec!["claim packets".into(), "research registry".into(), "handoff packets".into()],
            expected_fix_chain: vec!["warm_research_host".into(), "math_native_claim_semantics".into()],
            status: RemediationStatus::Open,
            notes: vec!["export repeatedly resisted generic claim shells divorced from real obligations like G2*, D1, R1, K0".into()],
        },
        RemediationLedgerEntry {
            id: "REM_EXPORT_WARM_HOST_GAP".into(),
            source_ref: "Kineton framework export".into(),
            class: RemediationClass::MissingFunctionality,
            summary: "the cold certifier needs a real warm research host for intake, routing, challenge, registry, benchmark, and handoff rather than implicit chat-side orchestration".into(),
            affected_surfaces: vec!["l64-research".into(), "routing".into(), "challenge workflow".into(), "registry".into()],
            expected_fix_chain: vec!["research_constitution_host".into(), "registry_distillate_repro_host".into()],
            status: RemediationStatus::InProgress,
            notes: vec!["export stated K2 is stronger as a cold certifier than as a full research host".into()],
        },
    ]
}

pub fn persist_seeded_export_remediation_entries() -> Result<Vec<RemediationLedgerEntry>> {
    let entries = seed_export_remediation_entries();
    for item in &entries {
        persist_remediation_entry(item)?;
    }
    Ok(entries)
}

pub fn summarize_remediation_entries(entries: &[RemediationLedgerEntry]) -> serde_json::Value {
    use std::collections::BTreeMap;
    let mut by_class: BTreeMap<String, usize> = BTreeMap::new();
    let mut by_status: BTreeMap<String, usize> = BTreeMap::new();
    for item in entries {
        *by_class.entry(format!("{:?}", item.class)).or_default() += 1;
        *by_status.entry(format!("{:?}", item.status)).or_default() += 1;
    }
    serde_json::json!({
        "count": entries.len(),
        "by_class": by_class,
        "by_status": by_status,
        "ids": entries.iter().map(|item| item.id.clone()).collect::<Vec<_>>(),
    })
}

pub fn derive_research_bundle_from_report(
    report: &CertificationReport,
) -> (
    FrameworkRegistryEntry,
    OperatorRecord,
    Vec<StrengtheningArtifact>,
    ReproducibilityPacket,
    ReviewReceipt,
    Option<ChallengeRecord>,
) {
    (
        derive_framework_registry_entry_from_report(report),
        derive_operator_record_from_report(report),
        derive_strengthening_artifacts_from_report(report),
        derive_reproducibility_packet_from_report(report),
        derive_review_receipt_from_report(report),
        derive_challenge_record_from_report(report),
    )
}

pub fn derive_lineage_record_from_report(report: &CertificationReport) -> ResearchLineageRecord {
    let subject_id = report
        .campaign_id
        .clone()
        .unwrap_or_else(|| report.theorem_id.clone());
    let canonical_hash = report
        .execution_envelope
        .as_ref()
        .map(|item| item.report_hash.clone())
        .unwrap_or_else(|| {
            format!(
                "{:x}",
                l64_core::stable_hash_u64(&format!(
                    "{}|{}|{:?}",
                    report.theorem_id, report.target_profile_id, report.verdict
                ))
            )
        });
    let lowering_receipt_id = format!("RPT_LINEAGE_{}", subject_id);
    let dependency_edges = report
        .reproducibility_packet_ids
        .iter()
        .cloned()
        .chain(report.checker_receipts.iter().map(|item| item.id.clone()))
        .collect::<Vec<_>>();
    let invariant_checks = vec![
        l64_core::InvariantCheck {
            name: "report_hash_available".into(),
            passed: !canonical_hash.is_empty(),
            detail: "report lineage requires a stable execution or fallback hash".into(),
        },
        l64_core::InvariantCheck {
            name: "dna_backed_report".into(),
            passed: report.execution_envelope.is_some()
                || !report.selected_path.is_empty()
                || report.certificate_id.is_some(),
            detail: "report lineage must come from an executable certification path".into(),
        },
    ];
    let failure_records = invariant_checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| l64_core::FailureRecord {
            code: "ReportLineageInvariantFailed".into(),
            message: format!("{}: {}", check.name, check.detail),
        })
        .collect::<Vec<_>>();
    let validation_result = if failure_records.is_empty() {
        l64_core::PhaseValidationResult::Passed
    } else {
        l64_core::PhaseValidationResult::Failed
    };
    let promotion_signal = if failure_records.is_empty() {
        l64_core::PromotionSignal::Promote
    } else {
        l64_core::PromotionSignal::Hold
    };

    ResearchLineageRecord {
        id: format!("LIN_{}", subject_id),
        subject_id: subject_id.clone(),
        artifact_class: l64_core::GenomeArtifactClass::Gene,
        source_surface: l64_core::GenomeSurface::Dna,
        target_surface: l64_core::GenomeSurface::Dna,
        grammar_id: "report-envelope.v1".into(),
        canonical_hash: canonical_hash.clone(),
        lowering_receipt_id,
        phase_ids: vec![
            l64_core::PhaseId::DnaProtocolFreeze,
            l64_core::PhaseId::ResearchHostReconnect,
        ],
        phase_ledger: vec![l64_core::ChangeLedgerEntry {
            phase_id: l64_core::PhaseId::ResearchHostReconnect,
            input_state_hash: canonical_hash.clone(),
            output_state_hash: Some(format!("{:x}", l64_core::stable_hash_u64(&subject_id))),
            dependency_edges,
            invariant_checks,
            failure_records,
            validation_result,
            promotion_signal,
            rollback_pointer: Some(format!("report:{}", report.theorem_id)),
        }],
        notes: vec![
            format!("theorem_id={}", report.theorem_id),
            format!("target_profile_id={}", report.target_profile_id),
            format!("verdict={:?}", report.verdict),
        ],
    }
}

pub fn derive_math_claim_packet_from_report(report: &CertificationReport) -> MathClaimPacket {
    let sector = if report.theorem_id.contains("CHAIN") || report.theorem_id.contains("BAYES") {
        l64_core::ResearchSector::Crosscut
    } else if report.theorem_id.contains("PROB") {
        l64_core::ResearchSector::Thermo
    } else {
        l64_core::ResearchSector::Control
    };
    let truth_class = if matches!(report.verdict, l64_core::CertificationVerdict::Integrated) {
        AuthorityState::Evidence
    } else if matches!(report.verdict, l64_core::CertificationVerdict::Certified) {
        AuthorityState::Benchmark
    } else {
        AuthorityState::Derived
    };
    let mut assumptions = Vec::new();
    assumptions.extend(
        report
            .burden_pack_ids
            .iter()
            .map(|id| format!("burden_pack:{id}")),
    );
    assumptions.extend(report.selected_path.iter().map(|id| format!("path:{id}")));
    let blocker_leaves = report
        .deficiencies
        .iter()
        .map(|d| format!("{}:{:?}", d.id, d.class))
        .collect::<Vec<_>>();
    // control-specific deficiencies collapse into the main deficiency set in this host slice.
    MathClaimPacket {
        id: format!("CLM_{}", report.theorem_id),
        title: format!("Claim packet for {}", report.theorem_id),
        statement: format!(
            "{} via {:?} with verdict {:?}",
            report.theorem_id, report.selected_path, report.verdict
        ),
        sector,
        truth_class,
        claim_class: ClaimClass::Host,
        assumptions,
        projection_refs: report.route_class_id.iter().cloned().collect(),
        reduction_refs: report.selected_atlas_cell.iter().cloned().collect(),
        benchmark_refs: report.benchmark_receipt_ids.clone(),
        blocker_leaves,
        report_refs: vec![format!("report:{}", report.theorem_id)],
    }
}

pub fn derive_reduction_map_record_from_report(report: &CertificationReport) -> ReductionMapRecord {
    ReductionMapRecord {
        id: format!("RDM_{}", report.theorem_id),
        theorem_id: report.theorem_id.clone(),
        target_profile_id: report.target_profile_id.clone(),
        route_class_id: report.route_class_id.clone(),
        atlas_cell_id: report.selected_atlas_cell.clone(),
        derivation_path: report.selected_path.join(" -> "),
        residual_obligation_ids: report
            .obligations
            .iter()
            .filter(|s| {
                !matches!(
                    s.verdict,
                    l64_core::CertificationVerdict::Integrated
                        | l64_core::CertificationVerdict::Certified
                ) && !matches!(
                    s.evaluation_mode,
                    l64_core::ObligationEvaluationMode::RecomputedExact
                )
            })
            .map(|s| s.obligation_id.clone())
            .collect(),
    }
}

pub fn derive_projection_map_record_from_report(
    report: &CertificationReport,
) -> ProjectionMapRecord {
    ProjectionMapRecord {
        id: format!("PRJ_{}", report.theorem_id),
        theorem_id: report.theorem_id.clone(),
        src_hosts: if report.selected_path.is_empty() {
            vec!["LocusKernel".into()]
        } else {
            report.selected_path.clone()
        },
        tgt_hosts: if report.selected_path.is_empty() {
            vec!["LocusKernel".into()]
        } else {
            report.selected_path.clone()
        },
        projection_summary: format!(
            "selected_path={:?} route_class={}",
            report.selected_path,
            report.route_class_id.clone().unwrap_or_default()
        ),
        bridge_refs: report.selected_path.clone(),
    }
}

pub fn derive_benchmark_schemas_from_report(report: &CertificationReport) -> Vec<BenchmarkSchema> {
    let mut out = Vec::new();
    for (idx, receipt_id) in report.benchmark_receipt_ids.iter().enumerate() {
        out.push(BenchmarkSchema {
            id: format!("BMS_{}_{}", report.theorem_id, idx + 1),
            target_description: format!(
                "Benchmark receipt {} for {}",
                receipt_id, report.theorem_id
            ),
            metric_name: "benchmark_receipt_presence".into(),
            tolerance: "presence-required".into(),
            reference_source: receipt_id.clone(),
            weight_basis_points: 1000,
            required_evidence: vec![receipt_id.clone()],
        });
    }
    if out.is_empty() {
        out.push(BenchmarkSchema {
            id: format!("BMS_{}_IMPLICIT", report.theorem_id),
            target_description: format!("Implicit benchmark floor for {}", report.theorem_id),
            metric_name: "report_verdict_floor".into(),
            tolerance: format!("{:?}", report.verdict),
            reference_source: format!("report:{}", report.theorem_id),
            weight_basis_points: 500,
            required_evidence: vec![format!("report:{}", report.theorem_id)],
        });
    }
    out
}

pub fn derive_benchmark_run_record_from_report(
    report: &CertificationReport,
    schema_ids: &[String],
) -> BenchmarkRunRecord {
    BenchmarkRunRecord {
        id: format!("BMR_{}", report.theorem_id),
        theorem_id: report.theorem_id.clone(),
        benchmark_schema_id: schema_ids.first().cloned(),
        receipt_ids: report.benchmark_receipt_ids.clone(),
        status: match report.verdict {
            l64_core::CertificationVerdict::Integrated => RegistryEntryStatus::Integrated,
            l64_core::CertificationVerdict::Certified => RegistryEntryStatus::Certified,
            l64_core::CertificationVerdict::Benchmarked => RegistryEntryStatus::Benchmarked,
            l64_core::CertificationVerdict::BlockedOpen
            | l64_core::CertificationVerdict::BlockedContradiction => RegistryEntryStatus::Blocked,
            _ => RegistryEntryStatus::Derived,
        },
        summary: format!(
            "benchmark receipts={} verdict={:?}",
            report.benchmark_receipt_ids.len(),
            report.verdict
        ),
    }
}

pub fn derive_handoff_packet_from_report(
    report: &CertificationReport,
    claim: &MathClaimPacket,
    operator: &OperatorRecord,
    strengthening: &[StrengtheningArtifact],
    repro: &ReproducibilityPacket,
    benchmarks: &[BenchmarkSchema],
) -> HandoffPacket {
    HandoffPacket {
        id: format!("HOF_{}", report.theorem_id),
        subject_id: report.theorem_id.clone(),
        summary: format!(
            "handoff packet for {} with verdict {:?}",
            report.theorem_id, report.verdict
        ),
        status: PromotionQueueStatus::Proposed,
        claim_refs: vec![claim.id.clone()],
        operator_refs: vec![operator.id.clone()],
        strengthening_refs: strengthening.iter().map(|i| i.id.clone()).collect(),
        repro_refs: vec![repro.id.clone()],
        benchmark_refs: benchmarks.iter().map(|i| i.id.clone()).collect(),
        benchmark_run_refs: Vec::new(),
        producer_host_refs: Vec::new(),
        route_refs: Vec::new(),
        coverage_refs: Vec::new(),
        lineage_refs: Vec::new(),
        tower_refs: Vec::new(),
        promotion_refs: Vec::new(),
        readiness_score_basis_points: 0,
        blockers: Vec::new(),
        positive_factors: Vec::new(),
    }
}

pub fn derive_promotion_readiness_from_report(
    report: &CertificationReport,
    reviews: &[ReviewReceipt],
    challenges: &[ChallengeRecord],
    repros: &[ReproducibilityPacket],
    strengthening: &[StrengtheningArtifact],
    remediation: &[RemediationLedgerEntry],
) -> PromotionReadinessReport {
    let mut score: i32 = 0;
    let mut positive = Vec::new();
    let mut blockers = Vec::new();
    let mut missing = Vec::new();
    let matching_reviews = reviews
        .iter()
        .filter(|r| r.subject_ref == report.theorem_id)
        .collect::<Vec<_>>();
    if !matching_reviews.is_empty() {
        score += 1500;
        positive.push("review receipt exists".into());
        if matching_reviews
            .iter()
            .all(|r| matches!(r.verdict, ReviewDecision::Proceed))
        {
            score += 1500;
            positive.push("review verdict proceed".into());
        }
    } else {
        missing.push("review receipt".into());
    }
    let open_challenges = challenges
        .iter()
        .filter(|c| c.target_ref == report.theorem_id && matches!(c.status, ChallengeStatus::Open))
        .collect::<Vec<_>>();
    if open_challenges.is_empty() {
        score += 1500;
        positive.push("no open challenges".into());
    } else {
        blockers.extend(
            open_challenges
                .iter()
                .map(|c| format!("challenge:{}:{:?}", c.id, c.severity)),
        );
        score -= 2500;
    }
    if repros
        .iter()
        .any(|r| r.claim_packet_id == report.theorem_id)
    {
        score += 1000;
        positive.push("reproducibility packet exists".into());
    } else {
        missing.push("reproducibility packet".into());
    }
    let str_count = strengthening
        .iter()
        .filter(|s| s.source_ref == report.theorem_id)
        .count();
    if str_count > 0 {
        score += 500 + (str_count as i32 * 250);
        positive.push(format!("strengthening artifacts={str_count}"));
    } else {
        missing.push("strengthening artifact".into());
    }
    let active_remediation = remediation
        .iter()
        .filter(|r| {
            !matches!(
                r.status,
                RemediationStatus::Verified | RemediationStatus::Deferred
            ) && r
                .affected_surfaces
                .iter()
                .any(|s| s.contains(&report.theorem_id))
        })
        .count();
    if active_remediation > 0 {
        blockers.push(format!("active_remediation={active_remediation}"));
        score -= 1000;
    }
    if matches!(
        report.verdict,
        l64_core::CertificationVerdict::Integrated | l64_core::CertificationVerdict::Certified
    ) {
        score += 2500;
        positive.push(format!("report verdict {:?}", report.verdict));
    } else {
        blockers.push(format!("report verdict {:?}", report.verdict));
        score -= 1500;
    }
    if !report.deficiencies.is_empty() {
        blockers.push("deficiencies present".into());
        score -= 2000;
    } else {
        score += 1000;
        positive.push("no outstanding deficiencies".into());
    }
    let score = score.clamp(0, 10000) as u16;
    let status = if !blockers.is_empty() {
        PromotionQueueStatus::Blocked
    } else if score >= 8000 {
        PromotionQueueStatus::Ready
    } else {
        PromotionQueueStatus::Proposed
    };
    PromotionReadinessReport {
        subject_id: report.theorem_id.clone(),
        readiness_score_basis_points: score,
        status,
        positive_factors: positive,
        blockers,
        missing_refs: missing,
    }
}

pub fn derive_governed_promotion_readiness_from_report(
    report: &CertificationReport,
) -> Result<PromotionReadinessReport> {
    let state = load_governance_state()?;
    let mut readiness = derive_promotion_readiness_from_report(
        report,
        &state.reviews,
        &state.challenges,
        &state.repro,
        &state.strengthening,
        &state.remediation,
    );
    if state.claims.iter().any(|c| {
        c.report_refs
            .iter()
            .any(|r| r.ends_with(&report.theorem_id))
            || c.id.ends_with(&report.theorem_id)
    }) {
        readiness.readiness_score_basis_points =
            readiness.readiness_score_basis_points.saturating_add(300);
        readiness
            .positive_factors
            .push("math claim packet exists".into());
    } else {
        readiness
            .missing_refs
            .push(format!("claim:CLM_{}", report.theorem_id));
    }
    if state
        .reductions
        .iter()
        .any(|c| c.theorem_id == report.theorem_id)
    {
        readiness.readiness_score_basis_points =
            readiness.readiness_score_basis_points.saturating_add(250);
        readiness
            .positive_factors
            .push("reduction map exists".into());
    }
    if state
        .projections
        .iter()
        .any(|c| c.theorem_id == report.theorem_id)
    {
        readiness.readiness_score_basis_points =
            readiness.readiness_score_basis_points.saturating_add(250);
        readiness
            .positive_factors
            .push("projection map exists".into());
    }
    let bench_runs = state
        .benchmark_runs
        .iter()
        .filter(|b| b.theorem_id == report.theorem_id)
        .collect::<Vec<_>>();
    if !bench_runs.is_empty() {
        readiness.readiness_score_basis_points =
            readiness.readiness_score_basis_points.saturating_add(500);
        readiness
            .positive_factors
            .push(format!("benchmark runs={}", bench_runs.len()));
        if bench_runs
            .iter()
            .any(|b| matches!(b.status, RegistryEntryStatus::Blocked))
        {
            readiness
                .blockers
                .push("blocked benchmark run present".into());
        }
    }
    let subject_id = report
        .campaign_id
        .clone()
        .unwrap_or_else(|| report.theorem_id.clone());
    let coverage = state
        .coverage_dispatches
        .iter()
        .find(|c| c.subject_id == report.theorem_id || c.subject_id == subject_id);
    if let Some(coverage) = coverage {
        readiness
            .positive_factors
            .push(format!("coverage {:?}", coverage.decision));
        if coverage.route_fast_path {
            readiness.readiness_score_basis_points =
                readiness.readiness_score_basis_points.saturating_add(350);
            readiness
                .positive_factors
                .push("fast-path reuse is available".into());
        }
        if coverage
            .lineage_refs
            .iter()
            .any(|item| item == &lineage_ref_for_subject(&subject_id))
        {
            readiness.readiness_score_basis_points =
                readiness.readiness_score_basis_points.saturating_add(150);
            readiness
                .positive_factors
                .push("coverage lineage linked".into());
        } else {
            readiness.blockers.push("coverage lineage missing".into());
        }
        if matches!(coverage.decision, l64_core::CoverageDecision::Unsupported) {
            readiness.blockers.push("coverage unsupported".into());
        }
    } else {
        readiness
            .missing_refs
            .push(format!("coverage:{}", report.theorem_id));
    }
    let producer_host_count = state
        .producer_hosts
        .iter()
        .filter(|h| h.id.ends_with(&report.theorem_id))
        .count();
    if producer_host_count > 0 {
        readiness.readiness_score_basis_points = readiness
            .readiness_score_basis_points
            .saturating_add((producer_host_count.min(5) as u16) * 80);
        readiness
            .positive_factors
            .push(format!("producer hosts={producer_host_count}"));
    } else {
        readiness
            .missing_refs
            .push(format!("producer-hosts:{}", report.theorem_id));
    }
    let lineage_record = state
        ._lineage_records
        .iter()
        .find(|item| item.subject_id == subject_id || item.subject_id == report.theorem_id);
    if let Some(lineage) = lineage_record {
        readiness.readiness_score_basis_points =
            readiness.readiness_score_basis_points.saturating_add(400);
        readiness
            .positive_factors
            .push(format!("lineage {}", lineage.id));
        if lineage
            .phase_ledger
            .iter()
            .any(|entry| entry.validation_result != l64_core::PhaseValidationResult::Passed)
        {
            readiness
                .blockers
                .push("lineage validation not fully passed".into());
        }
    } else {
        readiness
            .missing_refs
            .push(lineage_ref_for_subject(&subject_id));
        readiness.blockers.push("lineage missing".into());
    }
    if let Some((_, tower)) = state
        .tower_bundles
        .iter()
        .find(|(id, _)| id == &report.theorem_id)
    {
        readiness.positive_factors.push(format!(
            "tower recipes={} promotion_candidates={}",
            tower.recipes.len(),
            tower.promotion_candidates.len()
        ));
        if tower.lineage_required
            && !tower.lineage_refs.iter().any(|item| {
                item == &format!("LIN_{}", report.theorem_id)
                    || item == &format!("LIN_{}", subject_id)
            })
        {
            readiness.blockers.push("tower lineage missing".into());
        }
        if !tower.help_requests.is_empty() {
            readiness
                .blockers
                .push(format!("help_requests={}", tower.help_requests.len()));
        }
        if tower
            .calibration_pressure
            .as_ref()
            .map(|c| c.override_pressure)
            .unwrap_or_default()
            > 0
        {
            readiness.blockers.push("override pressure remains".into());
        }
    }
    readiness.readiness_score_basis_points = readiness.readiness_score_basis_points.min(10_000);
    readiness.status = if !readiness.blockers.is_empty() {
        PromotionQueueStatus::Blocked
    } else if readiness.readiness_score_basis_points >= 8500 {
        PromotionQueueStatus::Ready
    } else {
        PromotionQueueStatus::Proposed
    };
    Ok(readiness)
}

pub fn derive_governed_handoff_packet_from_report(
    report: &CertificationReport,
    claim: &MathClaimPacket,
    operator: &OperatorRecord,
    strengthening: &[StrengtheningArtifact],
    repro: &ReproducibilityPacket,
    benchmarks: &[BenchmarkSchema],
    lineage: &ResearchLineageRecord,
    readiness: &PromotionReadinessReport,
    route_assignment: Option<&RouteAssignment>,
    benchmark_runs: &[BenchmarkRunRecord],
    producer_hosts: &[ProducerHostSpec],
    coverage: Option<&ProofCoverageDispatch>,
    tower: Option<&VerticalCompoundingBundle>,
) -> HandoffPacket {
    let mut packet = derive_handoff_packet_from_report(
        report,
        claim,
        operator,
        strengthening,
        repro,
        benchmarks,
    );
    packet.status = readiness.status;
    packet.readiness_score_basis_points = readiness.readiness_score_basis_points;
    packet.blockers = readiness.blockers.clone();
    packet.positive_factors = readiness.positive_factors.clone();
    packet.summary = format!(
        "governed handoff for {} status {:?} readiness {}",
        report.theorem_id, readiness.status, readiness.readiness_score_basis_points
    );
    packet.benchmark_run_refs = benchmark_runs.iter().map(|b| b.id.clone()).collect();
    packet.producer_host_refs = producer_hosts.iter().map(|h| h.id.clone()).collect();
    if let Some(route) = route_assignment {
        packet.route_refs.push(route.task_id.clone());
    }
    if let Some(coverage) = coverage {
        packet.coverage_refs.push(coverage.subject_id.clone());
    }
    packet.lineage_refs.push(lineage.id.clone());
    if tower.is_some() {
        packet.tower_refs.push(report.theorem_id.clone());
    }
    packet
        .promotion_refs
        .push(format!("PQU_{}", report.theorem_id));
    packet
}

pub fn govern_report_from_cold_result(
    report: &CertificationReport,
) -> Result<(
    TaskEnvelope,
    DerivationSignature,
    RouteAssignment,
    PromotionReadinessReport,
    HandoffPacket,
)> {
    let state = load_governance_state()?;
    let task = derive_task_envelope_from_report(report);
    let signature = derive_derivation_signature_from_report(report, &task);
    let route = score_routes_internal(
        &task,
        &signature,
        &state.challenges,
        &state.registry_entries,
        &state.reviews,
        &state.repro,
        &state.strengthening,
        &state.remediation,
        &state.benchmark_runs,
        &state.claims,
        &state.reductions,
        &state.projections,
        &state.handoffs,
        &state.producer_hosts,
        &state.promotion_queue,
        &state.coverage_dispatches,
        &state.tower_bundles,
    );
    let claim = derive_math_claim_packet_from_report(report);
    let operator = derive_operator_record_from_report(report);
    let strengthening = derive_strengthening_artifacts_from_report(report);
    let repro = derive_reproducibility_packet_from_report(report);
    let benchmarks = derive_benchmark_schemas_from_report(report);
    let readiness = derive_governed_promotion_readiness_from_report(report)?;
    let benchmark_runs = vec![derive_benchmark_run_record_from_report(
        report,
        &benchmarks.iter().map(|b| b.id.clone()).collect::<Vec<_>>(),
    )];
    let producer_hosts = derive_producer_host_specs_from_report(report);
    let lineage = derive_lineage_record_from_report(report);
    let coverage = state
        .coverage_dispatches
        .iter()
        .find(|c| c.subject_id == report.theorem_id);
    let tower = state
        .tower_bundles
        .iter()
        .find(|(id, _)| id == &report.theorem_id)
        .map(|(_, t)| t);
    let handoff = derive_governed_handoff_packet_from_report(
        report,
        &claim,
        &operator,
        &strengthening,
        &repro,
        &benchmarks,
        &lineage,
        &readiness,
        Some(&route),
        &benchmark_runs,
        &producer_hosts,
        coverage,
        tower,
    );
    Ok((task, signature, route, readiness, handoff))
}

pub fn derive_producer_host_specs_from_report(
    report: &CertificationReport,
) -> Vec<ProducerHostSpec> {
    vec![
        ProducerHostSpec {
            id: format!("HST_REDUCTION_{}", report.theorem_id),
            kind: ProducerHostKind::Reduction,
            purpose: format!("reduction host for {}", report.theorem_id),
            owned_objects: vec![format!("RDM_{}", report.theorem_id)],
            feeds: vec![
                format!("CLM_{}", report.theorem_id),
                format!("REG_{}", report.theorem_id),
            ],
            notes: vec!["owns reduction maps and residual-obligation compression".into()],
        },
        ProducerHostSpec {
            id: format!("HST_PROJECTION_{}", report.theorem_id),
            kind: ProducerHostKind::Projection,
            purpose: format!("projection host for {}", report.theorem_id),
            owned_objects: vec![format!("PRJ_{}", report.theorem_id)],
            feeds: vec![format!("CLM_{}", report.theorem_id)],
            notes: vec!["owns projection-algebra and bridge-facing summaries".into()],
        },
        ProducerHostSpec {
            id: format!("HST_BENCHMARK_{}", report.theorem_id),
            kind: ProducerHostKind::Benchmark,
            purpose: format!("benchmark host for {}", report.theorem_id),
            owned_objects: vec![format!("BMR_{}", report.theorem_id)],
            feeds: vec![format!("HOF_{}", report.theorem_id)],
            notes: vec!["owns benchmark schemas and benchmark-floor execution surfaces".into()],
        },
        ProducerHostSpec {
            id: format!("HST_CHALLENGE_{}", report.theorem_id),
            kind: ProducerHostKind::ChallengeResolution,
            purpose: format!("challenge host for {}", report.theorem_id),
            owned_objects: vec![format!("CHG_{}", report.theorem_id)],
            feeds: vec![format!("PQU_{}", report.theorem_id)],
            notes: vec![
                "owns contradiction handling, challenge response, and rollback narrowing".into(),
            ],
        },
        ProducerHostSpec {
            id: format!("HST_HANDOFF_{}", report.theorem_id),
            kind: ProducerHostKind::Handoff,
            purpose: format!("handoff host for {}", report.theorem_id),
            owned_objects: vec![format!("HOF_{}", report.theorem_id)],
            feeds: vec![
                format!("RPR_{}", report.theorem_id),
                format!("OPR_{}", report.theorem_id),
            ],
            notes: vec!["owns downstream integration packets and reusable exported law".into()],
        },
    ]
}

pub fn derive_promotion_queue_entry_from_report(
    report: &CertificationReport,
    readiness: &PromotionReadinessReport,
) -> PromotionQueueEntry {
    let mut required_refs = Vec::new();
    required_refs.push(format!("review:RVR_{}", report.theorem_id));
    required_refs.push(format!("repro:RPR_{}", report.theorem_id));
    let subject_id = report
        .campaign_id
        .clone()
        .unwrap_or_else(|| report.theorem_id.clone());
    required_refs.push(lineage_ref_for_subject(&subject_id));
    let mut evidence_refs = report.benchmark_receipt_ids.clone();
    evidence_refs.extend(report.challenge_receipt_ids.clone());
    evidence_refs.extend(report.payoff_receipt_ids.clone());
    PromotionQueueEntry {
        id: format!("PQU_{}", report.theorem_id),
        subject_id: report.theorem_id.clone(),
        status: readiness.status,
        readiness_score_basis_points: readiness.readiness_score_basis_points,
        blockers: readiness.blockers.clone(),
        required_refs,
        evidence_refs,
    }
}

pub fn derive_complete_research_bundle_from_report(
    report: &CertificationReport,
    reviews: &[ReviewReceipt],
    challenges: &[ChallengeRecord],
    repros: &[ReproducibilityPacket],
    strengthening_existing: &[StrengtheningArtifact],
    remediation: &[RemediationLedgerEntry],
) -> (
    ResearchBundle,
    FrameworkRegistryEntry,
    OperatorRecord,
    Vec<StrengtheningArtifact>,
    ReproducibilityPacket,
    ReviewReceipt,
    Option<ChallengeRecord>,
    PromotionReadinessReport,
) {
    let task = derive_task_envelope_from_report(report);
    let signature = derive_derivation_signature_from_report(report, &task);
    let route_assignment = score_routes(
        &task,
        &signature,
        challenges,
        &[],
        reviews,
        repros,
        strengthening_existing,
        remediation,
    );
    let claim = derive_math_claim_packet_from_report(report);
    let reduction = derive_reduction_map_record_from_report(report);
    let projection = derive_projection_map_record_from_report(report);
    let strengthening = derive_strengthening_artifacts_from_report(report);
    let repro = derive_reproducibility_packet_from_report(report);
    let review = derive_review_receipt_from_report(report);
    let challenge = derive_challenge_record_from_report(report);
    let entry = derive_framework_registry_entry_from_report(report);
    let operator = derive_operator_record_from_report(report);
    let benchmarks = derive_benchmark_schemas_from_report(report);
    let bench_ids = benchmarks.iter().map(|b| b.id.clone()).collect::<Vec<_>>();
    let benchmark_run = derive_benchmark_run_record_from_report(report, &bench_ids);
    let producer_hosts = derive_producer_host_specs_from_report(report);
    let lineage = derive_lineage_record_from_report(report);
    let mut all_reviews = reviews.to_vec();
    all_reviews.push(review.clone());
    let mut all_challenges = challenges.to_vec();
    if let Some(c) = challenge.clone() {
        all_challenges.push(c);
    }
    let mut all_repros = repros.to_vec();
    all_repros.push(repro.clone());
    let mut all_strengthening = strengthening_existing.to_vec();
    all_strengthening.extend(strengthening.clone());
    let readiness = derive_promotion_readiness_from_report(
        report,
        &all_reviews,
        &all_challenges,
        &all_repros,
        &all_strengthening,
        remediation,
    );
    let promotion_queue = derive_promotion_queue_entry_from_report(report, &readiness);
    let handoff = derive_governed_handoff_packet_from_report(
        report,
        &claim,
        &operator,
        &strengthening,
        &repro,
        &benchmarks,
        &lineage,
        &readiness,
        Some(&route_assignment),
        &[benchmark_run.clone()],
        &producer_hosts,
        None,
        None,
    );
    let bundle = ResearchBundle {
        tasks: vec![task],
        signatures: vec![signature],
        route_assignments: vec![route_assignment],
        claims: vec![claim],
        reduction_maps: vec![reduction],
        projection_maps: vec![projection],
        benchmark_runs: vec![benchmark_run],
        handoff_packets: vec![handoff],
        producer_hosts,
        promotion_queue: vec![promotion_queue],
        promotion_reports: vec![readiness.clone()],
        lineage_records: vec![lineage],
    };
    (
        bundle,
        entry,
        operator,
        strengthening,
        repro,
        review,
        challenge,
        readiness,
    )
}

pub fn derive_governed_complete_research_bundle_from_report(
    report: &CertificationReport,
) -> Result<(
    ResearchBundle,
    FrameworkRegistryEntry,
    OperatorRecord,
    Vec<StrengtheningArtifact>,
    ReproducibilityPacket,
    ReviewReceipt,
    Option<ChallengeRecord>,
    PromotionReadinessReport,
    HandoffPacket,
    RouteAssignment,
)> {
    let state = load_governance_state()?;
    let task = derive_task_envelope_from_report(report);
    let signature = derive_derivation_signature_from_report(report, &task);
    let route_assignment = score_routes_internal(
        &task,
        &signature,
        &state.challenges,
        &state.registry_entries,
        &state.reviews,
        &state.repro,
        &state.strengthening,
        &state.remediation,
        &state.benchmark_runs,
        &state.claims,
        &state.reductions,
        &state.projections,
        &state.handoffs,
        &state.producer_hosts,
        &state.promotion_queue,
        &state.coverage_dispatches,
        &state.tower_bundles,
    );
    let claim = derive_math_claim_packet_from_report(report);
    let reduction = derive_reduction_map_record_from_report(report);
    let projection = derive_projection_map_record_from_report(report);
    let strengthening = derive_strengthening_artifacts_from_report(report);
    let repro = derive_reproducibility_packet_from_report(report);
    let review = derive_review_receipt_from_report(report);
    let challenge = derive_challenge_record_from_report(report);
    let entry = derive_framework_registry_entry_from_report(report);
    let operator = derive_operator_record_from_report(report);
    let benchmarks = derive_benchmark_schemas_from_report(report);
    let bench_ids = benchmarks.iter().map(|b| b.id.clone()).collect::<Vec<_>>();
    let benchmark_run = derive_benchmark_run_record_from_report(report, &bench_ids);
    let producer_hosts = derive_producer_host_specs_from_report(report);
    let lineage = derive_lineage_record_from_report(report);
    let readiness = derive_governed_promotion_readiness_from_report(report)?;
    let coverage = state
        .coverage_dispatches
        .iter()
        .find(|c| c.subject_id == report.theorem_id);
    let tower = state
        .tower_bundles
        .iter()
        .find(|(id, _)| id == &report.theorem_id)
        .map(|(_, t)| t);
    let handoff = derive_governed_handoff_packet_from_report(
        report,
        &claim,
        &operator,
        &strengthening,
        &repro,
        &benchmarks,
        &lineage,
        &readiness,
        Some(&route_assignment),
        &[benchmark_run.clone()],
        &producer_hosts,
        coverage,
        tower,
    );
    let promotion_queue = derive_promotion_queue_entry_from_report(report, &readiness);
    let bundle = ResearchBundle {
        tasks: vec![task],
        signatures: vec![signature],
        route_assignments: vec![route_assignment.clone()],
        claims: vec![claim],
        reduction_maps: vec![reduction],
        projection_maps: vec![projection],
        benchmark_runs: vec![benchmark_run],
        handoff_packets: vec![handoff.clone()],
        producer_hosts,
        promotion_queue: vec![promotion_queue],
        promotion_reports: vec![readiness.clone()],
        lineage_records: vec![lineage],
    };
    Ok((
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
    ))
}

pub fn research_status_summary() -> Result<serde_json::Value> {
    let reviews = list_review_receipts()?;
    let challenges = list_challenge_records()?;
    let registry = list_framework_registry_entries()?;
    let operators = list_operator_records()?;
    let benchmarks = list_benchmark_schemas()?;
    let benchmark_runs = list_benchmark_run_records()?;
    let claims = list_math_claim_packets()?;
    let reductions = list_reduction_map_records()?;
    let projections = list_projection_map_records()?;
    let handoffs = list_handoff_packets()?;
    let producer_hosts = list_producer_host_specs()?;
    let promotion_queue = list_promotion_queue_entries()?;
    let promotion_reports = list_promotion_readiness_reports()?;
    let route_assignments = list_route_assignments()?;
    let strengthening = list_strengthening_artifacts()?;
    let remediation = list_remediation_entries()?;
    let repro = list_reproducibility_packets()?;
    let coverage = list_coverage_dispatches()?;
    let towers = list_vertical_bundles()?;
    let lineage = list_lineage_records()?;

    let open_challenges = challenges
        .iter()
        .filter_map(|id| load_challenge_record(id).ok())
        .filter(|c| matches!(c.status, ChallengeStatus::Open))
        .count();
    let blocking_challenges = challenges
        .iter()
        .filter_map(|id| load_challenge_record(id).ok())
        .filter(|c| matches!(c.severity, ChallengeSeverity::Blocking))
        .count();
    let blocked_registry = registry
        .iter()
        .filter_map(|id| load_framework_registry_entry(id).ok())
        .filter(|r| matches!(r.status, RegistryEntryStatus::Blocked))
        .count();
    let remediation_open = remediation
        .iter()
        .filter_map(|id| load_remediation_entry(id).ok())
        .filter(|r| {
            !matches!(
                r.status,
                RemediationStatus::Verified | RemediationStatus::Deferred
            )
        })
        .count();
    let ready_promotion = promotion_queue
        .iter()
        .filter_map(|id| load_promotion_queue_entry(id).ok())
        .filter(|q| matches!(q.status, PromotionQueueStatus::Ready))
        .count();
    let blocked_promotion = promotion_queue
        .iter()
        .filter_map(|id| load_promotion_queue_entry(id).ok())
        .filter(|q| matches!(q.status, PromotionQueueStatus::Blocked))
        .count();
    let ready_handoffs = handoffs
        .iter()
        .filter_map(|id| load_handoff_packet(id).ok())
        .filter(|h| matches!(h.status, PromotionQueueStatus::Ready))
        .count();
    let blocked_handoffs = handoffs
        .iter()
        .filter_map(|id| load_handoff_packet(id).ok())
        .filter(|h| matches!(h.status, PromotionQueueStatus::Blocked))
        .count();

    Ok(serde_json::json!({
        "counts": {
            "reviews": reviews.len(),
            "challenges": challenges.len(),
            "registry": registry.len(),
            "operators": operators.len(),
            "benchmarks": benchmarks.len(),
            "benchmark_runs": benchmark_runs.len(),
            "claims": claims.len(),
            "reductions": reductions.len(),
            "projections": projections.len(),
            "handoffs": handoffs.len(),
            "producer_hosts": producer_hosts.len(),
            "promotion_queue": promotion_queue.len(),
            "promotion_reports": promotion_reports.len(),
            "route_assignments": route_assignments.len(),
            "strengthening": strengthening.len(),
            "remediation": remediation.len(),
            "repro": repro.len(),
            "coverage_dispatch": coverage.len(),
            "tower_bundles": towers.len(),
            "lineage": lineage.len(),
        },
        "open": {
            "challenges": open_challenges,
            "blocking_challenges": blocking_challenges,
            "blocked_registry": blocked_registry,
            "remediation": remediation_open,
            "ready_promotion": ready_promotion,
            "blocked_promotion": blocked_promotion,
            "ready_handoffs": ready_handoffs,
            "blocked_handoffs": blocked_handoffs,
        },
        "ids": {
            "claims": claims,
            "registry": registry,
            "promotion_queue": promotion_queue,
            "promotion_reports": promotion_reports,
            "route_assignments": route_assignments,
            "handoff": handoffs,
            "producer_hosts": producer_hosts,
            "repro": repro,
            "coverage_dispatch": coverage,
            "tower_bundles": towers,
            "remediation": remediation,
            "lineage": lineage,
        }
    }))
}

pub fn load_json_file<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let text = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&text)?)
}

pub fn write_json_stdout<T: Serialize>(value: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

pub fn persist_research_import(kind: &str, path: &Path) -> Result<String> {
    match kind {
        "task" => {
            let item: TaskEnvelope = load_json_file(path)?;
            let id = item.id.clone();
            persist_task_envelope(&item)?;
            Ok(id)
        }
        "signature" => {
            let item: DerivationSignature = load_json_file(path)?;
            let id = item.id.clone();
            persist_derivation_signature(&item)?;
            Ok(id)
        }
        "review" => {
            let item: ReviewReceipt = load_json_file(path)?;
            let id = item.id.clone();
            persist_review_receipt(&item)?;
            Ok(id)
        }
        "challenge" => {
            let item: ChallengeRecord = load_json_file(path)?;
            let id = item.id.clone();
            persist_challenge_record(&item)?;
            Ok(id)
        }
        "registry" => {
            let item: FrameworkRegistryEntry = load_json_file(path)?;
            let id = item.id.clone();
            persist_framework_registry_entry(&item)?;
            Ok(id)
        }
        "operator" => {
            let item: OperatorRecord = load_json_file(path)?;
            let id = item.id.clone();
            persist_operator_record(&item)?;
            Ok(id)
        }
        "benchmark" => {
            let item: BenchmarkSchema = load_json_file(path)?;
            let id = item.id.clone();
            persist_benchmark_schema(&item)?;
            Ok(id)
        }
        "claim" => {
            let item: MathClaimPacket = load_json_file(path)?;
            let id = item.id.clone();
            persist_math_claim_packet(&item)?;
            Ok(id)
        }
        "reduction" => {
            let item: ReductionMapRecord = load_json_file(path)?;
            let id = item.id.clone();
            persist_reduction_map_record(&item)?;
            Ok(id)
        }
        "projection" => {
            let item: ProjectionMapRecord = load_json_file(path)?;
            let id = item.id.clone();
            persist_projection_map_record(&item)?;
            Ok(id)
        }
        "benchmark-run" => {
            let item: BenchmarkRunRecord = load_json_file(path)?;
            let id = item.id.clone();
            persist_benchmark_run_record(&item)?;
            Ok(id)
        }
        "handoff" => {
            let item: HandoffPacket = load_json_file(path)?;
            let id = item.id.clone();
            persist_handoff_packet(&item)?;
            Ok(id)
        }
        "producer-host" => {
            let item: ProducerHostSpec = load_json_file(path)?;
            let id = item.id.clone();
            persist_producer_host_spec(&item)?;
            Ok(id)
        }
        "route-assignment" => {
            let item: RouteAssignment = load_json_file(path)?;
            let id = item.task_id.clone();
            persist_route_assignment(&item)?;
            Ok(id)
        }
        "promotion-report" => {
            let item: PromotionReadinessReport = load_json_file(path)?;
            let id = item.subject_id.clone();
            persist_promotion_readiness_report(&item)?;
            Ok(id)
        }
        "promotion" => {
            let item: PromotionQueueEntry = load_json_file(path)?;
            let id = item.id.clone();
            persist_promotion_queue_entry(&item)?;
            Ok(id)
        }
        "strengthening" => {
            let item: StrengtheningArtifact = load_json_file(path)?;
            let id = item.id.clone();
            persist_strengthening_artifact(&item)?;
            Ok(id)
        }
        "repro" => {
            let item: ReproducibilityPacket = load_json_file(path)?;
            let id = item.id.clone();
            persist_reproducibility_packet(&item)?;
            Ok(id)
        }
        "remediation" => {
            let item: RemediationLedgerEntry = load_json_file(path)?;
            let id = item.id.clone();
            persist_remediation_entry(&item)?;
            Ok(id)
        }
        "coverage" => {
            let item: ProofCoverageDispatch = load_json_file(path)?;
            let id = item.subject_id.clone();
            persist_coverage_dispatch(&item)?;
            Ok(id)
        }
        "tower" => {
            let item: VerticalCompoundingBundle = load_json_file(path)?;
            let id = item
                .frontier_ledger
                .frontiers
                .first()
                .map(|f| f.id.clone())
                .unwrap_or_else(|| "tower".into());
            persist_vertical_bundle(&id, &item)?;
            Ok(id)
        }
        "lineage" => {
            let item: ResearchLineageRecord = load_json_file(path)?;
            let id = item.id.clone();
            persist_lineage_record(&item)?;
            Ok(id)
        }
        _ => Err(anyhow!("unknown research import kind `{kind}`")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use l64_core::{
        CertificationReport, CertificationVerdict, ChangeLedgerEntry,
        DeterministicExecutionEnvelope, GenomeArtifactClass, GenomeSurface, InvariantCheck,
        PhaseId, PhaseValidationResult, PromotionSignal, ReplayStatus,
    };

    #[test]
    fn lineage_record_roundtrip_persists_through_research_store() {
        let namespace = format!("test_lineage_roundtrip_{}", std::process::id());
        unsafe {
            std::env::set_var("MF_CACHE_NAMESPACE", &namespace);
        }

        let record = ResearchLineageRecord {
            id: "LINEAGE_TEST_1".into(),
            subject_id: "CHAIN_RULE".into(),
            artifact_class: GenomeArtifactClass::Gene,
            source_surface: GenomeSurface::Rna,
            target_surface: GenomeSurface::Dna,
            grammar_id: "rna.v1".into(),
            canonical_hash: "abc123".into(),
            lowering_receipt_id: "SLR_abc123".into(),
            phase_ids: vec![
                PhaseId::RnaNormalization,
                PhaseId::StructuralResolution,
                PhaseId::CanonicalNormalization,
            ],
            phase_ledger: vec![ChangeLedgerEntry {
                phase_id: PhaseId::RnaNormalization,
                input_state_hash: "in".into(),
                output_state_hash: Some("out".into()),
                dependency_edges: Vec::new(),
                invariant_checks: vec![InvariantCheck {
                    name: "deterministic".into(),
                    passed: true,
                    detail: "ok".into(),
                }],
                failure_records: Vec::new(),
                validation_result: PhaseValidationResult::Passed,
                promotion_signal: PromotionSignal::Promote,
                rollback_pointer: Some("retain raw RNA".into()),
            }],
            notes: vec!["test".into()],
        };

        persist_lineage_record(&record).expect("persist lineage");
        let loaded = load_lineage_record(&record.id).expect("load lineage");
        assert_eq!(loaded, record);
        let listed = list_lineage_records().expect("list lineage");
        assert!(listed.contains(&record.id));
    }

    #[test]
    fn report_derivation_emits_lineage_record() {
        let report = CertificationReport {
            theorem_id: "THS_CHAIN_RULE".into(),
            campaign_id: Some("CPG_CHAIN_RULE".into()),
            target_profile_id: "TP_CHAIN_RULE".into(),
            verdict: CertificationVerdict::Integrated,
            selected_atlas_cell: Some("CELL_CHAIN".into()),
            selected_path: vec!["B_TOP_TO_CALC".into()],
            route_class_id: Some("RLC_CHAIN".into()),
            certificate_id: Some("CERT_CHAIN".into()),
            candidates: Vec::new(),
            obligations: Vec::new(),
            reasons: vec!["ok".into()],
            diagnostics: Vec::new(),
            deficiencies: Vec::new(),
            adequacy_records: Vec::new(),
            checker_receipts: Vec::new(),
            burden_pack_ids: Vec::new(),
            claim_packet_ids: Vec::new(),
            evidence_contract_ids: Vec::new(),
            benchmark_receipt_ids: Vec::new(),
            challenge_receipt_ids: Vec::new(),
            reproducibility_packet_ids: vec!["RPK_CHAIN".into()],
            promotion_artifact_ids: Vec::new(),
            reused_artifact_ids: Vec::new(),
            default_selected_artifact_ids: Vec::new(),
            payoff_receipt_ids: Vec::new(),
            policy_resolution: None,
            route_explanation: None,
            execution_envelope: Some(DeterministicExecutionEnvelope {
                bundle_hash: "bundle".into(),
                bundle_id: None,
                policy_hash: "policy".into(),
                policy_resolution_id: None,
                manifest_id: None,
                lock_id: None,
                route_winner_hash: "winner".into(),
                obligation_replay_keys: Vec::new(),
                report_hash: "reporthash".into(),
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

        let lineage = derive_lineage_record_from_report(&report);
        assert_eq!(lineage.subject_id, "CPG_CHAIN_RULE");
        assert_eq!(lineage.source_surface, GenomeSurface::Dna);
        assert_eq!(lineage.target_surface, GenomeSurface::Dna);
        assert_eq!(lineage.canonical_hash, "reporthash");
        assert!(lineage.phase_ids.contains(&PhaseId::ResearchHostReconnect));
        assert_eq!(lineage.phase_ledger.len(), 1);
        assert_eq!(
            lineage.phase_ledger[0].validation_result,
            PhaseValidationResult::Passed
        );
    }

    #[test]
    fn promotion_and_handoff_include_lineage_requirements() {
        let report = CertificationReport {
            theorem_id: "THS_CHAIN_RULE".into(),
            campaign_id: Some("CPG_CHAIN_RULE".into()),
            target_profile_id: "TGT_CHAIN_RULE".into(),
            verdict: CertificationVerdict::Integrated,
            selected_atlas_cell: Some("CELL_CHAIN".into()),
            selected_path: vec!["B_TOP_TO_CALC".into()],
            route_class_id: Some("RLC_CHAIN".into()),
            certificate_id: Some("CERT_CHAIN".into()),
            candidates: Vec::new(),
            obligations: Vec::new(),
            reasons: vec!["ok".into()],
            diagnostics: Vec::new(),
            deficiencies: Vec::new(),
            adequacy_records: Vec::new(),
            checker_receipts: Vec::new(),
            burden_pack_ids: Vec::new(),
            claim_packet_ids: Vec::new(),
            evidence_contract_ids: Vec::new(),
            benchmark_receipt_ids: Vec::new(),
            challenge_receipt_ids: Vec::new(),
            reproducibility_packet_ids: vec!["RPK_CHAIN".into()],
            promotion_artifact_ids: Vec::new(),
            reused_artifact_ids: Vec::new(),
            default_selected_artifact_ids: Vec::new(),
            payoff_receipt_ids: Vec::new(),
            policy_resolution: None,
            route_explanation: None,
            execution_envelope: Some(DeterministicExecutionEnvelope {
                bundle_hash: "bundle".into(),
                bundle_id: None,
                policy_hash: "policy".into(),
                policy_resolution_id: None,
                manifest_id: None,
                lock_id: None,
                route_winner_hash: "winner".into(),
                obligation_replay_keys: Vec::new(),
                report_hash: "reporthash".into(),
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

        let lineage = derive_lineage_record_from_report(&report);
        let readiness = PromotionReadinessReport {
            subject_id: report.theorem_id.clone(),
            readiness_score_basis_points: 9000,
            status: PromotionQueueStatus::Ready,
            positive_factors: vec!["lineage present".into()],
            blockers: Vec::new(),
            missing_refs: Vec::new(),
        };
        let claim = derive_math_claim_packet_from_report(&report);
        let operator = derive_operator_record_from_report(&report);
        let strengthening = derive_strengthening_artifacts_from_report(&report);
        let repro = derive_reproducibility_packet_from_report(&report);
        let benchmarks = derive_benchmark_schemas_from_report(&report);
        let benchmark_runs = vec![derive_benchmark_run_record_from_report(
            &report,
            &benchmarks.iter().map(|b| b.id.clone()).collect::<Vec<_>>(),
        )];
        let producer_hosts = derive_producer_host_specs_from_report(&report);
        let route_assignment = RouteAssignment {
            task_id: "TSK_THS_CHAIN_RULE".into(),
            signature_id: "SIG_THS_CHAIN_RULE".into(),
            recommended_route: ResearchRouteClass::Integrate,
            scores: Vec::new(),
        };

        let promotion = derive_promotion_queue_entry_from_report(&report, &readiness);
        assert!(
            promotion
                .required_refs
                .iter()
                .any(|item| item == "lineage:LIN_CPG_CHAIN_RULE")
        );

        let handoff = derive_governed_handoff_packet_from_report(
            &report,
            &claim,
            &operator,
            &strengthening,
            &repro,
            &benchmarks,
            &lineage,
            &readiness,
            Some(&route_assignment),
            &benchmark_runs,
            &producer_hosts,
            None,
            None,
        );
        assert!(
            handoff
                .lineage_refs
                .iter()
                .any(|item| item == "LIN_CPG_CHAIN_RULE")
        );
    }
}
