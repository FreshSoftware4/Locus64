use std::fmt;

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
    pub symbol: crate::SymbolicSeal,
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

impl fmt::Display for Obstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownNode { node } => write!(f, "unknown node {node}"),
            Self::UnknownContext { context } => write!(f, "unknown context {context}"),
            Self::RouteOccupied => f.write_str("route is already occupied"),
            Self::ExpectedType { node } => write!(f, "node {node} is not a type"),
            Self::ExpectedFunctionType { node } => write!(f, "node {node} is not a function type"),
            Self::ExpectedMatrixType { node } => write!(f, "node {node} is not a matrix type"),
            Self::ExpectedQuantityType { node } => write!(f, "node {node} is not a quantity type"),
            Self::MalformedType { node } => write!(f, "node {node} has a malformed type"),
            Self::EvidenceOnlyType { node } => write!(f, "node {node} is evidence-only"),
            Self::Arity { expected, actual } => write!(f, "arity mismatch: expected {expected}, got {actual}"),
            Self::FunctionBoundaryMismatch { left, right } => write!(f, "function boundary mismatch between nodes {left} and {right}"),
            Self::MatrixShapeMismatch { left_cols, right_rows } => write!(f, "matrix shape mismatch: left columns {left_cols}, right rows {right_rows}"),
            Self::MatrixElementMismatch { left, right } => write!(f, "matrix element mismatch between nodes {left} and {right}"),
            Self::FunctionOutputMismatch { output_type, domain, codomain } => write!(f, "function output type {output_type} does not match domain {domain} and codomain {codomain}"),
            Self::MatrixOutputMismatch { output_type, element, rows, cols } => write!(f, "matrix output type {output_type} does not match element {element} and shape {rows}x{cols}"),
            Self::QuantityCarrierMismatch { left, right } => write!(f, "quantity carrier mismatch between nodes {left} and {right}"),
            Self::DimensionMismatch { left, right } => write!(f, "dimension mismatch: {left:?} versus {right:?}"),
            Self::DimensionOverflow => f.write_str("dimension arithmetic overflow"),
            Self::NonIntegralDimensionRoot { dimension } => write!(f, "dimension {dimension:?} has no integral root"),
            Self::GuardViolated { subject, kind } => write!(f, "constraint guard {kind:?} is violated by node {subject}"),
            Self::QuantityOutputMismatch { output_type, carrier, dimension } => write!(f, "quantity output type {output_type} does not match carrier {carrier} and dimension {dimension:?}"),
            Self::MalformedConstraint { node } => write!(f, "node {node} is a malformed constraint"),
            Self::ContradictoryConstraint { subject, kind } => write!(f, "constraint {kind:?} contradicts node {subject}"),
            Self::MalformedEvidence { node } => write!(f, "node {node} is malformed evidence"),
            Self::EqualitySortMismatch { left, right } => write!(f, "equality sort mismatch between nodes {left} and {right}"),
            Self::InvalidEqualityRule => f.write_str("equality rule is invalid"),
            Self::MalformedEquality { node } => write!(f, "node {node} is a malformed equality"),
            Self::EqualityPremiseMismatch { premise } => write!(f, "equality premise {premise} does not match"),
            Self::EqualityContextEscape { premise, context } => write!(f, "equality premise {premise} escapes context {context}"),
            Self::UncanonicalizableNode { node } => write!(f, "node {node} cannot be canonicalized"),
            Self::NoEqualityPath { left, right } => write!(f, "no equality path connects nodes {left} and {right}"),
            Self::ContextNotDirectRefinement { parent, child } => write!(f, "context {child} is not a direct refinement of {parent}"),
            Self::RefinementBindingNotConstraint { context, binding } => write!(f, "binding {binding} in context {context} is not a constraint"),
        }
    }
}

impl std::error::Error for Obstruction {}
