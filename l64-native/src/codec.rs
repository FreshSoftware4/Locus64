use std::collections::BTreeMap;

use crate::kernel::{JUDGMENT_LOCUS, WITNESS_LOCUS};
use crate::{ContextDelta, Graph, LocusWord, Node, NodeId, OpCode, Port, PortRole, Route};

const CODEC_VERSION: u16 = 2;
const COMMITMENT_DOMAIN: &[u8] = b"l64-native-state-v2\0";
const MAX_NODES: usize = 1 << 20;
const MAX_PORTS: usize = 1 << 22;
const MAX_CONTEXTS: usize = 1 << 20;
const MAX_ROUTES: usize = 1 << 20;
const MAX_ROUTE_WORDS: usize = 64;
const NO_NODE: NodeId = u32::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    Truncated,
    BadMagic,
    UnsupportedVersion { version: u16 },
    StructuralBound,
    UnknownOpcode { opcode: u16 },
    UnknownPortRole { role: u8 },
    NonZeroPortFlags { flags: u8 },
    InvalidNode,
    InvalidPortRange,
    InvalidPortTarget,
    InvalidPortLaw,
    InvalidContext,
    InvalidRoute,
    DuplicateRoute,
    TrailingBytes,
    NonCanonical,
}

pub fn canonical_bytes(graph: &Graph) -> Vec<u8> {
    let mut out =
        Vec::with_capacity(22 + graph.nodes_raw().len() * 24 + graph.ports_raw().len() * 8);
    out.extend_from_slice(b"L64N");
    out.extend_from_slice(&CODEC_VERSION.to_le_bytes());
    push_len(&mut out, graph.nodes_raw().len());
    push_len(&mut out, graph.ports_raw().len());
    push_len(&mut out, graph.contexts_raw().len());
    push_len(&mut out, graph.routes_raw().len());

    for node in graph.nodes_raw() {
        out.extend_from_slice(&(node.opcode() as u16).to_le_bytes());
        out.extend_from_slice(&node.context().to_le_bytes());
        out.extend_from_slice(&node.ty().unwrap_or(NO_NODE).to_le_bytes());
        out.extend_from_slice(&node.payload().to_le_bytes());
        let range = node.port_range();
        out.extend_from_slice(&(range.start as u32).to_le_bytes());
        out.extend_from_slice(&((range.end - range.start) as u16).to_le_bytes());
    }

    for port in graph.ports_raw() {
        out.extend_from_slice(&port.target().to_le_bytes());
        out.push(port.role() as u8);
        out.push(port.flags());
        out.extend_from_slice(&port.ordinal().to_le_bytes());
    }

    for context in graph.contexts_raw() {
        out.extend_from_slice(&context.parent().to_le_bytes());
        out.extend_from_slice(&context.binding().unwrap_or(NO_NODE).to_le_bytes());
    }

    for (route, node) in graph.routes_raw() {
        out.extend_from_slice(&route.domain().0.to_le_bytes());
        push_len(&mut out, route.tail().len());
        for word in route.tail() {
            out.extend_from_slice(&word.0.to_le_bytes());
        }
        out.extend_from_slice(&node.to_le_bytes());
    }

    out
}

pub fn decode_canonical(bytes: &[u8]) -> Result<Graph, DecodeError> {
    let mut reader = Reader::new(bytes);
    if reader.take(4)? != b"L64N" {
        return Err(DecodeError::BadMagic);
    }
    let version = reader.u16()?;
    if version != CODEC_VERSION {
        return Err(DecodeError::UnsupportedVersion { version });
    }

    let node_count = reader.count(MAX_NODES)?;
    let port_count = reader.count(MAX_PORTS)?;
    let context_count = reader.count(MAX_CONTEXTS)?;
    let route_count = reader.count(MAX_ROUTES)?;
    if context_count == 0 || route_count != node_count {
        return Err(DecodeError::InvalidContext);
    }

    reader.require(node_count, 24)?;
    let mut nodes = Vec::with_capacity(node_count);
    for _ in 0..node_count {
        let raw_opcode = reader.u16()?;
        let opcode = OpCode::from_raw(raw_opcode)
            .filter(|opcode| opcode.is_persisted_node())
            .ok_or(DecodeError::UnknownOpcode { opcode: raw_opcode })?;
        let context = reader.u32()?;
        let ty = reader.u32()?;
        let payload = reader.u64()?;
        let first_port = reader.u32()?;
        let port_count = reader.u16()?;
        nodes.push(Node::from_raw(
            payload, context, ty, first_port, opcode, port_count,
        ));
    }

    reader.require(port_count, 8)?;
    let mut ports = Vec::with_capacity(port_count);
    for _ in 0..port_count {
        let target = reader.u32()?;
        let raw_role = reader.u8()?;
        let role =
            PortRole::from_raw(raw_role).ok_or(DecodeError::UnknownPortRole { role: raw_role })?;
        let flags = reader.u8()?;
        if flags != 0 {
            return Err(DecodeError::NonZeroPortFlags { flags });
        }
        let ordinal = reader.u16()?;
        ports.push(Port::from_raw(target, role, flags, ordinal));
    }

    reader.require(context_count, 8)?;
    let mut contexts = Vec::with_capacity(context_count);
    for _ in 0..context_count {
        contexts.push(ContextDelta::from_raw(reader.u32()?, reader.u32()?));
    }

    let mut routes = BTreeMap::new();
    for _ in 0..route_count {
        let domain = LocusWord(reader.u64()?);
        let tail_len = reader.count(MAX_ROUTE_WORDS)?;
        reader.require(tail_len, 8)?;
        let mut tail = Vec::with_capacity(tail_len);
        for _ in 0..tail_len {
            tail.push(LocusWord(reader.u64()?));
        }
        let node = reader.u32()?;
        let route = Route::from_parts(domain, tail.into_boxed_slice());
        if routes.insert(route, node).is_some() {
            return Err(DecodeError::DuplicateRoute);
        }
    }

    if !reader.is_empty() {
        return Err(DecodeError::TrailingBytes);
    }

    validate_structure(&nodes, &ports, &contexts, &routes)?;
    let commitment = commitment_bytes(bytes);
    let graph = Graph::from_decoded_parts(nodes, ports, contexts, routes, commitment);
    if canonical_bytes(&graph) != bytes {
        return Err(DecodeError::NonCanonical);
    }
    Ok(graph)
}

pub(crate) fn state_commitment(graph: &Graph) -> [u8; 32] {
    commitment_bytes(&canonical_bytes(graph))
}

fn commitment_bytes(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(COMMITMENT_DOMAIN);
    hasher.update(bytes);
    *hasher.finalize().as_bytes()
}

fn validate_structure(
    nodes: &[Node],
    ports: &[Port],
    contexts: &[ContextDelta],
    routes: &BTreeMap<Route, NodeId>,
) -> Result<(), DecodeError> {
    let root = contexts.first().ok_or(DecodeError::InvalidContext)?;
    if root.parent() != 0 || root.binding().is_some() {
        return Err(DecodeError::InvalidContext);
    }
    for (index, context) in contexts.iter().enumerate().skip(1) {
        if context.parent() as usize >= index {
            return Err(DecodeError::InvalidContext);
        }
        let binding = context.binding().ok_or(DecodeError::InvalidContext)?;
        if binding as usize >= nodes.len() {
            return Err(DecodeError::InvalidContext);
        }
    }

    let mut port_cursor = 0usize;
    for (index, node) in nodes.iter().enumerate() {
        if node.context() as usize >= contexts.len() {
            return Err(DecodeError::InvalidNode);
        }
        if let Some(ty) = node.ty() {
            if ty as usize >= index || !nodes[ty as usize].opcode().is_type() {
                return Err(DecodeError::InvalidNode);
            }
        } else if !node.opcode().is_type() {
            return Err(DecodeError::InvalidNode);
        }

        let range = node.port_range();
        if range.start != port_cursor || range.end > ports.len() {
            return Err(DecodeError::InvalidPortRange);
        }
        let node_ports = &ports[range.clone()];
        for (ordinal, port) in node_ports.iter().enumerate() {
            if port.target() as usize >= index {
                return Err(DecodeError::InvalidPortTarget);
            }
            if port.ordinal() as usize != ordinal {
                return Err(DecodeError::InvalidPortLaw);
            }
        }
        validate_port_law(node.opcode(), node_ports, node.ty().is_some())?;
        validate_node_semantics(index, node, node_ports, nodes, ports)?;
        port_cursor = range.end;
    }
    if port_cursor != ports.len() {
        return Err(DecodeError::InvalidPortRange);
    }

    let mut covered = vec![false; nodes.len()];
    for node in routes.values().copied() {
        let slot = covered
            .get_mut(node as usize)
            .ok_or(DecodeError::InvalidRoute)?;
        if *slot {
            return Err(DecodeError::InvalidRoute);
        }
        *slot = true;
    }
    if covered.iter().any(|covered| !covered) {
        return Err(DecodeError::InvalidRoute);
    }
    validate_evidence_routes(nodes, routes)?;
    Ok(())
}

fn validate_node_semantics(
    index: usize,
    node: &Node,
    node_ports: &[Port],
    nodes: &[Node],
    ports: &[Port],
) -> Result<(), DecodeError> {
    match node.opcode() {
        OpCode::Value => {
            let ty = node.ty().ok_or(DecodeError::InvalidNode)?;
            if nodes[ty as usize].opcode() == OpCode::TypeJudgment {
                return Err(DecodeError::InvalidNode);
            }
        }
        OpCode::KernelWitness => {
            let ty = node.ty().ok_or(DecodeError::InvalidNode)?;
            if nodes[ty as usize].opcode() != OpCode::TypeJudgment {
                return Err(DecodeError::InvalidNode);
            }
        }
        OpCode::TypeJudgment => {
            let rule = OpCode::from_raw(node.payload() as u16)
                .filter(|opcode| opcode.is_executable())
                .ok_or(DecodeError::InvalidNode)?;
            let subject = node_ports[0].target() as usize;
            if subject >= index || nodes[subject].opcode() != rule {
                return Err(DecodeError::InvalidNode);
            }
            let subject_node = &nodes[subject];
            if subject_node.ty() != Some(node_ports[3].target()) {
                return Err(DecodeError::InvalidNode);
            }
            let subject_ports = ports
                .get(subject_node.port_range())
                .ok_or(DecodeError::InvalidPortRange)?;
            if subject_ports.len() != 2
                || subject_ports[0].target() != node_ports[1].target()
                || subject_ports[1].target() != node_ports[2].target()
            {
                return Err(DecodeError::InvalidNode);
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_evidence_routes(
    nodes: &[Node],
    routes: &BTreeMap<Route, NodeId>,
) -> Result<(), DecodeError> {
    let mut attached = vec![false; nodes.len()];
    for (route, node_id) in routes {
        let node = &nodes[*node_id as usize];
        if !node.opcode().is_executable() {
            continue;
        }
        let judgment = *routes
            .get(&route.composed(JUDGMENT_LOCUS))
            .ok_or(DecodeError::InvalidRoute)?;
        let witness = *routes
            .get(&route.composed(WITNESS_LOCUS))
            .ok_or(DecodeError::InvalidRoute)?;
        if nodes[judgment as usize].opcode() != OpCode::TypeJudgment
            || nodes[witness as usize].opcode() != OpCode::KernelWitness
            || nodes[witness as usize].ty() != Some(judgment)
        {
            return Err(DecodeError::InvalidRoute);
        }
        attached[judgment as usize] = true;
        attached[witness as usize] = true;
    }
    for (index, node) in nodes.iter().enumerate() {
        if matches!(node.opcode(), OpCode::TypeJudgment | OpCode::KernelWitness) && !attached[index]
        {
            return Err(DecodeError::InvalidRoute);
        }
    }
    Ok(())
}

fn validate_port_law(opcode: OpCode, ports: &[Port], has_type: bool) -> Result<(), DecodeError> {
    let valid = match opcode {
        OpCode::TypeAtom => ports.is_empty() && !has_type,
        OpCode::TypeMatrix => {
            ports.len() == 1 && ports[0].role() == PortRole::Parameter && !has_type
        }
        OpCode::TypeFunction => {
            ports.len() == 2
                && ports[0].role() == PortRole::Domain
                && ports[1].role() == PortRole::Codomain
                && !has_type
        }
        OpCode::Value => ports.is_empty() && has_type,
        OpCode::Compose | OpCode::MatMul => {
            ports.len() == 2
                && ports.iter().all(|port| port.role() == PortRole::Argument)
                && has_type
        }
        OpCode::TypeJudgment => {
            ports.len() == 4
                && ports[0].role() == PortRole::Subject
                && ports[1].role() == PortRole::Premise
                && ports[2].role() == PortRole::Premise
                && ports[3].role() == PortRole::Conclusion
                && !has_type
        }
        OpCode::KernelWitness => ports.is_empty() && has_type,
        OpCode::ExtendContext => false,
    };
    valid.then_some(()).ok_or(DecodeError::InvalidPortLaw)
}

fn push_len(out: &mut Vec<u8>, len: usize) {
    let value = u32::try_from(len).expect("native graph section exceeds u32 length");
    out.extend_from_slice(&value.to_le_bytes());
}

struct Reader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    fn is_empty(&self) -> bool {
        self.cursor == self.bytes.len()
    }

    fn take(&mut self, len: usize) -> Result<&'a [u8], DecodeError> {
        let end = self.cursor.checked_add(len).ok_or(DecodeError::Truncated)?;
        let value = self
            .bytes
            .get(self.cursor..end)
            .ok_or(DecodeError::Truncated)?;
        self.cursor = end;
        Ok(value)
    }

    fn require(&self, count: usize, width: usize) -> Result<(), DecodeError> {
        let bytes = count
            .checked_mul(width)
            .ok_or(DecodeError::StructuralBound)?;
        if self.bytes.len().saturating_sub(self.cursor) < bytes {
            return Err(DecodeError::Truncated);
        }
        Ok(())
    }

    fn count(&mut self, maximum: usize) -> Result<usize, DecodeError> {
        let count = self.u32()? as usize;
        if count > maximum {
            return Err(DecodeError::StructuralBound);
        }
        Ok(count)
    }

    fn u8(&mut self) -> Result<u8, DecodeError> {
        Ok(self.take(1)?[0])
    }

    fn u16(&mut self) -> Result<u16, DecodeError> {
        Ok(u16::from_le_bytes(
            self.take(2)?.try_into().expect("fixed-width read"),
        ))
    }

    fn u32(&mut self) -> Result<u32, DecodeError> {
        Ok(u32::from_le_bytes(
            self.take(4)?.try_into().expect("fixed-width read"),
        ))
    }

    fn u64(&mut self) -> Result<u64, DecodeError> {
        Ok(u64::from_le_bytes(
            self.take(8)?.try_into().expect("fixed-width read"),
        ))
    }
}
