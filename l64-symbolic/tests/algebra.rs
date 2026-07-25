use l64_symbolic::{
    Composer, SealAxis, SymbolicIdentity, SymbolicSeal, exact_identity, seal_bytes,
};
use std::collections::BTreeSet;

fn atom(domain: &str, label: &str, bytes: &[u8]) -> SymbolicSeal {
    let mut composer = Composer::new(domain);
    composer.field(label, bytes);
    composer.finish()
}

#[test]
fn exact_identity_is_collision_free_by_representation() {
    let identity = SymbolicIdentity::new("l64.test", b"abc".as_slice());
    assert!(identity.verify_exact("l64.test", b"abc"));
    assert!(!identity.verify_exact("l64.test", b"abd"));
    assert!(!identity.verify_exact("l64.other", b"abc"));
    assert_eq!(
        SymbolicIdentity::from_exact_bytes(&identity.to_exact_bytes()).unwrap(),
        identity
    );
}

#[test]
fn domains_are_separated() {
    assert_ne!(
        atom("left", "payload", b"same"),
        atom("right", "payload", b"same")
    );
}

#[test]
fn ordered_composition_preserves_order() {
    let a = atom("leaf", "a", b"a");
    let b = atom("leaf", "b", b"b");
    let mut left = Composer::new("ordered");
    left.ordered("parts", &[a, b]);
    let mut right = Composer::new("ordered");
    right.ordered("parts", &[b, a]);
    assert_ne!(left.finish(), right.finish());
}

#[test]
fn commutative_composition_normalizes_order() {
    let a = atom("leaf", "a", b"a");
    let b = atom("leaf", "b", b"b");
    let mut left = Composer::new("set");
    left.commutative("parts", &[a, b]);
    let mut right = Composer::new("set");
    right.commutative("parts", &[b, a]);
    assert_eq!(left.finish(), right.finish());
}

#[test]
fn canonical_text_round_trips() {
    let seal = atom("roundtrip", "payload", b"hello");
    assert_eq!(seal.to_string().parse::<SymbolicSeal>().unwrap(), seal);
    assert_eq!(SymbolicSeal::from_bytes(seal.to_bytes()), seal);
}

#[test]
fn axis_changes_are_explainable() {
    let left = atom("diff", "payload", b"abc");
    let right = atom("diff", "payload", b"abd");
    let axes = left.changed_axes(right);
    assert!(!axes.is_empty());
    assert!(axes.contains(&SealAxis::Sigma));
    let changes = left.explain_changes(right);
    assert_eq!(changes.len(), axes.len());
    assert!(changes.iter().all(|change| change.before != change.after));
    assert!(left.pretty("diff").contains('Σ'));
    assert!(left.pretty("diff").contains('Ω'));
}

#[test]
fn small_input_space_has_no_observed_full_seal_collision() {
    let mut seen = BTreeSet::new();
    for value in 0_u16..=u16::MAX {
        let seal = atom("exhaustive-u16", "value", &value.to_le_bytes());
        assert!(seen.insert(seal), "collision at {value}");
    }
}

#[test]
fn convenience_helpers_preserve_exact_and_compact_layers() {
    let identity = exact_identity("l64.easy", b"payload");
    let seal = seal_bytes("l64.easy", b"payload");
    assert_eq!(identity.seal(), seal);
    assert!(identity.verify_exact("l64.easy", b"payload"));
    assert!(seal.fast_matches(identity.seal()));
}

#[test]
fn axis_law_is_typed_and_self_describing() {
    let roles = SealAxis::ALL.map(|axis| (axis.symbol(), axis.role()));
    assert_eq!(roles[0], ("Σ", "aggregate content mass"));
    assert_eq!(roles[1], ("Π", "ordered composition"));
    assert_eq!(roles[2], ("Δ", "adjacent transition structure"));
    assert_eq!(roles[3], ("Ω", "nonlinear boundary closure"));
}

#[test]
fn exact_envelope_is_self_delimiting() {
    let identity = exact_identity("l64.envelope", b"payload");
    let mut encoded = identity.to_exact_bytes();
    assert_eq!(
        SymbolicIdentity::from_exact_bytes(&encoded).unwrap(),
        identity
    );

    encoded.push(0);
    assert!(SymbolicIdentity::from_exact_bytes(&encoded).is_err());
    encoded.pop();
    encoded[0] ^= 1;
    assert!(SymbolicIdentity::from_exact_bytes(&encoded).is_err());
}
