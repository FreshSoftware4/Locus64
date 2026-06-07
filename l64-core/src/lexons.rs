use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LexonClass {
    Claim,
    Definition,
    Composition,
    Sector,
    Bridge,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LexonScope {
    GlobalPattern,
    Genome,
    Plasmid,
    Cassette,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LexonSpec {
    pub symbol: String,
    pub scope: LexonScope,
    pub class: LexonClass,
    pub canonical_target: String,
    pub aliases: Vec<String>,
    pub gloss: String,
    pub binding_receipt: String,
}

pub fn lexon_specs() -> Vec<LexonSpec> {
    vec![
        LexonSpec {
            symbol: "φ_chain".into(),
            scope: LexonScope::GlobalPattern,
            class: LexonClass::Claim,
            canonical_target: "chain-rule-claim-pattern".into(),
            aliases: vec!["chain rule".into(), "chain_rule".into()],
            gloss: "chain rule claim pattern".into(),
            binding_receipt: "lexon-binding.chain-rule.v1".into(),
        },
        LexonSpec {
            symbol: "δ_deriv".into(),
            scope: LexonScope::GlobalPattern,
            class: LexonClass::Definition,
            canonical_target: "derivative-definition-pattern".into(),
            aliases: vec!["derivative".into(), "derivative_definition".into()],
            gloss: "derivative definition pattern".into(),
            binding_receipt: "lexon-binding.derivative.v1".into(),
        },
        LexonSpec {
            symbol: "δ_comp".into(),
            scope: LexonScope::GlobalPattern,
            class: LexonClass::Composition,
            canonical_target: "composition-definition-pattern".into(),
            aliases: vec!["composition".into(), "function_composition".into()],
            gloss: "composition definition pattern".into(),
            binding_receipt: "lexon-binding.composition.v1".into(),
        },
        LexonSpec {
            symbol: "β_tri".into(),
            scope: LexonScope::GlobalPattern,
            class: LexonClass::Sector,
            canonical_target: "triadic-bridge-sector-pattern".into(),
            aliases: vec![
                "triadic bridge sector".into(),
                "triadic_bridge_sector".into(),
            ],
            gloss: "triadic bridge-sector structural pattern".into(),
            binding_receipt: "lexon-binding.triadic-bridge-sector.v1".into(),
        },
        LexonSpec {
            symbol: "β_spec".into(),
            scope: LexonScope::GlobalPattern,
            class: LexonClass::Bridge,
            canonical_target: "spectral-generation-bridge-pattern".into(),
            aliases: vec![
                "spectral generation bridge".into(),
                "spectral_generation_bridge".into(),
            ],
            gloss: "spectral generation bridge pattern".into(),
            binding_receipt: "lexon-binding.spectral-generation-bridge.v1".into(),
        },
    ]
}

pub fn resolve_lexon_symbol_or_alias(value: &str) -> Option<LexonSpec> {
    let normalized = value.trim().to_ascii_lowercase();
    lexon_specs().into_iter().find(|lexon| {
        lexon.symbol == value
            || lexon
                .aliases
                .iter()
                .any(|alias| alias.eq_ignore_ascii_case(&normalized))
    })
}
