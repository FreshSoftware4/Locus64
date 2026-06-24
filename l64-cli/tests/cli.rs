use assert_cmd::Command;
use l64_core::{
    LocusCapabilityMask, LocusOpcode, LocusPacketKind, QaDocument, QaEntry, decode_locus_packet,
};
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn write_fixture(name: &str, content: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("l64_cli_tests");
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!(
        "{}_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
        name
    ));
    fs::write(&path, content).unwrap();
    path
}

fn write_bundle_dna_fixture(name: &str, document: &QaDocument) -> PathBuf {
    let bytes = l64_locus::encode_section_packet(
        LocusPacketKind::CanonicalTransfer,
        LocusOpcode::CanonicalPayload,
        "BND_TEST",
        "bundle_document.v1",
        document,
        LocusCapabilityMask::default(),
        1,
    )
    .unwrap();
    let path = write_fixture(name, "");
    fs::write(&path, bytes).unwrap();
    path
}

fn qa_entry(kind: &str, payload: &str) -> QaEntry {
    QaEntry::from_surface_json(kind, payload).unwrap()
}

fn bundle_document_from_text(bundle: &str) -> QaDocument {
    let entries = bundle
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with("!l64-bundle") {
                return None;
            }
            let (kind, payload) = line.split_once(' ').unwrap();
            Some(qa_entry(kind, payload))
        })
        .collect();
    QaDocument { entries }
}

fn chain_rule_bundle() -> &'static str {
    concat!(
        "!l64-bundle v1\n",
        "policy-object {\"id\":\"MOP_BND_CHAIN_RULE_CLI_SCHED\",\"kind\":\"ReportExport\",\"scope\":{\"Bundle\":\"BND_CHAIN_RULE_CLI\"},\"extends\":null,\"optimizer\":null,\"evaluator\":null,\"replay_cache\":null,\"report\":{\"export_surfaces\":[],\"include_policy_trace\":true,\"include_route_explanation\":true,\"include_obligation_logs\":true},\"scheduler\":{\"parallelization\":\"ParallelIndependent\",\"max_workers\":2,\"allow_parallel_replay\":true,\"allow_parallel_certification\":true,\"allow_parallel_exports\":true,\"deterministic_ordering\":true,\"allow_parallel_obligations\":true,\"max_obligation_workers\":3,\"allow_parallel_obligation_replay\":true,\"serialize_canonicalization_sensitive\":true},\"canonicalizer_mode\":null,\"merge_policy\":null,\"notes\":[\"cli flagship chain rule scheduler\"]}\n",
        "theorem {\"id\":\"THS_CHAIN_RULE\",\"statement\":\"DER(g∘f,x) ≈1 DER(g,f(x))∘DER(f,x)\",\"hosts\":[\"R_TOP\",\"R_CALC\"],\"bridges\":[\"B_TOP_TO_CALC\"],\"operators\":[\"OPR.Chain1\"],\"target_equivalence\":\"first-order jet equivalence\",\"obligations\":[\"OblEq\",\"OblAdm\",\"OblLoc\",\"OblRed\"],\"primary_zone\":\"PmzStructural\",\"verdict\":\"Benchmarked\",\"proof_shapes\":[\"PS_SQUARE_TOPO\"]}\n",
        "obligation {\"id\":\"OBL_CHAIN_EQ\",\"kind\":\"OblEq\",\"description\":\"first-order slack equivalence preserved under composition\",\"status\":\"Benchmarked\"}\n",
        "obligation {\"id\":\"OBL_CHAIN_ADM\",\"kind\":\"OblAdm\",\"description\":\"both derivatives admitted in R_CALC\",\"status\":\"Benchmarked\"}\n",
        "obligation {\"id\":\"OBL_CHAIN_LOC\",\"kind\":\"OblLoc\",\"description\":\"same brace-localization at x\",\"status\":\"Benchmarked\"}\n",
        "obligation {\"id\":\"OBL_CHAIN_RED\",\"kind\":\"OblRed\",\"description\":\"reduction of remainder terms preserves first-order equivalence\",\"status\":\"RouteFound\"}\n",
        "target {\"id\":\"TGT_CHAIN_RULE\",\"burden_class\":\"DerivativeLocalWitnessExtraction\",\"host_cluster\":[\"R_TOP\",\"R_CALC\"],\"target_equivalence\":\"first-order jet equivalence\",\"allowed_bridge_classes\":[\"Enriching\",\"Conservative\"],\"loss_ceiling\":1,\"rollback_ceiling\":1,\"required_receipt_class\":\"RC_ths\",\"required_proof_shape_family\":\"MixedBattery\",\"promotion_goal\":\"PromoteOperator\",\"primary_zone\":\"PmzStructural\",\"surface_requirement\":null,\"preferred_surface_target\":null,\"optimizer_policy\":null,\"policy_binding_ids\":[]}\n",
        "ledger {\"id\":\"TRL_CHAIN_RULE\",\"theorem\":\"THS_CHAIN_RULE\",\"paths\":[[\"B_TOP_TO_CALC\"]],\"budget\":{\"max_loss\":1,\"allow_lossy_supported\":false,\"require_proof\":true},\"losses\":[],\"receipts\":[\"Ref_1\",\"Can\",\"Red\"],\"normalized_path\":[\"B_TOP_TO_CALC\"]}\n",
        "certificate {\"id\":\"CRT_CHAIN_RULE\",\"theorem\":\"THS_CHAIN_RULE\",\"route_ledger\":\"TRL_CHAIN_RULE\",\"proof_shapes\":[\"PS_SQUARE_TOPO\"],\"receipts\":[\"RC_ths\",\"TR_chain\",\"BENCH\"],\"verdict\":\"Benchmarked\"}\n",
        "campaign {\"id\":\"CPG_CHAIN_RULE\",\"theorem\":\"THS_CHAIN_RULE\",\"target_profile\":\"TGT_CHAIN_RULE\",\"route_ledger\":\"TRL_CHAIN_RULE\",\"obligations\":[\"OBL_CHAIN_EQ\",\"OBL_CHAIN_ADM\",\"OBL_CHAIN_LOC\",\"OBL_CHAIN_RED\"],\"certificates\":[\"CRT_CHAIN_RULE\"],\"dependencies\":[],\"campaign_class\":\"COperator\",\"verdict\":\"Benchmarked\",\"payoff\":[\"OPR.Chain1\",\"ATL.RouteClass.Chain\"]}\n",
        "diagnostic {\"id\":\"DGN_CHAIN_RULE_ADEQUACY\",\"class\":\"DNoAdequacy\",\"atlas_cell\":\"A_TOP_TO_CALC\",\"theorem\":\"THS_CHAIN_RULE\",\"message\":\"semantic adequacy remains scaffolded rather than discharged\"}\n"
    )
}

fn integrated_chain_rule_bundle() -> String {
    format!(concat!(
        "!l64-bundle v1\n",
        "policy-object {{\"id\":\"MOP_BND_CHAIN_RULE_INT_SCHED\",\"kind\":\"ReportExport\",\"scope\":{{\"Bundle\":\"BND_CHAIN_RULE_INT\"}},\"extends\":null,\"optimizer\":null,\"evaluator\":null,\"replay_cache\":null,\"report\":{{\"export_surfaces\":[],\"include_policy_trace\":true,\"include_route_explanation\":true,\"include_obligation_logs\":true}},\"scheduler\":{{\"parallelization\":\"ParallelIndependent\",\"max_workers\":2,\"allow_parallel_replay\":true,\"allow_parallel_certification\":true,\"allow_parallel_exports\":true,\"deterministic_ordering\":true,\"allow_parallel_obligations\":true,\"max_obligation_workers\":3,\"allow_parallel_obligation_replay\":true,\"serialize_canonicalization_sensitive\":true}},\"canonicalizer_mode\":null,\"merge_policy\":null,\"notes\":[\"cli integrated chain rule scheduler\"]}}\n",
        "object {{\"id\":\"OPR_PROMOTED_OPR_CHAIN1\",\"identity\":{{\"tag\":\"OPR\",\"cid\":\"cid:OPR_PROMOTED_OPR_CHAIN1\",\"codebook\":\"QC0_CORE\",\"remap\":\"none\",\"lineage\":\"derived-from:THS_CHAIN_RULE\"}},\"structural\":{{\"head\":\"operator\",\"args\":[\"THS_CHAIN_RULE\",\"CPG_CHAIN_RULE\"],\"local_sections\":[\"first-order derivative composition\"],\"morphism_hooks\":[\"B_TOP_TO_CALC\"]}},\"constraint\":{{\"regime\":\"R_CALC\",\"contracts\":[\"chain-rule\",\"first-order\"],\"invariants\":[\"jet-compose\",\"reduction-exact\"],\"equivalence\":\"first-order jet equivalence\",\"admissibility\":\"promoted after exact certified discharge\"}},\"evidence\":{{\"evidence_class\":\"DerivedPromotion\",\"traces\":[\"THS_CHAIN_RULE\"],\"receipts\":[\"CRT_CHAIN_RULE\",\"REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE\"],\"maturity\":\"Certified\",\"gate_verdict\":\"Pass\"}},\"alias\":{{\"aliases\":[\"OPR.Chain1\"],\"profile_pack\":[\"STD\",\"chain-rule\"],\"qm_binding\":\"THS·ChainRule\",\"qa_binding\":\"OPR.Chain1\",\"projection_policy\":\"canonical-authored\"}}}}\n",
        "theorem {{\"id\":\"THS_CHAIN_RULE\",\"statement\":\"DER(g∘f,x) ≈1 DER(g,f(x))∘DER(f,x)\",\"hosts\":[\"R_TOP\",\"R_CALC\"],\"bridges\":[\"B_TOP_TO_CALC\"],\"operators\":[\"OPR.Chain1\"],\"target_equivalence\":\"first-order jet equivalence\",\"obligations\":[\"OblEq\",\"OblAdm\",\"OblLoc\",\"OblRed\"],\"primary_zone\":\"PmzStructural\",\"verdict\":\"Benchmarked\",\"proof_shapes\":[\"PS_SQUARE_TOPO\"]}}\n",
        "obligation {{\"id\":\"OBL_CHAIN_EQ\",\"kind\":\"OblEq\",\"description\":\"first-order slack equivalence preserved under composition\",\"status\":\"Benchmarked\"}}\n",
        "obligation {{\"id\":\"OBL_CHAIN_ADM\",\"kind\":\"OblAdm\",\"description\":\"both derivatives admitted in R_CALC\",\"status\":\"Benchmarked\"}}\n",
        "obligation {{\"id\":\"OBL_CHAIN_LOC\",\"kind\":\"OblLoc\",\"description\":\"same brace-localization at x\",\"status\":\"Benchmarked\"}}\n",
        "obligation {{\"id\":\"OBL_CHAIN_RED\",\"kind\":\"OblRed\",\"description\":\"reduction of remainder terms preserves first-order equivalence\",\"status\":\"RouteFound\"}}\n",
        "target {{\"id\":\"TGT_CHAIN_RULE\",\"burden_class\":\"DerivativeLocalWitnessExtraction\",\"host_cluster\":[\"R_TOP\",\"R_CALC\"],\"target_equivalence\":\"first-order jet equivalence\",\"allowed_bridge_classes\":[\"Enriching\",\"Conservative\"],\"loss_ceiling\":1,\"rollback_ceiling\":1,\"required_receipt_class\":\"RC_ths\",\"required_proof_shape_family\":\"MixedBattery\",\"promotion_goal\":\"PromoteOperator\",\"primary_zone\":\"PmzStructural\",\"surface_requirement\":null,\"preferred_surface_target\":null,\"optimizer_policy\":null,\"policy_binding_ids\":[]}}\n",
        "ledger {{\"id\":\"TRL_CHAIN_RULE\",\"theorem\":\"THS_CHAIN_RULE\",\"paths\":[[\"B_TOP_TO_CALC\"]],\"budget\":{{\"max_loss\":1,\"allow_lossy_supported\":false,\"require_proof\":true}},\"losses\":[],\"receipts\":[\"Ref_1\",\"Can\",\"Red\"],\"normalized_path\":[\"B_TOP_TO_CALC\"]}}\n",
        "certificate {{\"id\":\"CRT_CHAIN_RULE\",\"theorem\":\"THS_CHAIN_RULE\",\"route_ledger\":\"TRL_CHAIN_RULE\",\"proof_shapes\":[\"PS_SQUARE_TOPO\"],\"receipts\":[\"RC_ths\",\"TR_chain\",\"BENCH\"],\"verdict\":\"Benchmarked\"}}\n",
        "campaign {{\"id\":\"CPG_CHAIN_RULE\",\"theorem\":\"THS_CHAIN_RULE\",\"target_profile\":\"TGT_CHAIN_RULE\",\"route_ledger\":\"TRL_CHAIN_RULE\",\"obligations\":[\"OBL_CHAIN_EQ\",\"OBL_CHAIN_ADM\",\"OBL_CHAIN_LOC\",\"OBL_CHAIN_RED\"],\"certificates\":[\"CRT_CHAIN_RULE\"],\"dependencies\":[],\"campaign_class\":\"COperator\",\"verdict\":\"Benchmarked\",\"payoff\":[\"OPR.Chain1\",\"ATL.RouteClass.Chain\"]}}\n",
        "diagnostic {{\"id\":\"DGN_CHAIN_RULE_ADEQUACY\",\"class\":\"DNoAdequacy\",\"atlas_cell\":\"A_TOP_TO_CALC\",\"theorem\":\"THS_CHAIN_RULE\",\"message\":\"semantic adequacy remains scaffolded rather than discharged\"}}\n"
    ))
}

fn broken_chain_rule_bridge_bundle() -> &'static str {
    concat!(
        "!l64-bundle v1\n",
        "theorem {\"id\":\"THS_CHAIN_RULE\",\"statement\":\"DER(g∘f,x) ≈1 DER(g,f(x))∘DER(f,x)\",\"hosts\":[\"R_TOP\",\"R_CALC\"],\"bridges\":[\"B_TOP_TO_CALC\"],\"operators\":[\"OPR.Chain1\"],\"target_equivalence\":\"first-order jet equivalence\",\"obligations\":[\"OblEq\",\"OblAdm\",\"OblLoc\",\"OblRed\"],\"primary_zone\":\"PmzStructural\",\"verdict\":\"Benchmarked\",\"proof_shapes\":[\"PS_SQUARE_TOPO\"]}\n",
        "obligation {\"id\":\"OBL_CHAIN_EQ\",\"kind\":\"OblEq\",\"description\":\"first-order slack equivalence preserved under composition\",\"status\":\"Benchmarked\"}\n",
        "obligation {\"id\":\"OBL_CHAIN_ADM\",\"kind\":\"OblAdm\",\"description\":\"both derivatives admitted in R_CALC\",\"status\":\"Benchmarked\"}\n",
        "obligation {\"id\":\"OBL_CHAIN_LOC\",\"kind\":\"OblLoc\",\"description\":\"same brace-localization at x\",\"status\":\"Benchmarked\"}\n",
        "obligation {\"id\":\"OBL_CHAIN_RED\",\"kind\":\"OblRed\",\"description\":\"reduction of remainder terms preserves first-order equivalence\",\"status\":\"RouteFound\"}\n",
        "target {\"id\":\"TGT_CHAIN_RULE\",\"burden_class\":\"DerivativeLocalWitnessExtraction\",\"host_cluster\":[\"R_TOP\",\"R_CALC\"],\"target_equivalence\":\"first-order jet equivalence\",\"allowed_bridge_classes\":[\"Enriching\",\"Conservative\"],\"loss_ceiling\":1,\"rollback_ceiling\":1,\"required_receipt_class\":\"RC_ths\",\"required_proof_shape_family\":\"MixedBattery\",\"promotion_goal\":\"PromoteOperator\",\"primary_zone\":\"PmzStructural\",\"surface_requirement\":null,\"preferred_surface_target\":null,\"optimizer_policy\":null,\"policy_binding_ids\":[]}\n",
        "ledger {\"id\":\"TRL_CHAIN_RULE\",\"theorem\":\"THS_CHAIN_RULE\",\"paths\":[[\"B_TOP_TO_CALC\"]],\"budget\":{\"max_loss\":1,\"allow_lossy_supported\":false,\"require_proof\":true},\"losses\":[],\"receipts\":[\"Can\",\"Red\"],\"normalized_path\":[\"B_TOP_TO_CALC\"]}\n",
        "certificate {\"id\":\"CRT_CHAIN_RULE\",\"theorem\":\"THS_CHAIN_RULE\",\"route_ledger\":\"TRL_CHAIN_RULE\",\"proof_shapes\":[\"PS_SQUARE_TOPO\"],\"receipts\":[\"RC_ths\",\"TR_chain\",\"BENCH\"],\"verdict\":\"Benchmarked\"}\n",
        "campaign {\"id\":\"CPG_CHAIN_RULE\",\"theorem\":\"THS_CHAIN_RULE\",\"target_profile\":\"TGT_CHAIN_RULE\",\"route_ledger\":\"TRL_CHAIN_RULE\",\"obligations\":[\"OBL_CHAIN_EQ\",\"OBL_CHAIN_ADM\",\"OBL_CHAIN_LOC\",\"OBL_CHAIN_RED\"],\"certificates\":[\"CRT_CHAIN_RULE\"],\"dependencies\":[],\"campaign_class\":\"COperator\",\"verdict\":\"Benchmarked\",\"payoff\":[\"OPR.Chain1\",\"ATL.RouteClass.Chain\"]}\n"
    )
}

fn test_namespace(prefix: &str) -> String {
    format!(
        "{}_{}",
        prefix,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    )
}

#[test]
fn standalone_projection_leaf_commands_are_removed() {
    let path = write_fixture(
        "sample.projection",
        "object [tag=CTX;cid=OBJ_CTX_SET;codebook=GEN1;remap=none;lineage=seed]<head=carrier;args=set;locals=base;hooks=identity>[regime=R_SET;contracts=carrier-total;invariants=extensional-stable;equivalence=eq-set;admissibility=admit-basic]{evidence_class=Seed;traces=T_set;receipts=RC_set;maturity=Validated;gate_verdict=Pass}<<aliases=ctx_set;profiles=STD;qm_binding=reserved;qa_binding=ctx_set;projection_policy=qa-first>>\n",
    );

    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args(["normalize", path.to_str().unwrap(), "--as", "qa0"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unrecognized subcommand"));
}

#[test]
fn select_route_uses_seed_atlas_winner() {
    Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args([
            "select-route",
            "--src",
            "R_TYP",
            "--tgt",
            "R_SET",
            "--proof-target",
            "extensional carrier reasoning",
        ])
        .assert()
        .success();
}

#[test]
fn certify_campaign_returns_report() {
    Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args(["certify", "--campaign", "CPG_CHAIN_RULE"])
        .assert()
        .success();
}

#[test]
fn certify_derived_campaign_returns_report() {
    Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args(["certify-derived", "--campaign", "CPG_CHAIN_RULE"])
        .assert()
        .success();
}

#[test]
fn compile_atlas_succeeds() {
    Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args(["compile-atlas"])
        .assert()
        .success();
}

#[test]
fn rna_dna_primary_authority_commands_work() {
    let rna_path = write_fixture("primary.gene.rna", "ι ≔ σ ‖ κ\n");
    let dna_path = rna_path.with_extension("gene.dna");
    let canonical_rna_path = rna_path.with_extension("canonical.gene.rna");
    let recompiled_dna_path = rna_path.with_extension("recompiled.gene.dna");

    Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args([
            "compile-rna",
            rna_path.to_str().unwrap(),
            "--out",
            dna_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert!(dna_path.exists());
    assert!(!fs::read(&dna_path).unwrap().is_empty());

    let sequence_output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args(["sequence-dna", dna_path.to_str().unwrap()])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let canonical_rna = String::from_utf8(sequence_output).unwrap();
    assert!(!canonical_rna.trim_start().starts_with('{'));
    fs::write(&canonical_rna_path, canonical_rna).unwrap();

    Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args([
            "compile-rna",
            canonical_rna_path.to_str().unwrap(),
            "--out",
            recompiled_dna_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    let original_packet = decode_locus_packet(&fs::read(&dna_path).unwrap()).unwrap();
    let recompiled_packet = decode_locus_packet(&fs::read(&recompiled_dna_path).unwrap()).unwrap();
    assert_eq!(
        original_packet.header.integrity_hash,
        recompiled_packet.header.integrity_hash
    );

    Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args(["verify-roundtrip", rna_path.to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn cki_registry_fixture_preserves_rna_dna_fixed_point() {
    let source_path = workspace_root()
        .join("fixtures")
        .join("cki_registry.genome.rna");
    assert!(source_path.exists(), "controlled CKI fixture is missing");
    let out_dir = std::env::temp_dir().join(format!(
        "l64_cki_fixture_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&out_dir).unwrap();
    let dna_path = out_dir.join("cki_registry.genome.dna");
    let canonical_rna_path = out_dir.join("cki_registry.canonical.rna");
    let recompiled_dna_path = out_dir.join("cki_registry.recompiled.dna");

    Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args([
            "compile-rna",
            source_path.to_str().unwrap(),
            "--artifact-class",
            "genome",
            "--out",
            dna_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    let canonical_rna = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args(["sequence-dna", dna_path.to_str().unwrap()])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let canonical_rna = String::from_utf8(canonical_rna).unwrap();
    assert!(!canonical_rna.trim_start().starts_with('{'));
    fs::write(&canonical_rna_path, canonical_rna).unwrap();

    Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args([
            "compile-rna",
            canonical_rna_path.to_str().unwrap(),
            "--artifact-class",
            "genome",
            "--out",
            recompiled_dna_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    let original_packet = decode_locus_packet(&fs::read(&dna_path).unwrap()).unwrap();
    let recompiled_packet = decode_locus_packet(&fs::read(&recompiled_dna_path).unwrap()).unwrap();
    assert_eq!(
        original_packet.header.integrity_hash,
        recompiled_packet.header.integrity_hash
    );
}

#[test]
fn inspect_dna_output_is_not_rna_source() {
    let rna_path = write_fixture("inspect-source.gene.rna", "ι ≔ σ ‖ κ\n");
    let dna_path = rna_path.with_extension("gene.dna");
    let inspection_path = rna_path.with_extension("inspection.gene.rna");
    let rejected_dna_path = rna_path.with_extension("rejected.gene.dna");

    Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args([
            "compile-rna",
            rna_path.to_str().unwrap(),
            "--out",
            dna_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    let inspection_output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args(["inspect-dna", dna_path.to_str().unwrap()])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let inspection = String::from_utf8(inspection_output).unwrap();
    assert!(inspection.trim_start().starts_with('{'));
    fs::write(&inspection_path, inspection).unwrap();

    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args([
            "compile-rna",
            inspection_path.to_str().unwrap(),
            "--out",
            rejected_dna_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("source RNA only") || stderr.contains("projection/report"));
}

#[test]
fn genome_release_exports_coordinate_spine_and_rejects_views_as_source() {
    let rna_path = write_fixture("release-source.gene.rna", "ι ≔ σ ‖ κ\n");
    let release_dir = std::env::temp_dir().join(format!(
        "l64_release_test_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let rejected_dna_path = release_dir.join("rejected.dna");

    Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args([
            "export-genome-release",
            "--rna",
            rna_path.to_str().unwrap(),
            "--out",
            release_dir.to_str().unwrap(),
        ])
        .assert()
        .success();

    let manifest = release_dir.join("release_manifest.record");
    let subject_id = rna_path.file_stem().unwrap().to_str().unwrap();
    let source_rna = release_dir
        .join("genome")
        .join(format!("{subject_id}.source.rna"));
    let canonical_rna = release_dir
        .join("genome")
        .join(format!("{subject_id}.canonical.rna"));
    let non_source_artifacts = [
        manifest,
        release_dir
            .join("claims")
            .join(format!("{subject_id}.claim")),
        release_dir
            .join("spine")
            .join("dependency_spine.projection"),
        release_dir.join("spine").join("closure_map.projection"),
        release_dir.join("spine").join("lineage.record"),
        release_dir
            .join("frontier")
            .join("closure_frontier.projection"),
        release_dir.join("frontier").join("stress_map.projection"),
        release_dir.join("replay").join("replay_record.record"),
        release_dir.join("views").join("overview.md"),
        release_dir
            .join("views")
            .join("view_receipts")
            .join("overview.view.rcp"),
    ];
    assert!(source_rna.exists());
    assert!(canonical_rna.exists());

    for source in [&source_rna, &canonical_rna] {
        let accepted_dna_path = release_dir.join("accepted.dna");
        Command::cargo_bin("l64-cli")
            .unwrap()
            .current_dir(workspace_root())
            .args([
                "compile-rna",
                source.to_str().unwrap(),
                "--out",
                accepted_dna_path.to_str().unwrap(),
            ])
            .assert()
            .success();
    }

    for artifact in &non_source_artifacts {
        assert!(
            artifact.exists(),
            "missing release artifact: {}",
            artifact.display()
        );
        let output = Command::cargo_bin("l64-cli")
            .unwrap()
            .current_dir(workspace_root())
            .args([
                "compile-rna",
                artifact.to_str().unwrap(),
                "--out",
                rejected_dna_path.to_str().unwrap(),
            ])
            .output()
            .unwrap();
        assert!(
            !output.status.success(),
            "release artifact should not compile as source: {}",
            artifact.display()
        );
    }
}

#[test]
fn compile_bundle_moves_projection_fixture_to_dna() {
    let namespace = test_namespace("cli_compile_bundle");
    let source_path = write_fixture("chain_rule_bundle.locus.rna", chain_rule_bundle());
    let dna_path = source_path.with_extension("dna");

    Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args([
            "compile-bundle",
            source_path.to_str().unwrap(),
            "--out",
            dna_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert!(dna_path.exists());
    assert!(!fs::read(&dna_path).unwrap().is_empty());

    Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", &namespace)
        .args([
            "certify-bundle",
            "--file",
            dna_path.to_str().unwrap(),
            "--conflict-policy",
            "exact-match",
        ])
        .assert()
        .success();
}

#[test]
fn theorem_and_campaign_dna_bundle_execution_work() {
    let namespace = test_namespace("cli_theorem_campaign");
    let document = bundle_document_from_text(chain_rule_bundle());
    let bundle_path = write_bundle_dna_fixture("theorem_campaign.dna", &document);

    Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", &namespace)
        .args(["run-theorem", "--file", bundle_path.to_str().unwrap()])
        .assert()
        .success();

    Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", &namespace)
        .args(["certify-derived", "--file", bundle_path.to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn bundle_commands_work_for_overlay_bundle() {
    let namespace = test_namespace("cli_overlay_bundle");
    let overlay_document = QaDocument {
        entries: vec![
            qa_entry(
                "regime",
                r#"{"id":"R_TOP","ctx_law":"ctx","cut_law":"cut","thr_law":"thr","brc_law":"brc","slk_law":"slk","tol_law":"tol","knt_law":"knt","eq_law":"eq","adm_law":"adm","promoted_ops":[]}"#,
            ),
            qa_entry(
                "regime",
                r#"{"id":"R_CALC","ctx_law":"ctx","cut_law":"cut","thr_law":"thr","brc_law":"brc","slk_law":"slk","tol_law":"tol","knt_law":"knt","eq_law":"eq","adm_law":"adm","promoted_ops":[]}"#,
            ),
            qa_entry(
                "bridge",
                r#"{"id":"B_LOCAL_TOP_TO_CALC","src":"R_TOP","tgt":"R_CALC","id_pres":"local-identity","eq_pres":"local-eq","forget":[],"enrich":["derivative witness"],"loss":[],"reversibility":"Enriching","receipts":["RC_LOCAL"],"rollback":"allowed"}"#,
            ),
            qa_entry(
                "proof",
                r#"{"id":"PS_LOCAL_SQUARE","kind":"Square","nodes":["top","calc"],"edges":[{"from":"top","to":"calc","label":"derive"}],"equations":["derive=derive"],"target_equivalence":"eq","receipts":["RC_LOCAL"],"gate":"Pass"}"#,
            ),
            qa_entry(
                "theorem",
                r#"{"id":"THS_LOCAL_BUNDLE","statement":"local overlay theorem","hosts":["R_TOP","R_CALC"],"bridges":["B_LOCAL_TOP_TO_CALC"],"operators":["OPR.Local"],"target_equivalence":"eq","obligations":["OblLoc"],"primary_zone":"PmzStructural","verdict":"RouteFound","proof_shapes":["PS_LOCAL_SQUARE"]}"#,
            ),
            qa_entry(
                "obligation",
                r#"{"id":"OBL_LOCAL_LOC","kind":"OblLoc","description":"local compatibility","status":"RouteFound"}"#,
            ),
            qa_entry(
                "target",
                r#"{"id":"TGT_LOCAL_BUNDLE","burden_class":"DerivativeLocalWitnessExtraction","host_cluster":["R_TOP","R_CALC"],"target_equivalence":"eq","allowed_bridge_classes":["Enriching"],"loss_ceiling":1,"rollback_ceiling":1,"required_receipt_class":"RC","required_proof_shape_family":"Square","promotion_goal":"PromoteOperator","primary_zone":"PmzStructural","surface_requirement":null,"preferred_surface_target":null,"optimizer_policy":null,"policy_binding_ids":[]}"#,
            ),
            qa_entry(
                "atlas",
                r#"{"id":"A_LOCAL_BUNDLE","source_regime":"R_TOP","target_regime":"R_CALC","burden_class":"DerivativeLocalWitnessExtraction","proof_target":"local derivative witness extraction","candidate_paths":[["B_LOCAL_TOP_TO_CALC"]],"normalized_winner":["B_LOCAL_TOP_TO_CALC"],"winner_state":"Candidate","loss_profile":{"items":[]},"proof_shapes_checked":["PS_LOCAL_SQUARE"],"recipe_maturity":"Stable","failure_signatures":[],"side_conditions":["surface-preserving"],"surface_transition":{"compatibility":"SymbolicFidelityPreserving","penalties":[],"total_penalty":0}}"#,
            ),
            qa_entry(
                "campaign",
                r#"{"id":"CPG_LOCAL_BUNDLE","theorem":"THS_LOCAL_BUNDLE","target_profile":"TGT_LOCAL_BUNDLE","route_ledger":"TRL_LOCAL_BUNDLE","obligations":["OBL_LOCAL_LOC"],"certificates":[],"dependencies":[],"campaign_class":"CBridge","verdict":"RouteFound","payoff":["portable-overlay"]}"#,
            ),
        ],
    };
    let bundle_path = write_bundle_dna_fixture("overlay_bundle.dna", &overlay_document);

    Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", &namespace)
        .args([
            "run-bundle",
            "--file",
            bundle_path.to_str().unwrap(),
            "--overlay-only",
            "--conflict-policy",
            "exact-match",
        ])
        .assert()
        .success();

    Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", &namespace)
        .args([
            "certify-bundle",
            "--file",
            bundle_path.to_str().unwrap(),
            "--overlay-only",
            "--conflict-policy",
            "exact-match",
        ])
        .assert()
        .success();
}

#[test]
fn surfaced_chain_rule_bundle_certifies_through_cli() {
    let document = bundle_document_from_text(chain_rule_bundle());
    let bundle_path = write_bundle_dna_fixture("chain_rule_cli.dna", &document);
    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", test_namespace("cli_chain_rule"))
        .args([
            "certify-bundle",
            "--file",
            bundle_path.to_str().unwrap(),
            "--conflict-policy",
            "exact-match",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json[0]["campaign_id"], "CPG_CHAIN_RULE");
    assert_eq!(json[0]["verdict"], "Integrated");
    let eq = json[0]["obligations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["obligation_id"] == "OBL_CHAIN_EQ")
        .unwrap();
    assert_eq!(eq["evaluation_mode"], "RecomputedExact");
    assert!(
        eq["receipts"]
            .as_array()
            .unwrap()
            .iter()
            .any(|receipt| receipt["id"] == "EQR_CHAIN_OPERATOR_DEFAULT_SELECTION")
    );
    assert!(
        json[0]["obligations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| {
                item["obligation_id"] == "OBL_CHAIN_RED"
                    && item["evaluation_mode"] == "RecomputedExact"
                    && item["receipts"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|receipt| receipt["id"] == "RED_CHAIN_OPERATOR_DEFAULT_SELECTION")
            })
    );
    assert!(
        !json[0]["deficiencies"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["id"] == "DGN_CHAIN_RULE_ADEQUACY")
    );
    assert!(
        json[0]["promotion_artifact_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "OPR_PROMOTED_OPR_CHAIN1")
    );
    assert!(
        json[0]["default_selected_artifact_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "OPR_PROMOTED_OPR_CHAIN1")
    );
}

#[test]
fn integrated_chain_rule_bundle_reuses_promoted_operator_through_cli() {
    let document = bundle_document_from_text(&integrated_chain_rule_bundle());
    let bundle_path = write_bundle_dna_fixture("chain_rule_integrated_cli.dna", &document);
    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env(
            "MF_CACHE_NAMESPACE",
            test_namespace("cli_chain_rule_integrated"),
        )
        .args([
            "certify-bundle",
            "--file",
            bundle_path.to_str().unwrap(),
            "--conflict-policy",
            "exact-match",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json[0]["verdict"], "Integrated");
    assert!(
        json[0]["reused_artifact_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "OPR_PROMOTED_OPR_CHAIN1")
    );
    assert!(
        json[0]["default_selected_artifact_ids"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert!(
        json[0]["obligations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| {
                item["obligation_id"] == "OBL_CHAIN_EQ"
                    && item["receipts"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|receipt| receipt["id"] == "EQR_CHAIN_OPERATOR_REUSE")
            })
    );
    assert!(
        json[0]["obligations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| {
                item["obligation_id"] == "OBL_CHAIN_RED"
                    && item["receipts"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|receipt| receipt["id"] == "RED_CHAIN_OPERATOR_REUSE")
            })
    );
}

#[test]
fn chain_rule_recipe_campaign_auto_reuses_default_operator_with_payoff() {
    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env(
            "MF_CACHE_NAMESPACE",
            test_namespace("cli_chain_rule_recipe"),
        )
        .args(["certify-derived", "--campaign", "CPG_CHAIN_RULE_RECIPE"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["campaign_id"], "CPG_CHAIN_RULE_RECIPE");
    assert_eq!(json["verdict"], "Integrated");
    assert!(
        json["default_selected_artifact_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "OPR_PROMOTED_OPR_CHAIN1")
    );
    assert!(
        json["payoff_receipt_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "PAY_CHAIN1_SECOND_BURDEN_DEFAULT_REUSE")
    );
    assert!(
        json["payoff_receipt_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "PAY_CHAIN1_OBLIGATION_COUNT_REDUCTION_4_TO_2")
    );
    assert_eq!(json["obligations"].as_array().unwrap().len(), 2);
    assert!(json["obligations"].as_array().unwrap().iter().any(|item| {
        item["obligation_id"] == "OBL_CHAIN_EQ"
            && item["receipts"]
                .as_array()
                .unwrap()
                .iter()
                .any(|receipt| receipt["id"] == "EQR_CHAIN_OPERATOR_DEFAULT_SELECTION")
    }));
}

#[test]
fn chain_rule_transport_campaign_auto_reuses_default_operator_with_payoff() {
    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env(
            "MF_CACHE_NAMESPACE",
            test_namespace("cli_chain_rule_transport"),
        )
        .args(["certify-derived", "--campaign", "CPG_CHAIN_RULE_TRANSPORT"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["campaign_id"], "CPG_CHAIN_RULE_TRANSPORT");
    assert_eq!(json["verdict"], "Integrated");
    assert!(
        json["default_selected_artifact_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "OPR_PROMOTED_OPR_CHAIN1")
    );
    assert!(
        json["payoff_receipt_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "PAY_CHAIN1_ADJACENT_TRANSPORT_DEFAULT_REUSE")
    );
    assert!(
        json["payoff_receipt_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "PAY_CHAIN1_LOCALITY_WITNESS_RETAINED")
    );
    assert!(
        json["payoff_receipt_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "PAY_CHAIN1_OBLIGATION_COUNT_REDUCTION_4_TO_3")
    );
    assert_eq!(json["obligations"].as_array().unwrap().len(), 3);
    assert!(
        json["obligations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["obligation_id"] == "OBL_CHAIN_LOC")
    );
}

#[test]
fn bayes_brace_campaign_certifies_with_active_adequacy_on_top_prob_cluster() {
    let namespace = test_namespace("cli_bayes_brace");
    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", &namespace)
        .args(["certify-derived", "--campaign", "CPG_BAYES_BRACE"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["campaign_id"], "CPG_BAYES_BRACE");
    assert_eq!(json["verdict"], "Certified");
    assert_eq!(json["selected_atlas_cell"], "A_TOP_TO_PROB");
    assert!(
        json["adequacy_records"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["clause_id"] == "ADQ_BAYES_TOP_PROB_BRIDGE")
    );
    assert!(
        json["adequacy_records"]
            .as_array()
            .unwrap()
            .iter()
            .all(|item| item["verdict"] == "Certified")
    );
    assert!(json["deficiencies"].as_array().unwrap().is_empty());
    let adequacy = json["obligations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["obligation_id"] == "OBL_BAYES_ADE")
        .unwrap();
    assert_eq!(adequacy["evaluation_mode"], "RecomputedExact");
}

#[test]
fn ch_norm_campaign_certifies_with_exact_type_normalization_witness() {
    let namespace = test_namespace("cli_ch_norm");
    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", &namespace)
        .args(["certify-derived", "--campaign", "CPG_CH_NORM"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["campaign_id"], "CPG_CH_NORM");
    assert_eq!(json["verdict"], "Certified");
    assert_eq!(json["selected_atlas_cell"], "A_TYPE_TO_SET");
    assert!(
        json["adequacy_records"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["clause_id"] == "ADQ_CH_TYP_SET_BRIDGE")
    );
    assert!(json["deficiencies"].as_array().unwrap().is_empty());
    let adequacy = json["obligations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["obligation_id"] == "OBL_CH_ADE")
        .unwrap();
    assert_eq!(adequacy["evaluation_mode"], "RecomputedExact");
    assert!(
        adequacy["receipts"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["id"] == "CHN_ADE_CARRIER_COLLAPSE")
    );
}

#[test]
fn exec_infer_campaign_certifies_with_active_adequacy_on_prob_comp_cluster() {
    let namespace = test_namespace("cli_exec_infer");
    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", &namespace)
        .args(["certify-derived", "--campaign", "CPG_EXEC_INFER"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["campaign_id"], "CPG_EXEC_INFER");
    assert_eq!(json["verdict"], "Certified");
    assert_eq!(json["selected_atlas_cell"], "A_PROB_TO_COMP");
    assert!(
        json["adequacy_records"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["clause_id"] == "ADQ_EXEC_PROB_COMP_BRIDGE")
    );
    assert!(
        json["adequacy_records"]
            .as_array()
            .unwrap()
            .iter()
            .all(|item| item["verdict"] == "Certified")
    );
    assert!(json["deficiencies"].as_array().unwrap().is_empty());
}

#[test]
fn prob_judg_campaign_certifies_with_active_adequacy_on_prob_log_cluster() {
    let namespace = test_namespace("cli_prob_judg");
    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", &namespace)
        .args(["certify-derived", "--campaign", "CPG_PROB_JUDG"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["campaign_id"], "CPG_PROB_JUDG");
    assert_eq!(json["verdict"], "Certified");
    assert_eq!(json["selected_atlas_cell"], "A_PROB_TO_LOG");
    assert!(
        json["adequacy_records"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["clause_id"] == "ADQ_JDG_PROB_LOG_BRIDGE")
    );
    assert!(json["deficiencies"].as_array().unwrap().is_empty());
}

#[test]
fn cert_prop_campaign_certifies_with_active_adequacy_on_comp_log_cluster() {
    let namespace = test_namespace("cli_cert_prop");
    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", &namespace)
        .args(["certify-derived", "--campaign", "CPG_CERT_PROP"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["campaign_id"], "CPG_CERT_PROP");
    assert_eq!(json["verdict"], "Certified");
    assert_eq!(json["selected_atlas_cell"], "A_COMP_TO_LOG");
    assert!(
        json["adequacy_records"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["clause_id"] == "ADQ_CERT_COMP_LOG_BRIDGE")
    );
    assert!(json["deficiencies"].as_array().unwrap().is_empty());
    assert!(
        json["checker_receipts"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["subject_id"] == "CPG_CERT_PROP")
    );
    assert!(
        json["checker_receipts"]
            .as_array()
            .unwrap()
            .iter()
            .all(|item| item["verdict"] == "Certified")
    );
}

#[test]
fn ch_inh_campaign_certifies_with_exact_type_algebra_witness() {
    let namespace = test_namespace("cli_ch_inh");
    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", &namespace)
        .args(["certify-derived", "--campaign", "CPG_CH_INH"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["campaign_id"], "CPG_CH_INH");
    assert_eq!(json["verdict"], "Certified");
    assert_eq!(json["selected_atlas_cell"], "A_TYPE_TO_ALG");
    assert!(
        json["adequacy_records"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["clause_id"] == "ADQ_CHI_TYP_ALG_BRIDGE")
    );
    let adequacy = json["obligations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["obligation_id"] == "OBL_CHI_ADE")
        .unwrap();
    assert_eq!(adequacy["evaluation_mode"], "RecomputedExact");
    assert!(json["deficiencies"].as_array().unwrap().is_empty());
    let eq = json["obligations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["obligation_id"] == "OBL_CHI_EQ")
        .unwrap();
    assert_eq!(eq["evaluation_mode"], "RecomputedExact");
    assert!(eq["receipts"].as_array().unwrap().iter().any(|item| {
        item["id"] == "CHI_EQ_INHERITANCE"
            && item["subreceipts"]
                .as_array()
                .unwrap()
                .iter()
                .any(|sub| sub["id"] == "CHI_EQ_PROOF_TERM_TRANSPORT")
            && item["subreceipts"]
                .as_array()
                .unwrap()
                .iter()
                .any(|sub| sub["id"] == "CHI_EQ_PROOF_TERM_NORMAL_FORM")
    }));
    assert!(
        json["checker_receipts"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| {
                item["kind"] == "ObligationVerdict" && item["subject_id"] == "OBL_CHI_EQ"
            })
    );
}

#[test]
fn chain_rule_reports_active_adequacy_records() {
    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env(
            "MF_CACHE_NAMESPACE",
            test_namespace("cli_chain_rule_adequacy"),
        )
        .args(["certify-derived", "--campaign", "CPG_CHAIN_RULE"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(
        json["adequacy_records"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["clause_id"] == "ADQ_CHAIN_TOP_CALC_BRIDGE")
    );
    assert!(
        json["adequacy_records"]
            .as_array()
            .unwrap()
            .iter()
            .all(|item| item["verdict"] == "Certified")
    );
}

#[test]
fn broken_bridge_adequacy_blocks_campaign_and_promotion() {
    let document = bundle_document_from_text(broken_chain_rule_bridge_bundle());
    let bundle_path = write_bundle_dna_fixture("chain_rule_broken_bridge.dna", &document);
    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env(
            "MF_CACHE_NAMESPACE",
            test_namespace("cli_chain_rule_broken_bridge"),
        )
        .args([
            "certify-bundle",
            "--file",
            bundle_path.to_str().unwrap(),
            "--conflict-policy",
            "exact-match",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json[0]["verdict"], "BlockedOpen");
    assert!(
        json[0]["promotion_artifact_ids"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let deficiency = json[0]["deficiencies"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["class"] == "DBridge")
        .unwrap();
    assert_eq!(deficiency["blocking_scope"], "Campaign");
    assert!(
        deficiency["control_effects"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "BlockPromotion")
    );
}

#[test]
fn validation_dna_bundle_carries_certification_artifacts() {
    let namespace = test_namespace("cli_chain_rule_export_adequacy");
    let _ = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", &namespace)
        .args(["certify-derived", "--campaign", "CPG_CHAIN_RULE"])
        .output()
        .unwrap();
    let report_id = "REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE";
    let report_dir = workspace_root()
        .join(".l64-cache")
        .join("namespaces")
        .join(&namespace)
        .join("reports");
    assert!(report_dir.join(format!("{report_id}.dna")).exists());
    assert!(!report_dir.join(format!("{report_id}.locus")).exists());
    let out = std::env::temp_dir()
        .join("l64_cli_tests")
        .join(format!("{namespace}.adequacy.validation.dna"));
    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", &namespace)
        .args([
            "export-validation-dna-bundle",
            "--id",
            report_id,
            "--out",
            out.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let bytes = fs::read(out).unwrap();
    let document: QaDocument =
        l64_locus::decode_section_payload(&bytes, LocusOpcode::CanonicalPayload).unwrap();
    for id in [
        "THS_CHAIN_RULE",
        "CPG_CHAIN_RULE",
        "CRT_REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE",
        "TRL_REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE",
    ] {
        assert!(document.entries.iter().any(|entry| entry.id() == id));
    }
}

#[test]
fn export_validation_dna_bundle_imports_without_projection_surface() {
    let namespace = test_namespace("cli_validation_dna_bundle");
    let report_id = "REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE";
    let out = std::env::temp_dir()
        .join("l64_cli_tests")
        .join(format!("{namespace}.validation.dna"));
    let _ = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", &namespace)
        .args(["certify-derived", "--campaign", "CPG_CHAIN_RULE"])
        .output()
        .unwrap();
    let export = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", &namespace)
        .args([
            "export-validation-dna-bundle",
            "--id",
            report_id,
            "--out",
            out.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(export.status.success());
    assert!(out.exists());
    let bytes = fs::read(&out).unwrap();
    let document: QaDocument =
        l64_locus::decode_section_payload(&bytes, LocusOpcode::CanonicalPayload).unwrap();
    assert!(
        document
            .entries
            .iter()
            .any(|entry| entry.id() == "THS_CHAIN_RULE")
    );

    let import = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", &namespace)
        .args([
            "import-bundle",
            out.to_str().unwrap(),
            "--conflict-policy",
            "exact-match",
        ])
        .output()
        .unwrap();
    assert!(import.status.success());
}

#[test]
fn textual_validation_bundle_export_command_is_removed() {
    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args([
            "export-validation-bundle",
            "--id",
            "REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE",
            "--to",
            "qc0",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("export-validation-dna-bundle"));
}

#[test]
fn textual_report_projection_commands_are_removed() {
    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args([
            "export-report",
            "--id",
            "REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE",
            "--to",
            "qc0",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("export-report-dna"));

    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .args([
            "export-report-projection",
            "--id",
            "REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE",
            "--to",
            "qc0",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("export-report-dna"));
}

#[test]
fn validation_dna_bundle_surfaces_imported_claim_entries() {
    let namespace = test_namespace("cli_export_imported_claim");
    let imported_claim_bundle = r#"!l64-bundle v1
proof {"id":"PS_X","kind":"Square","nodes":["a","b","c","d"],"edges":[{"from":"a","to":"b","label":"f"},{"from":"b","to":"d","label":"g"},{"from":"a","to":"c","label":"h"},{"from":"c","to":"d","label":"i"}],"equations":["g∘f=i∘h"],"target_equivalence":"eq","receipts":["r"],"gate":"Pass"}
bridge {"id":"B_X","src":"R_TOP","tgt":"R_CALC","id_pres":"pres","eq_pres":"eq","forget":[],"enrich":["der"],"loss":[],"reversibility":"Enriching","receipts":["r"],"rollback":"allowed"}
atlas {"id":"A_X","source_regime":"R_TOP","target_regime":"R_CALC","burden_class":"ImportedKernelClaim","proof_target":"kernel-claim","candidate_paths":[["B_X"]],"normalized_winner":["B_X"],"winner_state":"Candidate","loss_profile":{"items":[]},"proof_shapes_checked":["PS_X"],"recipe_maturity":"Stable","failure_signatures":[],"side_conditions":[],"surface_transition":{"compatibility":"AuthorityPreserving","penalties":[],"total_penalty":0}}
theorem {"id":"THS_X","statement":"imported kernel claim","hosts":["R_TOP","R_CALC"],"bridges":["B_X"],"operators":["OPR.X"],"target_equivalence":"eq","obligations":["OblAdm"],"primary_zone":"PmzStructural","verdict":"Benchmarked","proof_shapes":["PS_X"]}
obligation {"id":"OBL_X","kind":"OblAdm","description":"imported admissibility","status":"Benchmarked"}
target {"id":"TGT_X","burden_class":"ImportedKernelClaim","host_cluster":["R_TOP","R_CALC"],"target_equivalence":"eq","allowed_bridge_classes":["Enriching"],"loss_ceiling":1,"rollback_ceiling":1,"required_receipt_class":"RC","required_proof_shape_family":"Square","promotion_goal":"OpenBlocked","primary_zone":"PmzStructural","surface_requirement":null,"preferred_surface_target":null,"optimizer_policy":null,"policy_binding_ids":[]}
ledger {"id":"TRL_X","theorem":"THS_X","paths":[["B_X"]],"budget":{"max_loss":1,"allow_lossy_supported":false,"require_proof":true},"losses":[],"receipts":["Trace_X"],"normalized_path":["B_X"]}
campaign {"id":"CPG_X","theorem":"THS_X","target_profile":"TGT_X","route_ledger":"TRL_X","obligations":["OBL_X"],"certificates":[],"dependencies":[],"campaign_class":"CBasic","verdict":"Benchmarked","payoff":["kernel-claim"]}
burden-pack {"id":"BPK_X","allowed_host_cluster":["R_TOP","R_CALC"],"obligation_ids":["OBL_X"],"adequacy_clause_ids":["ADQ_X_EVID"],"required_proof_shape_family":"Square","route_class_constraints":[],"evidence_contract_ids":["ECT_X"],"promotion_ceiling":"Certified","blocker_taxonomy":["DEvidenceContract"]}
claim-packet {"id":"CLM_X","claim_class":"Kernel","authority_state":"Evidence","target_sector":"kernel-claim","statement":"X imported claim","assumptions":["A1"],"open_caveats":[]}
evidence-contract {"id":"ECT_X","required_evidence_kinds":["kernel-claim"],"required_benchmark_roles":["TargetCase"],"requires_stress":false,"requires_challenge":false,"admissibility_thresholds":["stable"],"promotion_ceiling":"Certified"}
benchmark-receipt {"id":"BMR_X","claim_packet_id":"CLM_X","role":"TargetCase","verdict":"Certified","metrics":{"score":"1.0"},"reproducibility_ref":"RPK_X"}
reproducibility-packet {"id":"RPK_X","claim_packet_id":"CLM_X","derivation_path":["lab"],"code_refs":["src"],"benchmark_refs":["BMR_X"],"artifact_refs":["CLM_X"]}
adequacy {"id":"ADQ_X_EVID","kind":"EvidenceContractInterpretation","regime_ids":["R_TOP","R_CALC"],"bridge_ids":[],"theorem_ids":["THS_X"],"burden_pack_ids":["BPK_X"],"claim_packet_ids":["CLM_X"],"evidence_contract_ids":["ECT_X"],"benchmark_receipt_ids":["BMR_X"],"challenge_receipt_ids":[],"reproducibility_packet_ids":["RPK_X"],"description":"evidence contract present","blocking":true}"#;
    let document = bundle_document_from_text(imported_claim_bundle);
    let bundle_path = write_bundle_dna_fixture("imported_claim.dna", &document);

    let certify = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", &namespace)
        .args([
            "certify-bundle",
            "--file",
            bundle_path.to_str().unwrap(),
            "--conflict-policy",
            "exact-match",
        ])
        .output()
        .unwrap();
    assert!(certify.status.success());

    let out = std::env::temp_dir()
        .join("l64_cli_tests")
        .join(format!("{namespace}.imported.validation.dna"));
    let export = Command::cargo_bin("l64-cli")
        .unwrap()
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", &namespace)
        .args([
            "export-validation-dna-bundle",
            "--id",
            "REPORT_THS_X_CPG_X",
            "--out",
            out.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(export.status.success());
    let bytes = fs::read(out).unwrap();
    let document: QaDocument =
        l64_locus::decode_section_payload(&bytes, LocusOpcode::CanonicalPayload).unwrap();
    for id in ["BPK_X", "CLM_X", "ECT_X", "BMR_X", "RPK_X", "ADQ_X_EVID"] {
        assert!(document.entries.iter().any(|entry| entry.id() == id));
    }
}
