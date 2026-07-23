use crate::{EventId, NodeId, OpCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JournalEvent {
    pub(crate) operation: OpCode,
    pub(crate) subject: NodeId,
    pub(crate) before: [u8; 32],
    pub(crate) after: [u8; 32],
    pub(crate) parent: EventId,
}

impl JournalEvent {
    pub fn operation(&self) -> OpCode {
        self.operation
    }

    pub fn subject(&self) -> NodeId {
        self.subject
    }

    pub fn before(&self) -> [u8; 32] {
        self.before
    }

    pub fn after(&self) -> [u8; 32] {
        self.after
    }

    pub fn parent(&self) -> Option<EventId> {
        (self.parent != u32::MAX).then_some(self.parent)
    }
}
