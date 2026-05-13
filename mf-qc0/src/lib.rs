use mf_core::{HeaderEnvelope, QaDocument, QaEntry, SurfaceKind};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Qc0Error {
    #[error("missing or invalid qc0 header")]
    MissingHeader,
    #[error("invalid header payload: {0}")]
    InvalidHeader(String),
    #[error("invalid entry payload: {0}")]
    InvalidEntry(String),
}

pub fn parse_qc0(input: &str) -> Result<(HeaderEnvelope, QaDocument), Qc0Error> {
    let mut lines = input.lines().filter(|line| !line.trim().is_empty());
    let header_line = lines.next().ok_or(Qc0Error::MissingHeader)?;
    let header_payload = header_line
        .strip_prefix("!qc0 ")
        .ok_or(Qc0Error::MissingHeader)?;
    let header: HeaderEnvelope = serde_json::from_str(header_payload)
        .map_err(|err| Qc0Error::InvalidHeader(err.to_string()))?;
    if header.surface_kind != SurfaceKind::Qc0 {
        return Err(Qc0Error::MissingHeader);
    }
    let mut entries = Vec::new();
    for line in lines {
        let (kind, payload) = line
            .split_once(' ')
            .ok_or_else(|| Qc0Error::InvalidEntry(line.to_string()))?;
        let entry = QaEntry::from_surface_json(kind, payload)
            .map_err(|err| Qc0Error::InvalidEntry(err.to_string()))?;
        entries.push(entry);
    }
    Ok((header, QaDocument { entries }))
}

pub fn render_qc0(header: &HeaderEnvelope, document: &QaDocument) -> String {
    let mut lines = vec![format!("!qc0 {}", serde_json::to_string(header).unwrap())];
    for entry in &document.entries {
        let (kind, payload) = entry.to_surface_json().unwrap();
        lines.push(format!("{kind} {payload}"));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn qc0_header_roundtrips() {
        let header = HeaderEnvelope {
            surface_kind: SurfaceKind::Qc0,
            version: "1".into(),
            policy_id: "POL_QC0_CORE".into(),
            capability_id: Some("CAP_QC0_CORE".into()),
        };
        let document = QaDocument { entries: vec![] };
        let rendered = render_qc0(&header, &document);
        let (parsed, _) = parse_qc0(&rendered).unwrap();
        assert_eq!(parsed.surface_kind, SurfaceKind::Qc0);
    }

    #[test]
    fn qc0_roundtrips_imported_claim_entries() {
        let header = HeaderEnvelope {
            surface_kind: SurfaceKind::Qc0,
            version: "1".into(),
            policy_id: "POL_QC0_CORE".into(),
            capability_id: Some("CAP_QC0_CORE".into()),
        };
        let document = QaDocument {
            entries: vec![
                QaEntry::ClaimPacket(mf_core::ClaimPacket {
                    id: "CLM_T".into(),
                    claim_class: mf_core::ClaimClass::Kernel,
                    authority_state: mf_core::AuthorityState::Evidence,
                    target_sector: "kernel-claim".into(),
                    statement: "test".into(),
                    assumptions: vec!["A1".into()],
                    open_caveats: vec![],
                }),
                QaEntry::EvidenceContract(mf_core::EvidenceContract {
                    id: "ECT_T".into(),
                    required_evidence_kinds: vec!["kernel-claim".into()],
                    required_benchmark_roles: vec![mf_core::BenchmarkRole::TargetCase],
                    requires_stress: false,
                    requires_challenge: false,
                    admissibility_thresholds: vec!["stable".into()],
                    promotion_ceiling: mf_core::CertificationVerdict::Certified,
                }),
                QaEntry::BenchmarkReceipt(mf_core::BenchmarkReceipt {
                    id: "BMR_T".into(),
                    claim_packet_id: "CLM_T".into(),
                    role: mf_core::BenchmarkRole::TargetCase,
                    verdict: mf_core::CertificationVerdict::Certified,
                    metrics: std::collections::BTreeMap::new(),
                    reproducibility_ref: "RPK_T".into(),
                }),
                QaEntry::ChallengeReceipt(mf_core::ChallengeReceipt {
                    id: "CHR_T".into(),
                    claim_packet_id: "CLM_T".into(),
                    grounds: vec!["g".into()],
                    required_response: "r".into(),
                    status: mf_core::ChallengeStatus::Addressed,
                }),
                QaEntry::ReproducibilityPacket(mf_core::ReproducibilityPacket {
                    id: "RPK_T".into(),
                    claim_packet_id: "CLM_T".into(),
                    derivation_path: vec!["lab".into()],
                    code_refs: vec!["src".into()],
                    benchmark_refs: vec!["BMR_T".into()],
                    artifact_refs: vec!["CLM_T".into()],
                }),
            ],
        };
        let rendered = render_qc0(&header, &document);
        let (_, parsed) = parse_qc0(&rendered).unwrap();
        assert_eq!(parsed.entries.len(), 5);
    }
}
