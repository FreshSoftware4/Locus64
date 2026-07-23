use std::collections::BTreeMap;

use crate::kernel::{ConstraintState, EqualityRule, EvidencePlan};
use crate::{
    ConstraintKind, ContextDelta, Dimension, JournalEvent, LocusWord, Obstruction, OpCode, Port,
    PortRole, Route,
};

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
    pub(crate) fn from_raw(
        payload: u64,
        context: ContextId,
        ty: NodeId,
        first_port: u32,
        opcode: OpCode,
        port_count: u16,
    ) -> Self {
        Self {
            payload,
            context,
            ty,
            first_port,
            opcode: opcode as u16,
            port_count,
        }
    }

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

include!("graph/construction.rs");
include!("graph/typing.rs");
include!("graph/storage.rs");
include!("graph/context.rs");
