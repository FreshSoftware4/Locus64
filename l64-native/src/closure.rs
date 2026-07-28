use crate::{ContextId, NodeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClosureState {
    Closed,
    Open,
    Invalid,
}

impl ClosureState {
    pub(crate) fn combine(self, other: Self) -> Self {
        self.max(other)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClosureTransition {
    node: NodeId,
    before: ClosureState,
    after: ClosureState,
    cause: NodeId,
    from_context: ContextId,
    to_context: ContextId,
}

impl ClosureTransition {
    pub(crate) fn new(
        node: NodeId,
        before: ClosureState,
        after: ClosureState,
        cause: NodeId,
        from_context: ContextId,
        to_context: ContextId,
    ) -> Self {
        Self {
            node,
            before,
            after,
            cause,
            from_context,
            to_context,
        }
    }

    pub fn node(&self) -> NodeId {
        self.node
    }
    pub fn before(&self) -> ClosureState {
        self.before
    }
    pub fn after(&self) -> ClosureState {
        self.after
    }
    pub fn cause(&self) -> NodeId {
        self.cause
    }
    pub fn from_context(&self) -> ContextId {
        self.from_context
    }
    pub fn to_context(&self) -> ContextId {
        self.to_context
    }
}
