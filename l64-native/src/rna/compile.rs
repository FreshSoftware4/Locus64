pub fn compile_rna(source: &[u8]) -> Result<Graph, RnaError> {
    if source.len() > MAX_NATIVE_RNA_BYTES {
        return Err(RnaError::SourceTooLarge {
            actual: source.len(),
            limit: MAX_NATIVE_RNA_BYTES,
        });
    }

    let mut lines = source.split(|byte| *byte == b'\n').enumerate();
    let (header_line, header_raw) = next_nonempty(&mut lines).ok_or(RnaError::Empty)?;
    let header_line_number = line_number(header_line);
    let header_tokens = tokens(header_raw);
    if header_tokens.len() != 2 || header_tokens[0].bytes != RNA_MAGIC {
        return Err(RnaError::BadHeader {
            span: line_span(header_line_number, header_raw),
        });
    }
    let domain_token = header_tokens[1];
    let domain = parse_u64(domain_token.bytes).ok_or(RnaError::InvalidNumber {
        span: domain_token.span(header_line_number),
    })?;

    let root = Route::root(LocusWord(domain));
    let mut graph = Graph::new_bulk();
    let mut slots = BTreeMap::new();
    let mut last_slot = None;
    let mut instruction_count = 0usize;

    for (line_index, raw_line) in lines {
        let parts = tokens(raw_line);
        if parts.is_empty() {
            continue;
        }
        let line = line_number(line_index);
        let instruction = parts[0];
        let instruction_span = instruction.span(line);

        if instruction.bytes == b"h" {
            if parts.len() != 4 {
                return Err(RnaError::InvalidArity {
                    span: line_span(line, raw_line),
                });
            }
            let context = parse_u32_part(&parts, 1, line, raw_line)?;
            if context as usize != graph.context_count() {
                return Err(RnaError::ContextOrder {
                    span: parts[1].span(line),
                    context,
                });
            }
            let parent = parse_u32_part(&parts, 2, line, raw_line)?;
            let binding = resolve_part(&slots, &parts, 3, line, raw_line)?;
            let created = graph_at(
                graph.extend_context(parent, binding),
                instruction_span,
            )?;
            if created != context {
                return Err(RnaError::ContextOrder {
                    span: parts[1].span(line),
                    context,
                });
            }
            continue;
        }

        if parts.len() < 2 {
            return Err(RnaError::InvalidArity {
                span: line_span(line, raw_line),
            });
        }
        let slot = parse_part(&parts, 1, line, raw_line)?;
        if let Some(previous) = last_slot
            && slot <= previous
        {
            return Err(RnaError::SlotOrder {
                span: parts[1].span(line),
                slot,
            });
        }
        let route = root.composed(LocusWord(slot));

        let node = match instruction.bytes {
            b"a" if parts.len() == 3 => graph_at(
                graph.declare_atom_type(
                    route,
                    LocusWord(parse_part(&parts, 2, line, raw_line)?),
                ),
                instruction_span,
            )?,
            b"m" if parts.len() == 5 => {
                let element = resolve_part(&slots, &parts, 2, line, raw_line)?;
                let rows = parse_u32_part(&parts, 3, line, raw_line)?;
                let cols = parse_u32_part(&parts, 4, line, raw_line)?;
                graph_at(
                    graph.declare_matrix_type(route, element, rows, cols),
                    instruction_span,
                )?
            }
            b"f" if parts.len() == 4 => {
                let domain = resolve_part(&slots, &parts, 2, line, raw_line)?;
                let codomain = resolve_part(&slots, &parts, 3, line, raw_line)?;
                graph_at(
                    graph.declare_function_type(route, domain, codomain),
                    instruction_span,
                )?
            }
            b"q" if parts.len() == 10 => {
                let carrier = resolve_part(&slots, &parts, 2, line, raw_line)?;
                let mut exponents = [0_i8; 7];
                for (axis, exponent) in exponents.iter_mut().enumerate() {
                    *exponent = parse_i8_part(&parts, axis + 3, line, raw_line)?;
                }
                graph_at(
                    graph.declare_quantity_type(route, carrier, Dimension::new(exponents)),
                    instruction_span,
                )?
            }
            b"v" if matches!(parts.len(), 3 | 4) => {
                let ty = resolve_part(&slots, &parts, 2, line, raw_line)?;
                let context = parse_optional_context(&parts, 3, line, raw_line)?;
                graph_at(graph.insert_value(route, context, ty), instruction_span)?
            }
            b"k" if matches!(parts.len(), 5 | 6) => {
                let subject = resolve_part(&slots, &parts, 2, line, raw_line)?;
                let kind_raw = parse_u8_part(&parts, 3, line, raw_line)?;
                let kind = ConstraintKind::from_raw(kind_raw).ok_or(RnaError::InvalidNumber {
                    span: parts[3].span(line),
                })?;
                let holds = parse_bool_part(&parts, 4, line, raw_line)?;
                let context = parse_optional_context(&parts, 5, line, raw_line)?;
                graph_at(
                    graph.declare_constraint(route, context, subject, kind, holds),
                    instruction_span,
                )?
            }
            b"c" | b"x" | b"+" | b"*" | b"/" if matches!(parts.len(), 5 | 6) => {
                let left = resolve_part(&slots, &parts, 2, line, raw_line)?;
                let right = resolve_part(&slots, &parts, 3, line, raw_line)?;
                let output = resolve_part(&slots, &parts, 4, line, raw_line)?;
                let context = parse_optional_context(&parts, 5, line, raw_line)?;
                let proposal = match instruction.bytes {
                    b"c" => Proposal::compose(route, context, left, right, output),
                    b"x" => Proposal::matrix_multiply(route, context, left, right, output),
                    b"+" => Proposal::add(route, context, left, right, output),
                    b"*" => Proposal::multiply(route, context, left, right, output),
                    b"/" => Proposal::divide(route, context, left, right, output),
                    _ => unreachable!(),
                };
                graph_at(graph.transact(proposal), instruction_span)?.node
            }
            b"r" if matches!(parts.len(), 4 | 5) => {
                let input = resolve_part(&slots, &parts, 2, line, raw_line)?;
                let output = resolve_part(&slots, &parts, 3, line, raw_line)?;
                let context = parse_optional_context(&parts, 4, line, raw_line)?;
                graph_at(
                    graph.transact(Proposal::square_root(route, context, input, output)),
                    instruction_span,
                )?
                .node
            }
            b"e" if parts.len() >= 6 => {
                let rule_raw = parse_u8_part(&parts, 2, line, raw_line)?;
                let rule = EqualityRule::from_raw(rule_raw).ok_or(RnaError::InvalidNumber {
                    span: parts[2].span(line),
                })?;
                let left = resolve_part(&slots, &parts, 3, line, raw_line)?;
                let right = resolve_part(&slots, &parts, 4, line, raw_line)?;
                let context = parse_u32_part(&parts, 5, line, raw_line)?;
                let premises = parts[6..]
                    .iter()
                    .map(|part| {
                        let slot = parse_u64(part.bytes).ok_or(RnaError::InvalidNumber {
                            span: part.span(line),
                        })?;
                        slots
                            .get(&slot)
                            .copied()
                            .ok_or(RnaError::UnknownSlot {
                                span: part.span(line),
                                slot,
                            })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                graph_at(
                    graph.prove_equality(route, context, left, right, rule, &premises),
                    instruction_span,
                )?
                .node
            }
            b"a" | b"m" | b"f" | b"q" | b"v" | b"k" | b"c" | b"x" | b"+" | b"*"
            | b"/" | b"r" | b"e" => {
                return Err(RnaError::InvalidArity {
                    span: line_span(line, raw_line),
                });
            }
            _ => {
                return Err(RnaError::UnknownInstruction {
                    span: instruction_span,
                });
            }
        };

        slots.insert(slot, node);
        last_slot = Some(slot);
        instruction_count += 1;
    }

    if instruction_count == 0 {
        return Err(RnaError::Empty);
    }
    Ok(graph.finish_bulk())
}
