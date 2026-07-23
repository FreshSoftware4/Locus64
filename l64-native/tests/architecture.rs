use core::mem::size_of;
use std::{fs, path::Path};

use l64_native::{ClosureTransition, Dimension, Node, Port};

#[test]
fn compact_layout_budgets_hold() {
    assert!(
        size_of::<Node>() <= 32,
        "Node grew to {} bytes",
        size_of::<Node>()
    );
    assert!(
        size_of::<Port>() <= 8,
        "Port grew to {} bytes",
        size_of::<Port>()
    );
    assert!(
        size_of::<ClosureTransition>() <= 24,
        "ClosureTransition grew to {} bytes",
        size_of::<ClosureTransition>()
    );
    assert!(
        size_of::<Dimension>() <= 8,
        "Dimension grew to {} bytes",
        size_of::<Dimension>()
    );
}

#[test]
fn native_source_rejects_coordination_heavy_dependencies() {
    fn collect_rust(path: &Path, source: &mut std::string::String) {
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                collect_rust(&path, source);
            } else if path.extension().and_then(|part| part.to_str()) == Some("rs") {
                source.push_str(&fs::read_to_string(path).unwrap());
            }
        }
    }

    let source_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut source = std::string::String::new();
    collect_rust(&source_root, &mut source);

    let forbidden = [
        ["Str", "ing"].concat(),
        ["HashMap", "<", "Str", "ing"].concat(),
        ["serde", "_json"].concat(),
        ["bin", "code"].concat(),
        ["Ser", "ialize"].concat(),
        ["Deser", "ialize"].concat(),
        ["claim", "_packet"].concat(),
        ["theorem", "_id"].concat(),
        ["campaign", "_id"].concat(),
    ];
    for token in forbidden {
        assert!(!source.contains(&token), "forbidden native token: {token}");
    }

    let public_structs = source.matches("pub struct ").count();
    let public_enums = source.matches("pub enum ").count();
    assert!(
        public_structs <= 12,
        "public struct budget exceeded: {public_structs}"
    );
    assert!(
        public_enums <= 8,
        "public enum budget exceeded: {public_enums}"
    );
}
