use crate::{ContextId, Graph, NodeId, Route};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum OpCode {
    TypeAtom = 1,
    TypeMatrix = 2,
    TypeFunction = 3,
    Value = 4,
    Compose = 5,
    MatMul = 6,
    ExtendContext = 7,
}

impl OpCode {
    pub(crate) fn from_raw(value: u16) -> Option<Self> {
        match value {
            1 => Some(Self::TypeAtom),
            2 => Some(Self::TypeMatrix),
            3 => Some(Self::TypeFunction),
            4 => Some(Self::Value),
            5 => Some(Self::Compose),
            6 => Some(Self::MatMul),
            7 => Some(Self::ExtendContext),
            _ => None,
        }
    }

    pub(crate) fn is_type(self) -> bool {
        matches!(self, Self::TypeAtom | Self::TypeMatrix | Self::TypeFunction)
    }

    pub(crate) fn is_persisted_node(self) -> bool {
        self != Self::ExtendContext
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PortRole {
    Parameter = 1,
    Domain = 2,
    Codomain = 3,
    Argument = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Port {
    target: NodeId,
    role: u8,
    flags: u8,
    ordinal: u16,
}

impl PortRole {
    pub(crate) fn from_raw(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Parameter),
            2 => Some(Self::Domain),
            3 => Some(Self::Codomain),
            4 => Some(Self::Argument),
            _ => None,
        }
    }
}

impl Port {
    pub(crate) fn new(target: NodeId, role: PortRole, ordinal: u16) -> Self {
        Self {
            target,
            role: role as u8,
            flags: 0,
            ordinal,
        }
    }

    pub(crate) fn from_raw(target: NodeId, role: PortRole, flags: u8, ordinal: u16) -> Self {
        Self {
            target,
            role: role as u8,
            flags,
            ordinal,
        }
    }

    pub fn target(&self) -> NodeId {
        self.target
    }

    pub fn role(&self) -> PortRole {
        match self.role {
            1 => PortRole::Parameter,
            2 => PortRole::Domain,
            3 => PortRole::Codomain,
            4 => PortRole::Argument,
            _ => unreachable!("stored port role is constructed internally"),
        }
    }

    pub fn ordinal(&self) -> u16 {
        self.ordinal
    }

    pub(crate) fn flags(&self) -> u8 {
        self.flags
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    route: Route,
    context: ContextId,
    opcode: OpCode,
    inputs: Box<[NodeId]>,
    output_type: NodeId,
}

impl Proposal {
    pub fn compose(
        route: Route,
        context: ContextId,
        first: NodeId,
        second: NodeId,
        output_type: NodeId,
    ) -> Self {
        Self {
            route,
            context,
            opcode: OpCode::Compose,
            inputs: Box::new([first, second]),
            output_type,
        }
    }

    pub fn matrix_multiply(
        route: Route,
        context: ContextId,
        left: NodeId,
        right: NodeId,
        output_type: NodeId,
    ) -> Self {
        Self {
            route,
            context,
            opcode: OpCode::MatMul,
            inputs: Box::new([left, right]),
            output_type,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommitResult {
    pub node: NodeId,
    pub event: u32,
    pub commitment: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Obstruction {
    UnknownNode {
        node: NodeId,
    },
    UnknownContext {
        context: ContextId,
    },
    RouteOccupied,
    ExpectedType {
        node: NodeId,
    },
    ExpectedFunctionType {
        node: NodeId,
    },
    ExpectedMatrixType {
        node: NodeId,
    },
    MalformedType {
        node: NodeId,
    },
    Arity {
        expected: u16,
        actual: u16,
    },
    FunctionBoundaryMismatch {
        left: NodeId,
        right: NodeId,
    },
    MatrixShapeMismatch {
        left_cols: u32,
        right_rows: u32,
    },
    MatrixElementMismatch {
        left: NodeId,
        right: NodeId,
    },
    FunctionOutputMismatch {
        output_type: NodeId,
        domain: NodeId,
        codomain: NodeId,
    },
    MatrixOutputMismatch {
        output_type: NodeId,
        element: NodeId,
        rows: u32,
        cols: u32,
    },
}

impl Graph {
    pub fn transact(&mut self, proposal: Proposal) -> Result<CommitResult, Obstruction> {
        self.ensure_context(proposal.context)?;
        self.ensure_type(proposal.output_type)?;
        if proposal.inputs.len() != 2 {
            return Err(Obstruction::Arity {
                expected: 2,
                actual: proposal.inputs.len() as u16,
            });
        }
        let left = proposal.inputs[0];
        let right = proposal.inputs[1];
        let left_node = self.ensure_node(left)?;
        let right_node = self.ensure_node(right)?;
        let left_ty = left_node
            .ty()
            .ok_or(Obstruction::ExpectedType { node: left })?;
        let right_ty = right_node
            .ty()
            .ok_or(Obstruction::ExpectedType { node: right })?;

        match proposal.opcode {
            OpCode::Compose => {
                let (first_domain, first_codomain) = self.function_parts(left_ty)?;
                let (second_domain, second_codomain) = self.function_parts(right_ty)?;
                if first_codomain != second_domain {
                    return Err(Obstruction::FunctionBoundaryMismatch {
                        left: first_codomain,
                        right: second_domain,
                    });
                }
                if !self.matches_function_type(proposal.output_type, first_domain, second_codomain)
                {
                    return Err(Obstruction::FunctionOutputMismatch {
                        output_type: proposal.output_type,
                        domain: first_domain,
                        codomain: second_codomain,
                    });
                }
            }
            OpCode::MatMul => {
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
                if !self.matches_matrix_type(
                    proposal.output_type,
                    left_element,
                    left_rows,
                    right_cols,
                ) {
                    return Err(Obstruction::MatrixOutputMismatch {
                        output_type: proposal.output_type,
                        element: left_element,
                        rows: left_rows,
                        cols: right_cols,
                    });
                }
            }
            _ => unreachable!("public proposal constructors expose executable operations only"),
        }

        self.insert_operation(
            proposal.route,
            proposal.context,
            proposal.output_type,
            proposal.opcode,
            &proposal.inputs,
        )
    }
}
