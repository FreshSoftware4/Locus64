use mf_core::{
    AliasFace, ConstraintFace, EvidenceFace, EvidenceMaturity, GateVerdict, IdentityFace,
    ObjectTag, QaDocument, QaEntry, QcObject, StructuralFace,
};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum QaError {
    #[error("line {line}: missing prefix")]
    MissingPrefix { line: usize },
    #[error("line {line}: malformed object shell")]
    MalformedObjectShell { line: usize },
    #[error("line {line}: missing delimiter `{delimiter}`")]
    MissingDelimiter {
        line: usize,
        delimiter: &'static str,
    },
    #[error("line {line}: invalid json payload: {message}")]
    InvalidJson { line: usize, message: String },
    #[error("line {line}: invalid object tag `{tag}`")]
    InvalidObjectTag { line: usize, tag: String },
    #[error("line {line}: invalid maturity `{value}`")]
    InvalidMaturity { line: usize, value: String },
    #[error("line {line}: invalid gate verdict `{value}`")]
    InvalidGateVerdict { line: usize, value: String },
}

pub fn parse_document(input: &str) -> Result<QaDocument, QaError> {
    let mut entries = Vec::new();
    for (index, raw_line) in input.lines().enumerate() {
        let line_no = index + 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(rest) = line.strip_prefix("object ") {
            entries.push(QaEntry::Object(parse_object_shell(rest.trim(), line_no)?));
            continue;
        }
        entries.push(parse_json_entry(line, line_no)?);
    }
    Ok(QaDocument { entries })
}

pub fn normalize_document(document: &QaDocument) -> Result<String, QaError> {
    let mut lines = Vec::new();
    for entry in &document.entries {
        match entry {
            QaEntry::Object(object) => {
                lines.push(format!("object {}", format_object_shell(object)))
            }
            _ => {
                let (kind, payload) = entry.to_surface_json().unwrap();
                lines.push(format!("{kind} {payload}"));
            }
        }
    }
    Ok(lines.join("\n"))
}

pub fn parse_object_shell(input: &str, line: usize) -> Result<QcObject, QaError> {
    let (identity_raw, rest) = extract_segment(input, '[', ']', line)?;
    let (structural_raw, rest) = extract_segment(rest, '<', '>', line)?;
    let (constraint_raw, rest) = extract_segment(rest, '[', ']', line)?;
    let (evidence_raw, rest) = extract_segment(rest, '{', '}', line)?;
    let alias_raw = extract_double_angle(rest, line)?;

    let identity = parse_identity(&split_fields(identity_raw), line)?;
    let structural = parse_structural(&split_fields(structural_raw));
    let constraint = parse_constraint(&split_fields(constraint_raw));
    let evidence = parse_evidence(&split_fields(evidence_raw), line)?;
    let alias = parse_alias(&split_fields(alias_raw));

    Ok(QcObject {
        id: identity.cid.clone(),
        identity,
        structural,
        constraint,
        evidence,
        alias,
    })
}

pub fn format_object_shell(object: &QcObject) -> String {
    let identity = [
        ("tag", format!("{:?}", object.identity.tag).to_uppercase()),
        ("cid", object.identity.cid.clone()),
        ("codebook", object.identity.codebook.clone()),
        ("remap", object.identity.remap.clone()),
        ("lineage", object.identity.lineage.clone()),
    ];
    let structural = [
        ("head", object.structural.head.clone()),
        ("args", join_list(&object.structural.args)),
        ("locals", join_list(&object.structural.local_sections)),
        ("hooks", join_list(&object.structural.morphism_hooks)),
    ];
    let constraint = [
        ("regime", object.constraint.regime.clone()),
        ("contracts", join_list(&object.constraint.contracts)),
        ("invariants", join_list(&object.constraint.invariants)),
        ("equivalence", object.constraint.equivalence.clone()),
        ("admissibility", object.constraint.admissibility.clone()),
    ];
    let evidence = [
        ("evidence_class", object.evidence.evidence_class.clone()),
        ("traces", join_list(&object.evidence.traces)),
        ("receipts", join_list(&object.evidence.receipts)),
        ("maturity", format!("{:?}", object.evidence.maturity)),
        (
            "gate_verdict",
            format!("{:?}", object.evidence.gate_verdict),
        ),
    ];
    let alias = [
        ("aliases", join_list(&object.alias.aliases)),
        ("profiles", join_list(&object.alias.profile_pack)),
        ("qm_binding", object.alias.qm_binding.clone()),
        ("qa_binding", object.alias.qa_binding.clone()),
        ("projection_policy", object.alias.projection_policy.clone()),
    ];
    format!(
        "[{}]<{}>[{}]{{{}}}<<{}>>",
        format_fields(&identity),
        format_fields(&structural),
        format_fields(&constraint),
        format_fields(&evidence),
        format_fields(&alias)
    )
}

fn parse_json_entry(line: &str, line_no: usize) -> Result<QaEntry, QaError> {
    for prefix in [
        "regime ",
        "bridge ",
        "proof ",
        "atlas ",
        "mechanization ",
        "theorem ",
        "obligation ",
        "target ",
        "ledger ",
        "certificate ",
        "campaign ",
        "portfolio ",
        "route-class ",
        "diagnostic ",
        "adequacy ",
        "burden-pack ",
        "claim-packet ",
        "evidence-contract ",
        "benchmark-receipt ",
        "challenge-receipt ",
        "reproducibility-packet ",
        "surface-policy ",
        "transform-receipt ",
        "roundtrip-report ",
        "capability ",
        "surface-budget ",
        "policy-object ",
        "policy-binding ",
        "policy-resolution ",
        "bundle-lock ",
        "execution-manifest ",
        "replay-lock ",
        "lock-receipt ",
        "lock-diff ",
        "recompute-plan ",
        "plan-execution ",
        "prediction-assessment ",
        "reconciliation ",
        "root-resolution ",
    ] {
        if let Some(payload) = line.strip_prefix(prefix) {
            return match prefix.trim() {
                "object" => {
                    return Err(QaError::MissingPrefix { line: line_no });
                }
                other => QaEntry::from_surface_json(other, payload),
            }
            .map_err(|err| QaError::InvalidJson {
                line: line_no,
                message: err.to_string(),
            });
        }
    }
    Err(QaError::MissingPrefix { line: line_no })
}

fn extract_segment(
    input: &str,
    open: char,
    close: char,
    line: usize,
) -> Result<(&str, &str), QaError> {
    let input = input.trim_start();
    if !input.starts_with(open) {
        return Err(QaError::MalformedObjectShell { line });
    }
    let close_pos = input.find(close).ok_or(QaError::MissingDelimiter {
        line,
        delimiter: "segment end",
    })?;
    Ok((&input[1..close_pos], &input[close_pos + 1..]))
}

fn extract_double_angle(input: &str, line: usize) -> Result<&str, QaError> {
    let input = input.trim();
    if !input.starts_with("<<") {
        return Err(QaError::MissingDelimiter {
            line,
            delimiter: "<<",
        });
    }
    input
        .strip_prefix("<<")
        .and_then(|rest| rest.strip_suffix(">>"))
        .ok_or(QaError::MissingDelimiter {
            line,
            delimiter: ">>",
        })
}

fn split_fields(input: &str) -> BTreeMap<String, String> {
    input
        .split(';')
        .filter_map(|entry| {
            let trimmed = entry.trim();
            if trimmed.is_empty() {
                return None;
            }
            let (key, value) = trimmed.split_once('=')?;
            Some((key.trim().to_string(), value.trim().to_string()))
        })
        .collect()
}

fn parse_identity(fields: &BTreeMap<String, String>, line: usize) -> Result<IdentityFace, QaError> {
    let tag = match fields.get("tag").map(String::as_str).unwrap_or_default() {
        "CTX" => ObjectTag::Ctx,
        "CUT" => ObjectTag::Cut,
        "THR" => ObjectTag::Thr,
        "BRC" => ObjectTag::Brc,
        "SLK" => ObjectTag::Slk,
        "TOL" => ObjectTag::Tol,
        "KNT" => ObjectTag::Knt,
        "WIT" => ObjectTag::Wit,
        "CAN" => ObjectTag::Can,
        "PRM" => ObjectTag::Prm,
        "CAR" => ObjectTag::Car,
        "REL" => ObjectTag::Rel,
        "MAP" => ObjectTag::Map,
        "OPR" => ObjectTag::Opr,
        "INV" => ObjectTag::Inv,
        "QOT" => ObjectTag::Qot,
        "LOC" => ObjectTag::Loc,
        "GLB" => ObjectTag::Glb,
        "DER" => ObjectTag::Der,
        "INT" => ObjectTag::Int,
        "MSR" => ObjectTag::Msr,
        "PRF" => ObjectTag::Prf,
        "JDG" => ObjectTag::Jdg,
        "TYP" => ObjectTag::Typ,
        "MOR" => ObjectTag::Mor,
        "FTR" => ObjectTag::Ftr,
        "CMP" => ObjectTag::Cmp,
        "RED" => ObjectTag::Red,
        "CST" => ObjectTag::Cst,
        "BRG" => ObjectTag::Brg,
        "BRP" => ObjectTag::Brp,
        "BUD" => ObjectTag::Bud,
        "BLAW" => ObjectTag::Blaw,
        "BHEU" => ObjectTag::Bheu,
        "BREC" => ObjectTag::Brec,
        "PRS" => ObjectTag::Prs,
        "CNZ" => ObjectTag::Cnz,
        "RGY" => ObjectTag::Rgy,
        "SEL" => ObjectTag::Sel,
        "CHK" => ObjectTag::Chk,
        "MEC" => ObjectTag::Mec,
        "ADE" => ObjectTag::Ade,
        "TCM" => ObjectTag::Tcm,
        tag => {
            return Err(QaError::InvalidObjectTag {
                line,
                tag: tag.to_string(),
            });
        }
    };
    Ok(IdentityFace {
        tag,
        cid: fields.get("cid").cloned().unwrap_or_default(),
        codebook: fields.get("codebook").cloned().unwrap_or_default(),
        remap: fields.get("remap").cloned().unwrap_or_default(),
        lineage: fields.get("lineage").cloned().unwrap_or_default(),
    })
}

fn parse_structural(fields: &BTreeMap<String, String>) -> StructuralFace {
    StructuralFace {
        head: fields.get("head").cloned().unwrap_or_default(),
        args: split_list(fields.get("args")),
        local_sections: split_list(fields.get("locals")),
        morphism_hooks: split_list(fields.get("hooks")),
    }
}

fn parse_constraint(fields: &BTreeMap<String, String>) -> ConstraintFace {
    ConstraintFace {
        regime: fields.get("regime").cloned().unwrap_or_default(),
        contracts: split_list(fields.get("contracts")),
        invariants: split_list(fields.get("invariants")),
        equivalence: fields.get("equivalence").cloned().unwrap_or_default(),
        admissibility: fields.get("admissibility").cloned().unwrap_or_default(),
    }
}

fn parse_evidence(fields: &BTreeMap<String, String>, line: usize) -> Result<EvidenceFace, QaError> {
    let maturity = match fields
        .get("maturity")
        .map(String::as_str)
        .unwrap_or_default()
    {
        "Candidate" => EvidenceMaturity::Candidate,
        "Registered" => EvidenceMaturity::Registered,
        "Validated" => EvidenceMaturity::Validated,
        "Certified" => EvidenceMaturity::Certified,
        value => {
            return Err(QaError::InvalidMaturity {
                line,
                value: value.to_string(),
            });
        }
    };
    let gate_verdict = match fields
        .get("gate_verdict")
        .map(String::as_str)
        .unwrap_or_default()
    {
        "Pass" => GateVerdict::Pass,
        "Fail" => GateVerdict::Fail,
        "Review" => GateVerdict::Review,
        value => {
            return Err(QaError::InvalidGateVerdict {
                line,
                value: value.to_string(),
            });
        }
    };
    Ok(EvidenceFace {
        evidence_class: fields.get("evidence_class").cloned().unwrap_or_default(),
        traces: split_list(fields.get("traces")),
        receipts: split_list(fields.get("receipts")),
        maturity,
        gate_verdict,
    })
}

fn parse_alias(fields: &BTreeMap<String, String>) -> AliasFace {
    AliasFace {
        aliases: split_list(fields.get("aliases")),
        profile_pack: split_list(fields.get("profiles")),
        qm_binding: fields.get("qm_binding").cloned().unwrap_or_default(),
        qa_binding: fields.get("qa_binding").cloned().unwrap_or_default(),
        projection_policy: fields.get("projection_policy").cloned().unwrap_or_default(),
    }
}

fn split_list(value: Option<&String>) -> Vec<String> {
    value
        .map(|value| {
            value
                .split('|')
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(ToString::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn join_list(values: &[String]) -> String {
    values.join("|")
}

fn format_fields(fields: &[(&str, String)]) -> String {
    fields
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join(";")
}

#[cfg(test)]
mod tests {
    use super::*;
    use mf_core::{
        AtlasCell, BridgeContract, BurdenClass, Campaign, CampaignClass, CertificationVerdict,
        FailureSignature, LossProfile, ProofEdge, ProofShape, ProofShapeKind, RecipeMaturity,
        ReversibilityClass, WinnerState,
    };

    #[test]
    fn object_round_trip() {
        let input = "object [tag=CTX;cid=ctx.demo;codebook=seed;remap=none;lineage=origin]<head=carrier;args=x|y;locals=section;hooks=map>[regime=R_SET;contracts=total;invariants=stable;equivalence=eq-set;admissibility=admit]{evidence_class=Seed;traces=t1;receipts=r1;maturity=Validated;gate_verdict=Pass}<<aliases=ctx_demo;profiles=STD;qm_binding=reserved;qa_binding=ctx_demo;projection_policy=qa-first>>";
        let parsed = parse_document(input).unwrap();
        let normalized = normalize_document(&parsed).unwrap();
        assert_eq!(input, normalized);
    }

    #[test]
    fn malformed_object_rejected() {
        let err = parse_document("object [tag=CTX]<head=x>").unwrap_err();
        assert!(matches!(err, QaError::MalformedObjectShell { .. }));
    }

    #[test]
    fn parses_non_object_entries() {
        let bridge = BridgeContract {
            id: "B1".into(),
            src: "R_TYP".into(),
            tgt: "R_SET".into(),
            id_pres: "carrier".into(),
            eq_pres: "extensional".into(),
            forget: vec!["proof relevance".into()],
            enrich: vec![],
            loss: vec!["proof terms".into()],
            reversibility: ReversibilityClass::Conservative,
            receipts: vec!["rc1".into()],
            rollback: "allowed".into(),
        };
        let proof = ProofShape {
            id: "P1".into(),
            kind: ProofShapeKind::Triangle,
            nodes: vec!["a".into(), "b".into(), "c".into()],
            edges: vec![ProofEdge {
                from: "a".into(),
                to: "b".into(),
                label: "f".into(),
            }],
            equations: vec!["f=g".into()],
            target_equivalence: "eq".into(),
            receipts: vec!["r".into()],
            gate: GateVerdict::Pass,
        };
        let atlas = AtlasCell {
            id: "A1".into(),
            source_regime: "R_TYP".into(),
            target_regime: "R_SET".into(),
            burden_class: BurdenClass::ExtensionalCarrierReasoning,
            proof_target: "carrier".into(),
            candidate_paths: vec![vec!["B1".into()]],
            normalized_winner: vec!["B1".into()],
            winner_state: WinnerState::SeedWinner,
            loss_profile: LossProfile {
                items: vec!["proof terms".into()],
            },
            proof_shapes_checked: vec!["P1".into()],
            recipe_maturity: RecipeMaturity::Seeded,
            failure_signatures: vec![FailureSignature {
                code: "MISSING_EXT".into(),
                message: "extensionality required".into(),
            }],
            side_conditions: vec!["proof relevance not queried".into()],
            surface_transition: None,
        };
        let input = format!(
            "bridge {}\nproof {}\natlas {}\ncampaign {}",
            serde_json::to_string(&bridge).unwrap(),
            serde_json::to_string(&proof).unwrap(),
            serde_json::to_string(&atlas).unwrap(),
            serde_json::to_string(&Campaign {
                id: "C1".into(),
                theorem: "TH1".into(),
                target_profile: "T1".into(),
                route_ledger: "L1".into(),
                obligations: vec!["O1".into()],
                certificates: vec!["CRT1".into()],
                dependencies: vec![],
                campaign_class: CampaignClass::CBridge,
                verdict: CertificationVerdict::RouteFound,
                payoff: vec!["OPR.Chain1".into()],
            })
            .unwrap()
        );
        let parsed = parse_document(&input).unwrap();
        assert_eq!(parsed.entries.len(), 4);
    }
}
