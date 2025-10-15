#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

/// Ancillary data formatting utilities.
pub mod ancillary;
/// Condition identifier helpers (Keccak-based).
pub mod condition;
/// Shared UMA/CTF constants.
pub mod constants;
/// UMA question identifier helper.
pub mod question;

/// Create UMA ancillary data bytes from human-readable inputs.
pub use ancillary::AncillaryError;
/// Validation errors that can occur during ancillary data construction.
pub use ancillary::create_ancillary_data;
/// Compute a condition id with explicit oracle/outcome slot count.
pub use condition::get_condition_id;
/// Compute a condition id using the UMA oracle and default 2-slot outcome count.
pub use condition::get_condition_id_with_defaults;
/// Default number of outcomes (binary market).
pub use constants::DEFAULT_OUTCOME_SLOT_COUNT;
/// UMA oracle address for Polymarket markets.
pub use constants::UMA_ORACLE;
/// Retrieve the default outcome slot count.
pub use constants::outcome_slot_count;
/// Calculate the UMA question id by hashing ancillary bytes.
pub use question::calculate_question_id;

#[cfg(feature = "std")]
/// ERC1155 position identifier helpers (std-only).
pub mod position;

#[cfg(feature = "std")]
/// Compute the ERC1155 token id for a specific outcome.
pub use position::calculate_position_id;
#[cfg(feature = "std")]
/// Compute both ERC1155 token ids for a condition (long/short).
pub use position::calculate_position_ids;
