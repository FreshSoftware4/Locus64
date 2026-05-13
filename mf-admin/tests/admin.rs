use assert_cmd::cargo::cargo_bin;
use serde_json::Value;
use serial_test::serial;
use std::{
    cell::RefCell,
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

thread_local! {
    static TEST_NAMESPACE: RefCell<Option<String>> = const { RefCell::new(None) };
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn test_namespace() -> String {
    TEST_NAMESPACE.with(|slot| {
        let mut slot = slot.borrow_mut();
        if slot.is_none() {
            *slot = Some(format!(
                "test_{}",
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
        }
        slot.clone().unwrap()
    })
}

fn temp_dir() -> PathBuf {
    let root = std::env::temp_dir().join("mf_admin_tests").join(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_string(),
    );
    fs::create_dir_all(&root).unwrap();
    root
}

fn write_bundle(path: &PathBuf, content: &str) {
    fs::write(path, content).unwrap();
}

fn run_json(bin: &str, args: &[&str]) -> Value {
    let output = Command::new(cargo_bin(bin))
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", test_namespace())
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

fn run_text(bin: &str, args: &[&str]) -> String {
    let output = Command::new(cargo_bin(bin))
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", test_namespace())
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn run_json_ns(namespace: &str, bin: &str, args: &[&str]) -> Value {
    let output = Command::new(cargo_bin(bin))
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", namespace)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "bin={bin} args={args:?} stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

fn clear_cache() {
    let output = Command::new(cargo_bin("mf-cli"))
        .current_dir(workspace_root())
        .env("MF_CACHE_NAMESPACE", test_namespace())
        .args(["clear-cache", "--scope", "all"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let _ = fs::remove_dir_all(
        workspace_root()
            .join(".mf-cache")
            .join("namespaces")
            .join(test_namespace()),
    );
}

fn route_bundle(stem: &str, optimizer_policy: &str) -> String {
    route_bundle_named(stem, optimizer_policy, "THS_LOCAL_ROUTE", "CPG_LOCAL_ROUTE")
}

fn route_bundle_named(
    stem: &str,
    optimizer_policy: &str,
    theorem_id: &str,
    campaign_id: &str,
) -> String {
    let bundle_id = format!("BND_{}", stem.to_ascii_uppercase().replace('-', "_"));
    let axes = if optimizer_policy == "ExecutionFirst" {
        r#"["ExecutionCost","LossCompliance","Lawfulness","BundleResolution","SymbolicFidelity"]"#
    } else {
        r#"["SymbolicFidelity","SurfaceTransitionPenalty","IdentityPreservation","LossCompliance","ProofShapeSatisfiability","ExecutionCost"]"#
    };
    format!(
        r#"!qc0 {{"surface_kind":"Qc0","version":"1","policy_id":"POL_QC0_CORE","capability_id":"CAP_QC0_CORE"}}
policy-object {{"id":"MOP_{bundle_id}_OPT","kind":"Optimizer","scope":{{"Bundle":"{bundle_id}"}},"extends":null,"optimizer":{{"optimizer_policy":"{optimizer_policy}","backend":"Lexicographic","active_axes":{axes},"route_explanation_verbosity":"standard","symbolic_fidelity_preferred":{symbolic},"tie_break_rules":["shorter-path"]}},"evaluator":null,"replay_cache":null,"report":null,"canonicalizer_mode":null,"merge_policy":null,"notes":["bundle local optimizer"]}}
proof {{"id":"PS_LOCAL","kind":"Square","nodes":["a","b","c","d"],"edges":[{{"from":"a","to":"b","label":"f"}},{{"from":"b","to":"d","label":"g"}},{{"from":"a","to":"c","label":"h"}},{{"from":"c","to":"d","label":"i"}}],"equations":["g∘f=i∘h"],"target_equivalence":"eq","receipts":["r"],"gate":"Pass"}}
bridge {{"id":"B_FAST","src":"R_TOP","tgt":"R_CALC","id_pres":"pres","eq_pres":"eq","forget":[],"enrich":["der"],"loss":["symbolic"],"reversibility":"LossySupported","receipts":["r"],"rollback":"conditional"}}
bridge {{"id":"B_FAITHFUL_1","src":"R_TOP","tgt":"R_SET","id_pres":"pres","eq_pres":"eq","forget":[],"enrich":[],"loss":[],"reversibility":"Conservative","receipts":["r"],"rollback":"allowed"}}
bridge {{"id":"B_FAITHFUL_2","src":"R_SET","tgt":"R_CALC","id_pres":"pres","eq_pres":"eq","forget":[],"enrich":["der"],"loss":[],"reversibility":"Enriching","receipts":["r"],"rollback":"allowed"}}
atlas {{"id":"A_FAST","source_regime":"R_TOP","target_regime":"R_CALC","burden_class":"General","proof_target":"route","candidate_paths":[["B_FAST"]],"normalized_winner":["B_FAST"],"winner_state":"Candidate","loss_profile":{{"items":["symbolic-loss"]}},"proof_shapes_checked":["PS_LOCAL"],"recipe_maturity":"Stable","failure_signatures":[],"side_conditions":[],"surface_transition":{{"compatibility":"DebugMirrorOnly","penalties":[{{"reason":"debug-collapse","amount":4}}],"total_penalty":4}}}}
atlas {{"id":"A_FAITHFUL","source_regime":"R_TOP","target_regime":"R_CALC","burden_class":"General","proof_target":"route","candidate_paths":[["B_FAITHFUL_1","B_FAITHFUL_2"]],"normalized_winner":["B_FAITHFUL_1","B_FAITHFUL_2"],"winner_state":"Candidate","loss_profile":{{"items":[]}},"proof_shapes_checked":["PS_LOCAL"],"recipe_maturity":"Stable","failure_signatures":[],"side_conditions":[],"surface_transition":{{"compatibility":"SymbolicFidelityPreserving","penalties":[],"total_penalty":0}}}}
theorem {{"id":"{theorem_id}","statement":"local route","hosts":["R_TOP","R_CALC"],"bridges":["B_FAST","B_FAITHFUL_1","B_FAITHFUL_2"],"operators":["OPR.Local"],"target_equivalence":"eq","obligations":["OblAdm"],"primary_zone":"PmzStructural","verdict":"Benchmarked","proof_shapes":["PS_LOCAL"]}}
obligation {{"id":"OBL_LOCAL_ADM","kind":"OblAdm","description":"adm","status":"Benchmarked"}}
target {{"id":"TGT_LOCAL_ROUTE","burden_class":"General","host_cluster":["R_TOP","R_CALC"],"target_equivalence":"eq","allowed_bridge_classes":["LossySupported","Conservative","Enriching"],"loss_ceiling":4,"rollback_ceiling":4,"required_receipt_class":"RC","required_proof_shape_family":"Square","promotion_goal":"PromoteOperator","primary_zone":"PmzStructural","surface_requirement":null,"preferred_surface_target":null,"optimizer_policy":null,"policy_binding_ids":[]}}
ledger {{"id":"TRL_LOCAL_ROUTE","theorem":"{theorem_id}","paths":[["B_FAST"],["B_FAITHFUL_1","B_FAITHFUL_2"]],"budget":{{"max_loss":4,"allow_lossy_supported":true,"require_proof":true}},"losses":[],"receipts":[],"normalized_path":["B_FAST"]}}
campaign {{"id":"{campaign_id}","theorem":"{theorem_id}","target_profile":"TGT_LOCAL_ROUTE","route_ledger":"TRL_LOCAL_ROUTE","obligations":["OBL_LOCAL_ADM"],"certificates":[],"dependencies":[],"campaign_class":"CBasic","verdict":"Benchmarked","payoff":["route"]}}
"#,
        symbolic = if optimizer_policy == "SymbolicFidelityFirst" {
            "true"
        } else {
            "false"
        },
        axes = axes,
        theorem_id = theorem_id,
        campaign_id = campaign_id,
    )
}

fn evaluator_bundle(stem: &str, strict: bool) -> String {
    let bundle_id = format!("BND_{}", stem.to_ascii_uppercase().replace('-', "_"));
    let policy_line = if strict {
        format!(
            r#"policy-object {{"id":"MOP_{bundle_id}_EVAL","kind":"Evaluator","scope":{{"Bundle":"{bundle_id}"}},"extends":null,"optimizer":null,"evaluator":{{"evidence_preference":"RecomputeIfSupported","allow_approximation":false,"unsupported_mode":"StrictFail","require_symbolic_fidelity_route":false,"prefer_comp_replay":false}},"replay_cache":null,"report":null,"canonicalizer_mode":null,"merge_policy":null,"notes":["strict evaluator"]}}"#
        )
    } else {
        String::new()
    };
    format!(
        r#"!qc0 {{"surface_kind":"Qc0","version":"1","policy_id":"POL_QC0_CORE","capability_id":"CAP_QC0_CORE"}}
{policy_line}
proof {{"id":"PS_EVAL","kind":"Square","nodes":["a","b","c","d"],"edges":[{{"from":"a","to":"b","label":"f"}},{{"from":"b","to":"d","label":"g"}},{{"from":"a","to":"c","label":"h"}},{{"from":"c","to":"d","label":"i"}}],"equations":["g∘f=i∘h"],"target_equivalence":"eq","receipts":["r"],"gate":"Pass"}}
bridge {{"id":"B_EVAL","src":"R_TOP","tgt":"R_CALC","id_pres":"pres","eq_pres":"eq","forget":[],"enrich":["der"],"loss":[],"reversibility":"Enriching","receipts":["r"],"rollback":"allowed"}}
atlas {{"id":"A_EVAL","source_regime":"R_TOP","target_regime":"R_CALC","burden_class":"General","proof_target":"eval","candidate_paths":[["B_EVAL"]],"normalized_winner":["B_EVAL"],"winner_state":"Candidate","loss_profile":{{"items":[]}},"proof_shapes_checked":["PS_EVAL"],"recipe_maturity":"Stable","failure_signatures":[],"side_conditions":[],"surface_transition":{{"compatibility":"AuthorityPreserving","penalties":[],"total_penalty":0}}}}
theorem {{"id":"THS_LOCAL_EVAL","statement":"local eval","hosts":["R_TOP","R_CALC"],"bridges":["B_EVAL"],"operators":["OPR.Local"],"target_equivalence":"eq","obligations":["OblEq"],"primary_zone":"PmzStructural","verdict":"Benchmarked","proof_shapes":["PS_EVAL"]}}
obligation {{"id":"OBL_LOCAL_EQ","kind":"OblEq","description":"unsupported eq","status":"Benchmarked"}}
target {{"id":"TGT_LOCAL_EVAL","burden_class":"General","host_cluster":["R_TOP","R_CALC"],"target_equivalence":"eq","allowed_bridge_classes":["Enriching"],"loss_ceiling":1,"rollback_ceiling":1,"required_receipt_class":"RC","required_proof_shape_family":"Square","promotion_goal":"PromoteOperator","primary_zone":"PmzStructural","surface_requirement":null,"preferred_surface_target":null,"optimizer_policy":null,"policy_binding_ids":[]}}
ledger {{"id":"TRL_LOCAL_EVAL","theorem":"THS_LOCAL_EVAL","paths":[["B_EVAL"]],"budget":{{"max_loss":1,"allow_lossy_supported":false,"require_proof":true}},"losses":[],"receipts":[],"normalized_path":["B_EVAL"]}}
campaign {{"id":"CPG_LOCAL_EVAL","theorem":"THS_LOCAL_EVAL","target_profile":"TGT_LOCAL_EVAL","route_ledger":"TRL_LOCAL_EVAL","obligations":["OBL_LOCAL_EQ"],"certificates":[],"dependencies":[],"campaign_class":"CBasic","verdict":"Benchmarked","payoff":["eval"]}}
"#
    )
}

fn replay_bundle(stem: &str, replay_allowed: bool) -> String {
    let bundle_id = format!("BND_{}", stem.to_ascii_uppercase().replace('-', "_"));
    let policy_line = format!(
        r#"policy-object {{"id":"MOP_{bundle_id}_REPLAY","kind":"ReplayCache","scope":{{"Bundle":"{bundle_id}"}},"extends":null,"optimizer":null,"evaluator":null,"replay_cache":{{"replay_allowed":{replay_allowed},"exact_policy_match_required":true,"survive_surface_only_changes":false,"reuse_approximate_results":true,"optimizer_change_invalidates":true,"surface_pack_change_invalidates":true,"trust_class":"ExactPolicyOnly"}},"report":null,"canonicalizer_mode":null,"merge_policy":null,"notes":["replay policy"]}}"#
    );
    format!(
        r#"!qc0 {{"surface_kind":"Qc0","version":"1","policy_id":"POL_QC0_CORE","capability_id":"CAP_QC0_CORE"}}
{policy_line}
proof {{"id":"PS_REPLAY","kind":"Square","nodes":["a","b","c","d"],"edges":[{{"from":"a","to":"b","label":"f"}},{{"from":"b","to":"d","label":"g"}},{{"from":"a","to":"c","label":"h"}},{{"from":"c","to":"d","label":"i"}}],"equations":["g∘f=i∘h"],"target_equivalence":"eq","receipts":["r"],"gate":"Pass"}}
bridge {{"id":"B_REPLAY","src":"R_TOP","tgt":"R_CALC","id_pres":"pres","eq_pres":"eq","forget":[],"enrich":["der"],"loss":[],"reversibility":"Enriching","receipts":["r"],"rollback":"allowed"}}
atlas {{"id":"A_REPLAY","source_regime":"R_TOP","target_regime":"R_CALC","burden_class":"General","proof_target":"replay","candidate_paths":[["B_REPLAY"]],"normalized_winner":["B_REPLAY"],"winner_state":"Candidate","loss_profile":{{"items":[]}},"proof_shapes_checked":["PS_REPLAY"],"recipe_maturity":"Stable","failure_signatures":[],"side_conditions":[],"surface_transition":{{"compatibility":"AuthorityPreserving","penalties":[],"total_penalty":0}}}}
theorem {{"id":"THS_LOCAL_REPLAY","statement":"local replay","hosts":["R_TOP","R_CALC"],"bridges":["B_REPLAY"],"operators":["OPR.Local"],"target_equivalence":"eq","obligations":["OblAdm"],"primary_zone":"PmzStructural","verdict":"Benchmarked","proof_shapes":["PS_REPLAY"]}}
obligation {{"id":"OBL_LOCAL_REPLAY","kind":"OblAdm","description":"adm","status":"Benchmarked"}}
target {{"id":"TGT_LOCAL_REPLAY","burden_class":"General","host_cluster":["R_TOP","R_CALC"],"target_equivalence":"eq","allowed_bridge_classes":["Enriching"],"loss_ceiling":1,"rollback_ceiling":1,"required_receipt_class":"RC","required_proof_shape_family":"Square","promotion_goal":"PromoteOperator","primary_zone":"PmzStructural","surface_requirement":null,"preferred_surface_target":null,"optimizer_policy":null,"policy_binding_ids":[]}}
ledger {{"id":"TRL_LOCAL_REPLAY","theorem":"THS_LOCAL_REPLAY","paths":[["B_REPLAY"]],"budget":{{"max_loss":1,"allow_lossy_supported":false,"require_proof":true}},"losses":[],"receipts":[],"normalized_path":["B_REPLAY"]}}
campaign {{"id":"CPG_LOCAL_REPLAY","theorem":"THS_LOCAL_REPLAY","target_profile":"TGT_LOCAL_REPLAY","route_ledger":"TRL_LOCAL_REPLAY","obligations":["OBL_LOCAL_REPLAY"],"certificates":[],"dependencies":[],"campaign_class":"CBasic","verdict":"Benchmarked","payoff":["replay"]}}
"#
    )
}

fn parallel_campaign_bundle(stem: &str) -> String {
    let bundle_id = format!("BND_{}", stem.to_ascii_uppercase().replace('-', "_"));
    format!(
        r#"!qc0 {{"surface_kind":"Qc0","version":"1","policy_id":"POL_QC0_CORE","capability_id":"CAP_QC0_CORE"}}
policy-object {{"id":"MOP_{bundle_id}_SCHED","kind":"ReportExport","scope":{{"Bundle":"{bundle_id}"}},"extends":null,"optimizer":null,"evaluator":null,"replay_cache":null,"report":{{"export_surfaces":["Qc0"],"include_policy_trace":true,"include_route_explanation":true,"include_obligation_logs":true}},"scheduler":{{"parallelization":"ParallelIndependent","max_workers":2,"allow_parallel_replay":false,"allow_parallel_certification":true,"allow_parallel_exports":true,"deterministic_ordering":true}},"canonicalizer_mode":null,"merge_policy":null,"notes":["parallel scheduler policy"]}}
proof {{"id":"PS_PAR","kind":"Square","nodes":["a","b","c","d"],"edges":[{{"from":"a","to":"b","label":"f"}},{{"from":"b","to":"d","label":"g"}},{{"from":"a","to":"c","label":"h"}},{{"from":"c","to":"d","label":"i"}}],"equations":["g∘f=i∘h"],"target_equivalence":"eq","receipts":["r"],"gate":"Pass"}}
bridge {{"id":"B_PAR","src":"R_TOP","tgt":"R_CALC","id_pres":"pres","eq_pres":"eq","forget":[],"enrich":["der"],"loss":[],"reversibility":"Enriching","receipts":["r"],"rollback":"allowed"}}
atlas {{"id":"A_PAR","source_regime":"R_TOP","target_regime":"R_CALC","burden_class":"General","proof_target":"parallel","candidate_paths":[["B_PAR"]],"normalized_winner":["B_PAR"],"winner_state":"Candidate","loss_profile":{{"items":[]}},"proof_shapes_checked":["PS_PAR"],"recipe_maturity":"Stable","failure_signatures":[],"side_conditions":[],"surface_transition":{{"compatibility":"AuthorityPreserving","penalties":[],"total_penalty":0}}}}
theorem {{"id":"THS_PAR_A","statement":"parallel A","hosts":["R_TOP","R_CALC"],"bridges":["B_PAR"],"operators":["OPR.ParA"],"target_equivalence":"eq","obligations":["OblAdm"],"primary_zone":"PmzStructural","verdict":"Benchmarked","proof_shapes":["PS_PAR"]}}
theorem {{"id":"THS_PAR_B","statement":"parallel B","hosts":["R_TOP","R_CALC"],"bridges":["B_PAR"],"operators":["OPR.ParB"],"target_equivalence":"eq","obligations":["OblAdm"],"primary_zone":"PmzStructural","verdict":"Benchmarked","proof_shapes":["PS_PAR"]}}
obligation {{"id":"OBL_PAR","kind":"OblAdm","description":"adm","status":"Benchmarked"}}
target {{"id":"TGT_PAR","burden_class":"General","host_cluster":["R_TOP","R_CALC"],"target_equivalence":"eq","allowed_bridge_classes":["Enriching"],"loss_ceiling":1,"rollback_ceiling":1,"required_receipt_class":"RC","required_proof_shape_family":"Square","promotion_goal":"PromoteOperator","primary_zone":"PmzStructural","surface_requirement":null,"preferred_surface_target":null,"optimizer_policy":null,"policy_binding_ids":[]}}
ledger {{"id":"TRL_PAR_A","theorem":"THS_PAR_A","paths":[["B_PAR"]],"budget":{{"max_loss":1,"allow_lossy_supported":false,"require_proof":true}},"losses":[],"receipts":[],"normalized_path":["B_PAR"]}}
ledger {{"id":"TRL_PAR_B","theorem":"THS_PAR_B","paths":[["B_PAR"]],"budget":{{"max_loss":1,"allow_lossy_supported":false,"require_proof":true}},"losses":[],"receipts":[],"normalized_path":["B_PAR"]}}
campaign {{"id":"CPG_PAR_A","theorem":"THS_PAR_A","target_profile":"TGT_PAR","route_ledger":"TRL_PAR_A","obligations":["OBL_PAR"],"certificates":[],"dependencies":[],"campaign_class":"CBasic","verdict":"Benchmarked","payoff":["parallel"]}}
campaign {{"id":"CPG_PAR_B","theorem":"THS_PAR_B","target_profile":"TGT_PAR","route_ledger":"TRL_PAR_B","obligations":["OBL_PAR"],"certificates":[],"dependencies":[],"campaign_class":"CBasic","verdict":"Benchmarked","payoff":["parallel"]}}
"#
    )
}

fn parallel_campaign_bundle_with_optimizer(stem: &str, optimizer: &str) -> String {
    let bundle_id = format!("BND_{}", stem.to_ascii_uppercase().replace('-', "_"));
    let axes = if optimizer == "ExecutionFirst" {
        r#"["ExecutionCost","LossCompliance","Lawfulness","BundleResolution","SymbolicFidelity"]"#
    } else {
        r#"["SymbolicFidelity","SurfaceTransitionPenalty","IdentityPreservation","LossCompliance","ExecutionCost"]"#
    };
    let mut bundle = parallel_campaign_bundle(stem);
    bundle.push_str(&format!(
        "policy-object {{\"id\":\"MOP_{bundle_id}_OPT\",\"kind\":\"Optimizer\",\"scope\":{{\"Bundle\":\"{bundle_id}\"}},\"extends\":null,\"optimizer\":{{\"optimizer_policy\":\"{optimizer}\",\"backend\":\"Lexicographic\",\"active_axes\":{axes},\"route_explanation_verbosity\":\"standard\",\"symbolic_fidelity_preferred\":true,\"tie_break_rules\":[\"shorter-path\"]}},\"evaluator\":null,\"replay_cache\":null,\"report\":null,\"scheduler\":null,\"canonicalizer_mode\":null,\"merge_policy\":null,\"notes\":[\"optimizer override\"]}}\n"
    ));
    bundle
}

fn obligation_parallel_bundle(stem: &str, allow_parallel: bool) -> String {
    let bundle_id = format!("BND_{}", stem.to_ascii_uppercase().replace('-', "_"));
    let workers = if allow_parallel { 3 } else { 1 };
    format!(
        r#"!qc0 {{"surface_kind":"Qc0","version":"1","policy_id":"POL_QC0_CORE","capability_id":"CAP_QC0_CORE"}}
policy-object {{"id":"MOP_{bundle_id}_SCHED","kind":"ReportExport","scope":{{"Bundle":"{bundle_id}"}},"extends":null,"optimizer":null,"evaluator":null,"replay_cache":null,"report":{{"export_surfaces":["Qc0"],"include_policy_trace":true,"include_route_explanation":true,"include_obligation_logs":true}},"scheduler":{{"parallelization":"ParallelIndependent","max_workers":2,"allow_parallel_replay":true,"allow_parallel_certification":true,"allow_parallel_exports":true,"deterministic_ordering":true,"allow_parallel_obligations":{allow_parallel},"max_obligation_workers":{workers},"allow_parallel_obligation_replay":{allow_parallel},"serialize_canonicalization_sensitive":true}},"canonicalizer_mode":null,"merge_policy":null,"notes":["obligation scheduler policy"]}}
proof {{"id":"PS_OBL","kind":"Square","nodes":["a","b","c","d"],"edges":[{{"from":"a","to":"b","label":"f"}},{{"from":"b","to":"d","label":"g"}},{{"from":"a","to":"c","label":"h"}},{{"from":"c","to":"d","label":"i"}}],"equations":["g∘f=i∘h"],"target_equivalence":"eq","receipts":["r"],"gate":"Pass"}}
bridge {{"id":"B_OBL","src":"R_TOP","tgt":"R_CALC","id_pres":"pres","eq_pres":"eq","forget":[],"enrich":["der"],"loss":[],"reversibility":"Enriching","receipts":["r"],"rollback":"allowed"}}
atlas {{"id":"A_OBL","source_regime":"R_TOP","target_regime":"R_CALC","burden_class":"General","proof_target":"obligation","candidate_paths":[["B_OBL"]],"normalized_winner":["B_OBL"],"winner_state":"Candidate","loss_profile":{{"items":[]}},"proof_shapes_checked":["PS_OBL"],"recipe_maturity":"Stable","failure_signatures":[],"side_conditions":[],"surface_transition":{{"compatibility":"AuthorityPreserving","penalties":[],"total_penalty":0}}}}
theorem {{"id":"THS_OBL","statement":"obligation parallel theorem","hosts":["R_TOP","R_CALC"],"bridges":["B_OBL"],"operators":["OPR.Obl"],"target_equivalence":"eq","obligations":["OblAdm","OblTol","OblRed"],"primary_zone":"PmzStructural","verdict":"Benchmarked","proof_shapes":["PS_OBL"]}}
obligation {{"id":"OBL_PAR_ADM","kind":"OblAdm","description":"adm","status":"Benchmarked"}}
obligation {{"id":"OBL_PAR_TOL","kind":"OblTol","description":"tol","status":"Benchmarked"}}
obligation {{"id":"OBL_PAR_RED","kind":"OblRed","description":"red","status":"Benchmarked"}}
target {{"id":"TGT_OBL","burden_class":"General","host_cluster":["R_TOP","R_CALC"],"target_equivalence":"eq","allowed_bridge_classes":["Enriching"],"loss_ceiling":1,"rollback_ceiling":1,"required_receipt_class":"RC","required_proof_shape_family":"Square","promotion_goal":"PromoteOperator","primary_zone":"PmzStructural","surface_requirement":null,"preferred_surface_target":null,"optimizer_policy":null,"policy_binding_ids":[]}}
ledger {{"id":"TRL_OBL","theorem":"THS_OBL","paths":[["B_OBL"]],"budget":{{"max_loss":1,"allow_lossy_supported":false,"require_proof":true}},"losses":[],"receipts":[],"normalized_path":["B_OBL"]}}
campaign {{"id":"CPG_OBL","theorem":"THS_OBL","target_profile":"TGT_OBL","route_ledger":"TRL_OBL","obligations":["OBL_PAR_ADM","OBL_PAR_TOL","OBL_PAR_RED"],"certificates":[],"dependencies":[],"campaign_class":"CBasic","verdict":"Benchmarked","payoff":["obligation"]}}
"#,
        allow_parallel = if allow_parallel { "true" } else { "false" },
        workers = workers,
    )
}

fn chain_rule_bundle(
    stem: &str,
    allow_parallel: bool,
    strict_evaluator: bool,
    include_promoted_operator: bool,
) -> String {
    let bundle_id = format!("BND_{}", stem.to_ascii_uppercase().replace('-', "_"));
    let scheduler_policy = format!(
        r#"policy-object {{"id":"MOP_{bundle_id}_SCHED","kind":"ReportExport","scope":{{"Bundle":"{bundle_id}"}},"extends":null,"optimizer":null,"evaluator":null,"replay_cache":null,"report":{{"export_surfaces":["Qc0","Qa0"],"include_policy_trace":true,"include_route_explanation":true,"include_obligation_logs":true}},"scheduler":{{"parallelization":"ParallelIndependent","max_workers":2,"allow_parallel_replay":true,"allow_parallel_certification":true,"allow_parallel_exports":true,"deterministic_ordering":true,"allow_parallel_obligations":{allow_parallel},"max_obligation_workers":3,"allow_parallel_obligation_replay":{allow_parallel},"serialize_canonicalization_sensitive":true}},"canonicalizer_mode":null,"merge_policy":null,"notes":["flagship chain rule scheduler"]}}"#
    );
    let evaluator_policy = if strict_evaluator {
        format!(
            r#"
policy-object {{"id":"MOP_{bundle_id}_EVAL","kind":"Evaluator","scope":{{"Bundle":"{bundle_id}"}},"extends":null,"optimizer":null,"evaluator":{{"evidence_preference":"RecomputeIfSupported","allow_approximation":false,"unsupported_mode":"StrictFail","require_symbolic_fidelity_route":false,"prefer_comp_replay":true}},"replay_cache":null,"report":null,"scheduler":null,"canonicalizer_mode":null,"merge_policy":null,"notes":["strict chain rule evaluator"]}}"#
        )
    } else {
        String::new()
    };
    let promoted_operator = if include_promoted_operator {
        r#"
object {"id":"OPR_PROMOTED_OPR_CHAIN1","identity":{"tag":"OPR","cid":"cid:OPR_PROMOTED_OPR_CHAIN1","codebook":"QC0_CORE","remap":"none","lineage":"derived-from:THS_CHAIN_RULE"},"structural":{"head":"operator","args":["THS_CHAIN_RULE","CPG_CHAIN_RULE"],"local_sections":["first-order derivative composition"],"morphism_hooks":["B_TOP_TO_CALC"]},"constraint":{"regime":"R_CALC","contracts":["chain-rule","first-order"],"invariants":["jet-compose","reduction-exact"],"equivalence":"first-order jet equivalence","admissibility":"promoted after exact certified discharge"},"evidence":{"evidence_class":"DerivedPromotion","traces":["THS_CHAIN_RULE"],"receipts":["CRT_CHAIN_RULE","REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE"],"maturity":"Certified","gate_verdict":"Pass"},"alias":{"aliases":["OPR.Chain1"],"profile_pack":["STD","chain-rule"],"qm_binding":"THS·ChainRule","qa_binding":"OPR.Chain1","projection_policy":"canonical-authored"}} 
"#
    } else {
        ""
    };
    format!(
        r#"!qc0 {{"surface_kind":"Qc0","version":"1","policy_id":"POL_QC0_CORE","capability_id":"CAP_QC0_CORE"}}
{scheduler_policy}{evaluator_policy}{promoted_operator}
theorem {{"id":"THS_CHAIN_RULE","statement":"DER(g∘f,x) ≈1 DER(g,f(x))∘DER(f,x)","hosts":["R_TOP","R_CALC"],"bridges":["B_TOP_TO_CALC"],"operators":["OPR.Chain1"],"target_equivalence":"first-order jet equivalence","obligations":["OblEq","OblAdm","OblLoc","OblRed"],"primary_zone":"PmzStructural","verdict":"Benchmarked","proof_shapes":["PS_SQUARE_TOPO"]}}
obligation {{"id":"OBL_CHAIN_EQ","kind":"OblEq","description":"first-order slack equivalence preserved under composition","status":"Benchmarked"}}
obligation {{"id":"OBL_CHAIN_ADM","kind":"OblAdm","description":"both derivatives admitted in R_CALC","status":"Benchmarked"}}
obligation {{"id":"OBL_CHAIN_LOC","kind":"OblLoc","description":"same brace-localization at x","status":"Benchmarked"}}
obligation {{"id":"OBL_CHAIN_RED","kind":"OblRed","description":"reduction of remainder terms preserves first-order equivalence","status":"RouteFound"}}
target {{"id":"TGT_CHAIN_RULE","burden_class":"DerivativeLocalWitnessExtraction","host_cluster":["R_TOP","R_CALC"],"target_equivalence":"first-order jet equivalence","allowed_bridge_classes":["Enriching","Conservative"],"loss_ceiling":1,"rollback_ceiling":1,"required_receipt_class":"RC_ths","required_proof_shape_family":"MixedBattery","promotion_goal":"PromoteOperator","primary_zone":"PmzStructural","surface_requirement":null,"preferred_surface_target":null,"optimizer_policy":null,"policy_binding_ids":[]}}
ledger {{"id":"TRL_CHAIN_RULE","theorem":"THS_CHAIN_RULE","paths":[["B_TOP_TO_CALC"]],"budget":{{"max_loss":1,"allow_lossy_supported":false,"require_proof":true}},"losses":[],"receipts":["Ref_1","Can","Red"],"normalized_path":["B_TOP_TO_CALC"]}}
certificate {{"id":"CRT_CHAIN_RULE","theorem":"THS_CHAIN_RULE","route_ledger":"TRL_CHAIN_RULE","proof_shapes":["PS_SQUARE_TOPO"],"receipts":["RC_ths","TR_chain","BENCH"],"verdict":"Benchmarked"}}
campaign {{"id":"CPG_CHAIN_RULE","theorem":"THS_CHAIN_RULE","target_profile":"TGT_CHAIN_RULE","route_ledger":"TRL_CHAIN_RULE","obligations":["OBL_CHAIN_EQ","OBL_CHAIN_ADM","OBL_CHAIN_LOC","OBL_CHAIN_RED"],"certificates":["CRT_CHAIN_RULE"],"dependencies":[],"campaign_class":"COperator","verdict":"Benchmarked","payoff":["OPR.Chain1","ATL.RouteClass.Chain"]}}
diagnostic {{"id":"DGN_CHAIN_RULE_ADEQUACY","class":"DNoAdequacy","atlas_cell":"A_TOP_TO_CALC","theorem":"THS_CHAIN_RULE","message":"semantic adequacy remains scaffolded rather than discharged"}}
"#,
        scheduler_policy = scheduler_policy,
        evaluator_policy = evaluator_policy,
        promoted_operator = promoted_operator,
    )
}

#[test]
#[serial]
fn authored_optimizer_policy_changes_route_winner() {
    clear_cache();
    let dir = temp_dir();
    let exec_bundle = dir.join("opt_exec.qc0");
    let sym_bundle = dir.join("opt_symbolic.qc0");
    write_bundle(&exec_bundle, &route_bundle("opt_exec", "ExecutionFirst"));
    write_bundle(
        &sym_bundle,
        &route_bundle("opt_symbolic", "SymbolicFidelityFirst"),
    );

    let exec = run_json(
        "mf-cli",
        &["certify-bundle", "--file", exec_bundle.to_str().unwrap()],
    );
    let sym = run_json(
        "mf-cli",
        &["certify-bundle", "--file", sym_bundle.to_str().unwrap()],
    );
    assert_eq!(exec[0]["selected_atlas_cell"], "A_FAST");
    assert_eq!(sym[0]["selected_atlas_cell"], "A_FAITHFUL");
    assert!(
        sym[0]["policy_resolution"]["applied_policy_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item.as_str().unwrap().contains("MOP_BND_OPT_SYMBOLIC_OPT"))
    );
}

#[test]
#[serial]
fn authored_evaluator_policy_changes_verdict() {
    clear_cache();
    let dir = temp_dir();
    let base_bundle = dir.join("eval_base.qc0");
    let strict_bundle = dir.join("eval_strict.qc0");
    write_bundle(&base_bundle, &evaluator_bundle("eval_base", false));
    write_bundle(&strict_bundle, &evaluator_bundle("eval_strict", true));

    let base = run_json(
        "mf-cli",
        &["certify-bundle", "--file", base_bundle.to_str().unwrap()],
    );
    let strict = run_json(
        "mf-cli",
        &["certify-bundle", "--file", strict_bundle.to_str().unwrap()],
    );
    assert_ne!(base[0]["verdict"], strict[0]["verdict"]);
    assert_eq!(strict[0]["verdict"], "BlockedOpen");
}

#[test]
#[serial]
fn authored_replay_policy_changes_cache_behavior() {
    clear_cache();
    let dir = temp_dir();
    let cache_bundle = dir.join("replay_cache.qc0");
    let nocache_bundle = dir.join("replay_nocache.qc0");
    write_bundle(&cache_bundle, &replay_bundle("replay_cache", true));
    write_bundle(&nocache_bundle, &replay_bundle("replay_nocache", false));

    let _ = run_json(
        "mf-cli",
        &["certify-bundle", "--file", cache_bundle.to_str().unwrap()],
    );
    let second = run_json(
        "mf-cli",
        &["certify-bundle", "--file", cache_bundle.to_str().unwrap()],
    );
    assert_eq!(second[0]["execution_envelope"]["replay_status"], "CacheHit");

    let _ = run_json(
        "mf-cli",
        &["certify-bundle", "--file", nocache_bundle.to_str().unwrap()],
    );
    let second_no = run_json(
        "mf-cli",
        &["certify-bundle", "--file", nocache_bundle.to_str().unwrap()],
    );
    assert_eq!(second_no[0]["execution_envelope"]["replay_status"], "Fresh");
}

#[test]
#[serial]
fn lock_manifest_and_replay_flow_work() {
    clear_cache();
    let dir = temp_dir();
    let bundle = dir.join("opt_exec.qc0");
    let other = dir.join("opt_symbolic.qc0");
    write_bundle(&bundle, &route_bundle("opt_exec", "ExecutionFirst"));
    write_bundle(
        &other,
        &route_bundle("opt_symbolic", "SymbolicFidelityFirst"),
    );

    let locked = run_json(
        "mf-admin",
        &[
            "lock-bundle",
            bundle.to_str().unwrap(),
            "--optimizer-policy",
            "execution-first",
        ],
    );
    let lock_id = locked["lock"]["id"].as_str().unwrap().to_string();
    let manifest_id = locked["manifest"]["id"].as_str().unwrap().to_string();

    let manifest = run_json("mf-admin", &["dump-execution-manifest", &manifest_id]);
    assert_eq!(manifest["bundle_id"], "BND_OPT_EXEC");

    let replay = run_json("mf-admin", &["replay-with-lock", &lock_id]);
    assert_eq!(replay[0]["selected_atlas_cell"], "A_FAST");

    let exported = run_text(
        "mf-admin",
        &["export-artifact", "--id", &manifest_id, "--to", "qc0"],
    );
    let exported_path = dir.join("manifest.qc0");
    fs::write(&exported_path, exported).unwrap();
    let import_output = Command::new(cargo_bin("mf-cli"))
        .current_dir(workspace_root())
        .args(["import", exported_path.to_str().unwrap(), "--as", "qc0"])
        .output()
        .unwrap();
    assert!(
        import_output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&import_output.stderr)
    );

    let locked_other = run_json(
        "mf-admin",
        &[
            "lock-bundle",
            other.to_str().unwrap(),
            "--optimizer-policy",
            "symbolic-fidelity-first",
        ],
    );
    let other_lock_id = locked_other["lock"]["id"].as_str().unwrap().to_string();
    let diff = run_json("mf-admin", &["compare-locks", &lock_id, &other_lock_id]);
    assert!(
        diff["semantic_changes"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item.as_str().unwrap().contains("policy graph")
                || item.as_str().unwrap().contains("route winners"))
    );
}

#[test]
#[serial]
fn observe_predict_plan_and_assess_route_drift_workflow() {
    clear_cache();
    let dir = temp_dir();
    let exec_bundle = dir.join("predict_exec.qc0");
    let sym_bundle = dir.join("predict_sym.qc0");
    write_bundle(
        &exec_bundle,
        &route_bundle_named(
            "predict_exec",
            "ExecutionFirst",
            "THS_LOCAL_ROUTE",
            "CPG_ROUTE_EXEC",
        ),
    );
    write_bundle(
        &sym_bundle,
        &route_bundle_named(
            "predict_sym",
            "SymbolicFidelityFirst",
            "THS_LOCAL_ROUTE",
            "CPG_ROUTE_SYMBOLIC",
        ),
    );

    let baseline = run_json(
        "mf-cli",
        &["certify-bundle", "--file", exec_bundle.to_str().unwrap()],
    );
    assert_eq!(baseline[0]["selected_atlas_cell"], "A_FAST");
    let baseline_report_id = "REPORT_THS_LOCAL_ROUTE_CPG_ROUTE_EXEC";

    let observed = run_json("mf-admin", &["observe-run", "--report", baseline_report_id]);
    assert_eq!(
        observed[0]["artifact"]["record"]["report_id"],
        baseline_report_id
    );

    let prediction = run_json(
        "mf-admin",
        &[
            "predict-impact",
            "--report",
            baseline_report_id,
            "--bundle-file",
            sym_bundle.to_str().unwrap(),
        ],
    );
    assert_eq!(prediction["prediction"]["class"], "RouteWinnerChangeLikely");
    let prediction_id = prediction["prediction"]["id"].as_str().unwrap();

    let plan = run_json(
        "mf-admin",
        &["plan-recompute", "--prediction", prediction_id],
    );
    assert!(
        plan["steps"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["kind"] == "ReRoute")
    );

    let actual = run_json(
        "mf-cli",
        &["certify-bundle", "--file", sym_bundle.to_str().unwrap()],
    );
    assert_eq!(actual[0]["selected_atlas_cell"], "A_FAITHFUL");
    let actual_report_id = "REPORT_THS_LOCAL_ROUTE_CPG_ROUTE_SYMBOLIC";

    let diff = run_json(
        "mf-admin",
        &["compare-reports", baseline_report_id, actual_report_id],
    );
    assert!(
        diff["summary"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item.as_str().unwrap().contains("route winner changed"))
    );

    let assessment = run_json(
        "mf-admin",
        &["assess-prediction", prediction_id, actual_report_id],
    );
    assert_eq!(assessment["outcome"], "PredictionUnderestimated");
}

#[test]
#[serial]
fn surface_only_bundle_change_predicts_no_semantic_impact() {
    clear_cache();
    let dir = temp_dir();
    let base_bundle = dir.join("surface_base.qc0");
    let same_bundle = dir.join("surface_same.qc0");
    let content = evaluator_bundle("surface_base", false)
        .replace("THS_LOCAL_EVAL", "THS_SURFACE")
        .replace("CPG_LOCAL_EVAL", "CPG_SURFACE")
        .replace("TGT_LOCAL_EVAL", "TGT_SURFACE")
        .replace("TRL_LOCAL_EVAL", "TRL_SURFACE")
        .replace("OBL_LOCAL_EQ", "OBL_SURFACE");
    write_bundle(&base_bundle, &content);
    write_bundle(&same_bundle, &content);

    let _ = run_json(
        "mf-cli",
        &["certify-bundle", "--file", base_bundle.to_str().unwrap()],
    );
    let prediction = run_json(
        "mf-admin",
        &[
            "predict-impact",
            "--report",
            "REPORT_THS_SURFACE_CPG_SURFACE",
            "--bundle-file",
            same_bundle.to_str().unwrap(),
        ],
    );
    assert_eq!(prediction["prediction"]["class"], "NoImpactPredicted");
}

#[test]
#[serial]
fn execute_plan_reconciles_predicted_route_drift() {
    clear_cache();
    let dir = temp_dir();
    let exec_bundle = dir.join("sched_exec.qc0");
    let sym_bundle = dir.join("sched_sym.qc0");
    write_bundle(
        &exec_bundle,
        &route_bundle_named(
            "sched_exec",
            "ExecutionFirst",
            "THS_SCHED_ROUTE",
            "CPG_SCHED_EXEC",
        ),
    );
    write_bundle(
        &sym_bundle,
        &route_bundle_named(
            "sched_sym",
            "SymbolicFidelityFirst",
            "THS_SCHED_ROUTE",
            "CPG_SCHED_SYMBOLIC",
        ),
    );

    let _ = run_json(
        "mf-cli",
        &["certify-bundle", "--file", exec_bundle.to_str().unwrap()],
    );
    let prediction = run_json(
        "mf-admin",
        &[
            "predict-impact",
            "--report",
            "REPORT_THS_SCHED_ROUTE_CPG_SCHED_EXEC",
            "--bundle-file",
            sym_bundle.to_str().unwrap(),
        ],
    );
    let prediction_id = prediction["prediction"]["id"].as_str().unwrap();
    let execution = run_json("mf-admin", &["execute-plan", "--prediction", prediction_id]);
    assert!(
        execution["outcomes"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["status"] == "Executed")
    );
    assert_eq!(
        execution["resulting_reports"][0]["selected_atlas_cell"],
        "A_FAITHFUL"
    );
    assert!(
        execution["resulting_reports"][0]["reconciliation_summary"]
            .as_array()
            .unwrap()
            .len()
            > 0
    );
}

#[test]
#[serial]
fn runtime_roots_and_artifact_resolution_use_workspace_root() {
    clear_cache();
    let dir = temp_dir();
    let bundle = dir.join("root_exec.qc0");
    write_bundle(&bundle, &route_bundle("root_exec", "ExecutionFirst"));

    let locked = run_json(
        "mf-admin",
        &[
            "lock-bundle",
            bundle.to_str().unwrap(),
            "--optimizer-policy",
            "execution-first",
        ],
    );
    let manifest_id = locked["manifest"]["id"].as_str().unwrap().to_string();

    let roots = run_json("mf-admin", &["dump-runtime-roots"]);
    assert!(
        roots["project_root"]["absolute_path"]
            .as_str()
            .unwrap()
            .contains("Locus64")
    );

    let locator = run_json("mf-admin", &["resolve-artifact-path", &manifest_id]);
    assert!(
        locator["normalized_path"]
            .as_str()
            .unwrap()
            .contains(".mf-cache/namespaces/")
    );
    assert!(
        locator["normalized_path"]
            .as_str()
            .unwrap()
            .contains("/manifests/")
    );
}

#[test]
#[serial]
fn compare_executions_and_reconcile_run_are_live() {
    clear_cache();
    let dir = temp_dir();
    let exec_bundle = dir.join("cmp_exec.qc0");
    let sym_bundle = dir.join("cmp_sym.qc0");
    write_bundle(
        &exec_bundle,
        &route_bundle_named(
            "cmp_exec",
            "ExecutionFirst",
            "THS_CMP_ROUTE",
            "CPG_CMP_EXEC",
        ),
    );
    write_bundle(
        &sym_bundle,
        &route_bundle_named(
            "cmp_sym",
            "SymbolicFidelityFirst",
            "THS_CMP_ROUTE",
            "CPG_CMP_SYM",
        ),
    );
    let _ = run_json(
        "mf-cli",
        &["certify-bundle", "--file", exec_bundle.to_str().unwrap()],
    );
    let pred_a = run_json(
        "mf-admin",
        &[
            "predict-impact",
            "--report",
            "REPORT_THS_CMP_ROUTE_CPG_CMP_EXEC",
            "--bundle-file",
            sym_bundle.to_str().unwrap(),
        ],
    );
    let run_a = run_json(
        "mf-admin",
        &[
            "execute-plan",
            "--prediction",
            pred_a["prediction"]["id"].as_str().unwrap(),
        ],
    );

    let pred_b = run_json(
        "mf-admin",
        &[
            "predict-impact",
            "--report",
            "REPORT_THS_CMP_ROUTE_CPG_CMP_EXEC",
            "--bundle-file",
            exec_bundle.to_str().unwrap(),
        ],
    );
    let run_b = run_json(
        "mf-admin",
        &[
            "execute-plan",
            "--prediction",
            pred_b["prediction"]["id"].as_str().unwrap(),
            "--dry-run",
        ],
    );

    let diff = run_json(
        "mf-admin",
        &[
            "compare-executions",
            run_a["id"].as_str().unwrap(),
            run_b["id"].as_str().unwrap(),
        ],
    );
    assert!(diff["summary"].as_array().unwrap().len() > 0);

    let reconciliation = run_json(
        "mf-admin",
        &[
            "reconcile-run",
            "--prediction",
            pred_a["prediction"]["id"].as_str().unwrap(),
            "--actual-report",
            "REPORT_THS_CMP_ROUTE_CPG_CMP_SYM",
        ],
    );
    assert!(
        reconciliation["assessment"]["outcome"]
            .as_str()
            .unwrap()
            .starts_with("Prediction")
    );
}

#[test]
fn parallel_execute_plan_uses_multiple_lanes_under_namespace_isolation() {
    let namespace = format!(
        "ns_parallel_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let dir = temp_dir();
    let baseline_bundle = dir.join("parallel_base.qc0");
    let proposed_bundle = dir.join("parallel_proposed.qc0");
    write_bundle(&baseline_bundle, &parallel_campaign_bundle("parallel_base"));
    write_bundle(
        &proposed_bundle,
        &parallel_campaign_bundle_with_optimizer("parallel_proposed", "SymbolicFidelityFirst"),
    );

    let _baseline = run_json_ns(
        &namespace,
        "mf-cli",
        &[
            "certify-bundle",
            "--file",
            baseline_bundle.to_str().unwrap(),
        ],
    );
    let report_ref = "REPORT_THS_PAR_A_CPG_PAR_A".to_string();
    let prediction = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "predict-impact",
            "--report",
            &report_ref,
            "--bundle-file",
            proposed_bundle.to_str().unwrap(),
        ],
    );
    let execution = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "execute-plan",
            "--prediction",
            prediction["prediction"]["id"].as_str().unwrap(),
            "--parallel",
            "--max-workers",
            "2",
            "--strict-determinism",
        ],
    );
    assert!(execution["lane_records"].as_array().unwrap().len() >= 2);
    assert_eq!(
        execution["scheduler_policy"]["parallelization"],
        "ParallelIndependent"
    );
    assert_eq!(
        execution["execution_scope"]["cache_namespace"]["id"],
        namespace
    );
}

#[test]
fn schedule_and_namespace_tools_work_without_global_serialization() {
    let namespace = format!(
        "ns_sched_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let dir = temp_dir();
    let baseline_bundle = dir.join("parallel_tools_base.qc0");
    let proposed_bundle = dir.join("parallel_tools_proposed.qc0");
    write_bundle(
        &baseline_bundle,
        &parallel_campaign_bundle("parallel_tools_base"),
    );
    write_bundle(
        &proposed_bundle,
        &parallel_campaign_bundle_with_optimizer("parallel_tools_proposed", "ExecutionFirst"),
    );

    let _baseline = run_json_ns(
        &namespace,
        "mf-cli",
        &[
            "certify-bundle",
            "--file",
            baseline_bundle.to_str().unwrap(),
        ],
    );
    let report_ref = "REPORT_THS_PAR_A_CPG_PAR_A".to_string();
    let prediction = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "predict-impact",
            "--report",
            &report_ref,
            "--bundle-file",
            proposed_bundle.to_str().unwrap(),
        ],
    );
    let execution = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "execute-plan",
            "--prediction",
            prediction["prediction"]["id"].as_str().unwrap(),
            "--parallel",
            "--max-workers",
            "2",
        ],
    );
    assert!(execution["schedule_hash"]["hash"].as_str().unwrap().len() > 0);
    let namespaces = run_json_ns(&namespace, "mf-admin", &["dump-cache-namespaces"]);
    assert!(
        namespaces
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item.as_str() == Some(namespace.as_str()))
    );
}

#[test]
fn obligation_parallel_groups_are_visible_in_reports_and_execution() {
    let namespace = format!(
        "ns_obl_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let dir = temp_dir();
    let baseline_bundle = dir.join("obl_serial.qc0");
    let parallel_bundle = dir.join("obl_parallel.qc0");
    write_bundle(
        &baseline_bundle,
        &obligation_parallel_bundle("obl_serial", false),
    );
    write_bundle(
        &parallel_bundle,
        &obligation_parallel_bundle("obl_parallel", true),
    );

    let baseline = run_json_ns(
        &namespace,
        "mf-cli",
        &[
            "certify-bundle",
            "--file",
            baseline_bundle.to_str().unwrap(),
        ],
    );
    let parallel = run_json_ns(
        &namespace,
        "mf-cli",
        &[
            "certify-bundle",
            "--file",
            parallel_bundle.to_str().unwrap(),
        ],
    );
    assert_eq!(baseline[0]["obligation_lanes"].as_array().unwrap().len(), 1);
    assert!(parallel[0]["obligation_lanes"].as_array().unwrap().len() >= 2);
    assert!(
        parallel[0]["replay_legality_checks"]
            .as_array()
            .unwrap()
            .len()
            >= 3
    );
}

#[test]
fn replay_with_lock_exposes_obligation_parallel_replay_structure() {
    let namespace = format!(
        "ns_obl_replay_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let dir = temp_dir();
    let bundle = dir.join("obl_replay.qc0");
    write_bundle(&bundle, &obligation_parallel_bundle("obl_replay", true));

    let lock_output = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "lock-bundle",
            bundle.to_str().unwrap(),
            "--optimizer-policy",
            "conservative",
        ],
    );
    let lock_id = lock_output["lock"]["id"].as_str().unwrap();
    let replay = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "replay-with-lock",
            lock_id,
            "--parallel-obligations",
            "--max-obligation-workers",
            "3",
        ],
    );
    assert!(
        replay.as_array().unwrap()[0]["replay_legality_checks"]
            .as_array()
            .unwrap()
            .len()
            >= 3
    );
    assert!(
        replay.as_array().unwrap()[0]["obligation_lanes"]
            .as_array()
            .unwrap()
            .len()
            >= 2
    );
}

#[test]
fn flagship_chain_rule_bundle_uses_scheduler_and_surfaces_deficiencies() {
    let namespace = format!(
        "ns_chain_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let dir = temp_dir();
    let bundle = dir.join("chain_rule_parallel.qc0");
    write_bundle(
        &bundle,
        &chain_rule_bundle("chain_rule_parallel", true, false, false),
    );

    let report = run_json_ns(
        &namespace,
        "mf-cli",
        &[
            "certify-bundle",
            "--file",
            bundle.to_str().unwrap(),
            "--conflict-policy",
            "exact-match",
        ],
    );
    assert_eq!(report[0]["campaign_id"], "CPG_CHAIN_RULE");
    assert_eq!(report[0]["selected_atlas_cell"], "A_TOP_TO_CALC");
    assert_eq!(report[0]["verdict"], "Integrated");
    assert!(
        report[0]["obligations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| {
                item["obligation_id"] == "OBL_CHAIN_EQ"
                    && item["evaluation_mode"] == "RecomputedExact"
                    && item["receipts"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|receipt| receipt["id"] == "EQR_CHAIN_OPERATOR_DEFAULT_SELECTION")
            })
    );
    assert!(
        report[0]["obligations"]
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
    assert!(report[0]["reasons"].as_array().unwrap().iter().any(|item| {
        item.as_str()
            .unwrap()
            .contains("flagship adequacy-first campaign selected")
    }));
    assert!(
        report[0]["reasons"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item.as_str().unwrap().contains("reduction witness exactly"))
    );
    assert!(report[0]["obligation_lanes"].as_array().unwrap().len() >= 2);
    assert!(
        !report[0]["deficiencies"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["id"] == "DGN_CHAIN_RULE_ADEQUACY")
    );
    assert!(
        report[0]["promotion_artifact_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "OPR_PROMOTED_OPR_CHAIN1")
    );
    assert!(
        report[0]["default_selected_artifact_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "OPR_PROMOTED_OPR_CHAIN1")
    );
    assert!(report[0]["reasons"].as_array().unwrap().iter().any(|item| {
        item.as_str()
            .unwrap()
            .contains("default-selected the canonical Chain₁ operator")
    }));
}

#[test]
fn flagship_chain_rule_lock_replay_and_obligation_comparison_are_deterministic() {
    let namespace = format!(
        "ns_chain_lock_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let dir = temp_dir();
    let baseline_bundle = dir.join("chain_rule_serial.qc0");
    let proposed_bundle = dir.join("chain_rule_parallel.qc0");
    write_bundle(
        &baseline_bundle,
        &chain_rule_bundle("chain_rule_serial", false, false, false),
    );
    write_bundle(
        &proposed_bundle,
        &chain_rule_bundle("chain_rule_parallel", true, false, false),
    );

    let _baseline = run_json_ns(
        &namespace,
        "mf-cli",
        &[
            "certify-bundle",
            "--file",
            baseline_bundle.to_str().unwrap(),
            "--conflict-policy",
            "exact-match",
        ],
    );
    let prediction = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "predict-impact",
            "--report",
            "REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE",
            "--bundle-file",
            proposed_bundle.to_str().unwrap(),
            "--conflict-policy",
            "exact-match",
        ],
    );
    let prediction_id = prediction["prediction"]["id"].as_str().unwrap();
    let execution_a = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "execute-plan",
            "--prediction",
            prediction_id,
            "--parallel-obligations",
            "--max-obligation-workers",
            "3",
        ],
    );
    let execution_b = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "execute-plan",
            "--prediction",
            prediction_id,
            "--parallel-obligations",
            "--max-obligation-workers",
            "3",
        ],
    );
    let explain = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "explain-obligation-plan",
            execution_a["id"].as_str().unwrap(),
        ],
    );
    assert!(explain["plans"].as_array().unwrap().len() > 0);
    assert!(
        explain["obligation_statuses"][0]["obligations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| {
                item["obligation_id"] == "OBL_CHAIN_EQ"
                    && item["receipts"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|receipt| receipt["id"] == "EQR_CHAIN_OPERATOR_DEFAULT_SELECTION")
            })
    );
    assert!(
        explain["obligation_statuses"][0]["obligations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| {
                item["obligation_id"] == "OBL_CHAIN_RED"
                    && item["receipts"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|receipt| receipt["id"] == "RED_CHAIN_OPERATOR_DEFAULT_SELECTION")
            })
    );
    let compare = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "compare-obligation-executions",
            execution_a["id"].as_str().unwrap(),
            execution_b["id"].as_str().unwrap(),
        ],
    );
    assert_eq!(compare["same"], true);

    let lock_output = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "lock-bundle",
            proposed_bundle.to_str().unwrap(),
            "--optimizer-policy",
            "conservative",
            "--conflict-policy",
            "exact-match",
        ],
    );
    let lock_id = lock_output["lock"]["id"].as_str().unwrap();
    let replay = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "replay-with-lock",
            lock_id,
            "--parallel-obligations",
            "--max-obligation-workers",
            "3",
        ],
    );
    assert!(
        replay.as_array().unwrap()[0]["obligation_lanes"]
            .as_array()
            .unwrap()
            .len()
            >= 2
    );
    assert!(
        replay.as_array().unwrap()[0]["obligations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| {
                item["obligation_id"] == "OBL_CHAIN_EQ"
                    && item["receipts"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|receipt| receipt["id"] == "EQR_CHAIN_OPERATOR_DEFAULT_SELECTION")
            })
    );
}

#[test]
fn flagship_chain_rule_strict_evaluator_emits_structured_blockers() {
    let namespace = format!(
        "ns_chain_fail_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let dir = temp_dir();
    let bundle = dir.join("chain_rule_strict.qc0");
    write_bundle(
        &bundle,
        &chain_rule_bundle("chain_rule_strict", true, true, false),
    );

    let report = run_json_ns(
        &namespace,
        "mf-cli",
        &[
            "certify-bundle",
            "--file",
            bundle.to_str().unwrap(),
            "--conflict-policy",
            "exact-match",
        ],
    );
    assert_eq!(report[0]["verdict"], "Integrated");
    assert!(
        !report[0]["deficiencies"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["id"] == "DGN_CHAIN_RULE_ADEQUACY")
    );
    assert!(
        !report[0]["deficiencies"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["message"].as_str().unwrap().contains("OBL_CHAIN_RED"))
    );
    assert!(
        report[0]["promotion_artifact_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "OPR_PROMOTED_OPR_CHAIN1")
    );
    assert!(
        report[0]["default_selected_artifact_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "OPR_PROMOTED_OPR_CHAIN1")
    );
}

#[test]
fn flagship_chain_rule_promoted_operator_is_reused_and_integrated() {
    let namespace = format!(
        "ns_chain_integrated_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let dir = temp_dir();
    let baseline_bundle = dir.join("chain_rule_integrated_base.qc0");
    let integrated_bundle = dir.join("chain_rule_integrated.qc0");
    write_bundle(
        &baseline_bundle,
        &chain_rule_bundle("chain_rule_integrated_base", true, false, false),
    );
    write_bundle(
        &integrated_bundle,
        &chain_rule_bundle("chain_rule_integrated", true, false, true),
    );

    let baseline = run_json_ns(
        &namespace,
        "mf-cli",
        &[
            "certify-bundle",
            "--file",
            baseline_bundle.to_str().unwrap(),
            "--conflict-policy",
            "exact-match",
        ],
    );
    let integrated = run_json_ns(
        &namespace,
        "mf-cli",
        &[
            "certify-bundle",
            "--file",
            integrated_bundle.to_str().unwrap(),
            "--conflict-policy",
            "exact-match",
        ],
    );

    assert_eq!(baseline[0]["verdict"], "Integrated");
    assert!(
        baseline[0]["default_selected_artifact_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "OPR_PROMOTED_OPR_CHAIN1")
    );
    assert!(
        baseline[0]["obligations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["obligation_id"] == "OBL_CHAIN_EQ"
                && item["receipts"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|receipt| receipt["id"] == "EQR_CHAIN_OPERATOR_DEFAULT_SELECTION"))
    );
    assert_eq!(integrated[0]["verdict"], "Integrated");
    assert!(
        integrated[0]["reused_artifact_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "OPR_PROMOTED_OPR_CHAIN1")
    );
    assert!(
        integrated[0]["default_selected_artifact_ids"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert!(
        integrated[0]["obligations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["obligation_id"] == "OBL_CHAIN_EQ"
                && item["receipts"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|receipt| receipt["id"] == "EQR_CHAIN_OPERATOR_REUSE"))
    );
    assert!(
        integrated[0]["obligations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["obligation_id"] == "OBL_CHAIN_RED"
                && item["receipts"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|receipt| receipt["id"] == "RED_CHAIN_OPERATOR_REUSE"))
    );

    let baseline_eq_receipts = baseline[0]["obligations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["obligation_id"] == "OBL_CHAIN_EQ")
        .unwrap()["receipts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item["id"].as_str().unwrap().to_string())
        .collect::<Vec<_>>();
    let integrated_eq_receipts = integrated[0]["obligations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["obligation_id"] == "OBL_CHAIN_EQ")
        .unwrap()["receipts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item["id"].as_str().unwrap().to_string())
        .collect::<Vec<_>>();
    assert!(
        baseline_eq_receipts
            .iter()
            .any(|item| item == "EQR_CHAIN_OPERATOR_DEFAULT_SELECTION")
    );
    assert!(
        integrated_eq_receipts
            .iter()
            .any(|item| item == "EQR_CHAIN_OPERATOR_REUSE")
    );
    assert!(
        integrated[0]["reasons"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item
                .as_str()
                .unwrap()
                .contains("reused promoted Chain₁ operator"))
    );
    assert!(
        baseline[0]["reasons"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item
                .as_str()
                .unwrap()
                .contains("default-selected the canonical Chain₁ operator"))
    );

    let lock_output = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "lock-bundle",
            integrated_bundle.to_str().unwrap(),
            "--optimizer-policy",
            "conservative",
            "--conflict-policy",
            "exact-match",
        ],
    );
    let lock_id = lock_output["lock"]["id"].as_str().unwrap();
    let replay = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "replay-with-lock",
            lock_id,
            "--parallel-obligations",
            "--max-obligation-workers",
            "3",
        ],
    );
    assert_eq!(replay[0]["verdict"], "Integrated");
    assert!(
        replay[0]["reused_artifact_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "OPR_PROMOTED_OPR_CHAIN1")
    );
}

#[test]
fn second_burden_recipe_path_auto_reuses_chain1_with_payoff() {
    let namespace = format!(
        "ns_chain_recipe_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let report = run_json_ns(
        &namespace,
        "mf-cli",
        &["certify-derived", "--campaign", "CPG_CHAIN_RULE_RECIPE"],
    );
    assert_eq!(report["campaign_id"], "CPG_CHAIN_RULE_RECIPE");
    assert_eq!(report["verdict"], "Integrated");
    assert!(
        report["default_selected_artifact_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "OPR_PROMOTED_OPR_CHAIN1")
    );
    assert!(
        report["payoff_receipt_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "PAY_CHAIN1_SECOND_BURDEN_DEFAULT_REUSE")
    );
    assert!(
        report["payoff_receipt_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "PAY_CHAIN1_OBLIGATION_COUNT_REDUCTION_4_TO_2")
    );
    assert_eq!(report["obligations"].as_array().unwrap().len(), 2);
    assert!(
        report["reasons"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item.as_str().unwrap().contains("adjacent recipe burden"))
    );
}

#[test]
fn adjacent_transport_path_auto_reuses_chain1_with_payoff() {
    let namespace = format!(
        "ns_chain_transport_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let report = run_json_ns(
        &namespace,
        "mf-cli",
        &["certify-derived", "--campaign", "CPG_CHAIN_RULE_TRANSPORT"],
    );
    assert_eq!(report["campaign_id"], "CPG_CHAIN_RULE_TRANSPORT");
    assert_eq!(report["verdict"], "Integrated");
    assert!(
        report["default_selected_artifact_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "OPR_PROMOTED_OPR_CHAIN1")
    );
    assert!(
        report["payoff_receipt_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "PAY_CHAIN1_ADJACENT_TRANSPORT_DEFAULT_REUSE")
    );
    assert!(
        report["payoff_receipt_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "PAY_CHAIN1_LOCALITY_WITNESS_RETAINED")
    );
    assert!(
        report["payoff_receipt_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "PAY_CHAIN1_OBLIGATION_COUNT_REDUCTION_4_TO_3")
    );
    assert_eq!(report["obligations"].as_array().unwrap().len(), 3);
    assert!(
        report["obligations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["obligation_id"] == "OBL_CHAIN_LOC")
    );
    assert!(
        report["reasons"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item.as_str().unwrap().contains("adjacent transport burden"))
    );
}

#[test]
fn bayes_brace_observation_surfaces_active_adequacy_without_deficiencies() {
    let namespace = format!(
        "ns_bayes_brace_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let report = run_json_ns(
        &namespace,
        "mf-cli",
        &["certify-derived", "--campaign", "CPG_BAYES_BRACE"],
    );
    assert_eq!(report["campaign_id"], "CPG_BAYES_BRACE");
    assert_eq!(report["verdict"], "Certified");
    let observed = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "observe-run",
            "--report",
            "REPORT_THS_BAYES_BRACE_CPG_BAYES_BRACE",
        ],
    );
    assert!(
        observed[0]["artifact"]["graph"]["obligation_traces"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["obligation_id"] == "OBL_BAYES_ADE"
                && item["evaluation_mode"] == "RecomputedExact")
    );
    assert!(
        report["adequacy_records"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["clause_id"] == "ADQ_BAYES_TOP_PROB_BRIDGE")
    );
    assert!(report["deficiencies"].as_array().unwrap().is_empty());
}

#[test]
fn ch_norm_observation_surfaces_exact_type_normalization_closure() {
    let namespace = format!(
        "ns_ch_norm_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let report = run_json_ns(
        &namespace,
        "mf-cli",
        &["certify-derived", "--campaign", "CPG_CH_NORM"],
    );
    assert_eq!(report["campaign_id"], "CPG_CH_NORM");
    assert_eq!(report["verdict"], "Certified");
    assert!(report["deficiencies"].as_array().unwrap().is_empty());
    let observed = run_json_ns(
        &namespace,
        "mf-admin",
        &["observe-run", "--report", "REPORT_THS_CH_NORM_CPG_CH_NORM"],
    );
    assert!(
        observed[0]["artifact"]["graph"]["obligation_traces"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["obligation_id"] == "OBL_CH_ADE"
                && item["evaluation_mode"] == "RecomputedExact")
    );
}

#[test]
fn exec_infer_observation_surfaces_active_adequacy_without_deficiencies() {
    let namespace = format!(
        "ns_exec_infer_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let report = run_json_ns(
        &namespace,
        "mf-cli",
        &["certify-derived", "--campaign", "CPG_EXEC_INFER"],
    );
    assert_eq!(report["campaign_id"], "CPG_EXEC_INFER");
    assert_eq!(report["verdict"], "Certified");
    let observed = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "observe-run",
            "--report",
            "REPORT_THS_EXEC_INFER_CPG_EXEC_INFER",
        ],
    );
    assert!(
        observed[0]["artifact"]["graph"]["obligation_traces"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["obligation_id"] == "OBL_EXEC_RED"
                && item["evaluation_mode"] == "RecomputedExact")
    );
    assert!(
        report["adequacy_records"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["clause_id"] == "ADQ_EXEC_PROB_COMP_BRIDGE")
    );
    assert!(report["deficiencies"].as_array().unwrap().is_empty());
}

#[test]
fn cert_prop_observation_surfaces_active_adequacy_without_deficiencies() {
    let namespace = format!(
        "ns_cert_prop_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let report = run_json_ns(
        &namespace,
        "mf-cli",
        &["certify-derived", "--campaign", "CPG_CERT_PROP"],
    );
    assert_eq!(report["campaign_id"], "CPG_CERT_PROP");
    assert_eq!(report["verdict"], "Certified");
    let observed = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "observe-run",
            "--report",
            "REPORT_THS_CERT_PROP_CPG_CERT_PROP",
        ],
    );
    assert!(
        observed[0]["artifact"]["graph"]["obligation_traces"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["obligation_id"] == "OBL_CERT_OBS"
                && item["evaluation_mode"] == "RecomputedExact")
    );
    assert!(
        report["adequacy_records"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["clause_id"] == "ADQ_CERT_COMP_LOG_BRIDGE")
    );
    assert!(report["deficiencies"].as_array().unwrap().is_empty());
}

#[test]
fn ch_inh_observation_surfaces_exact_type_algebra_closure() {
    let namespace = format!(
        "ns_ch_inh_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let report = run_json_ns(
        &namespace,
        "mf-cli",
        &["certify-derived", "--campaign", "CPG_CH_INH"],
    );
    assert_eq!(report["campaign_id"], "CPG_CH_INH");
    assert_eq!(report["verdict"], "Certified");
    assert!(report["deficiencies"].as_array().unwrap().is_empty());
    let observed = run_json_ns(
        &namespace,
        "mf-admin",
        &["observe-run", "--report", "REPORT_THS_CH_INH_CPG_CH_INH"],
    );
    assert!(
        observed[0]["artifact"]["graph"]["obligation_traces"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["obligation_id"] == "OBL_CHI_EQ"
                && item["evaluation_mode"] == "RecomputedExact")
    );
    assert!(
        observed[0]["artifact"]["graph"]["obligation_traces"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["obligation_id"] == "OBL_CHI_ADE"
                && item["evaluation_mode"] == "RecomputedExact")
    );
}

#[test]
fn replay_with_lock_preserves_active_adequacy_records() {
    let namespace = format!(
        "ns_chain_adequacy_replay_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let lock_output = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "lock-bundle",
            "samples/chain_rule_integrated_bundle.qc0",
            "--optimizer-policy",
            "conservative",
            "--conflict-policy",
            "exact-match",
        ],
    );
    let lock_id = lock_output["lock"]["id"].as_str().unwrap();
    let replay = run_json_ns(
        &namespace,
        "mf-admin",
        &[
            "replay-with-lock",
            lock_id,
            "--parallel-obligations",
            "--max-obligation-workers",
            "3",
        ],
    );
    assert!(
        replay[0]["adequacy_records"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["clause_id"] == "ADQ_CHAIN_TOP_CALC_BRIDGE")
    );
    assert!(
        replay[0]["adequacy_records"]
            .as_array()
            .unwrap()
            .iter()
            .all(|item| item["verdict"] == "Certified")
    );
}
