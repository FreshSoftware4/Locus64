use std::process::Command;

#[test]
fn exports_seed_atlas_through_binary_surface() {
    let output = Command::new(env!("CARGO_BIN_EXE_l64-atlas-export"))
        .args(["--dump-edges", "--dump-cells"])
        .output()
        .expect("run l64-atlas-export");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("parse atlas export JSON");
    assert!(value["edge_count"].as_u64().unwrap_or(0) > 0);
    assert!(value["atlas_cell_count"].as_u64().unwrap_or(0) > 0);
    assert_eq!(
        value["edges"].as_array().map(Vec::len),
        value["edge_count"].as_u64().map(|count| count as usize)
    );
    assert_eq!(
        value["cells"].as_array().map(Vec::len),
        value["atlas_cell_count"]
            .as_u64()
            .map(|count| count as usize)
    );
}
