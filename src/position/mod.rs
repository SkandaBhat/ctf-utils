//! Position identifier helpers built for binary CTF markets.

use alloy_primitives::{Address, B256, U256, keccak256};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use once_cell::sync::Lazy;

const BN254_MODULUS_DECIMAL: &[u8] =
    b"21888242871839275222246405745257275088696311157297823662689037894645226208583";

static P_MODULUS: Lazy<BigUint> =
    Lazy::new(|| BigUint::parse_bytes(BN254_MODULUS_DECIMAL, 10).unwrap());
static CURVE_B: Lazy<BigUint> = Lazy::new(|| BigUint::from(3u32));
static ONE: Lazy<BigUint> = Lazy::new(BigUint::one);
static TOGGLE_BIT: Lazy<BigUint> = Lazy::new(|| BigUint::one() << 254usize);

fn p_modulus() -> &'static BigUint {
    &P_MODULUS
}

fn curve_b() -> &'static BigUint {
    &CURVE_B
}

fn one() -> &'static BigUint {
    &ONE
}

fn toggle_bit() -> &'static BigUint {
    &TOGGLE_BIT
}

fn be_bytes_to_biguint(bytes: &[u8]) -> BigUint {
    BigUint::from_bytes_be(bytes)
}

fn biguint_to_be_bytes32(value: &BigUint) -> [u8; 32] {
    let bytes = value.to_bytes_be();
    let mut out = [0u8; 32];
    let start = 32 - bytes.len();
    out[start..].copy_from_slice(&bytes);
    out
}

fn add_mod(a: &BigUint, b: &BigUint, modulus: &BigUint) -> BigUint {
    (a + b) % modulus
}

fn mul_mod(a: &BigUint, b: &BigUint, modulus: &BigUint) -> BigUint {
    (a * b) % modulus
}

fn sqrt_mod_p(a: &BigUint, modulus: &BigUint) -> BigUint {
    let exp = (modulus + one()) >> 2usize;
    a.modpow(&exp, modulus)
}

fn collection_payload(condition_id: B256, outcome_index: u8) -> [u8; 64] {
    let mut payload = [0u8; 64];
    payload[..32].copy_from_slice(condition_id.as_slice());

    let index_set = U256::from(1u64) << outcome_index;
    payload[32..].copy_from_slice(&index_set.to_be_bytes::<32>());

    payload
}

fn finalize_collection_hash(raw_hash: B256) -> B256 {
    let modulus = p_modulus();
    let b = curve_b();
    let mut x1 = be_bytes_to_biguint(raw_hash.as_slice());
    let odd = ((&x1 >> 255usize) & one()) == *one();

    x1 %= modulus;

    loop {
        x1 = add_mod(&x1, one(), modulus);
        let x1_sq = mul_mod(&x1, &x1, modulus);
        let x1_cu = mul_mod(&x1_sq, &x1, modulus);
        let yy = add_mod(&x1_cu, b, modulus);
        let mut y1 = sqrt_mod_p(&yy, modulus);
        if mul_mod(&y1, &y1, modulus) != yy {
            continue;
        }

        let y_is_even = (&y1 & one()).is_zero();
        if (odd && y_is_even) || (!odd && !y_is_even) {
            y1 = modulus - &y1;
        }

        if !((&y1 & one()).is_zero()) {
            let toggle = toggle_bit();
            x1 ^= toggle;
        }

        let encoded = biguint_to_be_bytes32(&x1);
        return B256::from(encoded);
    }
}

/// Compute the collection identifier hash (CTHelpers.getCollectionId analogue).
pub fn compute_collection_id_hash(condition_id: B256, outcome_index: u8) -> B256 {
    let payload = collection_payload(condition_id, outcome_index);
    let raw_hash = keccak256(payload);
    finalize_collection_hash(raw_hash)
}

fn compute_position_id_hash(collateral: Address, collection_id: B256) -> B256 {
    let mut payload = [0u8; 52];
    payload[..20].copy_from_slice(collateral.as_slice());
    payload[20..].copy_from_slice(collection_id.as_slice());

    keccak256(payload)
}

/// Calculate the ERC1155 position identifier for a specific outcome.
pub fn calculate_position_id(collateral: Address, condition_id: B256, outcome_index: u8) -> U256 {
    let collection_id = compute_collection_id_hash(condition_id, outcome_index);
    let position_hash = compute_position_id_hash(collateral, collection_id);
    U256::from_be_bytes(position_hash.into())
}

/// Convenience helper returning both long/short token identifiers.
pub fn calculate_position_ids(collateral: Address, condition_id: B256) -> [U256; 2] {
    [
        calculate_position_id(collateral, condition_id, 0),
        calculate_position_id(collateral, condition_id, 1),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ancillary::create_ancillary_data;
    use crate::condition::get_condition_id_with_defaults;
    use crate::question::calculate_question_id;
    use alloy_primitives::{address, b256};

    #[test]
    fn collection_ids_match_expected_values() {
        let condition_id =
            b256!("0x41771a29f1fa3b5ac743ddcf224017f802bd69152c1a65230ec666abfc22b708");
        let expected0 = b256!("0x63ecd1f555d88721e1d063640ec7719a904925b55bf9a8c3e9d1dddfa86b1a6b");
        let expected1 = b256!("0x610f95f837bfcb4a52eedebd551bbdd7f273edeb47ad140cfba8287e6fed1929");

        let actual0 = compute_collection_id_hash(condition_id, 0);
        let actual1 = compute_collection_id_hash(condition_id, 1);

        assert_eq!(expected0, actual0);
        assert_eq!(expected1, actual1);
    }

    #[test]
    fn position_ids_match_expected_values() {
        let condition_id =
            b256!("0x41771a29f1fa3b5ac743ddcf224017f802bd69152c1a65230ec666abfc22b708");
        let collateral = address!("0x2791bca1f2de4661ed88a30c99a7a9449aa84174");

        let collection0 = compute_collection_id_hash(condition_id, 0);
        let collection1 = compute_collection_id_hash(condition_id, 1);
        assert_eq!(
            alloy_primitives::hex::encode(collection0),
            "63ecd1f555d88721e1d063640ec7719a904925b55bf9a8c3e9d1dddfa86b1a6b"
        );
        assert_eq!(
            alloy_primitives::hex::encode(collection1),
            "610f95f837bfcb4a52eedebd551bbdd7f273edeb47ad140cfba8287e6fed1929"
        );

        let token0 = calculate_position_id(collateral, condition_id, 0);
        let token1 = calculate_position_id(collateral, condition_id, 1);

        assert_eq!(
            "87848146419241057657677458104196204655830537958664996373577118668208015365957",
            token0.to_string()
        );
        assert_eq!(
            "11831001752042525186810643219442170205387399550590230096735412415464402686073",
            token1.to_string()
        );
    }

    #[test]
    fn position_pipeline_produces_consistent_ids() {
        let title = "ETH greater than 10000?";
        let description = "Will the price of ETH on the ETH/USDC Uniswap V3 5bps pool be greater than 10000 USDC by December 31st 2024?";
        let outcomes = ["Yes", "No"];
        let creator = address!("0x6d8c4e9adf5748af82dabe2c6225207770d6b4fa");

        let ancillary = create_ancillary_data(title, description, creator, outcomes).unwrap();
        let question_id = calculate_question_id(&ancillary);
        let condition_id = get_condition_id_with_defaults(question_id);
        let collateral = address!("0x2791bca1f2de4661ed88a30c99a7a9449aa84174");

        let ids = calculate_position_ids(collateral, condition_id);
        assert_ne!(ids[0], ids[1]);
        assert!(ids[0] != U256::ZERO);
    }
}
