fn validate_structure(
    nodes: &[Node],
    ports: &[Port],
    contexts: &[ContextDelta],
    routes: &BTreeMap<Route, NodeId>,
) -> Result<(), DecodeError> {
    let root = contexts.first().ok_or(DecodeError::InvalidContext)?;
    if root.parent() != 0 || root.binding().is_some() {
        return Err(DecodeError::InvalidContext);
    }
    for (index, context) in contexts.iter().enumerate().skip(1) {
        if context.parent() as usize >= index {
            return Err(DecodeError::InvalidContext);
        }
        let binding = context.binding().ok_or(DecodeError::InvalidContext)?;
        if binding as usize >= nodes.len() {
            return Err(DecodeError::InvalidContext);
        }
    }

    let mut port_cursor = 0usize;
    for (index, node) in nodes.iter().enumerate() {
        if node.context() as usize >= contexts.len() {
            return Err(DecodeError::InvalidNode);
        }
        if let Some(ty) = node.ty() {
            if ty as usize >= index || !nodes[ty as usize].opcode().is_type() {
                return Err(DecodeError::InvalidNode);
            }
        } else if !node.opcode().is_untyped_authority() {
            return Err(DecodeError::InvalidNode);
        }

        let range = node.port_range();
        if range.start != port_cursor || range.end > ports.len() {
            return Err(DecodeError::InvalidPortRange);
        }
        let node_ports = &ports[range.clone()];
        for (ordinal, port) in node_ports.iter().enumerate() {
            if port.target() as usize >= index {
                return Err(DecodeError::InvalidPortTarget);
            }
            if port.ordinal() as usize != ordinal {
                return Err(DecodeError::InvalidPortLaw);
            }
        }
        validate_port_law(node.opcode(), node_ports, node.ty().is_some())?;
        validate_node_semantics(index, node, node_ports, nodes, ports)?;
        port_cursor = range.end;
    }
    if port_cursor != ports.len() {
        return Err(DecodeError::InvalidPortRange);
    }

    let mut covered = vec![false; nodes.len()];
    for node in routes.values().copied() {
        let slot = covered
            .get_mut(node as usize)
            .ok_or(DecodeError::InvalidRoute)?;
        if *slot {
            return Err(DecodeError::InvalidRoute);
        }
        *slot = true;
    }
    if covered.iter().any(|covered| !covered) {
        return Err(DecodeError::InvalidRoute);
    }
    validate_evidence_routes(nodes, routes)?;
    Ok(())
}

fn validate_node_semantics(
    index: usize,
    node: &Node,
    node_ports: &[Port],
    nodes: &[Node],
    ports: &[Port],
) -> Result<(), DecodeError> {
    match node.opcode() {
        OpCode::Value => {
            let ty = node.ty().ok_or(DecodeError::InvalidNode)?;
            if nodes[ty as usize].opcode() == OpCode::TypeJudgment {
                return Err(DecodeError::InvalidNode);
            }
        }
        OpCode::KernelWitness | OpCode::Obligation => {
            let ty = node.ty().ok_or(DecodeError::InvalidNode)?;
            if nodes[ty as usize].opcode() != OpCode::TypeJudgment {
                return Err(DecodeError::InvalidNode);
            }
            if node.opcode() == OpCode::Obligation
                && crate::ConstraintKind::from_payload(node.payload()).is_none()
            {
                return Err(DecodeError::InvalidNode);
            }
        }
        OpCode::TypeQuantity => {
            if crate::Dimension::from_bits(node.payload()).is_none() {
                return Err(DecodeError::InvalidNode);
            }
        }
        OpCode::Constraint => {
            if crate::ConstraintKind::from_payload(node.payload()).is_none() {
                return Err(DecodeError::InvalidNode);
            }
        }
        OpCode::TypeJudgment => {
            let rule = OpCode::from_raw(node.payload() as u16)
                .filter(|opcode| opcode.is_executable())
                .ok_or(DecodeError::InvalidNode)?;
            if node_ports.len() < 3 {
                return Err(DecodeError::InvalidNode);
            }
            let subject = node_ports[0].target() as usize;
            if subject >= index || nodes[subject].opcode() != rule {
                return Err(DecodeError::InvalidNode);
            }
            let subject_node = &nodes[subject];
            let conclusion = node_ports.last().ok_or(DecodeError::InvalidNode)?.target();
            if subject_node.ty() != Some(conclusion) {
                return Err(DecodeError::InvalidNode);
            }
            let subject_ports = ports
                .get(subject_node.port_range())
                .ok_or(DecodeError::InvalidPortRange)?;
            if subject_ports.len() != node_ports.len() - 2
                || subject_ports
                    .iter()
                    .map(Port::target)
                    .ne(node_ports[1..node_ports.len() - 1].iter().map(Port::target))
            {
                return Err(DecodeError::InvalidNode);
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_evidence_routes(
    nodes: &[Node],
    routes: &BTreeMap<Route, NodeId>,
) -> Result<(), DecodeError> {
    let mut attached = vec![false; nodes.len()];
    for (route, node_id) in routes {
        let node = &nodes[*node_id as usize];
        if !node.opcode().is_executable() {
            continue;
        }
        let judgment = *routes
            .get(&route.composed(JUDGMENT_LOCUS))
            .ok_or(DecodeError::InvalidRoute)?;
        let witness = *routes
            .get(&route.composed(EVIDENCE_LOCUS))
            .ok_or(DecodeError::InvalidRoute)?;
        if nodes[judgment as usize].opcode() != OpCode::TypeJudgment
            || !matches!(
                nodes[witness as usize].opcode(),
                OpCode::KernelWitness | OpCode::Obligation
            )
            || nodes[witness as usize].ty() != Some(judgment)
        {
            return Err(DecodeError::InvalidRoute);
        }
        attached[judgment as usize] = true;
        attached[witness as usize] = true;
    }
    for (index, node) in nodes.iter().enumerate() {
        if matches!(
            node.opcode(),
            OpCode::TypeJudgment | OpCode::KernelWitness | OpCode::Obligation
        ) && !attached[index]
        {
            return Err(DecodeError::InvalidRoute);
        }
    }
    Ok(())
}

fn validate_port_law(opcode: OpCode, ports: &[Port], has_type: bool) -> Result<(), DecodeError> {
    let valid = match opcode {
        OpCode::TypeAtom => ports.is_empty() && !has_type,
        OpCode::TypeMatrix => {
            ports.len() == 1 && ports[0].role() == PortRole::Parameter && !has_type
        }
        OpCode::TypeFunction => {
            ports.len() == 2
                && ports[0].role() == PortRole::Domain
                && ports[1].role() == PortRole::Codomain
                && !has_type
        }
        OpCode::TypeQuantity => {
            ports.len() == 1 && ports[0].role() == PortRole::Parameter && !has_type
        }
        OpCode::Constraint => ports.len() == 1 && ports[0].role() == PortRole::Subject && !has_type,
        OpCode::Value => ports.is_empty() && has_type,
        OpCode::Compose | OpCode::MatMul | OpCode::Add | OpCode::Multiply | OpCode::Divide => {
            ports.len() == 2
                && ports.iter().all(|port| port.role() == PortRole::Argument)
                && has_type
        }
        OpCode::Sqrt => ports.len() == 1 && ports[0].role() == PortRole::Argument && has_type,
        OpCode::TypeJudgment => {
            ports.len() >= 3
                && ports[0].role() == PortRole::Subject
                && ports[1..ports.len() - 1]
                    .iter()
                    .all(|port| port.role() == PortRole::Premise)
                && ports
                    .last()
                    .is_some_and(|port| port.role() == PortRole::Conclusion)
                && !has_type
        }
        OpCode::KernelWitness => ports.is_empty() && has_type,
        OpCode::Obligation => ports.len() == 1 && ports[0].role() == PortRole::Premise && has_type,
        OpCode::ExtendContext => false,
    };
    valid.then_some(()).ok_or(DecodeError::InvalidPortLaw)
}
