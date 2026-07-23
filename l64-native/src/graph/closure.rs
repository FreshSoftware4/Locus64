impl Graph {
    pub fn closure_state(
        &self,
        context: ContextId,
        node: NodeId,
    ) -> Result<ClosureState, Obstruction> {
        self.ensure_context(context)?;
        self.ensure_node(node)?;
        let mut memo = vec![None; self.node_count()];
        self.closure_state_inner(context, node, &mut memo)
    }

    pub fn context_closure(&self, context: ContextId) -> Result<ClosureState, Obstruction> {
        let nodes = self.visible_nodes(context)?;
        let mut memo = vec![None; self.node_count()];
        let mut state = ClosureState::Closed;
        for node in nodes {
            state = state.combine(self.closure_state_inner(context, node, &mut memo)?);
            if state == ClosureState::Invalid {
                break;
            }
        }
        Ok(state)
    }

    pub fn global_closure(&self) -> Result<ClosureState, Obstruction> {
        let mut state = ClosureState::Closed;
        for context in 0..self.context_count() as ContextId {
            state = state.combine(self.context_closure(context)?);
            if state == ClosureState::Invalid {
                break;
            }
        }
        Ok(state)
    }

    pub fn closure_transitions(
        &self,
        from_context: ContextId,
        to_context: ContextId,
    ) -> Result<Vec<ClosureTransition>, Obstruction> {
        self.ensure_context(from_context)?;
        let refinement = self.ensure_context(to_context)?;
        if refinement.parent() != from_context {
            return Err(Obstruction::ContextNotDirectRefinement {
                parent: from_context,
                child: to_context,
            });
        }
        let cause = refinement.binding().ok_or(Obstruction::UnknownContext {
            context: to_context,
        })?;
        let (subject, _, _) = self.constraint_parts(cause).map_err(|_| {
            Obstruction::RefinementBindingNotConstraint {
                context: to_context,
                binding: cause,
            }
        })?;
        let affected = self.affected_nodes(subject)?;
        let mut before_memo = vec![None; self.node_count()];
        let mut after_memo = vec![None; self.node_count()];
        let mut transitions = Vec::new();
        for node in affected {
            let record = self.ensure_node(node)?;
            if !self.context_visible(record.context(), from_context) {
                continue;
            }
            let before = self.closure_state_inner(from_context, node, &mut before_memo)?;
            let after = self.closure_state_inner(to_context, node, &mut after_memo)?;
            if before != after {
                transitions.push(ClosureTransition::new(
                    node,
                    before,
                    after,
                    cause,
                    from_context,
                    to_context,
                ));
            }
        }
        Ok(transitions)
    }

    fn closure_state_inner(
        &self,
        context: ContextId,
        node: NodeId,
        memo: &mut [Option<ClosureState>],
    ) -> Result<ClosureState, Obstruction> {
        if let Some(state) = memo[node as usize] {
            return Ok(state);
        }
        let record = self.ensure_node(node)?;
        if !self.context_visible(record.context(), context) {
            memo[node as usize] = Some(ClosureState::Invalid);
            return Ok(ClosureState::Invalid);
        }

        let state = match record.opcode() {
            opcode if opcode.is_executable() => {
                self.executable_closure_state(context, node, memo)?
            }
            OpCode::TypeJudgment => {
                let subject = self
                    .ports(node)
                    .and_then(|ports| ports.first())
                    .map(Port::target)
                    .ok_or(Obstruction::MalformedEvidence { node })?;
                self.closure_state_inner(context, subject, memo)?
            }
            OpCode::KernelWitness | OpCode::Obligation => {
                let judgment = record
                    .ty()
                    .ok_or(Obstruction::MalformedEvidence { node })?;
                self.closure_state_inner(context, judgment, memo)?
            }
            OpCode::TypeEquality => self.equality_closure_state(context, node, memo)?,
            OpCode::EqualityWitness => {
                let judgment = record
                    .ty()
                    .ok_or(Obstruction::MalformedEquality { node })?;
                self.closure_state_inner(context, judgment, memo)?
            }
            _ => ClosureState::Closed,
        };
        memo[node as usize] = Some(state);
        Ok(state)
    }

    fn executable_closure_state(
        &self,
        context: ContextId,
        node: NodeId,
        memo: &mut [Option<ClosureState>],
    ) -> Result<ClosureState, Obstruction> {
        let record = self.ensure_node(node)?;
        let ports = self
            .ports(node)
            .ok_or(Obstruction::UnknownNode { node })?;
        let inputs = ports.iter().map(Port::target).collect::<Vec<_>>();
        let mut state = ClosureState::Closed;
        for input in &inputs {
            state = state.combine(self.closure_state_inner(context, *input, memo)?);
        }
        if state == ClosureState::Invalid {
            return Ok(state);
        }
        let output_type = record.ty().ok_or(Obstruction::ExpectedType { node })?;
        let own = match self.validate_operation(context, record.opcode(), &inputs, output_type) {
            Ok(EvidencePlan::Witness) => ClosureState::Closed,
            Ok(EvidencePlan::Obligation { .. }) => ClosureState::Open,
            Err(_) => ClosureState::Invalid,
        };
        Ok(state.combine(own))
    }

    fn equality_closure_state(
        &self,
        context: ContextId,
        judgment: NodeId,
        memo: &mut [Option<ClosureState>],
    ) -> Result<ClosureState, Obstruction> {
        let (left, right) = self.equality_parts(judgment)?;
        let mut state = self
            .closure_state_inner(context, left, memo)?
            .combine(self.closure_state_inner(context, right, memo)?);
        if state == ClosureState::Invalid {
            return Ok(state);
        }
        let witness = self
            .equality_witness_for(judgment)
            .ok_or(Obstruction::MalformedEquality { node: judgment })?;
        let witness_node = self.ensure_node(witness)?;
        let rule = EqualityRule::from_raw(witness_node.payload() as u8)
            .ok_or(Obstruction::MalformedEquality { node: witness })?;
        let premise_ports = self
            .ports(witness)
            .ok_or(Obstruction::MalformedEquality { node: witness })?;
        let premises = premise_ports.iter().map(Port::target).collect::<Vec<_>>();
        for premise in &premises {
            state = state.combine(self.closure_state_inner(context, *premise, memo)?);
        }
        if state == ClosureState::Invalid {
            return Ok(state);
        }
        match self.validate_equality_rule(context, left, right, rule, &premises) {
            Ok(()) => Ok(state),
            Err(_) => Ok(ClosureState::Invalid),
        }
    }
}
