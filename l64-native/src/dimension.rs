const AXIS_COUNT: usize = 7;
const USED_BITS: u32 = (AXIS_COUNT as u32) * 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Dimension(u64);

impl Dimension {
    pub const DIMENSIONLESS: Self = Self(0);
    pub const LENGTH: Self = Self(1);
    pub const MASS: Self = Self(1 << 8);
    pub const TIME: Self = Self(1 << 16);

    pub fn new(exponents: [i8; AXIS_COUNT]) -> Self {
        let mut bits = 0_u64;
        for (axis, exponent) in exponents.into_iter().enumerate() {
            bits |= u64::from(exponent as u8) << (axis * 8);
        }
        Self(bits)
    }

    pub fn exponents(self) -> [i8; AXIS_COUNT] {
        let mut exponents = [0_i8; AXIS_COUNT];
        for (axis, exponent) in exponents.iter_mut().enumerate() {
            *exponent = ((self.0 >> (axis * 8)) as u8) as i8;
        }
        exponents
    }

    pub fn bits(self) -> u64 {
        self.0
    }

    pub(crate) fn from_bits(bits: u64) -> Option<Self> {
        (bits >> USED_BITS == 0).then_some(Self(bits))
    }

    pub fn multiply(self, other: Self) -> Option<Self> {
        self.combine(other, i8::checked_add)
    }

    pub fn divide(self, other: Self) -> Option<Self> {
        self.combine(other, i8::checked_sub)
    }

    pub fn square_root(self) -> Option<Self> {
        let mut result = [0_i8; AXIS_COUNT];
        for (axis, exponent) in self.exponents().into_iter().enumerate() {
            if exponent % 2 != 0 {
                return None;
            }
            result[axis] = exponent / 2;
        }
        Some(Self::new(result))
    }

    fn combine(self, other: Self, operation: fn(i8, i8) -> Option<i8>) -> Option<Self> {
        let left = self.exponents();
        let right = other.exponents();
        let mut result = [0_i8; AXIS_COUNT];
        for axis in 0..AXIS_COUNT {
            result[axis] = operation(left[axis], right[axis])?;
        }
        Some(Self::new(result))
    }
}
