impl ContextDelta {
    fn root() -> Self {
        Self {
            parent: ROOT_CONTEXT,
            binding: NO_NODE,
        }
    }
}
