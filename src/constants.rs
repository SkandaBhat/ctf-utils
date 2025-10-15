//! Shared constants derived from UMA/CTF configuration.

use alloy_primitives::{Address, U256, address};

/// UMA oracle address used by Polymarket markets.
pub const UMA_ORACLE: Address = address!("0x6A9D222616C90FcA5754cd1333cFD9b7fb6a4F74");

/// Default number of outcomes for binary markets (two outcomes).
pub const DEFAULT_OUTCOME_SLOT_COUNT: U256 = U256::from_limbs([2, 0, 0, 0]);

/// Returns the default outcome slot count for convenience.
#[inline]
pub const fn outcome_slot_count() -> U256 {
    DEFAULT_OUTCOME_SLOT_COUNT
}
