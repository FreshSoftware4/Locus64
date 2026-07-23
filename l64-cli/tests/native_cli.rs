use assert_cmd::Command;
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

const SOURCE: &[u8] = b"L64R1 0x4c36344e41544956\na 1 0x41\na 2 0x42\na 3 0x43\nf 4 1 2\nf 5 2 3\nf 6 1 3\nv 7 4\nv 8 5\nc 9 7 8 6\n";

fn fixture(name: &str, bytes: &[u8]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "l64_native_cli_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    fs::write(&path, bytes).unwrap();
    path
}

#[test]
fn native_compile_and_sequence_use_existing_commands() {
    let rna = fixture("sample.rna", SOURCE);
    let dna = rna.with_extension("dna");

    Command::cargo_bin("l64-cli")
        .unwrap()
        .args([
            "compile-rna",
            rna.to_str().unwrap(),
            "--out",
            dna.to_str().unwrap(),
        ])
        .assert()
        .success();
    assert!(fs::read(&dna).unwrap().starts_with(b"L64D"));

    let output = Command::cargo_bin("l64-cli")
        .unwrap()
        .args(["sequence-dna", dna.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, SOURCE);
}

#[test]
fn invalid_native_operation_cannot_create_dna() {
    let source = b"L64R1 0x1\na 1 0x52\nm 2 1 2 3\nm 3 1 4 2\nm 4 1 2 2\nv 5 2\nv 6 3\nx 7 5 6 4\n";
    let rna = fixture("invalid.rna", source);
    let dna = rna.with_extension("dna");

    Command::cargo_bin("l64-cli")
        .unwrap()
        .args([
            "compile-rna",
            rna.to_str().unwrap(),
            "--out",
            dna.to_str().unwrap(),
        ])
        .assert()
        .failure();
    assert!(!dna.exists());
}

#[test]
fn native_path_rejects_legacy_only_flags() {
    let rna = fixture("sample.rna", SOURCE);
    Command::cargo_bin("l64-cli")
        .unwrap()
        .args(["compile-rna", rna.to_str().unwrap(), "--persist-lineage"])
        .assert()
        .failure();
    Command::cargo_bin("l64-cli")
        .unwrap()
        .args([
            "verify-roundtrip",
            rna.to_str().unwrap(),
            "--artifact-class",
            "genome",
        ])
        .assert()
        .failure();
}

#[test]
fn non_native_commands_still_reach_legacy_dispatch() {
    Command::cargo_bin("l64-cli")
        .unwrap()
        .arg("--help")
        .assert()
        .success();
}
