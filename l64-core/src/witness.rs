use serde::{Deserialize, Serialize};

use crate::stable_hash_u64;

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
    SubstrateWitness {
        id: format!(
            "WIT_{}",
            stable_hash_u64(&format!("{subject}:{receipt_id}"))
        ),
        subject: subject.into(),
        status_codon: "|-".into(),
        dependencies,
        proof_route,
        open_frontier,
        receipt_id: receipt_id.into(),
        derived: true,
    }
}
