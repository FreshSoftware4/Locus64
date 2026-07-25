use l64_symbolic::{Composer, SealAxis, SymbolicSeal};
use std::collections::BTreeSet;

fn seal(value: u64) -> SymbolicSeal {
    let mut composer = Composer::new("l64.audit.u64");
    composer.u64("value", value);
    composer.finish()
}

fn bit_distance(left: u64, right: u64) -> u32 {
    (left ^ right).count_ones()
}

fn main() {
    const SAMPLES: u64 = 1_000_000;
    let mut full = BTreeSet::new();
    let mut axes = [
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::new(),
    ];
    let mut diffusion = [0_u64; 4];

    for value in 0..SAMPLES {
        let current = seal(value);
        assert!(full.insert(current), "full seal collision at {value}");
        for (index, coordinate) in current.axes().into_iter().enumerate() {
            axes[index].insert(coordinate);
        }
        let flipped = seal(value ^ (1_u64 << (value % 64)));
        for (index, total) in diffusion.iter_mut().enumerate() {
            *total += bit_distance(current.axes()[index], flipped.axes()[index]) as u64;
        }
    }

    println!("samples={SAMPLES}");
    println!("full_unique={}", full.len());
    for (index, axis) in SealAxis::ALL.into_iter().enumerate() {
        println!(
            "{} role={} unique={} mean_bit_change={:.3}",
            axis.symbol(),
            axis.role(),
            axes[index].len(),
            diffusion[index] as f64 / SAMPLES as f64
        );
    }
}
