use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::{lexon_specs, macro_codon_specs};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CodonClass {
    Molecule,
    Role,
    State,
    Relation,
    Process,
    ExpressionControl,
    WitnessForm,
    Machine,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CodonArity {
    Nullary,
    Unary,
    Binary,
    Variadic,
    Transform,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CodonPhase {
    SourceRna,
    CanonicalRna,
    DnaAuthority,
    Product,
    Projection,
    Receipt,
    ForeignDebug,
    MachineMemo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CodonSpec {
    pub symbol: String,
    pub ascii_aliases: Vec<String>,
    pub class: CodonClass,
    pub arity: CodonArity,
    pub admitted_phases: Vec<CodonPhase>,
    pub opcode: u16,
    pub canonical: bool,
    pub tombstoned: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OpcodeRangeSpec {
    pub name: String,
    pub start: u16,
    pub end: u16,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TombstoneSpec {
    pub opcode: u16,
    pub symbol: String,
    pub reason: String,
    pub replacement: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NativeStructuralRegion {
    Header,
    CodonBody,
    StructuralToken,
    RoleField,
    Gloss,
    Comment,
    HumanView,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NativeStructuralSegment {
    pub region: NativeStructuralRegion,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CodonHeader {
    pub protocol: String,
    pub version: u16,
    pub phase: CodonPhase,
    pub class_symbol: String,
    pub role_symbol: String,
    pub subject: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PhaseAdmissionRule {
    pub phase: CodonPhase,
    pub admits_source_compile: bool,
    pub admits_authority_decode: bool,
    pub admits_product_expression: bool,
    pub admits_debug_export: bool,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CodonLexonLawReport {
    pub codon_count: usize,
    pub lexon_count: usize,
    pub macro_codon_count: usize,
    pub generated_word_ban_count: usize,
    pub valid: bool,
    pub failures: Vec<String>,
}

pub fn codon_specs() -> Vec<CodonSpec> {
    use CodonArity::*;
    use CodonClass::*;
    use CodonPhase::*;
    vec![
        CodonSpec {
            symbol: "<-".into(),
            ascii_aliases: vec!["dep".into(), "depends_on".into()],
            class: Relation,
            arity: Binary,
            admitted_phases: vec![SourceRna, CanonicalRna, DnaAuthority, Product, Receipt],
            opcode: 0x1100,
            canonical: true,
            tombstoned: false,
        },
        CodonSpec {
            symbol: "|-".into(),
            ascii_aliases: vec!["derives".into(), "proves".into()],
            class: WitnessForm,
            arity: Binary,
            admitted_phases: vec![DnaAuthority, Product, Receipt],
            opcode: 0x1101,
            canonical: true,
            tombstoned: false,
        },
        CodonSpec {
            symbol: "[]".into(),
            ascii_aliases: vec!["frontier".into(), "obligation".into()],
            class: State,
            arity: Variadic,
            admitted_phases: vec![DnaAuthority, Product, Receipt],
            opcode: 0x1102,
            canonical: true,
            tombstoned: false,
        },
        CodonSpec {
            symbol: "R!".into(),
            ascii_aliases: vec!["receipt".into()],
            class: Role,
            arity: Unary,
            admitted_phases: vec![Receipt, Product],
            opcode: 0x1103,
            canonical: true,
            tombstoned: false,
        },
        CodonSpec {
            symbol: "+>".into(),
            ascii_aliases: vec!["integrates".into()],
            class: Process,
            arity: Transform,
            admitted_phases: vec![DnaAuthority, Receipt],
            opcode: 0x1105,
            canonical: true,
            tombstoned: false,
        },
        CodonSpec {
            symbol: ">>".into(),
            ascii_aliases: vec!["expresses".into()],
            class: ExpressionControl,
            arity: Transform,
            admitted_phases: vec![DnaAuthority, Product, Receipt],
            opcode: 0x1106,
            canonical: true,
            tombstoned: false,
        },
        CodonSpec {
            symbol: "G#".into(),
            ascii_aliases: vec!["genome".into()],
            class: Molecule,
            arity: Unary,
            admitted_phases: vec![DnaAuthority, Product],
            opcode: 0x1107,
            canonical: true,
            tombstoned: false,
        },
        CodonSpec {
            symbol: "P#".into(),
            ascii_aliases: vec!["plasmid".into()],
            class: Molecule,
            arity: Unary,
            admitted_phases: vec![DnaAuthority, Product],
            opcode: 0x1108,
            canonical: true,
            tombstoned: false,
        },
        CodonSpec {
            symbol: "A#".into(),
            ascii_aliases: vec!["atom".into()],
            class: Molecule,
            arity: Unary,
            admitted_phases: vec![DnaAuthority, Product],
            opcode: 0x1109,
            canonical: true,
            tombstoned: false,
        },
        CodonSpec {
            symbol: "B#".into(),
            ascii_aliases: vec!["bond".into()],
            class: Relation,
            arity: Binary,
            admitted_phases: vec![DnaAuthority, Product],
            opcode: 0x110a,
            canonical: true,
            tombstoned: false,
        },
        CodonSpec {
            symbol: "X>".into(),
            ascii_aliases: vec!["reaction".into()],
            class: Process,
            arity: Transform,
            admitted_phases: vec![DnaAuthority, Receipt],
            opcode: 0x110b,
            canonical: true,
            tombstoned: false,
        },
        CodonSpec {
            symbol: "W!".into(),
            ascii_aliases: vec!["witness".into()],
            class: WitnessForm,
            arity: Unary,
            admitted_phases: vec![Product, Receipt],
            opcode: 0x110c,
            canonical: true,
            tombstoned: false,
        },
        CodonSpec {
            symbol: "C#".into(),
            ascii_aliases: vec!["chassis".into()],
            class: Molecule,
            arity: Unary,
            admitted_phases: vec![DnaAuthority, Product],
            opcode: 0x110d,
            canonical: true,
            tombstoned: false,
        },
        CodonSpec {
            symbol: "K#".into(),
            ascii_aliases: vec!["cassette".into()],
            class: Molecule,
            arity: Unary,
            admitted_phases: vec![DnaAuthority, Product],
            opcode: 0x110e,
            canonical: true,
            tombstoned: false,
        },
        CodonSpec {
            symbol: "M#".into(),
            ascii_aliases: vec!["memo".into(), "digest".into(), "hash".into()],
            class: Machine,
            arity: Unary,
            admitted_phases: vec![MachineMemo],
            opcode: 0x11ff,
            canonical: false,
            tombstoned: false,
        },
    ]
}

pub fn opcode_range_specs() -> Vec<OpcodeRangeSpec> {
    vec![
        OpcodeRangeSpec {
            name: "core-relations".into(),
            start: 0x1100,
            end: 0x113f,
            purpose: "active structural codons".into(),
        },
        OpcodeRangeSpec {
            name: "macro-patterns".into(),
            start: 0x1200,
            end: 0x12ff,
            purpose: "macro-codon pattern/reaction codons".into(),
        },
        OpcodeRangeSpec {
            name: "tombstones".into(),
            start: 0x1f00,
            end: 0x1ffe,
            purpose: "retired opcode tombstones; never reused".into(),
        },
        OpcodeRangeSpec {
            name: "machine-memo".into(),
            start: 0x11ff,
            end: 0x11ff,
            purpose: "machine memo/cache binding only".into(),
        },
    ]
}

pub fn tombstone_specs() -> Vec<TombstoneSpec> {
    vec![TombstoneSpec {
        opcode: 0x1f00,
        symbol: "Q!".into(),
        reason: "retired Q-surface authority marker".into(),
        replacement: Some("header codon phase law".into()),
    }]
}

pub fn phase_admission_matrix() -> Vec<PhaseAdmissionRule> {
    use CodonPhase::*;
    vec![
        PhaseAdmissionRule {
            phase: SourceRna,
            admits_source_compile: true,
            admits_authority_decode: false,
            admits_product_expression: false,
            admits_debug_export: false,
            explanation: "authored source may enter compile-rna".into(),
        },
        PhaseAdmissionRule {
            phase: CanonicalRna,
            admits_source_compile: true,
            admits_authority_decode: false,
            admits_product_expression: false,
            admits_debug_export: false,
            explanation: "canonical reconstruction may re-enter compile-rna for fixed-point checks".into(),
        },
        PhaseAdmissionRule {
            phase: DnaAuthority,
            admits_source_compile: false,
            admits_authority_decode: true,
            admits_product_expression: true,
            admits_debug_export: false,
            explanation: "DNA authority can decode and feed witnessed expression, not source compilation".into(),
        },
        PhaseAdmissionRule {
            phase: Product,
            admits_source_compile: false,
            admits_authority_decode: false,
            admits_product_expression: false,
            admits_debug_export: false,
            explanation: "products are expressed outputs and cannot act as source".into(),
        },
        PhaseAdmissionRule {
            phase: Projection,
            admits_source_compile: false,
            admits_authority_decode: false,
            admits_product_expression: false,
            admits_debug_export: false,
            explanation: "projections are views and cannot act as source or authority".into(),
        },
        PhaseAdmissionRule {
            phase: Receipt,
            admits_source_compile: false,
            admits_authority_decode: false,
            admits_product_expression: false,
            admits_debug_export: false,
            explanation: "receipts validate lineage but cannot act as source".into(),
        },
        PhaseAdmissionRule {
            phase: ForeignDebug,
            admits_source_compile: false,
            admits_authority_decode: false,
            admits_product_expression: false,
            admits_debug_export: true,
            explanation: "foreign/debug output is inspection-only unless explicitly imported by another adapter".into(),
        },
        PhaseAdmissionRule {
            phase: MachineMemo,
            admits_source_compile: false,
            admits_authority_decode: false,
            admits_product_expression: false,
            admits_debug_export: false,
            explanation: "machine memo/cache bindings are never public proof/source identity".into(),
        },
    ]
}

pub fn phase_admission_rule(phase: CodonPhase) -> Option<PhaseAdmissionRule> {
    phase_admission_matrix()
        .into_iter()
        .find(|rule| rule.phase == phase)
}

pub fn codon_header_admits_source_compile(header: &CodonHeader) -> bool {
    phase_admission_rule(header.phase)
        .map(|rule| rule.admits_source_compile)
        .unwrap_or(false)
}

pub fn generated_structural_word_ban_list() -> Vec<&'static str> {
    vec![
        "claim_page",
        "dependency_spine",
        "closure_map",
        "roundtrip_report",
        "view_receipt",
        "artifact_class",
        "payload_json",
        "metadata",
        "theorem",
        "campaign",
        "adequacy",
    ]
}

pub fn generated_structural_word_violations(text: &str) -> Vec<String> {
    generated_structural_word_violations_in_segments(&[NativeStructuralSegment {
        region: NativeStructuralRegion::CodonBody,
        text: text.into(),
    }])
}

pub fn generated_structural_word_violations_in_segments(
    segments: &[NativeStructuralSegment],
) -> Vec<String> {
    let mut violations = Vec::new();
    for segment in segments {
        if matches!(
            segment.region,
            NativeStructuralRegion::Gloss
                | NativeStructuralRegion::Comment
                | NativeStructuralRegion::HumanView
        ) {
            continue;
        }
        let lower = segment.text.to_ascii_lowercase();
        for word in generated_structural_word_ban_list() {
            if lower.contains(word) {
                violations.push(format!("{:?}:{word}", segment.region));
            }
        }
    }
    violations
}

pub fn resolve_codon_symbol_or_alias(value: &str) -> Option<CodonSpec> {
    let normalized = value.trim().to_ascii_lowercase();
    codon_specs().into_iter().find(|codon| {
        codon.symbol == value
            || codon
                .ascii_aliases
                .iter()
                .any(|alias| alias.eq_ignore_ascii_case(&normalized))
    })
}

pub fn codon_admitted_in_phase(codon: &CodonSpec, phase: CodonPhase) -> bool {
    codon.admitted_phases.contains(&phase)
}

pub fn parse_codon_header(text: &str) -> Result<CodonHeader, String> {
    let mut lines = text.lines().map(str::trim).filter(|line| !line.is_empty());
    let protocol = lines
        .next()
        .ok_or_else(|| "missing codon protocol header".to_string())?;
    if protocol != "!λ64/2" && protocol != "!l64/2" {
        return Err("codon header must start with !λ64/2 or !l64/2".into());
    }
    let frame = lines
        .next()
        .ok_or_else(|| "missing codon frame header".to_string())?;
    let frame = frame
        .strip_prefix('⟦')
        .and_then(|value| value.strip_suffix('⟧'))
        .ok_or_else(|| "codon frame header must be enclosed in ⟦...⟧".to_string())?;
    let (phase_alias, rest) = frame
        .split_once(' ')
        .ok_or_else(|| "codon frame header missing phase".to_string())?;
    let phase = match phase_alias {
        "τ" | "src" | "source" => CodonPhase::SourceRna,
        "τ̂" | "canonical-rna" => CodonPhase::CanonicalRna,
        "γ" | "dna" | "authority" => CodonPhase::DnaAuthority,
        "ξ" | "product" => CodonPhase::Product,
        "π" | "projection" => CodonPhase::Projection,
        "ρ" | "receipt" => CodonPhase::Receipt,
        "μ" | "memo" => CodonPhase::MachineMemo,
        _ => return Err(format!("unknown codon header phase {phase_alias}")),
    };
    let (class_role, subject) = rest
        .split_once('·')
        .ok_or_else(|| "codon frame header missing subject separator".to_string())?;
    let mut class_role = class_role.split_whitespace();
    let class_symbol = class_role
        .next()
        .ok_or_else(|| "missing class symbol".to_string())?
        .to_string();
    let role_symbol = class_role.next().unwrap_or("").to_string();
    let subject = subject.trim().to_string();
    if subject.is_empty() {
        return Err("codon header subject is empty".into());
    }
    Ok(CodonHeader {
        protocol: protocol.into(),
        version: 2,
        phase,
        class_symbol,
        role_symbol,
        subject,
    })
}

pub fn validate_codon_lexon_law() -> CodonLexonLawReport {
    let codons = codon_specs();
    let lexons = lexon_specs();
    let macro_codons = macro_codon_specs();
    let tombstones = tombstone_specs();
    let mut failures = Vec::new();
    let mut codon_symbols = HashSet::new();
    let mut active_opcodes = HashMap::new();
    let mut reserved_opcodes = HashSet::new();
    let mut aliases = HashSet::new();

    for tombstone in &tombstones {
        if !reserved_opcodes.insert(tombstone.opcode) {
            failures.push(format!(
                "duplicate tombstone opcode {:#06x}",
                tombstone.opcode
            ));
        }
    }

    for codon in &codons {
        if !codon_symbols.insert(codon.symbol.clone()) {
            failures.push(format!("duplicate codon symbol {}", codon.symbol));
        }
        if reserved_opcodes.contains(&codon.opcode) {
            failures.push(format!(
                "active codon {} reuses tombstoned opcode {:#06x}",
                codon.symbol, codon.opcode
            ));
        }
        if !codon.tombstoned {
            if let Some(prior) = active_opcodes.insert(codon.opcode, codon.symbol.clone()) {
                failures.push(format!(
                    "duplicate active opcode {:#06x} for {} and {}",
                    codon.opcode, prior, codon.symbol
                ));
            }
        }
        if codon.class == CodonClass::Machine
            && codon.admitted_phases != vec![CodonPhase::MachineMemo]
        {
            failures.push(format!(
                "machine codon {} admitted outside machine memo phase",
                codon.symbol
            ));
        }
        for alias in &codon.ascii_aliases {
            if !aliases.insert(alias.to_ascii_lowercase()) {
                failures.push(format!("duplicate alias {}", alias));
            }
        }
    }

    let lexon_symbols = lexons
        .iter()
        .map(|lexon| lexon.symbol.clone())
        .collect::<HashSet<_>>();
    for lexon in &lexons {
        if lexon.symbol.is_empty()
            || lexon.canonical_target.is_empty()
            || lexon.binding_receipt.is_empty()
        {
            failures.push(format!("incomplete lexon binding {}", lexon.symbol));
        }
        for alias in &lexon.aliases {
            if !aliases.insert(alias.to_ascii_lowercase()) {
                failures.push(format!("duplicate alias {}", alias));
            }
        }
    }

    for macro_codon in &macro_codons {
        if lexon_symbols.contains(&macro_codon.symbol) {
            failures.push(format!(
                "macro-codon {} collides with lexon registry",
                macro_codon.symbol
            ));
        }
        if reserved_opcodes.contains(&macro_codon.opcode) {
            failures.push(format!(
                "macro-codon {} reuses tombstoned opcode {:#06x}",
                macro_codon.symbol, macro_codon.opcode
            ));
        }
        if let Some(prior) = active_opcodes.insert(macro_codon.opcode, macro_codon.symbol.clone()) {
            failures.push(format!(
                "duplicate active opcode {:#06x} for {} and {}",
                macro_codon.opcode, prior, macro_codon.symbol
            ));
        }
        for alias in &macro_codon.ascii_aliases {
            if !aliases.insert(alias.to_ascii_lowercase()) {
                failures.push(format!("duplicate alias {}", alias));
            }
        }
    }

    CodonLexonLawReport {
        codon_count: codons.len(),
        lexon_count: lexons.len(),
        macro_codon_count: macro_codons.len(),
        generated_word_ban_count: generated_structural_word_ban_list().len(),
        valid: failures.is_empty(),
        failures,
    }
}
