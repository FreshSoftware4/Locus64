impl Graph {
    pub(crate) fn validate_context_consistency(&self) -> Result<(), Obstruction> {
        for context in 1..self.contexts.len() as ContextId {
            let delta = self.ensure_context(context)?;
            let binding = delta
                .binding()
                .ok_or(Obstruction::UnknownContext { context })?;
            self.validate_context_binding(delta.parent(), binding)?;
        }
        Ok(())
    }

    fn validate_context_binding(
        &self,
        parent: ContextId,
        binding: NodeId,
    ) -> Result<(), Obstruction> {
        let node = self.ensure_node(binding)?;
        if node.opcode() != OpCode::Constraint {
            return Ok(());
        }
        let (subject, kind, holds) = self.constraint_parts(binding)?;
        let inherited = self.constraint_state(parent, subject, kind)?;
        if matches!(
            (inherited, holds),
            (ConstraintState::Proven, false) | (ConstraintState::Refuted, true)
        ) {
            return Err(Obstruction::ContradictoryConstraint { subject, kind });
        }
        Ok(())
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
        self.routes.insert(route.clone(), node);
        self.register_derived_node(node);
        self.register_derived_route(&route, node);
        self.record_mutation(opcode, node);
        Ok(node)
    }

    fn push_event(
        &mut self,
        operation: OpCode,
        subject: NodeId,
        before: SymbolicSeal,
        after: SymbolicSeal,
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
