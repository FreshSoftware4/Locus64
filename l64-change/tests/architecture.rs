use std::{fs, path::Path};

#[test]
fn change_depends_only_on_current_native_carriers() {
    let manifest = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
        .expect("read change manifest");
    let dependency_block = manifest
        .split_once("[dependencies]")
        .map(|(_, tail)| tail)
        .expect("dependency block");
    let dependencies = dependency_block
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect::<Vec<_>>();
    assert_eq!(
        dependencies,
        vec![
            "l64-native = { path = \"../l64-native\" }",
            "l64-transport = { path = \"../l64-transport\" }",
            "l64-certification = { path = \"../l64-certification\" }",
            "l64-observation = { path = \"../l64-observation\" }",
        ]
    );
}

#[test]
fn change_source_contains_no_schema_forest() {
    let source = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("lib.rs"),
    )
    .expect("read change source");
    for forbidden in [
        "serde",
        "bincode",
        "serde_json",
        "std::fs",
        "HashMap",
        "thread",
        "Mutex",
        "RwLock",
        "cache",
        "Registry",
        "BundleWorld",
        "Policy",
        "Campaign",
        "Receipt",
        "Prediction",
        "PlanRecompute",
        "digest",
        "blake3",
    ] {
        assert!(
            !source.contains(forbidden),
            "native change source contains forbidden token {forbidden}"
        );
    }
}

#[test]
fn retired_legacy_authority_island_is_absent() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root");
    for deleted in [
        "l64-core",
        "l64-locus",
        "l64-research",
        "l64-kernel",
        "l64-command",
        "l64-registry",
        "l64-selector",
        "l64-runtime",
        "l64-canon",
        "l64-atlas",
        "l64-cert",
        "l64-testkit",
    ] {
        assert!(
            !root.join(deleted).exists(),
            "legacy crate {deleted} still exists"
        );
    }

    let membrane = fs::read_to_string(root.join("l64-cli/src/native_membrane.rs"))
        .expect("read native membrane");
    for command in [
        "run-theorem",
        "research-status",
        "tower-step",
        "replay-report",
    ] {
        assert!(
            membrane.contains(&format!("\"{command}\"")),
            "retired authority command {command} is not rejected by the membrane"
        );
    }
    assert!(membrane.contains("permanently deleted after historical export"));

    let wrapper =
        fs::read_to_string(root.join("l64/src/main.rs")).expect("read wrapper entry point");
    assert!(wrapper.contains("legacy theorem/campaign/research/registry/tower execution island"));
    assert!(!wrapper.contains("REMOVED_ADMIN_COMMANDS"));
}
