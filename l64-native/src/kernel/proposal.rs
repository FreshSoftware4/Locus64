#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    route: Route,
    context: ContextId,
    opcode: OpCode,
    inputs: Box<[NodeId]>,
    output_type: NodeId,
}

impl Proposal {
    fn binary(
        route: Route,
        context: ContextId,
        opcode: OpCode,
        left: NodeId,
        right: NodeId,
        output_type: NodeId,
    ) -> Self {
        Self {
            route,
            context,
            opcode,
            inputs: Box::new([left, right]),
            output_type,
        }
    }

    fn unary(
        route: Route,
        context: ContextId,
        opcode: OpCode,
        input: NodeId,
        output_type: NodeId,
    ) -> Self {
        Self {
            route,
            context,
            opcode,
            inputs: Box::new([input]),
            output_type,
        }
    }

    pub fn compose(
        route: Route,
        context: ContextId,
        first: NodeId,
        second: NodeId,
        output_type: NodeId,
    ) -> Self {
        Self::binary(route, context, OpCode::Compose, first, second, output_type)
    }

    pub fn matrix_multiply(
        route: Route,
        context: ContextId,
        left: NodeId,
        right: NodeId,
        output_type: NodeId,
    ) -> Self {
        Self::binary(route, context, OpCode::MatMul, left, right, output_type)
    }

    pub fn add(
        route: Route,
        context: ContextId,
        left: NodeId,
        right: NodeId,
        output_type: NodeId,
    ) -> Self {
        Self::binary(route, context, OpCode::Add, left, right, output_type)
    }

    pub fn multiply(
        route: Route,
        context: ContextId,
        left: NodeId,
        right: NodeId,
        output_type: NodeId,
    ) -> Self {
        Self::binary(route, context, OpCode::Multiply, left, right, output_type)
    }

    pub fn divide(
        route: Route,
        context: ContextId,
        left: NodeId,
        right: NodeId,
        output_type: NodeId,
    ) -> Self {
        Self::binary(route, context, OpCode::Divide, left, right, output_type)
    }

    pub fn square_root(
        route: Route,
        context: ContextId,
        input: NodeId,
        output_type: NodeId,
    ) -> Self {
        Self::unary(route, context, OpCode::Sqrt, input, output_type)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommitResult {
    pub node: NodeId,
    pub evidence: NodeId,
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
    ExpectedQuantityType {
        node: NodeId,
    },
    MalformedType {
        node: NodeId,
    },
    EvidenceOnlyType {
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
    QuantityCarrierMismatch {
        left: NodeId,
        right: NodeId,
    },
    DimensionMismatch {
        left: Dimension,
        right: Dimension,
    },
    DimensionOverflow,
    NonIntegralDimensionRoot {
        dimension: Dimension,
    },
    GuardViolated {
        subject: NodeId,
        kind: ConstraintKind,
    },
    QuantityOutputMismatch {
        output_type: NodeId,
        carrier: NodeId,
        dimension: Dimension,
    },
    MalformedConstraint {
        node: NodeId,
    },
    ContradictoryConstraint {
        subject: NodeId,
        kind: ConstraintKind,
    },
    MalformedEvidence {
        node: NodeId,
    },
    EqualitySortMismatch {
        left: NodeId,
        right: NodeId,
    },
    InvalidEqualityRule,
    MalformedEquality {
        node: NodeId,
    },
    EqualityPremiseMismatch {
        premise: NodeId,
    },
    EqualityContextEscape {
        premise: NodeId,
        context: ContextId,
    },
    UncanonicalizableNode {
        node: NodeId,
    },
    NoEqualityPath {
        left: NodeId,
        right: NodeId,
    },
    ContextNotDirectRefinement {
        parent: ContextId,
        child: ContextId,
    },
    RefinementBindingNotConstraint {
        context: ContextId,
        binding: NodeId,
    },
}

