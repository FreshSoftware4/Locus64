use l64_release::{ReleaseError, export_native_release};
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

const SOURCE: &[u8] = b"L64R1 0x4c36344e41544956\na 1 0x41\na 2 0x42\na 3 0x43\nf 4 1 2\nf 5 2 3\nf 6 1 3\nv 7 4\nv 8 5\nc 9 7 8 6\n";

fn root(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "l64_release_{name}_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

#[test]
fn release_contains_one_authority_and_verified_projection() {
    let out = root("complete");
    let release = export_native_release(SOURCE, &out).unwrap();
    assert!(fs::read(&release.authority).unwrap().starts_with(b"L64D"));
    assert_eq!(fs::read(&release.source).unwrap(), SOURCE);
    let projection = fs::read_to_string(&release.projection).unwrap();
    assert!(projection.starts_with("L64 NATIVE PROJECTION v1\n"));
    let record = fs::read_to_string(&release.record).unwrap();
    assert!(record.contains("projection_authority=non_authoritative"));
    assert!(record.contains("projection_verified=true"));
    assert_eq!(fs::read_dir(&release.root).unwrap().count(), 4);
    fs::remove_dir_all(out).unwrap();
}

#[test]
fn release_refuses_to_replace_existing_output() {
    let out = root("existing");
    fs::create_dir_all(&out).unwrap();
    assert!(matches!(
        export_native_release(SOURCE, &out),
        Err(ReleaseError::OutputExists(path)) if path == out
    ));
    fs::remove_dir_all(out).unwrap();
}
