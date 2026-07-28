use std::{fs, path::Path};

#[test]
fn observation_depends_only_on_current_native_carriers() {
    let manifest = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
        .expect("read observation manifest");
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
            "l64-projection = { path = \"../l64-projection\" }",
            "l64-transport = { path = \"../l64-transport\" }",
            "l64-certification = { path = \"../l64-certification\" }",
        ]
    );
}

#[test]
fn observation_source_contains_no_schema_forest() {
    let source = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("lib.rs"),
    )
    .expect("read observation source");
    for forbidden in [
        "serde",
        "bincode",
        "HashMap",
        "std::fs",
        "thread",
        "Mutex",
        "RwLock",
        "cache",
        "Registry",
        "BundleWorld",
        "Policy",
        "Campaign",
        "Receipt",
        "digest",
        "blake3",
    ] {
        assert!(
            !source.contains(forbidden),
            "native observation source contains forbidden token {forbidden}"
        );
    }
}
