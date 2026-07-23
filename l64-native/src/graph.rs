use std::collections::BTreeMap;

use crate::{ContextDelta, JournalEvent, LocusWord, Obstruction, OpCode, Port, PortRole, Route};

pub type NodeId = u32;
pub type ContextId = u32;
pub type EventId = u32;

pub const ROOT_CONTEXT: ContextId = 0;
const NO_NODE: NodeId = u32::MAX;
const META_TYPE: NodeId = u32::MAX;
const NO_EVENT: EventId = u32::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Node {
    payload: u64,
    context: ContextId,
    ty: NodeId,
    first_port: u32,
    opcode: u16,
    port_count: u16,
}

impl Node {
    pub fn opcode(&self) -> OpCode {
        OpCode::from_raw(self.opcode).expect("stored opcode is validated at insertion")
    }

    pub fn context(&self) -> ContextId {
        self.context
    }

    pub fn ty(&self) -> Option<NodeId> {
        (self.ty != META_TYPE).then_some(self.ty)
    }

    pub fn payload(&self) -> u64 {
        self.payload
    }

    pub fn port_range(&self) -> core::ops::Range<usize> {
        let start = self.first_port as usize;
        start..start + self.port_count as usize
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    nodes: Vec<Node>,
    ports: Vec<Port>,
    contexts: Vec<ContextDelta>,
    routes: BTreeMap<Route, NodeId>,
    journal: Vec<JournalEvent>,
    commitment: [u8; 32],
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    pub fn new() -> Self {
        let mut graph = Self {
            nodes: Vec::new(),
            ports: Vec::new(),
            contexts: vec![ContextDelta::root()],
            routes: BTreeMap::new(),
            journal: Vec::new(),
            commitment: [0; 32],
        };
        graph.commitment = crate::codec::state_commitment(&graph);
        graph
    }

    pub fn node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id as usize)
    }

    pub fn ports(&self, node: NodeId) -> Option<&[Port]> {
        let node = self.node(node)?;
        self.ports.get(node.port_range())
    }

    pub fn resolve(&self, route: &Route) -> Option<NodeId> {
        self.routes.get(route).copied()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn port_count(&self) -> usize {
        self.ports.len()
    }

    pub fn context_count(&self) -> usize {
        self.contexts.len()
    }

    pub fn journal_len(&self) -> usize {
        self.journal.len()
    }

    pub fn state_commitment(&self) -> [u8; 32] {
        self.commitment
    }

    pub fn journal(&self) -> &[JournalEvent] {
        &self.journal
    }

    pub fn extend_context(
        &mut self,
        parent: ContextId,
        binding: NodeId,
    ) -> Result<ContextId, Obstruction> {
        self.ensure_context(parent)?;
        self.ensure_node(binding)?;
        let before = self.commitment;
        let id = self.contexts.len() as ContextId;
        self.contexts.push(ContextDelta { parent, binding });
        let after = crate::codec::state_commitment(self);
        self.push_event(OpCode::ExtendContext, binding, before, after);
        self.commitment = after;
        Ok(id)
    }

    pub fn declare_atom_type(
        &mut self,
        route: Route,
        atom_code: LocusWord,
    ) -> Result<NodeId, Obstruction> {
        self.insert_committed_node(
            route,
            ROOT_CONTEXT,
            META_TYPE,
            OpCode::TypeAtom,
            atom_code.0,
            &[],
        )
    }

    pub fn declare_matrix_type(
        &mut self,
        route: Route,
        element: NodeId,
        rows: u32,
        cols: u32,
    ) -> Result<NodeId, Obstruction> {
        self.ensure_type(element)?;
        let payload = u64::from(rows) << 32 | u64::from(cols);
        let port = Port::new(element, PortRole::Parameter, 0);
        self.insert_committed_node(
            route,
            ROOT_CONTEXT,
            META_TYPE,
            OpCode::TypeMatrix,
            payload,
            &[port],
        )
    }

    pub fn declare_function_type(
        &mut self,
        route: Route,
        domain: NodeId,
        codomain: NodeId,
    ) -> Result<NodeId, Obstruction> {
        self.ensure_type(domain)?;
        self.ensure_type(codomain)?;
        let ports = [
            Port::new(domain, PortRole::Domain, 0),
            Port::new(codomain, PortRole::Codomain, 1),
        ];
        self.insert_committed_node(
            route,
            ROOT_CONTEXT,
            META_TYPE,
            OpCode::TypeFunction,
            0,
            &ports,
        )
    }

    pub fn insert_value(
        &mut self,
        route: Route,
        context: ContextId,
        ty: NodeId,
    ) -> Result<NodeId, Obstruction> {
        self.ensure_context(context)?;
        self.ensure_type(ty)?;
        self.insert_committed_node(route, context, ty, OpCode::Value, 0, &[])
    }

    pub(crate) fn insert_operation(
        &mut self,
        route: Route,
        context: ContextId,
        ty: NodeId,
        opcode: OpCode,
        inputs: &[NodeId],
    ) -> Result<crate::CommitResult, Obstruction> {
        let ports = inputs
            .iter()
            .enumerate()
            .map(|(ordinal, target)| Port::new(*target, PortRole::Argument, ordinal as u16))
            .collect::<Vec<_>>();
        let node = self.insert_committed_node(route, context, ty, opcode, 0, &ports)?;
        let event = self.journal.len().saturating_sub(1) as EventId;
        Ok(crate::CommitResult {
            node,
            event,
            commitment: self.commitment,
        })
    }

    pub(crate) fn function_parts(&self, ty: NodeId) -> Result<(NodeId, NodeId), Obstruction> {
        let node = self.ensure_type(ty)?;
        if node.opcode() != OpCode::TypeFunction {
            return Err(Obstruction::ExpectedFunctionType { node: ty });
        }
        let ports = self
            .ports(ty)
            .ok_or(Obstruction::UnknownNode { node: ty })?;
        if ports.len() != 2 {
            return Err(Obstruction::MalformedType { node: ty });
        }
        Ok((ports[0].target(), ports[1].target()))
    }

    pub(crate) fn matrix_parts(&self, ty: NodeId) -> Result<(NodeId, u32, u32), Obstruction> {
        let node = self.ensure_type(ty)?;
        if node.opcode() != OpCode::TypeMatrix {
            return Err(Obstruction::ExpectedMatrixType { node: ty });
        }
        let ports = self
            .ports(ty)
            .ok_or(Obstruction::UnknownNode { node: ty })?;
        if ports.len() != 1 {
            return Err(Obstruction::MalformedType { node: ty });
        }
        Ok((
            ports[0].target(),
            (node.payload >> 32) as u32,
            node.payload as u32,
        ))
    }

    pub(crate) fn matches_function_type(
        &self,
        ty: NodeId,
        domain: NodeId,
        codomain: NodeId,
    ) -> bool {
        self.function_parts(ty) == Ok((domain, codomain))
    }

    pub(crate) fn matches_matrix_type(
        &self,
        ty: NodeId,
        element: NodeId,
        rows: u32,
        cols: u32,
    ) -> bool {
        self.matrix_parts(ty) == Ok((element, rows, cols))
    }

    pub(crate) fn ensure_node(&self, id: NodeId) -> Result<&Node, Obstruction> {
        self.node(id).ok_or(Obstruction::UnknownNode { node: id })
    }

    pub(crate) fn ensure_type(&self, id: NodeId) -> Result<&Node, Obstruction> {
        let node = self.ensure_node(id)?;
        if !node.opcode().is_type() {
            return Err(Obstruction::ExpectedType { node: id });
        }
        Ok(node)
    }

    pub(crate) fn ensure_context(&self, id: ContextId) -> Result<&ContextDelta, Obstruction> {
        self.contexts
            .get(id as usize)
            .ok_or(Obstruction::UnknownContext { context: id })
    }

    fn insert_committed_node(
        &mut self,
        route: Route,
        context: ContextId,
        ty: NodeId,
        opcode: OpCode,
        payload: u64,
        ports: &[Port],
    ) -> Result<NodeId, Obstruction> {
        if self.routes.contains_key(&route) {
            return Err(Obstruction::RouteOccupied);
        }
        self.ensure_context(context)?;
        for port in ports {
            self.ensure_node(port.target())?;
        }
        if ty != META_TYPE {
            self.ensure_type(ty)?;
        }

        let before = self.commitment;
        let node = self.nodes.len() as NodeId;
        let first_port = self.ports.len() as u32;
        self.ports.extend_from_slice(ports);
        self.nodes.push(Node {
            payload,
            context,
            ty,
            first_port,
            opcode: opcode as u16,
            port_count: ports.len() as u16,
        });
        self.routes.insert(route, node);
        let after = crate::codec::state_commitment(self);
        self.push_event(opcode, node, before, after);
        self.commitment = after;
        Ok(node)
    }

    fn push_event(
        &mut self,
        operation: OpCode,
        subject: NodeId,
        before: [u8; 32],
        after: [u8; 32],
    ) {
        let parent = self
            .journal
            .len()
            .checked_sub(1)
            .map(|value| value as EventId)
            .unwrap_or(NO_EVENT);
        self.journal.push(JournalEvent {
            operation,
            subject,
            before,
            after,
            parent,
        });
    }

    pub(crate) fn nodes_raw(&self) -> &[Node] {
        &self.nodes
    }

    pub(crate) fn ports_raw(&self) -> &[Port] {
        &self.ports
    }

    pub(crate) fn contexts_raw(&self) -> &[ContextDelta] {
        &self.contexts
    }

    pub(crate) fn routes_raw(&self) -> &BTreeMap<Route, NodeId> {
        &self.routes
    }
}

impl ContextDelta {
    fn root() -> Self {
        Self {
            parent: ROOT_CONTEXT,
            binding: NO_NODE,
        }
    }
}
