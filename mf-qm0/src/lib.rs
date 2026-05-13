use mf_core::{
    AliasFace, ConstraintFace, EvidenceFace, EvidenceMaturity, GateVerdict, HeaderEnvelope,
    IdentityFace, ObjectTag, QaDocument, QaEntry, QcObject, StructuralFace, SurfaceKind,
};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Qm0Error {
    #[error("missing or invalid qm0 header")]
    MissingHeader,
    #[error("invalid payload: {0}")]
    InvalidPayload(String),
}

pub fn parse_qm0(input: &str) -> Result<(HeaderEnvelope, QaDocument), Qm0Error> {
    let mut lines = input.lines().filter(|line| !line.trim().is_empty());
    let header_line = lines.next().ok_or(Qm0Error::MissingHeader)?;
    let payload = header_line
        .strip_prefix("!qm0 ")
        .ok_or(Qm0Error::MissingHeader)?;
    let header: HeaderEnvelope =
        serde_json::from_str(payload).map_err(|err| Qm0Error::InvalidPayload(err.to_string()))?;
    if header.surface_kind != SurfaceKind::Qm0 {
        return Err(Qm0Error::MissingHeader);
    }
    let mut entries = Vec::new();
    for line in lines {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("object ") {
            entries.push(QaEntry::Object(parse_object_shell(rest)?));
            continue;
        }
        let (kind, payload) = line
            .split_once(' ')
            .ok_or_else(|| Qm0Error::InvalidPayload(line.to_string()))?;
        let entry = match kind {
            "object" => serde_json::from_str(payload).map(QaEntry::Object),
            "entry" => serde_json::from_str(payload),
            _ => QaEntry::from_surface_json(kind, payload),
        }
        .map_err(|err| Qm0Error::InvalidPayload(err.to_string()))?;
        entries.push(entry);
    }
    Ok((header, QaDocument { entries }))
}

pub fn render_qm0(header: &HeaderEnvelope, document: &QaDocument) -> String {
    let mut lines = vec![format!("!qm0 {}", serde_json::to_string(header).unwrap())];
    for entry in &document.entries {
        match entry {
            QaEntry::Object(item) => lines.push(format!("object {}", format_object_shell(item))),
            _ => {
                let (kind, payload) = entry.to_surface_json().unwrap();
                lines.push(format!("{kind} {payload}"));
            }
        }
    }
    lines.join("\n")
}

pub fn format_object_shell(object: &QcObject) -> String {
    format!(
        "⟦{}⟧⟨{}⟩〔{}〕⦃{}⦄⟪{}⟫",
        format_fields(&[
            ("tag", format!("{:?}", object.identity.tag).to_uppercase()),
            ("cid", object.identity.cid.clone()),
            ("codebook", object.identity.codebook.clone()),
            ("remap", object.identity.remap.clone()),
            ("lineage", object.identity.lineage.clone()),
        ]),
        format_fields(&[
            ("head", object.structural.head.clone()),
            ("args", object.structural.args.join("|")),
            ("locals", object.structural.local_sections.join("|")),
            ("hooks", object.structural.morphism_hooks.join("|")),
        ]),
        format_fields(&[
            ("regime", object.constraint.regime.clone()),
            ("contracts", object.constraint.contracts.join("|")),
            ("invariants", object.constraint.invariants.join("|")),
            ("equivalence", object.constraint.equivalence.clone()),
            ("admissibility", object.constraint.admissibility.clone()),
        ]),
        format_fields(&[
            ("evidence_class", object.evidence.evidence_class.clone()),
            ("traces", object.evidence.traces.join("|")),
            ("receipts", object.evidence.receipts.join("|")),
            ("maturity", format!("{:?}", object.evidence.maturity)),
            (
                "gate_verdict",
                format!("{:?}", object.evidence.gate_verdict)
            ),
        ]),
        format_fields(&[
            ("aliases", object.alias.aliases.join("|")),
            ("profiles", object.alias.profile_pack.join("|")),
            ("qm_binding", object.alias.qm_binding.clone()),
            ("qa_binding", object.alias.qa_binding.clone()),
            ("projection_policy", object.alias.projection_policy.clone()),
        ])
    )
}

fn parse_object_shell(input: &str) -> Result<QcObject, Qm0Error> {
    let (identity_raw, rest) = extract_segment(input, "⟦", "⟧")?;
    let (structural_raw, rest) = extract_segment(rest, "⟨", "⟩")?;
    let (constraint_raw, rest) = extract_segment(rest, "〔", "〕")?;
    let (evidence_raw, rest) = extract_segment(rest, "⦃", "⦄")?;
    let alias_raw = extract_segment(rest, "⟪", "⟫")?.0;
    let identity = parse_identity(&split_fields(identity_raw))?;
    let structural = parse_structural(&split_fields(structural_raw));
    let constraint = parse_constraint(&split_fields(constraint_raw));
    let evidence = parse_evidence(&split_fields(evidence_raw))?;
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

fn extract_segment<'a>(
    input: &'a str,
    open: &str,
    close: &str,
) -> Result<(&'a str, &'a str), Qm0Error> {
    let input = input.trim_start();
    let body = input
        .strip_prefix(open)
        .ok_or_else(|| Qm0Error::InvalidPayload(input.to_string()))?;
    let index = body
        .find(close)
        .ok_or_else(|| Qm0Error::InvalidPayload(input.to_string()))?;
    Ok((&body[..index], &body[index + close.len()..]))
}

fn split_fields(input: &str) -> BTreeMap<String, String> {
    input
        .split(';')
        .filter_map(|entry| {
            let (key, value) = entry.split_once('=')?;
            Some((key.trim().to_string(), value.trim().to_string()))
        })
        .collect()
}

fn parse_identity(fields: &BTreeMap<String, String>) -> Result<IdentityFace, Qm0Error> {
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
        "THS" => ObjectTag::Ths,
        "OBL" => ObjectTag::Obl,
        "POL" => ObjectTag::Pol,
        "XFR" => ObjectTag::Xfr,
        "RTP" => ObjectTag::Rtp,
        "CAP" => ObjectTag::Cap,
        "SBD" => ObjectTag::Sbd,
        other => return Err(Qm0Error::InvalidPayload(other.to_string())),
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
fn parse_evidence(fields: &BTreeMap<String, String>) -> Result<EvidenceFace, Qm0Error> {
    let maturity = match fields
        .get("maturity")
        .map(String::as_str)
        .unwrap_or_default()
    {
        "Candidate" => EvidenceMaturity::Candidate,
        "Registered" => EvidenceMaturity::Registered,
        "Validated" => EvidenceMaturity::Validated,
        "Certified" => EvidenceMaturity::Certified,
        other => return Err(Qm0Error::InvalidPayload(other.to_string())),
    };
    let gate_verdict = match fields
        .get("gate_verdict")
        .map(String::as_str)
        .unwrap_or_default()
    {
        "Pass" => GateVerdict::Pass,
        "Fail" => GateVerdict::Fail,
        "Review" => GateVerdict::Review,
        other => return Err(Qm0Error::InvalidPayload(other.to_string())),
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
        .map(|v| v.split('|').map(str::to_string).collect())
        .unwrap_or_default()
}
fn format_fields(fields: &[(&str, String)]) -> String {
    fields
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join(";")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn qm0_object_roundtrip() {
        let header = HeaderEnvelope {
            surface_kind: SurfaceKind::Qm0,
            version: "1".into(),
            policy_id: "POL_QM0_PACK_A".into(),
            capability_id: Some("CAP_QM0_PACK_A".into()),
        };
        let object = QcObject {
            id: "OBJ".into(),
            identity: IdentityFace {
                tag: ObjectTag::Ctx,
                cid: "OBJ".into(),
                codebook: "CB".into(),
                remap: "none".into(),
                lineage: "seed".into(),
            },
            structural: StructuralFace {
                head: "carrier".into(),
                args: vec!["x".into()],
                local_sections: vec![],
                morphism_hooks: vec![],
            },
            constraint: ConstraintFace {
                regime: "R_SET".into(),
                contracts: vec![],
                invariants: vec![],
                equivalence: "eq".into(),
                admissibility: "adm".into(),
            },
            evidence: EvidenceFace {
                evidence_class: "Seed".into(),
                traces: vec![],
                receipts: vec![],
                maturity: EvidenceMaturity::Validated,
                gate_verdict: GateVerdict::Pass,
            },
            alias: AliasFace {
                aliases: vec![],
                profile_pack: vec![],
                qm_binding: "qm".into(),
                qa_binding: "qa".into(),
                projection_policy: "policy".into(),
            },
        };
        let text = render_qm0(
            &header,
            &QaDocument {
                entries: vec![QaEntry::Object(object)],
            },
        );
        let (_, parsed) = parse_qm0(&text).unwrap();
        assert_eq!(parsed.entries.len(), 1);
    }
}
