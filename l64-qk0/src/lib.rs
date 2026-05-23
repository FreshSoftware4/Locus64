use l64_core::{ComboPack, HeaderEnvelope, SurfaceKind};
use l64_qm0::{parse_qm0, render_qm0};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Qk0Error {
    #[error("missing or invalid qk0 header")]
    MissingHeader,
    #[error("missing iei/eie gate")]
    MissingGate,
    #[error("unsupported combo `{0}`")]
    UnsupportedCombo(String),
    #[error("invalid qk0 payload: {0}")]
    InvalidPayload(String),
}

pub fn parse_qk0(
    input: &str,
    combo_pack: &ComboPack,
) -> Result<(HeaderEnvelope, String), Qk0Error> {
    let mut lines = input.lines().filter(|line| !line.trim().is_empty());
    let header_line = lines.next().ok_or(Qk0Error::MissingHeader)?;
    let payload = header_line
        .strip_prefix("!qk0 ")
        .ok_or(Qk0Error::MissingHeader)?;
    let header: HeaderEnvelope =
        serde_json::from_str(payload).map_err(|err| Qk0Error::InvalidPayload(err.to_string()))?;
    if header.surface_kind != SurfaceKind::Qk0 {
        return Err(Qk0Error::MissingHeader);
    }
    let body = lines.collect::<Vec<_>>().join("\n");
    let gated = body
        .strip_prefix("iei")
        .and_then(|rest| rest.strip_suffix("eie"))
        .ok_or(Qk0Error::MissingGate)?;
    let mut qm0 = String::new();
    for token in gated.split_whitespace() {
        if let Some(symbol) = combo_pack.combos.get(token) {
            qm0.push_str(symbol);
        } else if let Some(raw) = token.strip_prefix("txt:") {
            let decoded =
                hex::decode(raw).map_err(|err| Qk0Error::InvalidPayload(err.to_string()))?;
            qm0.push_str(
                &String::from_utf8(decoded)
                    .map_err(|err| Qk0Error::InvalidPayload(err.to_string()))?,
            );
        } else {
            return Err(Qk0Error::UnsupportedCombo(token.to_string()));
        }
    }
    Ok((header, qm0))
}

pub fn render_qk0(header: &HeaderEnvelope, qm0_text: &str, combo_pack: &ComboPack) -> String {
    let mut inverse = std::collections::BTreeMap::new();
    for (combo, symbol) in &combo_pack.combos {
        inverse.insert(symbol.as_str(), combo.as_str());
    }
    let mut tokens = Vec::new();
    for ch in qm0_text.chars() {
        let mut buf = [0u8; 4];
        let symbol = ch.encode_utf8(&mut buf);
        if let Some(combo) = inverse.get(symbol) {
            tokens.push((*combo).to_string());
        } else {
            tokens.push(format!("txt:{}", hex::encode(symbol.as_bytes())));
        }
    }
    format!(
        "!qk0 {}\niei {} eie",
        serde_json::to_string(header).unwrap(),
        tokens.join(" ")
    )
}

pub fn expand_qk0(input: &str, combo_pack: &ComboPack) -> Result<String, Qk0Error> {
    parse_qk0(input, combo_pack).map(|(_, qm0)| qm0)
}

pub fn qk0_to_document(
    input: &str,
    combo_pack: &ComboPack,
) -> Result<(HeaderEnvelope, l64_core::QaDocument), Qk0Error> {
    let (header, qm0) = parse_qk0(input, combo_pack)?;
    let (_, document) = parse_qm0(&qm0).map_err(|err| Qk0Error::InvalidPayload(err.to_string()))?;
    Ok((header, document))
}

pub fn document_to_qk0(
    header: &HeaderEnvelope,
    document: &l64_core::QaDocument,
    combo_pack: &ComboPack,
) -> String {
    let qm0_header = HeaderEnvelope {
        surface_kind: SurfaceKind::Qm0,
        version: header.version.clone(),
        policy_id: header.policy_id.clone(),
        capability_id: header.capability_id.clone(),
    };
    let qm0 = render_qm0(&qm0_header, document);
    render_qk0(header, &qm0, combo_pack)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn qk0_gate_roundtrip() {
        let pack = ComboPack {
            id: "COMBO".into(),
            version: "1".into(),
            pack: l64_core::SupportedPack::PackA,
            combos: BTreeMap::from([
                ("id_open".into(), "⟦".into()),
                ("id_close".into(), "⟧".into()),
            ]),
        };
        let header = HeaderEnvelope {
            surface_kind: SurfaceKind::Qk0,
            version: "1".into(),
            policy_id: "POL".into(),
            capability_id: None,
        };
        let text = render_qk0(&header, "⟦⟧", &pack);
        let (_, expanded) = parse_qk0(&text, &pack).unwrap();
        assert_eq!(expanded, "⟦⟧");
    }
}
