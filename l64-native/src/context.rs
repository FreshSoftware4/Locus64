use crate::{ContextId, NodeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ContextDelta {
    pub(crate) parent: ContextId,
    pub(crate) binding: NodeId,
}

impl ContextDelta {
    pub fn parent(&self) -> ContextId {
        self.parent
    }

    pub fn binding(&self) -> Option<NodeId> {
        (self.binding != u32::MAX).then_some(self.binding)
    }
}
