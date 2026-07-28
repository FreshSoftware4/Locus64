use super::{
    BUNDLE_VERSION, BundleError, FLAGS, HEADER_BYTES, MAGIC, MAX_NATIVE_BUNDLE_BYTES,
    MEMBER_HEADER_BYTES, parse_header, validate_dna, validate_member_count,
};
use l64_native::{Graph, MAX_NATIVE_DNA_PAYLOAD_BYTES};
use std::{
    fmt,
    io::{self, Read, Seek, SeekFrom, Write},
};

const MAX_NATIVE_DNA_FRAME_BYTES: usize = MAX_NATIVE_DNA_PAYLOAD_BYTES + 44;

#[derive(Debug)]
pub enum BundleStreamError {
    Transport(BundleError),
    Io(io::Error),
}

impl fmt::Display for BundleStreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(error) => error.fmt(f),
            Self::Io(error) => write!(f, "L64B stream I/O failed: {error}"),
        }
    }
}

impl std::error::Error for BundleStreamError {}

impl From<BundleError> for BundleStreamError {
    fn from(value: BundleError) -> Self {
        Self::Transport(value)
    }
}

impl From<io::Error> for BundleStreamError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Debug)]
pub struct OwnedBundleMember {
    index: usize,
    dna: Vec<u8>,
    graph: Graph,
}

impl OwnedBundleMember {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn dna(&self) -> &[u8] {
        &self.dna
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }
}

pub struct BundleDecoder<R> {
    reader: R,
    member_count: usize,
    remaining_payload: usize,
    next_index: usize,
    complete: bool,
}

impl<R: Read> BundleDecoder<R> {
    pub fn new(mut reader: R) -> Result<Self, BundleStreamError> {
        let mut header = [0u8; HEADER_BYTES];
        read_exact_transport(&mut reader, &mut header)?;
        let (member_count, payload_len) = parse_header(&header)?;
        Ok(Self {
            reader,
            member_count,
            remaining_payload: payload_len,
            next_index: 0,
            complete: false,
        })
    }

    pub fn member_count(&self) -> usize {
        self.member_count
    }

    pub fn next_member(&mut self) -> Result<Option<OwnedBundleMember>, BundleStreamError> {
        if self.complete {
            return Ok(None);
        }
        if self.next_index == self.member_count {
            if self.remaining_payload != 0 {
                return Err(BundleError::TrailingBytes.into());
            }
            let mut trailing = [0u8; 1];
            match self.reader.read(&mut trailing) {
                Ok(0) => {
                    self.complete = true;
                    return Ok(None);
                }
                Ok(_) => return Err(BundleError::TrailingBytes.into()),
                Err(error) => return Err(BundleStreamError::Io(error)),
            }
        }
        if self.remaining_payload < MEMBER_HEADER_BYTES {
            return Err(BundleError::Truncated.into());
        }
        let mut length = [0u8; MEMBER_HEADER_BYTES];
        read_exact_transport(&mut self.reader, &mut length)?;
        self.remaining_payload -= MEMBER_HEADER_BYTES;
        let member_len = u32::from_le_bytes(length) as usize;
        if member_len == 0 {
            return Err(BundleError::ZeroLengthMember {
                index: self.next_index,
            }
            .into());
        }
        if member_len > MAX_NATIVE_DNA_FRAME_BYTES {
            return Err(BundleError::TooLarge.into());
        }
        if member_len > self.remaining_payload {
            return Err(BundleError::Truncated.into());
        }
        let mut dna = vec![0u8; member_len];
        read_exact_transport(&mut self.reader, &mut dna)?;
        self.remaining_payload -= member_len;
        let index = self.next_index;
        let graph = validate_dna(index, &dna)?;
        self.next_index += 1;
        Ok(Some(OwnedBundleMember { index, dna, graph }))
    }

    pub fn finish(mut self) -> Result<(), BundleStreamError> {
        while self.next_member()?.is_some() {}
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BundleWriteSummary {
    pub members: usize,
    pub bytes: usize,
}

pub struct BundleEncoder<W> {
    writer: W,
    members: usize,
    payload_len: usize,
}

impl<W: Write + Seek> BundleEncoder<W> {
    pub fn new(mut writer: W) -> Result<Self, BundleStreamError> {
        writer.write_all(&[0u8; HEADER_BYTES])?;
        Ok(Self {
            writer,
            members: 0,
            payload_len: 0,
        })
    }

    pub fn push_member(&mut self, dna: &[u8]) -> Result<(), BundleStreamError> {
        let next_count = self
            .members
            .checked_add(1)
            .ok_or(BundleError::TooManyMembers)?;
        validate_member_count(next_count)?;
        validate_dna(self.members, dna)?;
        if dna.len() > u32::MAX as usize {
            return Err(BundleError::TooLarge.into());
        }
        let next_payload = self
            .payload_len
            .checked_add(MEMBER_HEADER_BYTES)
            .and_then(|value| value.checked_add(dna.len()))
            .ok_or(BundleError::TooLarge)?;
        let total = HEADER_BYTES
            .checked_add(next_payload)
            .ok_or(BundleError::TooLarge)?;
        if total > MAX_NATIVE_BUNDLE_BYTES || next_payload > u32::MAX as usize {
            return Err(BundleError::TooLarge.into());
        }
        self.writer.write_all(&(dna.len() as u32).to_le_bytes())?;
        self.writer.write_all(dna)?;
        self.members = next_count;
        self.payload_len = next_payload;
        Ok(())
    }

    pub fn finish(mut self) -> Result<BundleWriteSummary, BundleStreamError> {
        validate_member_count(self.members)?;
        let total = HEADER_BYTES + self.payload_len;
        self.writer.seek(SeekFrom::Start(0))?;
        self.writer.write_all(MAGIC)?;
        self.writer.write_all(&BUNDLE_VERSION.to_le_bytes())?;
        self.writer.write_all(&FLAGS.to_le_bytes())?;
        self.writer
            .write_all(&(self.members as u32).to_le_bytes())?;
        self.writer
            .write_all(&(self.payload_len as u32).to_le_bytes())?;
        self.writer.seek(SeekFrom::End(0))?;
        self.writer.flush()?;
        Ok(BundleWriteSummary {
            members: self.members,
            bytes: total,
        })
    }
}

fn read_exact_transport(
    reader: &mut impl Read,
    buffer: &mut [u8],
) -> Result<(), BundleStreamError> {
    match reader.read_exact(buffer) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => {
            Err(BundleError::Truncated.into())
        }
        Err(error) => Err(BundleStreamError::Io(error)),
    }
}
