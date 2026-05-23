use anyhow::{Result, anyhow};
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, exit};

const ADMIN_COMMANDS: &[&str] = &[
    "lock-bundle",
    "dump-execution-manifest",
    "dump-runtime-roots",
    "replay-with-lock",
    "observe-run",
    "compare-locks",
    "compare-reports",
    "compare-manifests",
    "compare-report-manifest",
    "predict-impact",
    "plan-recompute",
    "execute-plan",
    "explain-execution",
    "dump-execution-dag",
    "dump-lane-plan",
    "explain-obligation-plan",
    "dump-obligation-dag",
    "dump-obligation-lanes",
    "compare-obligation-executions",
    "compare-schedules",
    "explain-plan",
    "explain-drift",
    "compare-executions",
    "assess-prediction",
    "reconcile-run",
    "dump-policy-graph",
    "explain-policy-resolution",
    "dump-observation",
    "dump-drift",
    "dump-diff",
    "dump-prediction",
    "dump-plan",
    "reconcile-prediction",
    "compare-artifacts",
    "export-artifact",
    "resolve-artifact-path",
    "classify-artifact",
    "check-command-input",
    "dump-command-contracts",
    "dump-artifact-contracts",
    "dump-capability-readiness",
    "dump-cache-namespaces",
];

fn main() -> Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() || matches!(args[0].as_str(), "-h" | "--help" | "help") {
        print_help();
        return Ok(());
    }
    if args[0] == "authority-audit" {
        print_authority_audit();
        return Ok(());
    }

    let target = if ADMIN_COMMANDS.iter().any(|cmd| *cmd == args[0]) {
        sibling_binary("l64-admin")
    } else {
        sibling_binary("l64-cli")
    }?;

    let status = Command::new(target).args(&args).status()?;
    match status.code() {
        Some(code) => exit(code),
        None => Err(anyhow!("child process terminated without an exit code")),
    }
}

fn sibling_binary(name: &str) -> Result<PathBuf> {
    let exe = env::current_exe()?;
    let dir = exe
        .parent()
        .ok_or_else(|| anyhow!("current executable has no parent directory"))?;
    let candidates = binary_candidates(dir, name);
    candidates
        .into_iter()
        .find(|path| path.exists())
        .ok_or_else(|| anyhow!("could not locate sibling binary `{name}`"))
}

fn binary_candidates(dir: &Path, name: &str) -> Vec<PathBuf> {
    #[cfg(windows)]
    let mut paths = vec![dir.join(name)];
    #[cfg(not(windows))]
    let paths = vec![dir.join(name)];
    #[cfg(windows)]
    {
        paths.push(dir.join(format!("{name}.exe")));
    }
    paths
}

fn print_help() {
    println!("l64");
    println!();
    println!("Canonical Locus64 wrapper.");
    println!("Routes into the Locus Kernel command surface.");
    println!("Runs `l64-cli` or `l64-admin` based on the first command verb.");
    println!();
    println!("Examples:");
    println!("  l64 authority-audit");
    println!("  l64 normalize-rna sample.gene.rna");
    println!("  l64 compile-rna sample.gene.rna --out sample.gene.dna --artifact-class gene");
    println!("  l64 sequence-dna sample.gene.dna");
    println!("  l64 certify-derived --campaign CPG_CHAIN_RULE");
    println!("  l64 observe-run --report REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE");
    println!("  l64 research-import --kind task samples/research/task_operational_truth.json");
    println!(
        "  l64 research-route --task-id TASK_OPERATIONAL_TRUTH_HARDENING --signature-id SIG_OPERATIONAL_TRUTH_HARDENING"
    );
}

fn print_authority_audit() {
    println!("Locus64 authority audit");
    println!("substrate_authority: RNA,DNA,lower-chain receipts");
    println!("derived_semantic: certification,research,coverage,tower reports");
    println!("extraction_sources: QC0,QA0,QK0,QM0,JSON report imports");
    println!("deletion_targets: l64-qc0,l64-qa0,l64-qk0,l64-qm0,l64-surfaces");
    println!("public_surface_target: RNA,DNA");
    println!("public_command_target: l64");
}
