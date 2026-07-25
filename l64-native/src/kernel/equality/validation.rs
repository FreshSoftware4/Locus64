impl Graph {
    pub(crate) fn validate_equality_authority(&self) -> Result<(), Obstruction> {
        let mut attached = vec![false; self.node_count()];
        for (route, judgment) in self.routes_raw() {
            let node = self
                .node(*judgment)
                .ok_or(Obstruction::UnknownNode { node: *judgment })?;
            if node.opcode() != OpCode::TypeEquality {
                continue;
            }
            let evidence = self
                .resolve(&route.composed(EVIDENCE_LOCUS))
                .ok_or(Obstruction::MalformedEquality { node: *judgment })?;
            let evidence_node = self
                .node(evidence)
                .ok_or(Obstruction::MalformedEquality { node: evidence })?;
            let evidence_ports = self
                .ports(evidence)
                .ok_or(Obstruction::MalformedEquality { node: evidence })?;
            let rule = EqualityRule::from_raw(evidence_node.payload() as u8)
                .filter(|_| evidence_node.payload() <= u8::MAX as u64)
                .ok_or(Obstruction::MalformedEquality { node: evidence })?;
            if evidence_node.opcode() != OpCode::EqualityWitness
                || evidence_node.ty() != Some(*judgment)
                || evidence_node.context() != node.context()
                || evidence_ports
                    .iter()
                    .any(|port| port.role() != PortRole::Premise)
            {
                return Err(Obstruction::MalformedEquality { node: evidence });
            }
            let premises = evidence_ports.iter().map(Port::target).collect::<Vec<_>>();
            let (left, right) = self.equality_parts(*judgment)?;
            self.validate_equality_rule(node.context(), left, right, rule, &premises)?;
            attached[*judgment as usize] = true;
            attached[evidence as usize] = true;
        }
        for (index, node) in self.nodes_raw().iter().enumerate() {
            if matches!(node.opcode(), OpCode::TypeEquality | OpCode::EqualityWitness)
                && !attached[index]
            {
                return Err(Obstruction::MalformedEquality {
                    node: index as NodeId,
                });
            }
        }
        Ok(())
    }

    pub(crate) fn validate_equality_rule(
        &self,
        context: ContextId,
        left: NodeId,
        right: NodeId,
        rule: EqualityRule,
        premises: &[NodeId],
    ) -> Result<(), Obstruction> {
        self.ensure_context(context)?;
        let left_node = self.ensure_node(left)?;
        let right_node = self.ensure_node(right)?;
        if !self.context_visible(left_node.context(), context)
            || !self.context_visible(right_node.context(), context)
        {
            return Err(Obstruction::EqualityContextEscape {
                premise: if !self.context_visible(left_node.context(), context) {
                    left
                } else {
                    right
                },
                context,
            });
        }
        if !self.equality_endpoint_allowed(left_node)
            || !self.equality_endpoint_allowed(right_node)
            || !self.same_equality_sort(left_node, right_node)
        {
            return Err(Obstruction::EqualitySortMismatch { left, right });
        }

        let premise_pairs = premises
            .iter()
            .map(|premise| self.checked_premise(context, *premise))
            .collect::<Result<Vec<_>, _>>()?;

        match rule {
            EqualityRule::Reflexive => {
                if left != right || !premises.is_empty() {
                    return Err(Obstruction::InvalidEqualityRule);
                }
            }
            EqualityRule::Structural => {
                if !premises.is_empty() || !self.structurally_equal(left, right)? {
                    return Err(Obstruction::InvalidEqualityRule);
                }
            }
            EqualityRule::Symmetry => {
                if premise_pairs.as_slice() != [(right, left)] {
                    return Err(Obstruction::InvalidEqualityRule);
                }
            }
            EqualityRule::Transitive => {
                if premise_pairs.len() != 2
                    || premise_pairs[0].0 != left
                    || premise_pairs[0].1 != premise_pairs[1].0
                    || premise_pairs[1].1 != right
                {
                    return Err(Obstruction::InvalidEqualityRule);
                }
            }
            EqualityRule::Congruence => {
                self.validate_congruence(left, right, &premise_pairs)?;
            }
        }
        Ok(())
    }

    fn validate_congruence(
        &self,
        left: NodeId,
        right: NodeId,
        premise_pairs: &[(NodeId, NodeId)],
    ) -> Result<(), Obstruction> {
        let left_node = self.ensure_node(left)?;
        let right_node = self.ensure_node(right)?;
        if !self.congruence_subject_allowed(left_node.opcode())
            || left_node.opcode() != right_node.opcode()
            || left_node.payload() != right_node.payload()
            || left_node.context() != right_node.context()
            || left_node.ty() != right_node.ty()
        {
            return Err(Obstruction::InvalidEqualityRule);
        }
        let left_ports = self
            .ports(left)
            .ok_or(Obstruction::MalformedEquality { node: left })?;
        let right_ports = self
            .ports(right)
            .ok_or(Obstruction::MalformedEquality { node: right })?;
        if left_ports.is_empty()
            || left_ports.len() != right_ports.len()
            || left_ports.len() != premise_pairs.len()
        {
            return Err(Obstruction::InvalidEqualityRule);
        }
        for ((left_port, right_port), pair) in left_ports
            .iter()
            .zip(right_ports)
            .zip(premise_pairs)
        {
            if left_port.role() != right_port.role()
                || left_port.flags() != right_port.flags()
                || left_port.ordinal() != right_port.ordinal()
                || *pair != (left_port.target(), right_port.target())
            {
                return Err(Obstruction::InvalidEqualityRule);
            }
        }
        Ok(())
    }

    fn checked_premise(
        &self,
        context: ContextId,
        premise: NodeId,
    ) -> Result<(NodeId, NodeId), Obstruction> {
        let node = self.ensure_node(premise)?;
        if node.opcode() != OpCode::TypeEquality {
            return Err(Obstruction::EqualityPremiseMismatch { premise });
        }
        if !self.context_visible(node.context(), context) {
            return Err(Obstruction::EqualityContextEscape { premise, context });
        }
        let witness = self
            .equality_witness_for(premise)
            .ok_or(Obstruction::EqualityPremiseMismatch { premise })?;
        let witness_node = self.ensure_node(witness)?;
        if witness_node.opcode() != OpCode::EqualityWitness
            || witness_node.ty() != Some(premise)
            || witness_node.context() != node.context()
        {
            return Err(Obstruction::EqualityPremiseMismatch { premise });
        }
        self.equality_parts(premise)
    }

    pub fn equality_witness_for(&self, judgment: NodeId) -> Option<NodeId> {
        let route = self.route_for_node(judgment)?;
        self.resolve(&route.composed(EVIDENCE_LOCUS))
    }

    pub fn route_for_node(&self, node: NodeId) -> Option<&Route> {
        self.routes_raw()
            .iter()
            .find_map(|(route, candidate)| (*candidate == node).then_some(route))
    }

    pub(crate) fn context_visible(&self, ancestor: ContextId, mut context: ContextId) -> bool {
        loop {
            if ancestor == context {
                return true;
            }
            if context == ROOT_CONTEXT {
                return false;
            }
            let Some(delta) = self.contexts_raw().get(context as usize) else {
                return false;
            };
            context = delta.parent();
        }
    }

    fn same_equality_sort(&self, left: &crate::Node, right: &crate::Node) -> bool {
        match (left.ty(), right.ty()) {
            (Some(left_ty), Some(right_ty)) => left_ty == right_ty,
            (None, None) => {
                self.equality_endpoint_allowed(left) && self.equality_endpoint_allowed(right)
            }
            _ => false,
        }
    }

    fn equality_endpoint_allowed(&self, node: &crate::Node) -> bool {
        matches!(
            node.opcode(),
            OpCode::TypeAtom
                | OpCode::TypeMatrix
                | OpCode::TypeFunction
                | OpCode::TypeQuantity
                | OpCode::Value
                | OpCode::Compose
                | OpCode::MatMul
                | OpCode::Add
                | OpCode::Multiply
                | OpCode::Divide
                | OpCode::Sqrt
        )
    }

    fn congruence_subject_allowed(&self, opcode: OpCode) -> bool {
        matches!(
            opcode,
            OpCode::TypeMatrix
                | OpCode::TypeFunction
                | OpCode::TypeQuantity
                | OpCode::Compose
                | OpCode::MatMul
                | OpCode::Add
                | OpCode::Multiply
                | OpCode::Divide
                | OpCode::Sqrt
        )
    }

    fn structurally_equal(&self, left: NodeId, right: NodeId) -> Result<bool, Obstruction> {
        let left_node = self.ensure_node(left)?;
        let right_node = self.ensure_node(right)?;
        if !matches!(
            left_node.opcode(),
            OpCode::TypeAtom
                | OpCode::TypeMatrix
                | OpCode::TypeFunction
                | OpCode::TypeQuantity
                | OpCode::Compose
                | OpCode::MatMul
                | OpCode::Add
                | OpCode::Multiply
                | OpCode::Divide
                | OpCode::Sqrt
        ) || left_node.opcode() != right_node.opcode()
            || left_node.payload() != right_node.payload()
            || left_node.context() != right_node.context()
            || left_node.ty() != right_node.ty()
        {
            return Ok(false);
        }
        let left_ports = self
            .ports(left)
            .ok_or(Obstruction::MalformedEquality { node: left })?;
        let right_ports = self
            .ports(right)
            .ok_or(Obstruction::MalformedEquality { node: right })?;
        Ok(left_ports.len() == right_ports.len()
            && left_ports.iter().zip(right_ports).all(|(left, right)| {
                left.target() == right.target()
                    && left.role() == right.role()
                    && left.flags() == right.flags()
                    && left.ordinal() == right.ordinal()
            }))
    }
}
