use serde::{Deserialize, Serialize};

use crate::{DigestRole, role_digest_str};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubstrateWitness {
    pub id: String,
    pub subject: String,
    pub status_codon: String,
    pub dependencies: Vec<String>,
    pub proof_route: Vec<String>,
    pub open_frontier: Vec<String>,
    pub receipt_id: String,
    pub derived: bool,
}

pub fn derive_substrate_witness(
    subject: &str,
    dependencies: Vec<String>,
    proof_route: Vec<String>,
    open_frontier: Vec<String>,
    receipt_id: &str,
) -> SubstrateWitness {
    let digest = role_digest_str(DigestRole::WitnessId, &format!("{subject}:{receipt_id}"));
    SubstrateWitness {
        id: format!("WIT_{}", digest.value),
        subject: subject.into(),
        status_codon: "|-".into(),
        dependencies,
        proof_route,
        open_frontier,
        receipt_id: receipt_id.into(),
        derived: true,
    }
}
