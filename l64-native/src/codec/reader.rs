fn push_len(out: &mut Vec<u8>, len: usize) {
    let value = u32::try_from(len).expect("native graph section exceeds u32 length");
    out.extend_from_slice(&value.to_le_bytes());
}

struct Reader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    fn is_empty(&self) -> bool {
        self.cursor == self.bytes.len()
    }

    fn take(&mut self, len: usize) -> Result<&'a [u8], DecodeError> {
        let end = self.cursor.checked_add(len).ok_or(DecodeError::Truncated)?;
        let value = self
            .bytes
            .get(self.cursor..end)
            .ok_or(DecodeError::Truncated)?;
        self.cursor = end;
        Ok(value)
    }

    fn require(&self, count: usize, width: usize) -> Result<(), DecodeError> {
        let bytes = count
            .checked_mul(width)
            .ok_or(DecodeError::StructuralBound)?;
        if self.bytes.len().saturating_sub(self.cursor) < bytes {
            return Err(DecodeError::Truncated);
        }
        Ok(())
    }

    fn count(&mut self, maximum: usize) -> Result<usize, DecodeError> {
        let count = self.u32()? as usize;
        if count > maximum {
            return Err(DecodeError::StructuralBound);
        }
        Ok(count)
    }

    fn u8(&mut self) -> Result<u8, DecodeError> {
        Ok(self.take(1)?[0])
    }

    fn u16(&mut self) -> Result<u16, DecodeError> {
        Ok(u16::from_le_bytes(
            self.take(2)?.try_into().expect("fixed-width read"),
        ))
    }

    fn u32(&mut self) -> Result<u32, DecodeError> {
        Ok(u32::from_le_bytes(
            self.take(4)?.try_into().expect("fixed-width read"),
        ))
    }

    fn u64(&mut self) -> Result<u64, DecodeError> {
        Ok(u64::from_le_bytes(
            self.take(8)?.try_into().expect("fixed-width read"),
        ))
    }
}
