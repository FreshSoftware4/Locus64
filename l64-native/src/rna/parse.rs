#[derive(Clone, Copy)]
struct Token<'a> {
    bytes: &'a [u8],
    column: u32,
}

impl Token<'_> {
    fn span(self, line: u32) -> RnaSpan {
        RnaSpan::new(line, self.column, self.bytes.len().max(1) as u32)
    }
}

fn require_root(context: ContextId) -> Result<(), RnaError> {
    (context == ROOT_CONTEXT)
        .then_some(())
        .ok_or(RnaError::UnrepresentableGraph)
}

fn push_context(out: &mut Vec<u8>, context: ContextId) {
    if context != ROOT_CONTEXT {
        push_space_decimal(out, u64::from(context));
    }
}

fn next_nonempty<'a>(
    lines: &mut impl Iterator<Item = (usize, &'a [u8])>,
) -> Option<(usize, &'a [u8])> {
    lines.find(|(_, line)| !trim_ascii(line).is_empty())
}

fn trim_ascii(mut bytes: &[u8]) -> &[u8] {
    while bytes.first().is_some_and(u8::is_ascii_whitespace) {
        bytes = &bytes[1..];
    }
    while bytes.last().is_some_and(u8::is_ascii_whitespace) {
        bytes = &bytes[..bytes.len() - 1];
    }
    bytes
}

fn tokens(line: &[u8]) -> Vec<Token<'_>> {
    let mut out = Vec::new();
    let mut index = 0usize;
    while index < line.len() {
        while index < line.len() && line[index].is_ascii_whitespace() {
            index += 1;
        }
        if index == line.len() {
            break;
        }
        let start = index;
        while index < line.len() && !line[index].is_ascii_whitespace() {
            index += 1;
        }
        out.push(Token {
            bytes: &line[start..index],
            column: (start + 1) as u32,
        });
    }
    out
}

fn line_span(line: u32, raw_line: &[u8]) -> RnaSpan {
    let first = raw_line
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(0);
    let last = raw_line
        .iter()
        .rposition(|byte| !byte.is_ascii_whitespace())
        .map(|index| index + 1)
        .unwrap_or(first + 1);
    RnaSpan::new(line, (first + 1) as u32, last.saturating_sub(first) as u32)
}

fn missing_span(line: u32, raw_line: &[u8]) -> RnaSpan {
    RnaSpan::new(line, raw_line.len().saturating_add(1) as u32, 1)
}

fn part_span(parts: &[Token<'_>], index: usize, line: u32, raw_line: &[u8]) -> RnaSpan {
    parts
        .get(index)
        .copied()
        .map(|token| token.span(line))
        .unwrap_or_else(|| missing_span(line, raw_line))
}

fn parse_part(
    parts: &[Token<'_>],
    index: usize,
    line: u32,
    raw_line: &[u8],
) -> Result<u64, RnaError> {
    let token = parts.get(index).copied().ok_or(RnaError::InvalidNumber {
        span: missing_span(line, raw_line),
    })?;
    parse_u64(token.bytes).ok_or(RnaError::InvalidNumber {
        span: token.span(line),
    })
}

fn parse_u8_part(
    parts: &[Token<'_>],
    index: usize,
    line: u32,
    raw_line: &[u8],
) -> Result<u8, RnaError> {
    parse_part(parts, index, line, raw_line)?
        .try_into()
        .map_err(|_| RnaError::InvalidNumber {
            span: part_span(parts, index, line, raw_line),
        })
}

fn parse_u32_part(
    parts: &[Token<'_>],
    index: usize,
    line: u32,
    raw_line: &[u8],
) -> Result<u32, RnaError> {
    parse_part(parts, index, line, raw_line)?
        .try_into()
        .map_err(|_| RnaError::InvalidNumber {
            span: part_span(parts, index, line, raw_line),
        })
}

fn parse_i8_part(
    parts: &[Token<'_>],
    index: usize,
    line: u32,
    raw_line: &[u8],
) -> Result<i8, RnaError> {
    let token = parts.get(index).copied().ok_or(RnaError::InvalidNumber {
        span: missing_span(line, raw_line),
    })?;
    parse_i8(token.bytes).ok_or(RnaError::InvalidNumber {
        span: token.span(line),
    })
}

fn parse_bool_part(
    parts: &[Token<'_>],
    index: usize,
    line: u32,
    raw_line: &[u8],
) -> Result<bool, RnaError> {
    match parse_u8_part(parts, index, line, raw_line)? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(RnaError::InvalidNumber {
            span: part_span(parts, index, line, raw_line),
        }),
    }
}

fn parse_optional_context(
    parts: &[Token<'_>],
    index: usize,
    line: u32,
    raw_line: &[u8],
) -> Result<ContextId, RnaError> {
    if parts.len() == index {
        Ok(ROOT_CONTEXT)
    } else {
        parse_u32_part(parts, index, line, raw_line)
    }
}

fn resolve_part(
    slots: &BTreeMap<u64, NodeId>,
    parts: &[Token<'_>],
    index: usize,
    line: u32,
    raw_line: &[u8],
) -> Result<NodeId, RnaError> {
    let slot = parse_part(parts, index, line, raw_line)?;
    slots
        .get(&slot)
        .copied()
        .ok_or(RnaError::UnknownSlot {
            span: part_span(parts, index, line, raw_line),
            slot,
        })
}

fn graph_at<T>(result: Result<T, Obstruction>, span: RnaSpan) -> Result<T, RnaError> {
    result.map_err(|error| RnaError::Graph { span, error })
}

fn parse_i8(bytes: &[u8]) -> Option<i8> {
    let (negative, digits) = if let Some(digits) = bytes.strip_prefix(b"-") {
        (true, digits)
    } else if let Some(digits) = bytes.strip_prefix(b"+") {
        (false, digits)
    } else {
        (false, bytes)
    };
    let magnitude = parse_u64(digits)?;
    if negative {
        let magnitude: i16 = magnitude.try_into().ok()?;
        i8::try_from(-magnitude).ok()
    } else {
        magnitude.try_into().ok()
    }
}

fn parse_u64(bytes: &[u8]) -> Option<u64> {
    let (radix, digits) = if bytes.starts_with(b"0x") || bytes.starts_with(b"0X") {
        (16u64, &bytes[2..])
    } else {
        (10u64, bytes)
    };
    if digits.is_empty() {
        return None;
    }
    digits.iter().try_fold(0u64, |value, byte| {
        let digit = match *byte {
            b'0'..=b'9' => u64::from(*byte - b'0'),
            b'a'..=b'f' if radix == 16 => u64::from(*byte - b'a' + 10),
            b'A'..=b'F' if radix == 16 => u64::from(*byte - b'A' + 10),
            _ => return None,
        };
        if digit >= radix {
            return None;
        }
        value.checked_mul(radix)?.checked_add(digit)
    })
}
