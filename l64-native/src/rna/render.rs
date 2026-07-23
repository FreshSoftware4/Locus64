fn slot_for(slots: &BTreeMap<NodeId, u64>, node: NodeId) -> Result<u64, RnaError> {
    slots
        .get(&node)
        .copied()
        .ok_or(RnaError::UnrepresentableGraph)
}

fn push_prefix(out: &mut Vec<u8>, opcode: u8, slot: u64) {
    out.push(opcode);
    push_space_decimal(out, slot);
}

fn push_space_decimal(out: &mut Vec<u8>, value: u64) {
    out.push(b' ');
    push_decimal(out, value);
}

fn push_space_signed(out: &mut Vec<u8>, value: i8) {
    out.push(b' ');
    if value < 0 {
        out.push(b'-');
        push_decimal(out, u64::from(value.unsigned_abs()));
    } else {
        push_decimal(out, value as u64);
    }
}

fn push_space_hex(out: &mut Vec<u8>, value: u64) {
    out.push(b' ');
    push_hex(out, value);
}

fn push_decimal(out: &mut Vec<u8>, mut value: u64) {
    let mut buffer = [0u8; 20];
    let mut cursor = buffer.len();
    loop {
        cursor -= 1;
        buffer[cursor] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    out.extend_from_slice(&buffer[cursor..]);
}

fn push_hex(out: &mut Vec<u8>, mut value: u64) {
    out.extend_from_slice(b"0x");
    if value == 0 {
        out.push(b'0');
        return;
    }
    let mut buffer = [0u8; 16];
    let mut cursor = buffer.len();
    while value != 0 {
        cursor -= 1;
        let digit = (value & 0xf) as u8;
        buffer[cursor] = if digit < 10 {
            b'0' + digit
        } else {
            b'a' + digit - 10
        };
        value >>= 4;
    }
    out.extend_from_slice(&buffer[cursor..]);
}

fn line_number(index: usize) -> u32 {
    u32::try_from(index + 1).unwrap_or(u32::MAX)
}
