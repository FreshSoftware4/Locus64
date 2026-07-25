pub fn canonical_bytes(graph: &Graph) -> Vec<u8> {
    let mut out =
        Vec::with_capacity(22 + graph.nodes_raw().len() * 24 + graph.ports_raw().len() * 8);
    out.extend_from_slice(b"L64N");
    out.extend_from_slice(&CODEC_VERSION.to_le_bytes());
    push_len(&mut out, graph.nodes_raw().len());
    push_len(&mut out, graph.ports_raw().len());
    push_len(&mut out, graph.contexts_raw().len());
    push_len(&mut out, graph.routes_raw().len());

    for node in graph.nodes_raw() {
        out.extend_from_slice(&(node.opcode() as u16).to_le_bytes());
        out.extend_from_slice(&node.context().to_le_bytes());
        out.extend_from_slice(&node.ty().unwrap_or(NO_NODE).to_le_bytes());
        out.extend_from_slice(&node.payload().to_le_bytes());
        let range = node.port_range();
        out.extend_from_slice(&(range.start as u32).to_le_bytes());
        out.extend_from_slice(&((range.end - range.start) as u16).to_le_bytes());
    }

    for port in graph.ports_raw() {
        out.extend_from_slice(&port.target().to_le_bytes());
        out.push(port.role() as u8);
        out.push(port.flags());
        out.extend_from_slice(&port.ordinal().to_le_bytes());
    }

    for context in graph.contexts_raw() {
        out.extend_from_slice(&context.parent().to_le_bytes());
        out.extend_from_slice(&context.binding().unwrap_or(NO_NODE).to_le_bytes());
    }

    for (route, node) in graph.routes_raw() {
        out.extend_from_slice(&route.domain().0.to_le_bytes());
        push_len(&mut out, route.tail().len());
        for word in route.tail() {
            out.extend_from_slice(&word.0.to_le_bytes());
        }
        out.extend_from_slice(&node.to_le_bytes());
    }

    out
}

pub fn decode_canonical(bytes: &[u8]) -> Result<Graph, DecodeError> {
    let mut reader = Reader::new(bytes);
    if reader.take(4)? != b"L64N" {
        return Err(DecodeError::BadMagic);
    }
    let version = reader.u16()?;
    if version != CODEC_VERSION {
        return Err(DecodeError::UnsupportedVersion { version });
    }

    let node_count = reader.count(MAX_NODES)?;
    let port_count = reader.count(MAX_PORTS)?;
    let context_count = reader.count(MAX_CONTEXTS)?;
    let route_count = reader.count(MAX_ROUTES)?;
    if context_count == 0 || route_count != node_count {
        return Err(DecodeError::InvalidContext);
    }

    reader.require(node_count, 24)?;
    let mut nodes = Vec::with_capacity(node_count);
    for _ in 0..node_count {
        let raw_opcode = reader.u16()?;
        let opcode = OpCode::from_raw(raw_opcode)
            .filter(|opcode| opcode.is_persisted_node())
            .ok_or(DecodeError::UnknownOpcode { opcode: raw_opcode })?;
        let context = reader.u32()?;
        let ty = reader.u32()?;
        let payload = reader.u64()?;
        let first_port = reader.u32()?;
        let port_count = reader.u16()?;
        nodes.push(Node::from_raw(
            payload, context, ty, first_port, opcode, port_count,
        ));
    }

    reader.require(port_count, 8)?;
    let mut ports = Vec::with_capacity(port_count);
    for _ in 0..port_count {
        let target = reader.u32()?;
        let raw_role = reader.u8()?;
        let role =
            PortRole::from_raw(raw_role).ok_or(DecodeError::UnknownPortRole { role: raw_role })?;
        let flags = reader.u8()?;
        if flags != 0 {
            return Err(DecodeError::NonZeroPortFlags { flags });
        }
        let ordinal = reader.u16()?;
        ports.push(Port::from_raw(target, role, flags, ordinal));
    }

    reader.require(context_count, 8)?;
    let mut contexts = Vec::with_capacity(context_count);
    for _ in 0..context_count {
        contexts.push(ContextDelta::from_raw(reader.u32()?, reader.u32()?));
    }

    let mut routes = BTreeMap::new();
    for _ in 0..route_count {
        let domain = LocusWord(reader.u64()?);
        let tail_len = reader.count(MAX_ROUTE_WORDS)?;
        reader.require(tail_len, 8)?;
        let mut tail = Vec::with_capacity(tail_len);
        for _ in 0..tail_len {
            tail.push(LocusWord(reader.u64()?));
        }
        let node = reader.u32()?;
        let route = Route::from_parts(domain, tail.into_boxed_slice());
        if routes.insert(route, node).is_some() {
            return Err(DecodeError::DuplicateRoute);
        }
    }

    if !reader.is_empty() {
        return Err(DecodeError::TrailingBytes);
    }

    validate_structure(&nodes, &ports, &contexts, &routes)?;
    let graph = Graph::from_decoded_parts(nodes, ports, contexts, routes);
    graph
        .validate_decoded_authority()
        .map_err(|_| DecodeError::InvalidAuthority)?;
    if canonical_bytes(&graph) != bytes {
        return Err(DecodeError::NonCanonical);
    }
    Ok(graph)
}
