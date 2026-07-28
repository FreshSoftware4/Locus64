use std::{collections::BTreeMap, fmt};

use crate::kernel::EqualityRule;
use crate::{
    ConstraintKind, ContextId, Dimension, DnaError, Graph, LocusWord, NodeId, Obstruction, OpCode,
    Proposal, ROOT_CONTEXT, Route, decode_dna, dna_bytes,
};

const RNA_MAGIC: &[u8] = b"L64R1";
pub const MAX_NATIVE_RNA_BYTES: usize = 1 << 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RnaSpan {
    pub line: u32,
    pub column: u32,
    pub length: u32,
}

impl RnaSpan {
    pub const fn new(line: u32, column: u32, length: u32) -> Self {
        Self {
            line,
            column,
            length,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RnaError {
    SourceTooLarge { actual: usize, limit: usize },
    Empty,
    BadHeader { span: RnaSpan },
    InvalidArity { span: RnaSpan },
    InvalidNumber { span: RnaSpan },
    UnknownInstruction { span: RnaSpan },
    SlotOrder { span: RnaSpan, slot: u64 },
    ContextOrder { span: RnaSpan, context: u32 },
    UnknownSlot { span: RnaSpan, slot: u64 },
    UnrepresentableGraph,
    Graph { span: RnaSpan, error: Obstruction },
    Dna(DnaError),
}

impl RnaError {
    pub const fn span(self) -> Option<RnaSpan> {
        match self {
            Self::BadHeader { span }
            | Self::InvalidArity { span }
            | Self::InvalidNumber { span }
            | Self::UnknownInstruction { span }
            | Self::SlotOrder { span, .. }
            | Self::ContextOrder { span, .. }
            | Self::UnknownSlot { span, .. }
            | Self::Graph { span, .. } => Some(span),
            Self::SourceTooLarge { .. }
            | Self::Empty
            | Self::UnrepresentableGraph
            | Self::Dna(_) => None,
        }
    }
}

impl fmt::Display for RnaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceTooLarge { actual, limit } => write!(
                f,
                "L64R1 source is {actual} bytes; native limit is {limit} bytes"
            ),
            Self::Empty => f.write_str("L64R1 source is empty"),
            Self::BadHeader { span } => write!(
                f,
                "input is not valid L64R1 RNA at {}:{}",
                span.line, span.column
            ),
            Self::InvalidArity { span } => write!(
                f,
                "invalid instruction arity at RNA {}:{}",
                span.line, span.column
            ),
            Self::InvalidNumber { span } => {
                write!(f, "invalid number at RNA {}:{}", span.line, span.column)
            }
            Self::UnknownInstruction { span } => write!(
                f,
                "unknown instruction at RNA {}:{}",
                span.line, span.column
            ),
            Self::SlotOrder { span, slot } => write!(
                f,
                "slot {slot} is out of canonical order at RNA {}:{}",
                span.line, span.column
            ),
            Self::ContextOrder { span, context } => write!(
                f,
                "context {context} is out of canonical order at RNA {}:{}",
                span.line, span.column
            ),
            Self::UnknownSlot { span, slot } => write!(
                f,
                "RNA {}:{} references unknown slot {slot}",
                span.line, span.column
            ),
            Self::UnrepresentableGraph => {
                f.write_str("graph cannot be represented as canonical L64R1 RNA")
            }
            Self::Graph { span, error } => write!(
                f,
                "RNA graph construction failed at {}:{}: {error}",
                span.line, span.column
            ),
            Self::Dna(error) => write!(f, "RNA/DNA conversion failed: {error}"),
        }
    }
}

impl std::error::Error for RnaError {}

impl From<DnaError> for RnaError {
    fn from(value: DnaError) -> Self {
        Self::Dna(value)
    }
}

struct RnaDiagnostic<'a> {
    source: &'a [u8],
    error: RnaError,
}

pub fn rna_diagnostic(source: &[u8], error: RnaError) -> impl fmt::Display + '_ {
    RnaDiagnostic { source, error }
}

impl fmt::Display for RnaDiagnostic<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)?;
        let Some(span) = self.error.span() else {
            return Ok(());
        };
        let Some(raw_line) = self
            .source
            .split(|byte| *byte == b'\n')
            .nth(span.line.saturating_sub(1) as usize)
        else {
            return Ok(());
        };
        let raw_line = raw_line.strip_suffix(b"\r").unwrap_or(raw_line);
        let line_text = core::str::from_utf8(raw_line).unwrap_or("<non-UTF-8 RNA line>");
        let width = decimal_width(span.line);
        write!(f, "\n  {line:>width$} | {line_text}", line = span.line)?;
        write!(f, "\n  {:width$} | ", "")?;
        let column = span.column.saturating_sub(1) as usize;
        for byte in raw_line.iter().take(column) {
            if *byte == b'\t' {
                f.write_str("\t")?;
            } else {
                f.write_str(" ")?;
            }
        }
        let available = raw_line.len().saturating_sub(column);
        let caret_len = usize::max(1, usize::min(span.length as usize, available.max(1)));
        for _ in 0..caret_len {
            f.write_str("^")?;
        }
        Ok(())
    }
}

const fn decimal_width(mut value: u32) -> usize {
    let mut width = 1;
    while value >= 10 {
        value /= 10;
        width += 1;
    }
    width
}

include!("rna/compile.rs");
include!("rna/sequence.rs");
include!("rna/parse.rs");
include!("rna/render.rs");
