use std::env;
use std::io::{self, Write};
use std::process;
use std::str::FromStr;

use alloy_primitives::{Address, B256};
use ctf_utils::calculate_position_id;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);

    let collateral_str = args.next().ok_or_else(|| usage())?;
    let condition_str = args.next().ok_or_else(|| usage())?;
    let outcome_str = args.next().ok_or_else(|| usage())?;

    // Ensure there are no extra args.
    if args.next().is_some() {
        return Err(usage());
    }

    let collateral = Address::from_str(collateral_str.trim())
        .map_err(|_| "invalid collateral address".to_owned())?;
    let condition_id =
        B256::from_str(condition_str.trim()).map_err(|_| "invalid condition id".to_owned())?;

    let outcome_index: u8 = outcome_str
        .trim()
        .parse::<u8>()
        .map_err(|_| "invalid outcome index".to_owned())?;
    if outcome_index > 1 {
        return Err("outcome index must be 0 or 1".into());
    }

    let position_id = calculate_position_id(collateral, condition_id, outcome_index);
    let hex = alloy_primitives::hex::encode(position_id.to_be_bytes::<32>());

    writeln!(io::stdout(), "{hex}")
        .map_err(|err| format!("failed to write result: {err}"))?;
    Ok(())
}

fn usage() -> String {
    "usage: ctf-utils-cli <collateral> <condition_id> <outcome_index>".to_owned()
}
