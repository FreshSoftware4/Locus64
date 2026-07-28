use std::{fs, path::Path};

#[test]
fn projection_crate_depends_only_on_native_authority() {
    let manifest = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
        .expect("read projection manifest");
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
        vec!["l64-native = { path = \"../l64-native\" }"]
    );
}

#[test]
fn projection_source_contains_no_parallel_authority_machinery() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut source = String::new();
    for entry in fs::read_dir(root).expect("read source directory") {
        let path = entry.expect("source entry").path();
        if path.extension().and_then(|value| value.to_str()) == Some("rs") {
            source.push_str(&fs::read_to_string(path).expect("read source file"));
        }
    }
    for forbidden in [
        "std::fs",
        "File::",
        "OpenOptions",
        "serde",
        "bincode",
        "blake3",
        "HashMap",
        "Mutex",
        "RwLock",
        "thread::",
        "spawn(",
        "Registry",
        "persist_",
        "import_",
        "promote_",
    ] {
        assert!(
            !source.contains(forbidden),
            "projection source contains forbidden authority token {forbidden}"
        );
    }
}
