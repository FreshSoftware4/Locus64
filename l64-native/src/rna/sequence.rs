pub fn normalize_rna(source: &[u8]) -> Result<Vec<u8>, RnaError> {
    rna_bytes(&compile_rna(source)?)
}

pub fn rna_to_dna(source: &[u8]) -> Result<Vec<u8>, RnaError> {
    Ok(dna_bytes(&compile_rna(source)?)?)
}

pub fn dna_to_rna(source: &[u8]) -> Result<Vec<u8>, RnaError> {
    rna_bytes(&decode_dna(source)?)
}

pub fn rna_bytes(graph: &Graph) -> Result<Vec<u8>, RnaError> {
    let mut domain = None;
    let mut authored = BTreeMap::new();
    for (route, node) in graph.routes_raw() {
        if route.tail().len() != 1 {
            continue;
        }
        match domain {
            Some(existing) if existing != route.domain() => {
                return Err(RnaError::UnrepresentableGraph);
            }
            None => domain = Some(route.domain()),
            _ => {}
        }
        if authored.insert(*node, route.tail()[0].0).is_some() {
            return Err(RnaError::UnrepresentableGraph);
        }
    }

    let domain = domain.ok_or(RnaError::Empty)?;
    let expected_authored = graph
        .nodes_raw()
        .iter()
        .filter(|node| {
            !matches!(
                node.opcode(),
                OpCode::TypeJudgment | OpCode::KernelWitness | OpCode::Obligation
            )
        })
        .count();
    if authored.len() != expected_authored {
        return Err(RnaError::UnrepresentableGraph);
    }

    let mut slots = BTreeMap::new();
    let mut previous_slot = None;
    for (node, slot) in &authored {
        if previous_slot.is_some_and(|previous| *slot <= previous) {
            return Err(RnaError::UnrepresentableGraph);
        }
        slots.insert(*node, *slot);
        previous_slot = Some(*slot);
    }

    let mut out = Vec::with_capacity(32 + authored.len() * 32 + graph.context_count() * 16);
    out.extend_from_slice(RNA_MAGIC);
    out.push(b' ');
    push_hex(&mut out, domain.0);
    out.push(b'\n');

    let mut next_context = 1_u32;
    for (node_id, slot) in authored {
        emit_ready_contexts(graph, &slots, &mut out, &mut next_context, node_id)?;
        emit_node(graph, &slots, &mut out, node_id, slot)?;
    }
    emit_ready_contexts(graph, &slots, &mut out, &mut next_context, u32::MAX)?;
    if next_context as usize != graph.context_count() {
        return Err(RnaError::UnrepresentableGraph);
    }

    Ok(out)
}

fn emit_ready_contexts(
    graph: &Graph,
    slots: &BTreeMap<NodeId, u64>,
    out: &mut Vec<u8>,
    next_context: &mut ContextId,
    before_node: NodeId,
) -> Result<(), RnaError> {
    while (*next_context as usize) < graph.context_count() {
        let context = &graph.contexts_raw()[*next_context as usize];
        let binding = context.binding().ok_or(RnaError::UnrepresentableGraph)?;
        if binding >= before_node {
            break;
        }
        out.push(b'h');
        push_space_decimal(out, u64::from(*next_context));
        push_space_decimal(out, u64::from(context.parent()));
        push_space_decimal(out, slot_for(slots, binding)?);
        out.push(b'\n');
        *next_context += 1;
    }
    Ok(())
}

fn emit_node(
    graph: &Graph,
    slots: &BTreeMap<NodeId, u64>,
    out: &mut Vec<u8>,
    node_id: NodeId,
    slot: u64,
) -> Result<(), RnaError> {
    let node = graph.node(node_id).ok_or(RnaError::UnrepresentableGraph)?;
    let ports = graph.ports(node_id).ok_or(RnaError::UnrepresentableGraph)?;
    match node.opcode() {
        OpCode::TypeAtom if ports.is_empty() && node.ty().is_none() => {
            require_root(node.context())?;
            push_prefix(out, b'a', slot);
            push_space_hex(out, node.payload());
        }
        OpCode::TypeMatrix if ports.len() == 1 && node.ty().is_none() => {
            require_root(node.context())?;
            push_prefix(out, b'm', slot);
            push_space_decimal(out, slot_for(slots, ports[0].target())?);
            push_space_decimal(out, node.payload() >> 32);
            push_space_decimal(out, node.payload() as u32 as u64);
        }
        OpCode::TypeFunction if ports.len() == 2 && node.ty().is_none() => {
            require_root(node.context())?;
            push_prefix(out, b'f', slot);
            push_space_decimal(out, slot_for(slots, ports[0].target())?);
            push_space_decimal(out, slot_for(slots, ports[1].target())?);
        }
        OpCode::TypeQuantity if ports.len() == 1 && node.ty().is_none() => {
            require_root(node.context())?;
            push_prefix(out, b'q', slot);
            push_space_decimal(out, slot_for(slots, ports[0].target())?);
            let dimension =
                Dimension::from_bits(node.payload()).ok_or(RnaError::UnrepresentableGraph)?;
            for exponent in dimension.exponents() {
                push_space_signed(out, exponent);
            }
        }
        OpCode::Value if ports.is_empty() => {
            push_prefix(out, b'v', slot);
            push_space_decimal(
                out,
                slot_for(slots, node.ty().ok_or(RnaError::UnrepresentableGraph)?)?,
            );
            push_context(out, node.context());
        }
        OpCode::Constraint if ports.len() == 1 => {
            push_prefix(out, b'k', slot);
            push_space_decimal(out, slot_for(slots, ports[0].target())?);
            let (kind, holds) = ConstraintKind::from_payload(node.payload())
                .ok_or(RnaError::UnrepresentableGraph)?;
            push_space_decimal(out, kind as u64);
            push_space_decimal(out, u64::from(holds));
            push_context(out, node.context());
        }
        OpCode::Compose | OpCode::MatMul | OpCode::Add | OpCode::Multiply | OpCode::Divide
            if ports.len() == 2 =>
        {
            let opcode = match node.opcode() {
                OpCode::Compose => b'c',
                OpCode::MatMul => b'x',
                OpCode::Add => b'+',
                OpCode::Multiply => b'*',
                OpCode::Divide => b'/',
                _ => unreachable!(),
            };
            push_prefix(out, opcode, slot);
            push_space_decimal(out, slot_for(slots, ports[0].target())?);
            push_space_decimal(out, slot_for(slots, ports[1].target())?);
            push_space_decimal(
                out,
                slot_for(slots, node.ty().ok_or(RnaError::UnrepresentableGraph)?)?,
            );
            push_context(out, node.context());
        }
        OpCode::Sqrt if ports.len() == 1 => {
            push_prefix(out, b'r', slot);
            push_space_decimal(out, slot_for(slots, ports[0].target())?);
            push_space_decimal(
                out,
                slot_for(slots, node.ty().ok_or(RnaError::UnrepresentableGraph)?)?,
            );
            push_context(out, node.context());
        }
        _ => return Err(RnaError::UnrepresentableGraph),
    }
    out.push(b'\n');
    Ok(())
}
