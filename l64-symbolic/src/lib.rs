#![forbid(unsafe_code)]

use core::{fmt, str::FromStr};

const MODULUS: u64 = 18_446_744_073_709_551_557;
const BASE_SIGMA: u64 = 1_099_511_628_211;
const BASE_PI: u64 = 1_469_598_103_934_665_603;
const BASE_DELTA: u64 = 2_305_843_009_213_693_951;
const BASE_OMEGA: u64 = 6_364_136_223_846_793_005;
const IDENTITY_MAGIC: &[u8; 5] = b"L64S1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SealAxis {
    Sigma,
    Pi,
    Delta,
    Omega,
}

impl SealAxis {
    pub const ALL: [Self; 4] = [Self::Sigma, Self::Pi, Self::Delta, Self::Omega];

    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Sigma => "Σ",
            Self::Pi => "Π",
            Self::Delta => "Δ",
            Self::Omega => "Ω",
        }
    }

    pub const fn role(self) -> &'static str {
        match self {
            Self::Sigma => "aggregate content mass",
            Self::Pi => "ordered composition",
            Self::Delta => "adjacent transition structure",
            Self::Omega => "nonlinear boundary closure",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AxisChange {
    pub axis: SealAxis,
    pub before: u64,
    pub after: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymbolicSeal {
    axes: [u64; 4],
}

impl SymbolicSeal {
    pub const ZERO: Self = Self { axes: [0; 4] };

    pub const fn from_axes(sigma: u64, pi: u64, delta: u64, omega: u64) -> Self {
        Self {
            axes: [sigma, pi, delta, omega],
        }
    }

    pub const fn axes(self) -> [u64; 4] {
        self.axes
    }

    pub const fn sigma(self) -> u64 {
        self.axes[0]
    }

    pub const fn pi(self) -> u64 {
        self.axes[1]
    }

    pub const fn delta(self) -> u64 {
        self.axes[2]
    }

    pub const fn omega(self) -> u64 {
        self.axes[3]
    }

    pub fn to_bytes(self) -> [u8; 32] {
        let mut out = [0_u8; 32];
        for (index, axis) in self.axes.into_iter().enumerate() {
            out[index * 8..index * 8 + 8].copy_from_slice(&axis.to_le_bytes());
        }
        out
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        let mut axes = [0_u64; 4];
        for (index, axis) in axes.iter_mut().enumerate() {
            *axis = u64::from_le_bytes(
                bytes[index * 8..index * 8 + 8]
                    .try_into()
                    .expect("symbolic seal axis width"),
            );
        }
        Self { axes }
    }

    pub fn pretty(self, domain: &str) -> String {
        format!(
            "⟦{domain} ∷ Σ{:016x} ⊗ Π{:016x} ⊗ Δ{:016x} ⊗ Ω{:016x}⟧",
            self.sigma(),
            self.pi(),
            self.delta(),
            self.omega()
        )
    }

    pub fn changed_axes(self, other: Self) -> Vec<SealAxis> {
        SealAxis::ALL
            .into_iter()
            .zip(self.axes.into_iter().zip(other.axes))
            .filter_map(|(axis, (left, right))| (left != right).then_some(axis))
            .collect()
    }

    pub fn explain_changes(self, other: Self) -> Vec<AxisChange> {
        SealAxis::ALL
            .into_iter()
            .zip(self.axes.into_iter().zip(other.axes))
            .filter_map(|(axis, (before, after))| {
                (before != after).then_some(AxisChange {
                    axis,
                    before,
                    after,
                })
            })
            .collect()
    }

    pub fn fast_matches(self, other: Self) -> bool {
        self == other
    }
}

impl fmt::Display for SymbolicSeal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "s1:{:016x}.{:016x}.{:016x}.{:016x}",
            self.sigma(),
            self.pi(),
            self.delta(),
            self.omega()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseSealError;

impl FromStr for SymbolicSeal {
    type Err = ParseSealError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let body = value.strip_prefix("s1:").ok_or(ParseSealError)?;
        let mut parts = body.split('.');
        let mut axes = [0_u64; 4];
        for axis in &mut axes {
            let part = parts.next().ok_or(ParseSealError)?;
            if part.len() != 16 {
                return Err(ParseSealError);
            }
            *axis = u64::from_str_radix(part, 16).map_err(|_| ParseSealError)?;
        }
        if parts.next().is_some() {
            return Err(ParseSealError);
        }
        Ok(Self { axes })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicIdentity {
    domain: Box<str>,
    canonical: Box<[u8]>,
}

impl SymbolicIdentity {
    pub fn new(domain: impl Into<Box<str>>, canonical: impl Into<Box<[u8]>>) -> Self {
        Self {
            domain: domain.into(),
            canonical: canonical.into(),
        }
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub fn canonical(&self) -> &[u8] {
        &self.canonical
    }

    pub fn seal(&self) -> SymbolicSeal {
        let mut composer = Composer::new(&self.domain);
        composer.field("canonical", &self.canonical);
        composer.finish()
    }

    pub fn verify_exact(&self, domain: &str, canonical: &[u8]) -> bool {
        self.domain.as_ref() == domain && self.canonical.as_ref() == canonical
    }

    pub fn to_exact_bytes(&self) -> Vec<u8> {
        let domain_len: u32 = self
            .domain
            .len()
            .try_into()
            .expect("symbolic identity domain exceeds u32");
        let canonical_len: u64 = self
            .canonical
            .len()
            .try_into()
            .expect("symbolic identity payload exceeds u64");
        let mut out = Vec::with_capacity(
            IDENTITY_MAGIC.len() + 4 + 8 + self.domain.len() + self.canonical.len(),
        );
        out.extend_from_slice(IDENTITY_MAGIC);
        out.extend_from_slice(&domain_len.to_le_bytes());
        out.extend_from_slice(&canonical_len.to_le_bytes());
        out.extend_from_slice(self.domain.as_bytes());
        out.extend_from_slice(&self.canonical);
        out
    }

    pub fn from_exact_bytes(bytes: &[u8]) -> Result<Self, IdentityDecodeError> {
        const HEADER: usize = 5 + 4 + 8;
        if bytes.len() < HEADER {
            return Err(IdentityDecodeError::Truncated);
        }
        if &bytes[..5] != IDENTITY_MAGIC {
            return Err(IdentityDecodeError::BadMagic);
        }
        let domain_len = u32::from_le_bytes(
            bytes[5..9]
                .try_into()
                .map_err(|_| IdentityDecodeError::Truncated)?,
        ) as usize;
        let canonical_len = u64::from_le_bytes(
            bytes[9..17]
                .try_into()
                .map_err(|_| IdentityDecodeError::Truncated)?,
        );
        let canonical_len: usize = canonical_len
            .try_into()
            .map_err(|_| IdentityDecodeError::LengthOverflow)?;
        let domain_end = HEADER
            .checked_add(domain_len)
            .ok_or(IdentityDecodeError::LengthOverflow)?;
        let expected = domain_end
            .checked_add(canonical_len)
            .ok_or(IdentityDecodeError::LengthOverflow)?;
        if bytes.len() < expected {
            return Err(IdentityDecodeError::Truncated);
        }
        if bytes.len() > expected {
            return Err(IdentityDecodeError::TrailingBytes);
        }
        let domain = core::str::from_utf8(&bytes[HEADER..domain_end])
            .map_err(|_| IdentityDecodeError::InvalidDomain)?;
        Ok(Self::new(domain, &bytes[domain_end..]))
    }

    pub fn pretty(&self) -> String {
        format!(
            "{} ∣ |exact|={}B",
            self.seal().pretty(&self.domain),
            self.canonical.len()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentityDecodeError {
    Truncated,
    BadMagic,
    InvalidDomain,
    LengthOverflow,
    TrailingBytes,
}

pub fn exact_identity(domain: &str, canonical: &[u8]) -> SymbolicIdentity {
    SymbolicIdentity::new(domain, canonical)
}

pub fn seal_bytes(domain: &str, canonical: &[u8]) -> SymbolicSeal {
    exact_identity(domain, canonical).seal()
}

#[derive(Debug, Clone)]
pub struct Composer {
    axes: [u64; 4],
    position: u64,
    previous: u64,
    segments: u64,
}

impl Composer {
    pub fn new(domain: &str) -> Self {
        let mut composer = Self {
            axes: [
                0x5359_4d42_4f4c_4943,
                0x4c36_3453_4541_4c31,
                0x4f52_4445_5245_4431,
                0x424f_554e_4441_5259,
            ],
            position: 0,
            previous: 0,
            segments: 0,
        };
        composer.boundary(0x01);
        composer.raw(domain.as_bytes());
        composer.boundary(0x02);
        composer
    }

    pub fn field(&mut self, label: &str, bytes: &[u8]) -> &mut Self {
        self.boundary(0x10);
        self.length(label.len());
        self.raw(label.as_bytes());
        self.boundary(0x11);
        self.length(bytes.len());
        self.raw(bytes);
        self.boundary(0x12);
        self
    }

    pub fn u64(&mut self, label: &str, value: u64) -> &mut Self {
        self.field(label, &value.to_le_bytes())
    }

    pub fn u32(&mut self, label: &str, value: u32) -> &mut Self {
        self.field(label, &value.to_le_bytes())
    }

    pub fn u16(&mut self, label: &str, value: u16) -> &mut Self {
        self.field(label, &value.to_le_bytes())
    }

    pub fn u8(&mut self, label: &str, value: u8) -> &mut Self {
        self.field(label, &[value])
    }

    pub fn ordered(&mut self, label: &str, parts: &[SymbolicSeal]) -> &mut Self {
        self.boundary(0x20);
        self.length(label.len());
        self.raw(label.as_bytes());
        self.length(parts.len());
        for part in parts {
            self.boundary(0x21);
            self.raw(&part.to_bytes());
        }
        self.boundary(0x22);
        self
    }

    pub fn commutative(&mut self, label: &str, parts: &[SymbolicSeal]) -> &mut Self {
        let mut sorted = parts.to_vec();
        sorted.sort();
        self.boundary(0x30);
        self.length(label.len());
        self.raw(label.as_bytes());
        self.length(sorted.len());
        for part in sorted {
            self.boundary(0x31);
            self.raw(&part.to_bytes());
        }
        self.boundary(0x32);
        self
    }

    pub fn derive(
        domain: &str,
        relation: &str,
        source: SymbolicSeal,
        result: SymbolicSeal,
    ) -> SymbolicSeal {
        let mut composer = Self::new(domain);
        composer.field("relation", relation.as_bytes());
        composer.ordered("derivation", &[source, result]);
        composer.finish()
    }

    pub fn finish(mut self) -> SymbolicSeal {
        self.boundary(0xff);
        self.u64("length", self.position);
        self.u64("segments", self.segments);
        SymbolicSeal { axes: self.axes }
    }

    fn length(&mut self, value: usize) {
        self.raw(&(value as u64).to_le_bytes());
    }

    fn boundary(&mut self, marker: u8) {
        self.segments = self.segments.wrapping_add(1);
        self.absorb(u64::from(marker) + 0x100);
    }

    fn raw(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.absorb(u64::from(byte) + 1);
        }
    }

    fn absorb(&mut self, token: u64) {
        let index = self.position.wrapping_add(1);
        let sigma_term = mul_mod(token, add_mod(index, BASE_SIGMA));
        self.axes[0] = add_mod(self.axes[0], sigma_term);

        self.axes[1] = mul_mod(add_mod(self.axes[1], token), BASE_PI);

        let transition = if self.position == 0 {
            token
        } else if token >= self.previous {
            token - self.previous
        } else {
            MODULUS - (self.previous - token)
        };
        self.axes[2] = add_mod(
            mul_mod(self.axes[2], BASE_DELTA),
            mul_mod(transition, transition),
        );

        let omega_input = add_mod(add_mod(self.axes[3], token), index);
        self.axes[3] = add_mod(
            mul_mod(mul_mod(omega_input, omega_input), BASE_OMEGA),
            self.segments % MODULUS,
        );

        self.previous = token;
        self.position = index;
    }
}

fn add_mod(left: u64, right: u64) -> u64 {
    ((u128::from(left) + u128::from(right)) % u128::from(MODULUS)) as u64
}

fn mul_mod(left: u64, right: u64) -> u64 {
    ((u128::from(left) * u128::from(right)) % u128::from(MODULUS)) as u64
}
