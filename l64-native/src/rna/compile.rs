pub fn compile_rna(source: &[u8]) -> Result<Graph, RnaError> {
    if source.len() > MAX_NATIVE_RNA_BYTES {
        return Err(RnaError::SourceTooLarge);
    }

    let mut lines = source.split(|byte| *byte == b'\n').enumerate();
    let (header_line, header) = next_nonempty(&mut lines).ok_or(RnaError::Empty)?;
    let header_tokens = tokens(header);
    if header_tokens.len() != 2 || header_tokens[0] != RNA_MAGIC {
        return Err(RnaError::BadHeader);
    }
    let domain = parse_u64(header_tokens[1]).ok_or(RnaError::InvalidNumber {
        line: line_number(header_line),
    })?;

    let root = Route::root(LocusWord(domain));
    let mut graph = Graph::new();
    let mut slots = BTreeMap::new();
    let mut last_slot = None;
    let mut instruction_count = 0usize;

    for (line_index, raw_line) in lines {
        let source_line = trim_ascii(raw_line);
        if source_line.is_empty() {
            continue;
        }
        let parts = tokens(source_line);
        let line = line_number(line_index);
        let instruction = parts
            .first()
            .copied()
            .ok_or(RnaError::InvalidArity { line })?;

        if instruction == b"h" {
            if parts.len() != 4 {
                return Err(RnaError::InvalidArity { line });
            }
            let context = parse_u32_part(&parts, 1, line)?;
            if context as usize != graph.context_count() {
                return Err(RnaError::ContextOrder { line, context });
            }
            let parent = parse_u32_part(&parts, 2, line)?;
            let binding = resolve_part(&slots, &parts, 3, line)?;
            let created = graph.extend_context(parent, binding)?;
            if created != context {
                return Err(RnaError::ContextOrder { line, context });
            }
            continue;
        }

        let slot = parse_part(&parts, 1, line)?;
        if let Some(previous) = last_slot
            && slot <= previous
        {
            return Err(RnaError::SlotOrder { line, slot });
        }
        let route = root.composed(LocusWord(slot));

        let node = match instruction {
            b"a" if parts.len() == 3 => {
                graph.declare_atom_type(route, LocusWord(parse_part(&parts, 2, line)?))?
            }
            b"m" if parts.len() == 5 => {
                let element = resolve_part(&slots, &parts, 2, line)?;
                let rows = parse_u32_part(&parts, 3, line)?;
                let cols = parse_u32_part(&parts, 4, line)?;
                graph.declare_matrix_type(route, element, rows, cols)?
            }
            b"f" if parts.len() == 4 => {
                let domain = resolve_part(&slots, &parts, 2, line)?;
                let codomain = resolve_part(&slots, &parts, 3, line)?;
                graph.declare_function_type(route, domain, codomain)?
            }
            b"q" if parts.len() == 10 => {
                let carrier = resolve_part(&slots, &parts, 2, line)?;
                let mut exponents = [0_i8; 7];
                for (axis, exponent) in exponents.iter_mut().enumerate() {
                    *exponent = parse_i8_part(&parts, axis + 3, line)?;
                }
                graph.declare_quantity_type(route, carrier, Dimension::new(exponents))?
            }
            b"v" if matches!(parts.len(), 3 | 4) => {
                let ty = resolve_part(&slots, &parts, 2, line)?;
                let context = parse_optional_context(&parts, 3, line)?;
                graph.insert_value(route, context, ty)?
            }
            b"k" if matches!(parts.len(), 5 | 6) => {
                let subject = resolve_part(&slots, &parts, 2, line)?;
                let kind_raw = parse_u8_part(&parts, 3, line)?;
                let kind =
                    ConstraintKind::from_raw(kind_raw).ok_or(RnaError::InvalidNumber { line })?;
                let holds = parse_bool_part(&parts, 4, line)?;
                let context = parse_optional_context(&parts, 5, line)?;
                graph.declare_constraint(route, context, subject, kind, holds)?
            }
            b"c" | b"x" | b"+" | b"*" | b"/" if matches!(parts.len(), 5 | 6) => {
                let left = resolve_part(&slots, &parts, 2, line)?;
                let right = resolve_part(&slots, &parts, 3, line)?;
                let output = resolve_part(&slots, &parts, 4, line)?;
                let context = parse_optional_context(&parts, 5, line)?;
                let proposal = match instruction {
                    b"c" => Proposal::compose(route, context, left, right, output),
                    b"x" => Proposal::matrix_multiply(route, context, left, right, output),
                    b"+" => Proposal::add(route, context, left, right, output),
                    b"*" => Proposal::multiply(route, context, left, right, output),
                    b"/" => Proposal::divide(route, context, left, right, output),
                    _ => unreachable!(),
                };
                graph.transact(proposal)?.node
            }
            b"r" if matches!(parts.len(), 4 | 5) => {
                let input = resolve_part(&slots, &parts, 2, line)?;
                let output = resolve_part(&slots, &parts, 3, line)?;
                let context = parse_optional_context(&parts, 4, line)?;
                graph
                    .transact(Proposal::square_root(route, context, input, output))?
                    .node
            }
            b"e" if parts.len() >= 6 => {
                let rule_raw = parse_u8_part(&parts, 2, line)?;
                let rule = EqualityRule::from_raw(rule_raw)
                    .ok_or(RnaError::InvalidNumber { line })?;
                let left = resolve_part(&slots, &parts, 3, line)?;
                let right = resolve_part(&slots, &parts, 4, line)?;
                let context = parse_u32_part(&parts, 5, line)?;
                let premises = parts[6..]
                    .iter()
                    .map(|part| {
                        let slot = parse_u64(part).ok_or(RnaError::InvalidNumber { line })?;
                        slots
                            .get(&slot)
                            .copied()
                            .ok_or(RnaError::UnknownSlot { line, slot })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                graph
                    .prove_equality(route, context, left, right, rule, &premises)?
                    .node
            }
            b"a" | b"m" | b"f" | b"q" | b"v" | b"k" | b"c" | b"x" | b"+" | b"*" | b"/" | b"r" | b"e" => {
                return Err(RnaError::InvalidArity { line });
            }
            _ => return Err(RnaError::UnknownInstruction { line }),
        };

        slots.insert(slot, node);
        last_slot = Some(slot);
        instruction_count += 1;
    }

    if instruction_count == 0 {
        return Err(RnaError::Empty);
    }
    Ok(graph)
}
