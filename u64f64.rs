
// Pure-integer U64F64 fixed-point emulation (no floats, fully no_std compatible)

#[derive(Debug, Clone, Copy)]
pub struct U64F64(pub u128);

impl U64F64 {
    pub const FRACTIONAL_BITS: u32 = 64;
    pub const ONE: u128 = 1u128 << Self::FRACTIONAL_BITS;

    /// Creates a U64F64 from an integer
    pub fn from_integer(x: u64) -> Self {
        U64F64((x as u128) << Self::FRACTIONAL_BITS)
    }

    /// Converts U64F64 to an integer by truncating fractional bits
    pub fn to_integer(self) -> u64 {
        (self.0 >> Self::FRACTIONAL_BITS) as u64
    }

    /// Adds two U64F64 values
    pub fn add(self, other: U64F64) -> U64F64 {
        U64F64(self.0 + other.0)
    }

    /// Subtracts two U64F64 values
    pub fn sub(self, other: U64F64) -> U64F64 {
        U64F64(self.0 - other.0)
    }

    /// Multiplies two U64F64 values using manual 256-bit intermediate logic
    pub fn mul(self, other: U64F64) -> U64F64 {
        let a = self.0;
        let b = other.0;

        let a_lo = a & 0xFFFFFFFFFFFFFFFF;
        let a_hi = a >> 64;
        let b_lo = b & 0xFFFFFFFFFFFFFFFF;
        let b_hi = b >> 64;

        let lo_lo = a_lo * b_lo;
        let lo_hi = a_lo * b_hi;
        let hi_lo = a_hi * b_lo;
        let hi_hi = a_hi * b_hi;

        let mid = (lo_hi + hi_lo) << 64;
        let upper = hi_hi << 128;

        let result_256 = upper + mid + lo_lo;
        let result = result_256 >> Self::FRACTIONAL_BITS;

        U64F64(result)
    }

    /// Divides two U64F64 values using shift-first method
    pub fn div(self, other: U64F64) -> U64F64 {
        let numerator = self.0 << Self::FRACTIONAL_BITS;
        U64F64(numerator / other.0)
    }
}
