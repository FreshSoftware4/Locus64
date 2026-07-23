impl Graph {
    pub fn transact(&mut self, proposal: Proposal) -> Result<CommitResult, Obstruction> {
        self.ensure_context(proposal.context)?;
        self.ensure_type(proposal.output_type)?;
        let expected = proposal
            .opcode
            .arity()
            .expect("public proposal constructors expose executable operations only");
        if proposal.inputs.len() != expected as usize {
            return Err(Obstruction::Arity {
                expected,
                actual: proposal.inputs.len() as u16,
            });
        }

        let judgment_route = proposal.route.composed(JUDGMENT_LOCUS);
        let evidence_route = proposal.route.composed(EVIDENCE_LOCUS);
        if self.resolve(&proposal.route).is_some()
            || self.resolve(&judgment_route).is_some()
            || self.resolve(&evidence_route).is_some()
        {
            return Err(Obstruction::RouteOccupied);
        }

        let evidence = self.validate_operation(
            proposal.context,
            proposal.opcode,
            &proposal.inputs,
            proposal.output_type,
        )?;

        Ok(self.insert_admitted_operation(
            [proposal.route, judgment_route, evidence_route],
            proposal.context,
            proposal.output_type,
            proposal.opcode,
            &proposal.inputs,
            evidence,
        ))
    }

    pub(crate) fn validate_operation(
        &self,
        _context: ContextId,
        opcode: OpCode,
        inputs: &[NodeId],
        output_type: NodeId,
    ) -> Result<EvidencePlan, Obstruction> {
        let expected = opcode.arity().ok_or(Obstruction::Arity {
            expected: 0,
            actual: inputs.len() as u16,
        })?;
        if inputs.len() != expected as usize {
            return Err(Obstruction::Arity {
                expected,
                actual: inputs.len() as u16,
            });
        }
        self.ensure_type(output_type)?;

        let left = inputs[0];
        let left_ty = self
            .ensure_node(left)?
            .ty()
            .ok_or(Obstruction::ExpectedType { node: left })?;
        let right = inputs.get(1).copied();
        let right_ty = right
            .map(|node| {
                self.ensure_node(node)?
                    .ty()
                    .ok_or(Obstruction::ExpectedType { node })
            })
            .transpose()?;

        match opcode {
            OpCode::Compose => {
                let right_ty = right_ty.expect("binary arity validated");
                let (first_domain, first_codomain) = self.function_parts(left_ty)?;
                let (second_domain, second_codomain) = self.function_parts(right_ty)?;
                if first_codomain != second_domain {
                    return Err(Obstruction::FunctionBoundaryMismatch {
                        left: first_codomain,
                        right: second_domain,
                    });
                }
                if !self.matches_function_type(output_type, first_domain, second_codomain) {
                    return Err(Obstruction::FunctionOutputMismatch {
                        output_type,
                        domain: first_domain,
                        codomain: second_codomain,
                    });
                }
            }
            OpCode::MatMul => {
                let right_ty = right_ty.expect("binary arity validated");
                let (left_element, left_rows, left_cols) = self.matrix_parts(left_ty)?;
                let (right_element, right_rows, right_cols) = self.matrix_parts(right_ty)?;
                if left_cols != right_rows {
                    return Err(Obstruction::MatrixShapeMismatch {
                        left_cols,
                        right_rows,
                    });
                }
                if left_element != right_element {
                    return Err(Obstruction::MatrixElementMismatch {
                        left: left_element,
                        right: right_element,
                    });
                }
                if !self.matches_matrix_type(output_type, left_element, left_rows, right_cols) {
                    return Err(Obstruction::MatrixOutputMismatch {
                        output_type,
                        element: left_element,
                        rows: left_rows,
                        cols: right_cols,
                    });
                }
            }
            OpCode::Add | OpCode::Multiply | OpCode::Divide => {
                let right_ty = right_ty.expect("binary arity validated");
                let (left_carrier, left_dimension) = self.quantity_parts(left_ty)?;
                let (right_carrier, right_dimension) = self.quantity_parts(right_ty)?;
                if left_carrier != right_carrier {
                    return Err(Obstruction::QuantityCarrierMismatch {
                        left: left_carrier,
                        right: right_carrier,
                    });
                }
                let dimension = match opcode {
                    OpCode::Add => {
                        if left_dimension != right_dimension {
                            return Err(Obstruction::DimensionMismatch {
                                left: left_dimension,
                                right: right_dimension,
                            });
                        }
                        left_dimension
                    }
                    OpCode::Multiply => left_dimension
                        .multiply(right_dimension)
                        .ok_or(Obstruction::DimensionOverflow)?,
                    OpCode::Divide => left_dimension
                        .divide(right_dimension)
                        .ok_or(Obstruction::DimensionOverflow)?,
                    _ => unreachable!(),
                };
                if !self.matches_quantity_type(output_type, left_carrier, dimension) {
                    return Err(Obstruction::QuantityOutputMismatch {
                        output_type,
                        carrier: left_carrier,
                        dimension,
                    });
                }
            }
            OpCode::Sqrt => {
                let (carrier, dimension) = self.quantity_parts(left_ty)?;
                let output_dimension = dimension
                    .square_root()
                    .ok_or(Obstruction::NonIntegralDimensionRoot { dimension })?;
                if !self.matches_quantity_type(output_type, carrier, output_dimension) {
                    return Err(Obstruction::QuantityOutputMismatch {
                        output_type,
                        carrier,
                        dimension: output_dimension,
                    });
                }
                return match self.constraint_state(_context, left, ConstraintKind::NonNegative)? {
                    ConstraintState::Proven => Ok(EvidencePlan::Witness),
                    ConstraintState::Refuted => Err(Obstruction::GuardViolated {
                        subject: left,
                        kind: ConstraintKind::NonNegative,
                    }),
                    ConstraintState::Unknown => Ok(EvidencePlan::Obligation {
                        kind: ConstraintKind::NonNegative,
                        subject: left,
                    }),
                };
            }
            _ => unreachable!("validated opcode is executable"),
        }
        Ok(EvidencePlan::Witness)
    }
}
