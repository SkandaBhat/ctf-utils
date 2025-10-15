//! Condition identifier helpers mirroring the Solidity/Go implementation.

use alloy_primitives::{Address, B256, U256, keccak256};

use crate::constants::{DEFAULT_OUTCOME_SLOT_COUNT, UMA_ORACLE};

/// Compute the CTF condition identifier with explicit parameters.
pub fn get_condition_id(oracle: Address, question_id: B256, outcome_slot_count: U256) -> B256 {
    let mut encoded = Vec::with_capacity(20 + 32 + 32);
    encoded.extend_from_slice(oracle.as_slice());
    encoded.extend_from_slice(question_id.as_slice());
    encoded.extend_from_slice(&outcome_slot_count.to_be_bytes::<32>());

    keccak256(&encoded)
}

/// Convenience wrapper that uses the UMA oracle and default 2-outcome slot count.
#[inline]
pub fn get_condition_id_with_defaults(question_id: B256) -> B256 {
    get_condition_id(UMA_ORACLE, question_id, DEFAULT_OUTCOME_SLOT_COUNT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ancillary::create_ancillary_data;
    use crate::question::calculate_question_id;
    use alloy_primitives::address;

    #[test]
    fn condition_id_expected_value() {
        let title = "ETH greater than 10000?";
        let description = "Will the price of ETH on the ETH/USDC Uniswap V3 5bps pool be greater than 10000 USDC by December 31st 2024?";
        let outcomes = ["Yes", "No"];
        let creator = address!("0x6d8c4e9adf5748af82dabe2c6225207770d6b4fa");

        let ancillary = create_ancillary_data(title, description, creator, outcomes).unwrap();
        let question_id = calculate_question_id(&ancillary);
        let condition_id = get_condition_id_with_defaults(question_id);

        let expected = "0x491b47c68ed1de5b01c359fd5d14a285b68af60b14ec7939acfee2afbfbb8ec8";
        assert_eq!(expected, condition_id.to_string());
    }
}
