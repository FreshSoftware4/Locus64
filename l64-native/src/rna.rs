use std::collections::BTreeMap;

use crate::{
    DnaError, Graph, LocusWord, NodeId, Obstruction, OpCode, Proposal, ROOT_CONTEXT, Route,
    decode_dna, dna_bytes,
};

const RNA_MAGIC: &[u8] = b"L64R1";
pub const MAX_NATIVE_RNA_BYTES: usize = 1 << 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RnaError {
    SourceTooLarge,
    Empty,
    BadHeader,
    InvalidArity { line: u32 },
    InvalidNumber { line: u32 },
    UnknownInstruction { line: u32 },
    SlotOrder { line: u32, slot: u64 },
    UnknownSlot { line: u32, slot: u64 },
    UnrepresentableGraph,
    Graph(Obstruction),
    Dna(DnaError),
}

impl From<Obstruction> for RnaError {
    fn from(value: Obstruction) -> Self {
        Self::Graph(value)
    }
}

impl From<DnaError> for RnaError {
    fn from(value: DnaError) -> Self {
        Self::Dna(value)
    }
}

pub fn compile_rna(source: &[u8]) -> Result<Graph, RnaError> {
    if source.len() > MAX_NATIVE_RNA_BYTES {
        return Err(RnaError::SourceTooLarge);
    }

    let mut lines = source.split(|byte| *byte == b'\n').enumerate();
    let (header_line, header) = next_nonempty(&mut lines).ok_or(RnaError::Empty)?;
    let header_tokens = tokens(header);
    if header_tokens.len() != 2 || header_tokens[0] != RNA_MAGIC {
        return Err(RnaError::BadHeader);
    }
    let domain = parse_u64(header_tokens[1]).ok_or(RnaError::InvalidNumber {
        line: line_number(header_line),
    })?;

    let root = Route::root(LocusWord(domain));
    let mut graph = Graph::new();
    let mut slots = BTreeMap::new();
    let mut last_slot = None;
    let mut instruction_count = 0usize;

    for (line_index, raw_line) in lines {
        let line = trim_ascii(raw_line);
        if line.is_empty() {
            continue;
        }
        let parts = tokens(line);
        let line = line_number(line_index);
        let instruction = parts
            .first()
            .copied()
            .ok_or(RnaError::InvalidArity { line })?;
        let slot = parse_part(&parts, 1, line)?;
        if let Some(previous) = last_slot
            && slot <= previous
        {
            return Err(RnaError::SlotOrder { line, slot });
        }
        let route = root.composed(LocusWord(slot));

        let node = match instruction {
            b"a" if parts.len() == 3 => {
                graph.declare_atom_type(route, LocusWord(parse_part(&parts, 2, line)?))?
            }
            b"m" if parts.len() == 5 => {
                let element = resolve_part(&slots, &parts, 2, line)?;
                let rows = parse_u32_part(&parts, 3, line)?;
                let cols = parse_u32_part(&parts, 4, line)?;
                graph.declare_matrix_type(route, element, rows, cols)?
            }
            b"f" if parts.len() == 4 => {
                let domain = resolve_part(&slots, &parts, 2, line)?;
                let codomain = resolve_part(&slots, &parts, 3, line)?;
                graph.declare_function_type(route, domain, codomain)?
            }
            b"v" if parts.len() == 3 => {
                let ty = resolve_part(&slots, &parts, 2, line)?;
                graph.insert_value(route, ROOT_CONTEXT, ty)?
            }
            b"c" if parts.len() == 5 => {
                let first = resolve_part(&slots, &parts, 2, line)?;
                let second = resolve_part(&slots, &parts, 3, line)?;
                let output = resolve_part(&slots, &parts, 4, line)?;
                graph
                    .transact(Proposal::compose(
                        route,
                        ROOT_CONTEXT,
                        first,
                        second,
                        output,
                    ))?
                    .node
            }
            b"x" if parts.len() == 5 => {
                let left = resolve_part(&slots, &parts, 2, line)?;
                let right = resolve_part(&slots, &parts, 3, line)?;
                let output = resolve_part(&slots, &parts, 4, line)?;
                graph
                    .transact(Proposal::matrix_multiply(
                        route,
                        ROOT_CONTEXT,
                        left,
                        right,
                        output,
                    ))?
                    .node
            }
            b"a" | b"m" | b"f" | b"v" | b"c" | b"x" => {
                return Err(RnaError::InvalidArity { line });
            }
            _ => return Err(RnaError::UnknownInstruction { line }),
        };

        slots.insert(slot, node);
        last_slot = Some(slot);
        instruction_count += 1;
    }

    if instruction_count == 0 {
        return Err(RnaError::Empty);
    }
    Ok(graph)
}

pub fn normalize_rna(source: &[u8]) -> Result<Vec<u8>, RnaError> {
    rna_bytes(&compile_rna(source)?)
}

pub fn rna_to_dna(source: &[u8]) -> Result<Vec<u8>, RnaError> {
    Ok(dna_bytes(&compile_rna(source)?)?)
}

pub fn dna_to_rna(source: &[u8]) -> Result<Vec<u8>, RnaError> {
    rna_bytes(&decode_dna(source)?)
}

pub fn rna_bytes(graph: &Graph) -> Result<Vec<u8>, RnaError> {
    if graph.context_count() != 1 {
        return Err(RnaError::UnrepresentableGraph);
    }

    let mut domain = None;
    let mut authored = BTreeMap::new();
    for (route, node) in graph.routes_raw() {
        if route.tail().len() != 1 {
            continue;
        }
        match domain {
            Some(existing) if existing != route.domain() => {
                return Err(RnaError::UnrepresentableGraph);
            }
            None => domain = Some(route.domain()),
            _ => {}
        }
        if authored.insert(route.tail()[0].0, *node).is_some() {
            return Err(RnaError::UnrepresentableGraph);
        }
    }

    let domain = domain.ok_or(RnaError::Empty)?;
    let expected_authored = graph
        .nodes_raw()
        .iter()
        .filter(|node| !matches!(node.opcode(), OpCode::TypeJudgment | OpCode::KernelWitness))
        .count();
    if authored.len() != expected_authored {
        return Err(RnaError::UnrepresentableGraph);
    }

    let mut slots = BTreeMap::new();
    for (slot, node) in &authored {
        if slots.insert(*node, *slot).is_some() {
            return Err(RnaError::UnrepresentableGraph);
        }
    }

    let mut out = Vec::with_capacity(32 + authored.len() * 24);
    out.extend_from_slice(RNA_MAGIC);
    out.push(b' ');
    push_hex(&mut out, domain.0);
    out.push(b'\n');

    for (slot, node_id) in authored {
        let node = graph.node(node_id).ok_or(RnaError::UnrepresentableGraph)?;
        if node.context() != ROOT_CONTEXT {
            return Err(RnaError::UnrepresentableGraph);
        }
        let ports = graph.ports(node_id).ok_or(RnaError::UnrepresentableGraph)?;
        match node.opcode() {
            OpCode::TypeAtom if ports.is_empty() && node.ty().is_none() => {
                push_prefix(&mut out, b'a', slot);
                push_space_hex(&mut out, node.payload());
            }
            OpCode::TypeMatrix if ports.len() == 1 && node.ty().is_none() => {
                push_prefix(&mut out, b'm', slot);
                push_space_decimal(&mut out, slot_for(&slots, ports[0].target())?);
                push_space_decimal(&mut out, node.payload() >> 32);
                push_space_decimal(&mut out, node.payload() as u32 as u64);
            }
            OpCode::TypeFunction if ports.len() == 2 && node.ty().is_none() => {
                push_prefix(&mut out, b'f', slot);
                push_space_decimal(&mut out, slot_for(&slots, ports[0].target())?);
                push_space_decimal(&mut out, slot_for(&slots, ports[1].target())?);
            }
            OpCode::Value if ports.is_empty() => {
                push_prefix(&mut out, b'v', slot);
                push_space_decimal(
                    &mut out,
                    slot_for(&slots, node.ty().ok_or(RnaError::UnrepresentableGraph)?)?,
                );
            }
            OpCode::Compose | OpCode::MatMul if ports.len() == 2 => {
                push_prefix(
                    &mut out,
                    if node.opcode() == OpCode::Compose {
                        b'c'
                    } else {
                        b'x'
                    },
                    slot,
                );
                push_space_decimal(&mut out, slot_for(&slots, ports[0].target())?);
                push_space_decimal(&mut out, slot_for(&slots, ports[1].target())?);
                push_space_decimal(
                    &mut out,
                    slot_for(&slots, node.ty().ok_or(RnaError::UnrepresentableGraph)?)?,
                );
            }
            _ => return Err(RnaError::UnrepresentableGraph),
        }
        out.push(b'\n');
    }

    Ok(out)
}

fn next_nonempty<'a>(
    lines: &mut impl Iterator<Item = (usize, &'a [u8])>,
) -> Option<(usize, &'a [u8])> {
    lines.find_map(|(index, line)| {
        let line = trim_ascii(line);
        (!line.is_empty()).then_some((index, line))
    })
}

fn trim_ascii(mut bytes: &[u8]) -> &[u8] {
    while bytes.first().is_some_and(u8::is_ascii_whitespace) {
        bytes = &bytes[1..];
    }
    while bytes.last().is_some_and(u8::is_ascii_whitespace) {
        bytes = &bytes[..bytes.len() - 1];
    }
    bytes
}

fn tokens(line: &[u8]) -> Vec<&[u8]> {
    line.split(|byte| byte.is_ascii_whitespace())
        .filter(|part| !part.is_empty())
        .collect()
}

fn parse_part(parts: &[&[u8]], index: usize, line: u32) -> Result<u64, RnaError> {
    parts
        .get(index)
        .and_then(|part| parse_u64(part))
        .ok_or(RnaError::InvalidNumber { line })
}

fn parse_u32_part(parts: &[&[u8]], index: usize, line: u32) -> Result<u32, RnaError> {
    parse_part(parts, index, line)?
        .try_into()
        .map_err(|_| RnaError::InvalidNumber { line })
}

fn resolve_part(
    slots: &BTreeMap<u64, NodeId>,
    parts: &[&[u8]],
    index: usize,
    line: u32,
) -> Result<NodeId, RnaError> {
    let slot = parse_part(parts, index, line)?;
    slots
        .get(&slot)
        .copied()
        .ok_or(RnaError::UnknownSlot { line, slot })
}

fn parse_u64(bytes: &[u8]) -> Option<u64> {
    let (radix, digits) = if bytes.starts_with(b"0x") || bytes.starts_with(b"0X") {
        (16u64, &bytes[2..])
    } else {
        (10u64, bytes)
    };
    if digits.is_empty() {
        return None;
    }
    digits.iter().try_fold(0u64, |value, byte| {
        let digit = match *byte {
            b'0'..=b'9' => u64::from(*byte - b'0'),
            b'a'..=b'f' if radix == 16 => u64::from(*byte - b'a' + 10),
            b'A'..=b'F' if radix == 16 => u64::from(*byte - b'A' + 10),
            _ => return None,
        };
        if digit >= radix {
            return None;
        }
        value.checked_mul(radix)?.checked_add(digit)
    })
}

fn slot_for(slots: &BTreeMap<NodeId, u64>, node: NodeId) -> Result<u64, RnaError> {
    slots
        .get(&node)
        .copied()
        .ok_or(RnaError::UnrepresentableGraph)
}

fn push_prefix(out: &mut Vec<u8>, opcode: u8, slot: u64) {
    out.push(opcode);
    push_space_decimal(out, slot);
}

fn push_space_decimal(out: &mut Vec<u8>, value: u64) {
    out.push(b' ');
    push_decimal(out, value);
}

fn push_space_hex(out: &mut Vec<u8>, value: u64) {
    out.push(b' ');
    push_hex(out, value);
}

fn push_decimal(out: &mut Vec<u8>, mut value: u64) {
    let mut buffer = [0u8; 20];
    let mut cursor = buffer.len();
    loop {
        cursor -= 1;
        buffer[cursor] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    out.extend_from_slice(&buffer[cursor..]);
}

fn push_hex(out: &mut Vec<u8>, mut value: u64) {
    out.extend_from_slice(b"0x");
    if value == 0 {
        out.push(b'0');
        return;
    }
    let mut buffer = [0u8; 16];
    let mut cursor = buffer.len();
    while value != 0 {
        cursor -= 1;
        let digit = (value & 0xf) as u8;
        buffer[cursor] = if digit < 10 {
            b'0' + digit
        } else {
            b'a' + digit - 10
        };
        value >>= 4;
    }
    out.extend_from_slice(&buffer[cursor..]);
}

fn line_number(index: usize) -> u32 {
    u32::try_from(index + 1).unwrap_or(u32::MAX)
}
