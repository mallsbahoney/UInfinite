
// math.rs - Solana-compatible math types and traits

use core::ops::*;
use core::fmt;

pub const WAD: u128 = 1_000_000_000_000_000_000;
pub const HALF_WAD: u128 = WAD / 2;
pub const SCALE: u128 = 1_000_000_000_000_000_000;
pub const BIPS_SCALER: u128 = 10_000;
pub const PERCENT_SCALER: u128 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathError {
    Overflow,
    Underflow,
    DivisionByZero,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Decimal(pub u128); // 18-decimal fixed point

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rate(pub u128); // 18-decimal fixed point

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct U128(pub [u8; 16]); // Little endian

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct U192(pub [u8; 24]); // Little endian

pub trait TryAdd: Sized {
    fn try_add(self, rhs: Self) -> Result<Self, MathError>;
}
pub trait TrySub: Sized {
    fn try_sub(self, rhs: Self) -> Result<Self, MathError>;
}
pub trait TryMul: Sized {
    fn try_mul(self, rhs: Self) -> Result<Self, MathError>;
}
pub trait TryDiv: Sized {
    fn try_div(self, rhs: Self) -> Result<Self, MathError>;
}

impl TryAdd for Decimal {
    fn try_add(self, rhs: Self) -> Result<Self, MathError> {
        self.0.checked_add(rhs.0).map(Decimal).ok_or(MathError::Overflow)
    }
}
impl TrySub for Decimal {
    fn try_sub(self, rhs: Self) -> Result<Self, MathError> {
        self.0.checked_sub(rhs.0).map(Decimal).ok_or(MathError::Underflow)
    }
}
impl TryMul for Decimal {
    fn try_mul(self, rhs: Self) -> Result<Self, MathError> {
        self.0.checked_mul(rhs.0).and_then(|x| x.checked_div(SCALE)).map(Decimal).ok_or(MathError::Overflow)
    }
}
impl TryDiv for Decimal {
    fn try_div(self, rhs: Self) -> Result<Self, MathError> {
        if rhs.0 == 0 {
            Err(MathError::DivisionByZero)
        } else {
            self.0.checked_mul(SCALE).and_then(|x| x.checked_div(rhs.0)).map(Decimal).ok_or(MathError::Overflow)
        }
    }
}
