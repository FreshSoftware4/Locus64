impl Graph {
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

    pub(crate) fn quantity_parts(&self, ty: NodeId) -> Result<(NodeId, Dimension), Obstruction> {
        let node = self.ensure_type(ty)?;
        if node.opcode() != OpCode::TypeQuantity {
            return Err(Obstruction::ExpectedQuantityType { node: ty });
        }
        let ports = self
            .ports(ty)
            .ok_or(Obstruction::UnknownNode { node: ty })?;
        let dimension =
            Dimension::from_bits(node.payload()).ok_or(Obstruction::MalformedType { node: ty })?;
        if ports.len() != 1 {
            return Err(Obstruction::MalformedType { node: ty });
        }
        Ok((ports[0].target(), dimension))
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

    pub(crate) fn matches_quantity_type(
        &self,
        ty: NodeId,
        carrier: NodeId,
        dimension: Dimension,
    ) -> bool {
        self.quantity_parts(ty) == Ok((carrier, dimension))
    }

    pub(crate) fn constraint_parts(
        &self,
        node: NodeId,
    ) -> Result<(NodeId, ConstraintKind, bool), Obstruction> {
        let constraint = self.ensure_node(node)?;
        if constraint.opcode() != OpCode::Constraint {
            return Err(Obstruction::MalformedConstraint { node });
        }
        let ports = self
            .ports(node)
            .ok_or(Obstruction::MalformedConstraint { node })?;
        let (kind, holds) = ConstraintKind::from_payload(constraint.payload())
            .ok_or(Obstruction::MalformedConstraint { node })?;
        if ports.len() != 1 || ports[0].role() != PortRole::Subject {
            return Err(Obstruction::MalformedConstraint { node });
        }
        Ok((ports[0].target(), kind, holds))
    }

    pub(crate) fn constraint_state(
        &self,
        context: ContextId,
        subject: NodeId,
        kind: ConstraintKind,
    ) -> Result<ConstraintState, Obstruction> {
        let mut cursor = context;
        loop {
            let delta = self.ensure_context(cursor)?;
            if let Some(binding) = delta.binding()
                && self
                    .node(binding)
                    .is_some_and(|node| node.opcode() == OpCode::Constraint)
            {
                let (bound_subject, bound_kind, holds) = self.constraint_parts(binding)?;
                if bound_subject == subject && bound_kind == kind {
                    return Ok(if holds {
                        ConstraintState::Proven
                    } else {
                        ConstraintState::Refuted
                    });
                }
            }
            if cursor == ROOT_CONTEXT {
                return Ok(ConstraintState::Unknown);
            }
            cursor = delta.parent();
        }
    }
}
