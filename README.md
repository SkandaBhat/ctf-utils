# ctf-utils

Rust helpers for working with UMA Conditional Token Framework identifiers. The
crate mirrors the functionality of `github.com/polymarket/go-ctf-utils`, using
[`alloy-primitives`](https://docs.rs/alloy-primitives/) for Ethereum-native
types.

## Quick Start

```rust
use alloy_primitives::{address, U256};
use ctf_utils::{
    calculate_position_id, calculate_question_id, create_ancillary_data,
    get_condition_id_with_defaults,
};

let collateral = address!("0x2791bca1f2de4661ed88a30c99a7a9449aa84174");
let outcomes = ["Yes", "No"];

let ancillary = create_ancillary_data(
    "ETH greater than 10000?",
    "Will ETH trade above 10k USDC by 31 Dec 2024 on the Uniswap V3 5bps pool?",
    address!("0x6d8c4e9adf5748af82dabe2c6225207770d6b4fa"),
    outcomes,
)?;

let question_id = calculate_question_id(&ancillary);
let condition_id = get_condition_id_with_defaults(question_id);
let long_id: U256 = calculate_position_id(collateral, condition_id, 0);
let short_id: U256 = calculate_position_id(collateral, condition_id, 1);

println!("UMA question id  : {question_id}");
println!("Condition id     : {condition_id}");
println!("Long token id    : {long_id}");
println!("Short token id   : {short_id}");
# Ok::<(), ctf_utils::AncillaryError>(())
```

## Feature Flags

The crate defaults to `std`. Disable default features for `no_std` ancillary and
hash helpers. The `position` module (and `calculate_position_id(s)`) requires
`std` because it uses `num-bigint`.

## Testing

Run the unit tests to confirm parity with the Go helpers:

```bash
cargo test
```
