
// utils.rs - Token helpers, SPL compatibility, rounding, re-exporting U64F64

use crate::u64f64::U64F64;

/// Converts a raw token amount to its UI display value using decimals
pub fn amount_to_ui_amount(amount: u64, decimals: u8) -> f64 {
    amount as f64 / 10f64.powi(decimals as i32)
}

/// Converts a UI display value to raw token amount using decimals
pub fn ui_amount_to_amount(ui: f64, decimals: u8) -> u64 {
    (ui * 10f64.powi(decimals as i32)).round() as u64
}

/// Rounds up a u128 to the nearest multiple of precision
pub fn round_up(value: u128, precision: u128) -> u128 {
    ((value + precision - 1) / precision) * precision
}

/// Rounds down a u128 to the nearest multiple of precision
pub fn round_down(value: u128, precision: u128) -> u128 {
    (value / precision) * precision
}
