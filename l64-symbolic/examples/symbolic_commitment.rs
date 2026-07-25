use l64_symbolic::{Composer, exact_identity};

fn main() {
    let source = exact_identity("l64.example.source", b"typed(a -> b)");
    let result = exact_identity("l64.example.result", b"typed(a -> c)");
    let derivation = Composer::derive(
        "l64.example.derivation",
        "compose",
        source.seal(),
        result.seal(),
    );

    println!("source: {}", source.pretty());
    println!("result: {}", result.pretty());
    println!(
        "derived: {}",
        derivation.pretty("l64.example.derivation ∷ compose")
    );
    for change in source.seal().explain_changes(result.seal()) {
        println!(
            "{} ({}) {:016x} -> {:016x}",
            change.axis.symbol(),
            change.axis.role(),
            change.before,
            change.after
        );
    }
}
