//! UMA question identifier helpers.

use alloy_primitives::{B256, keccak256};

/// Calculate the UMA question identifier by hashing the ancillary data bytes.
#[inline]
pub fn calculate_question_id(ancillary: impl AsRef<[u8]>) -> B256 {
    keccak256(ancillary.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ancillary::create_ancillary_data;
    use alloy_primitives::address;

    #[test]
    fn question_id_expected_value() {
        let title = "ETH greater than 10000?";
        let description = "Will the price of ETH on the ETH/USDC Uniswap V3 5bps pool be greater than 10000 USDC by December 31st 2024?";
        let outcomes = ["Yes", "No"];
        let creator = address!("0x6d8c4e9adf5748af82dabe2c6225207770d6b4fa");

        let ancillary = create_ancillary_data(title, description, creator, outcomes).unwrap();
        let question_id = calculate_question_id(&ancillary);
        let expected = "0x01741d802f72305df80da4d6e8ecd3a50287f09ec62edb3bd95ac7c395b2f5ef";

        assert_eq!(expected, question_id.to_string());
    }
}
