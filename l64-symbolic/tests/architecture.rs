use core::mem::size_of;
use l64_symbolic::SymbolicSeal;
use std::{fs, path::Path};

#[test]
fn symbolic_seal_is_exactly_one_fixed_width_frame_field() {
    assert_eq!(size_of::<SymbolicSeal>(), 32);
    assert_eq!(SymbolicSeal::ZERO.to_bytes(), [0; 32]);
}

#[test]
fn symbolic_carrier_has_no_external_or_cryptographic_dependency() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = fs::read_to_string(root.join("Cargo.toml")).unwrap();
    assert!(!manifest.contains("[dependencies]"));

    let source = fs::read_to_string(root.join("src/lib.rs")).unwrap();
    for forbidden in [
        ["blake", "3"].concat(),
        ["sha", "2"].concat(),
        ["serde", "::"].concat(),
        ["unsafe", " {"].concat(),
    ] {
        assert!(!source.contains(&forbidden), "forbidden token: {forbidden}");
    }
}
