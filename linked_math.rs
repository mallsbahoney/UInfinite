
// Linked math utilities for UInfinite shard reconstruction

use alloc::vec::Vec;
use crate::u_infinite::UInfinite;

pub fn link_chunks(chunks: &[UInfinite]) -> Result<UInfinite, &'static str> {
    let mut result = UInfinite::from_string("0")?;
    for chunk in chunks {
        result = result.add(chunk)?;
    }
    Ok(result)
}

pub fn chain_mul(chunks: &[UInfinite], multiplier: &UInfinite) -> Result<UInfinite, &'static str> {
    let mut result = UInfinite::from_string("0")?;
    for chunk in chunks {
        result = result.add(&chunk.mul(multiplier)?)?;
    }
    Ok(result)
}
