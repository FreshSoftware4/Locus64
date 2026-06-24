use serde::{Deserialize, Serialize};

use crate::{
    CodonHeader, CodonPhase, DigestRole, SubstrateWitness, codon_admitted_in_phase,
    phase_admission_rule, resolve_codon_symbol_or_alias, resolve_lexon_symbol_or_alias,
    role_digest_value,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SubstratePrimitiveKind {
    Locus,
    Atom,
    Bond,
    Reaction,
    Witness,
    Chassis,
    Cassette,
    Plasmid,
}

pub fn substrate_primitive_kinds() -> Vec<SubstratePrimitiveKind> {
    vec![
        SubstratePrimitiveKind::Locus,
        SubstratePrimitiveKind::Atom,
        SubstratePrimitiveKind::Bond,
        SubstratePrimitiveKind::Reaction,
        SubstratePrimitiveKind::Witness,
        SubstratePrimitiveKind::Chassis,
        SubstratePrimitiveKind::Cassette,
        SubstratePrimitiveKind::Plasmid,
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubstrateLocus {
    pub id: String,
    pub subject: String,
    pub chassis_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubstrateAtom {
    pub id: String,
    pub codon_symbol: String,
    pub lexon_symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubstrateBond {
    pub id: String,
    pub relation_codon: String,
    pub left: String,
    pub right: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubstrateReaction {
    pub id: String,
    pub process_codon: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub receipt_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubstrateChassis {
    pub id: String,
    pub policy_codons: Vec<String>,
    pub admitted_phases: Vec<CodonPhase>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubstrateCassette {
    pub id: String,
    pub locus_ids: Vec<String>,
    pub reaction_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubstratePlasmid {
    pub id: String,
    pub cassette_ids: Vec<String>,
    pub integration_reaction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubstratePrimitive {
    Locus(SubstrateLocus),
    Atom(SubstrateAtom),
    Bond(SubstrateBond),
    Reaction(SubstrateReaction),
    Witness(SubstrateWitness),
    Chassis(SubstrateChassis),
    Cassette(SubstrateCassette),
    Plasmid(SubstratePlasmid),
}

impl SubstratePrimitive {
    pub fn kind(&self) -> SubstratePrimitiveKind {
        match self {
            SubstratePrimitive::Locus(_) => SubstratePrimitiveKind::Locus,
            SubstratePrimitive::Atom(_) => SubstratePrimitiveKind::Atom,
            SubstratePrimitive::Bond(_) => SubstratePrimitiveKind::Bond,
            SubstratePrimitive::Reaction(_) => SubstratePrimitiveKind::Reaction,
            SubstratePrimitive::Witness(_) => SubstratePrimitiveKind::Witness,
            SubstratePrimitive::Chassis(_) => SubstratePrimitiveKind::Chassis,
            SubstratePrimitive::Cassette(_) => SubstratePrimitiveKind::Cassette,
            SubstratePrimitive::Plasmid(_) => SubstratePrimitiveKind::Plasmid,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MolecularCodecRecord {
    pub codon_symbol: String,
    pub phase: CodonPhase,
    pub origin_authority: String,
    pub subject: String,
    pub primitive_kind: SubstratePrimitiveKind,
    pub primitive: SubstratePrimitive,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MolecularCodecEnvelope {
    pub header: CodonHeader,
    pub record: MolecularCodecRecord,
}

pub fn molecular_codec_record_field_names() -> Vec<&'static str> {
    vec![
        "codon_symbol",
        "phase",
        "origin_authority",
        "subject",
        "primitive_kind",
        "primitive",
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MolecularCodecValidationReport {
    pub valid: bool,
    pub codec_digest: String,
    pub failures: Vec<String>,
}

pub type MolecularCodecEnvelopeValidationReport = MolecularCodecValidationReport;

pub fn validate_molecular_codec_record(
    record: &MolecularCodecRecord,
) -> MolecularCodecValidationReport {
    let mut failures = Vec::new();
    match resolve_codon_symbol_or_alias(&record.codon_symbol) {
        Some(codon) => {
            if codon.symbol != record.codon_symbol {
                failures.push("codec record must use canonical codon symbol".into());
            }
            if !codon_admitted_in_phase(&codon, record.phase) {
                failures.push(format!(
                    "codon {} not admitted in phase {:?}",
                    record.codon_symbol, record.phase
                ));
            }
        }
        None => failures.push(format!("unknown codon {}", record.codon_symbol)),
    }
    if phase_admission_rule(record.phase).is_none() {
        failures.push(format!(
            "missing phase admission rule for {:?}",
            record.phase
        ));
    }
    if record.primitive.kind() != record.primitive_kind {
        failures.push("primitive kind does not match primitive variant".into());
    }
    validate_primitive_codons(record, &mut failures);
    if record.origin_authority.trim().is_empty() {
        failures.push("missing origin authority".into());
    }
    if record.subject.trim().is_empty() {
        failures.push("missing subject".into());
    }

    MolecularCodecValidationReport {
        valid: failures.is_empty(),
        codec_digest: role_digest_value(DigestRole::CodecDigest, record),
        failures,
    }
}

fn validate_primitive_codons(record: &MolecularCodecRecord, failures: &mut Vec<String>) {
    match &record.primitive {
        SubstratePrimitive::Atom(atom) => {
            if atom.codon_symbol != record.codon_symbol {
                failures.push("atom codon does not match record codon".into());
            }
            if let Some(lexon) = &atom.lexon_symbol {
                if resolve_lexon_symbol_or_alias(lexon).is_none() {
                    failures.push(format!("unknown atom lexon {lexon}"));
                }
            }
        }
        SubstratePrimitive::Bond(bond) => {
            if resolve_codon_symbol_or_alias(&bond.relation_codon)
                .map(|codon| codon.symbol != bond.relation_codon)
                .unwrap_or(true)
            {
                failures.push(format!(
                    "bond relation codon {} is not canonical",
                    bond.relation_codon
                ));
            }
        }
        SubstratePrimitive::Reaction(reaction) => {
            if resolve_codon_symbol_or_alias(&reaction.process_codon)
                .map(|codon| codon.symbol != reaction.process_codon)
                .unwrap_or(true)
            {
                failures.push(format!(
                    "reaction process codon {} is not canonical",
                    reaction.process_codon
                ));
            }
        }
        SubstratePrimitive::Witness(witness) => {
            if !witness.derived {
                failures.push("witness primitive must be derived, not authored truth".into());
            }
            if resolve_codon_symbol_or_alias(&witness.status_codon).is_none() {
                failures.push(format!(
                    "unknown witness status codon {}",
                    witness.status_codon
                ));
            }
        }
        SubstratePrimitive::Chassis(chassis) => {
            for codon in &chassis.policy_codons {
                if resolve_codon_symbol_or_alias(codon)
                    .map(|resolved| resolved.symbol != *codon)
                    .unwrap_or(true)
                {
                    failures.push(format!("chassis policy codon {codon} is not canonical"));
                }
            }
        }
        SubstratePrimitive::Locus(_)
        | SubstratePrimitive::Cassette(_)
        | SubstratePrimitive::Plasmid(_) => {}
    }
}

pub fn validate_molecular_codec_envelope(
    envelope: &MolecularCodecEnvelope,
) -> MolecularCodecEnvelopeValidationReport {
    let mut failures = validate_molecular_codec_record(&envelope.record).failures;
    let header = &envelope.header;

    if header.phase != envelope.record.phase {
        failures.push("codec envelope header phase does not match record phase".into());
    }
    if header.subject != envelope.record.subject {
        failures.push("codec envelope header subject does not match record subject".into());
    }
    validate_header_codon("class", &header.class_symbol, header.phase, &mut failures);
    validate_header_codon("role", &header.role_symbol, header.phase, &mut failures);

    MolecularCodecEnvelopeValidationReport {
        valid: failures.is_empty(),
        codec_digest: role_digest_value(DigestRole::CodecDigest, envelope),
        failures,
    }
}

fn validate_header_codon(label: &str, symbol: &str, phase: CodonPhase, failures: &mut Vec<String>) {
    match resolve_codon_symbol_or_alias(symbol) {
        Some(codon) => {
            if codon.symbol != symbol {
                failures.push(format!(
                    "codec envelope header {label} codon must be canonical"
                ));
            }
            if !codon_admitted_in_phase(&codon, phase) {
                failures.push(format!(
                    "codec envelope header {label} codon {symbol} not admitted in phase {phase:?}"
                ));
            }
        }
        None => failures.push(format!(
            "unknown codec envelope header {label} codon {symbol}"
        )),
    }
}

pub fn encode_molecular_codec_record(record: &MolecularCodecRecord) -> Result<Vec<u8>, String> {
    let report = validate_molecular_codec_record(record);
    if !report.valid {
        return Err(report.failures.join("; "));
    }
    bincode::serialize(record).map_err(|err| err.to_string())
}

pub fn decode_molecular_codec_record(bytes: &[u8]) -> Result<MolecularCodecRecord, String> {
    let record: MolecularCodecRecord =
        bincode::deserialize(bytes).map_err(|err| err.to_string())?;
    let report = validate_molecular_codec_record(&record);
    if !report.valid {
        return Err(report.failures.join("; "));
    }
    Ok(record)
}

pub fn encode_molecular_codec_envelope(
    envelope: &MolecularCodecEnvelope,
) -> Result<Vec<u8>, String> {
    let report = validate_molecular_codec_envelope(envelope);
    if !report.valid {
        return Err(report.failures.join("; "));
    }
    bincode::serialize(envelope).map_err(|err| err.to_string())
}

pub fn decode_molecular_codec_envelope(bytes: &[u8]) -> Result<MolecularCodecEnvelope, String> {
    let envelope: MolecularCodecEnvelope =
        bincode::deserialize(bytes).map_err(|err| err.to_string())?;
    let report = validate_molecular_codec_envelope(&envelope);
    if !report.valid {
        return Err(report.failures.join("; "));
    }
    Ok(envelope)
}

pub fn witness_to_molecular_codec_record(
    origin_authority: &str,
    witness: SubstrateWitness,
) -> MolecularCodecRecord {
    MolecularCodecRecord {
        codon_symbol: "W!".into(),
        phase: CodonPhase::Product,
        origin_authority: origin_authority.into(),
        subject: witness.subject.clone(),
        primitive_kind: SubstratePrimitiveKind::Witness,
        primitive: SubstratePrimitive::Witness(witness),
    }
}
