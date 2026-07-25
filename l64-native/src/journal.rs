use crate::{EventId, NodeId, OpCode, SymbolicSeal};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JournalEvent {
    pub(crate) operation: OpCode,
    pub(crate) subject: NodeId,
    pub(crate) before: SymbolicSeal,
    pub(crate) after: SymbolicSeal,
    pub(crate) parent: EventId,
}

impl JournalEvent {
    pub fn operation(&self) -> OpCode {
        self.operation
    }

    pub fn subject(&self) -> NodeId {
        self.subject
    }

    pub fn before(&self) -> SymbolicSeal {
        self.before
    }

    pub fn after(&self) -> SymbolicSeal {
        self.after
    }

    pub fn parent(&self) -> Option<EventId> {
        (self.parent != u32::MAX).then_some(self.parent)
    }
}
