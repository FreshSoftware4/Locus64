use std::{
    fs,
    path::{Path, PathBuf},
};

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn rust_source_under(path: &Path, output: &mut String) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            rust_source_under(&path, output);
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            output.push_str(&fs::read_to_string(path).unwrap());
            output.push('\n');
        }
    }
}

#[test]
fn transport_has_only_native_authority_execution_and_certification_contacts() {
    let manifest = fs::read_to_string(crate_root().join("Cargo.toml")).unwrap();
    assert!(manifest.contains("l64-native"));
    assert!(manifest.contains("l64-execution"));
    assert!(manifest.contains("l64-certification"));
    assert!(!manifest.contains("l64-projection"));
    for forbidden in [
        "serde",
        "serde_json",
        "bincode",
        "l64-core",
        "l64-locus",
        "l64-registry",
        "l64-bundle",
        "l64-runtime",
        "anyhow",
        "thiserror",
    ] {
        assert!(
            !manifest.contains(forbidden),
            "native transport must not depend on {forbidden}"
        );
    }
}

#[test]
fn transport_source_has_no_composite_authority_or_generic_digest_layer() {
    let mut source = String::new();
    rust_source_under(&crate_root().join("src"), &mut source);
    for forbidden in [
        "blake3",
        "sha2",
        "DigestRole",
        "RoleDigest",
        "BundleManifest",
        "OverlayRegistry",
        "conflict_policy",
        "registry",
    ] {
        assert!(
            !source.contains(forbidden),
            "transport source must not contain {forbidden}"
        );
    }
}
