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
    lines.find_map(|(index, line)| {
        let line = trim_ascii(line);
        (!line.is_empty()).then_some((index, line))
    })
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

fn tokens(line: &[u8]) -> Vec<&[u8]> {
    line.split(|byte| byte.is_ascii_whitespace())
        .filter(|part| !part.is_empty())
        .collect()
}

fn parse_part(parts: &[&[u8]], index: usize, line: u32) -> Result<u64, RnaError> {
    parts
        .get(index)
        .and_then(|part| parse_u64(part))
        .ok_or(RnaError::InvalidNumber { line })
}

fn parse_u8_part(parts: &[&[u8]], index: usize, line: u32) -> Result<u8, RnaError> {
    parse_part(parts, index, line)?
        .try_into()
        .map_err(|_| RnaError::InvalidNumber { line })
}

fn parse_u32_part(parts: &[&[u8]], index: usize, line: u32) -> Result<u32, RnaError> {
    parse_part(parts, index, line)?
        .try_into()
        .map_err(|_| RnaError::InvalidNumber { line })
}

fn parse_i8_part(parts: &[&[u8]], index: usize, line: u32) -> Result<i8, RnaError> {
    parts
        .get(index)
        .and_then(|part| parse_i8(part))
        .ok_or(RnaError::InvalidNumber { line })
}

fn parse_bool_part(parts: &[&[u8]], index: usize, line: u32) -> Result<bool, RnaError> {
    match parse_u8_part(parts, index, line)? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(RnaError::InvalidNumber { line }),
    }
}

fn parse_optional_context(parts: &[&[u8]], index: usize, line: u32) -> Result<ContextId, RnaError> {
    if parts.len() == index {
        Ok(ROOT_CONTEXT)
    } else {
        parse_u32_part(parts, index, line)
    }
}

fn resolve_part(
    slots: &BTreeMap<u64, NodeId>,
    parts: &[&[u8]],
    index: usize,
    line: u32,
) -> Result<NodeId, RnaError> {
    let slot = parse_part(parts, index, line)?;
    slots
        .get(&slot)
        .copied()
        .ok_or(RnaError::UnknownSlot { line, slot })
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
