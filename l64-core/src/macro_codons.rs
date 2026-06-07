use serde::{Deserialize, Serialize};

use crate::{CodonArity, CodonPhase};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MacroCodonClass {
    RoundTrip,
    Closure,
    Receipt,
    Integration,
    Expression,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MacroCodonSpec {
    pub symbol: String,
    pub ascii_aliases: Vec<String>,
    pub class: MacroCodonClass,
    pub reaction_shape: String,
    pub arity: CodonArity,
    pub admitted_phases: Vec<CodonPhase>,
    pub witness_requirements: Vec<String>,
    pub opcode: u16,
}

pub fn macro_codon_specs() -> Vec<MacroCodonSpec> {
    use CodonArity::*;
    use MacroCodonClass::*;
    vec![
        MacroCodonSpec {
            symbol: "↻✓".into(),
            ascii_aliases: vec!["roundtrip".into(), "roundtrip_fixed_point".into()],
            class: RoundTrip,
            reaction_shape: "source->dna->canonical-rna->dna fixed point".into(),
            arity: Transform,
            admitted_phases: vec![CodonPhase::DnaAuthority, CodonPhase::Receipt],
            witness_requirements: vec![
                "canonical-id-equality".into(),
                "dna-digest-stability".into(),
            ],
            opcode: 0x1200,
        },
        MacroCodonSpec {
            symbol: "□∅".into(),
            ascii_aliases: vec!["closure_frontier".into()],
            class: Closure,
            reaction_shape: "closed frontier with explicit empty residual obligation set".into(),
            arity: Variadic,
            admitted_phases: vec![CodonPhase::Product, CodonPhase::Receipt],
            witness_requirements: vec!["frontier-enumeration".into()],
            opcode: 0x1201,
        },
        MacroCodonSpec {
            symbol: "πρ".into(),
            ascii_aliases: vec!["view_receipt".into()],
            class: Receipt,
            reaction_shape: "projection/view receipt from witnessed authority".into(),
            arity: Binary,
            admitted_phases: vec![CodonPhase::Product, CodonPhase::Receipt],
            witness_requirements: vec!["origin-dna".into(), "projection-boundary".into()],
            opcode: 0x1202,
        },
        MacroCodonSpec {
            symbol: "⊕ρ".into(),
            ascii_aliases: vec!["integration_receipt".into()],
            class: Integration,
            reaction_shape: "plasmid/cassette integration receipt".into(),
            arity: Transform,
            admitted_phases: vec![CodonPhase::DnaAuthority, CodonPhase::Receipt],
            witness_requirements: vec!["pre-state".into(), "post-state".into(), "lineage".into()],
            opcode: 0x1203,
        },
        MacroCodonSpec {
            symbol: "⇓ρ".into(),
            ascii_aliases: vec!["expression_receipt".into()],
            class: Expression,
            reaction_shape: "witnessed authority expresses product set".into(),
            arity: Transform,
            admitted_phases: vec![CodonPhase::Product, CodonPhase::Receipt],
            witness_requirements: vec!["origin-dna".into(), "witness-form".into()],
            opcode: 0x1204,
        },
    ]
}

pub fn resolve_macro_codon_symbol_or_alias(value: &str) -> Option<MacroCodonSpec> {
    let normalized = value.trim().to_ascii_lowercase();
    macro_codon_specs().into_iter().find(|macro_codon| {
        macro_codon.symbol == value
            || macro_codon
                .ascii_aliases
                .iter()
                .any(|alias| alias.eq_ignore_ascii_case(&normalized))
    })
}
