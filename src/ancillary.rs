//! Helpers for constructing UMA ancillary data blobs.

use alloy_primitives::hex;
use alloy_primitives::{Address, Bytes};
use thiserror::Error;

/// Ancillary data format matching the upstream Go implementation.
const ANC_DATA_FMT: &str =
    "q: title: {title}, description: {description} res_data: {res_data},initializer:{initializer}";
/// Result data format with explicit outcome mapping.
const RES_DATA_FMT: &str = "p1: 0, p2: 1, p3: 0.5. Where p1 corresponds to {neg}, p2 to {pos}, p3 to unknown/50-50. Updates made by the question creator via the bulletin board at 0x6A9D222616C90FcA5754cd1333cFD9b7fb6a4F74 as described by https://polygonscan.com/tx/0xa14f01b115c4913624fc3f508f960f4dea252758e73c28f5f07f8e19d7bca066 should be considered.";

/// Errors that can occur while assembling ancillary data.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AncillaryError {
    /// Outcomes must contain exactly two entries.
    #[error("invalid number of outcomes: expected {expected}, found {found}")]
    InvalidOutcomeCount { expected: usize, found: usize },
    /// Each outcome label must be non-empty.
    #[error("outcome label at index {index} is empty")]
    EmptyOutcome { index: usize },
}

/// Construct UMA ancillary data bytes from user-provided metadata.
pub fn create_ancillary_data(
    title: &str,
    description: &str,
    creator: Address,
    outcomes: [&str; 2],
) -> Result<Bytes, AncillaryError> {
    create_ancillary_data_from_slice(title, description, creator, &outcomes)
}

/// Slice-based variant that performs validation before formatting.
pub fn create_ancillary_data_from_slice(
    title: &str,
    description: &str,
    creator: Address,
    outcomes: &[&str],
) -> Result<Bytes, AncillaryError> {
    if outcomes.len() != 2 {
        return Err(AncillaryError::InvalidOutcomeCount {
            expected: 2,
            found: outcomes.len(),
        });
    }

    for (index, outcome) in outcomes.iter().enumerate() {
        if outcome.trim().is_empty() {
            return Err(AncillaryError::EmptyOutcome { index });
        }
    }

    // Safety: we validated the slice length above.
    let outcomes_pair = [outcomes[0], outcomes[1]];
    let res_data = format_res_data(outcomes_pair);
    let initializer = format_initializer(creator);

    let ancillary = ANC_DATA_FMT
        .replace("{title}", title)
        .replace("{description}", description)
        .replace("{res_data}", &res_data)
        .replace("{initializer}", &initializer);

    Ok(Bytes::from(ancillary.into_bytes()))
}

fn format_initializer(address: Address) -> String {
    hex::encode(address.as_slice())
}

fn format_res_data(outcomes: [&str; 2]) -> String {
    RES_DATA_FMT
        .replace("{neg}", outcomes[1])
        .replace("{pos}", outcomes[0])
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::address;

    #[test]
    fn ancillary_data_round_trip() {
        let title = "ETH greater than 10000?";
        let description = "Will the price of ETH on the ETH/USDC Uniswap V3 5bps pool be greater than 10000 USDC by December 31st 2024?";
        let outcomes = ["Yes", "No"];
        let creator = address!("0x6d8c4e9adf5748af82dabe2c6225207770d6b4fa");

        let bytes = create_ancillary_data(title, description, creator, outcomes).unwrap();
        let actual = core::str::from_utf8(&bytes).unwrap();
        let expected = "q: title: ETH greater than 10000?, description: Will the price of ETH on the ETH/USDC Uniswap V3 5bps pool be greater than 10000 USDC by December 31st 2024? res_data: p1: 0, p2: 1, p3: 0.5. Where p1 corresponds to No, p2 to Yes, p3 to unknown/50-50. Updates made by the question creator via the bulletin board at 0x6A9D222616C90FcA5754cd1333cFD9b7fb6a4F74 as described by https://polygonscan.com/tx/0xa14f01b115c4913624fc3f508f960f4dea252758e73c28f5f07f8e19d7bca066 should be considered.,initializer:6d8c4e9adf5748af82dabe2c6225207770d6b4fa";
        assert_eq!(expected, actual);
    }

    #[test]
    fn ancillary_rejects_invalid_outcome_count() {
        let result = create_ancillary_data_from_slice("title", "desc", Address::ZERO, &["Yes"]);
        assert_eq!(
            result,
            Err(AncillaryError::InvalidOutcomeCount {
                expected: 2,
                found: 1
            })
        );
    }

    #[test]
    fn ancillary_rejects_empty_outcome_labels() {
        let result =
            create_ancillary_data_from_slice("title", "desc", Address::ZERO, &["Yes", " "]);
        assert_eq!(result, Err(AncillaryError::EmptyOutcome { index: 1 }));
    }
}
