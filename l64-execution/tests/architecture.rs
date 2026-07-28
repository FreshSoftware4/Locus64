use std::{fs, path::Path};

#[test]
fn execution_is_dependency_minimal_and_nonpersistent() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = fs::read_to_string(root.join("Cargo.toml")).unwrap();
    for dependency in ["l64-native", "l64-projection", "l64-certification"] {
        assert!(manifest.contains(dependency));
    }
    for forbidden in [
        "serde",
        "bincode",
        "blake3",
        "l64-transport",
        "l64-observation",
        "l64-change",
    ] {
        assert!(
            !manifest.contains(forbidden),
            "forbidden dependency {forbidden}"
        );
    }
    let source = fs::read_to_string(root.join("src/lib.rs")).unwrap();
    for forbidden in [
        "std::fs",
        "File::",
        "create_dir",
        "write(",
        "cache",
        "registry",
        "campaign",
    ] {
        assert!(
            !source.contains(forbidden),
            "forbidden execution surface {forbidden}"
        );
    }
}
