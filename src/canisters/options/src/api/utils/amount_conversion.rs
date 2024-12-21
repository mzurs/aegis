use candid::Nat;
use ethers_core::{
    types::U256,
    utils::{format_units, parse_units},
};
use ic_utils::{biguint_f64::f64_to_biguint, biguint_u128::biguint_to_u128_func};
use num_bigint::BigUint;

use crate::api::interfaces::options_assets::OptionsAssets;

const BASE_OF_EIGHT: u64 = 100_000_000;
pub const BASE_OF_EIGHTEEN: u128 = 1_000_000_000_000_000_000;
const BASE_OF_XRC: u64 = 1_000_000_000;

pub fn convert_asset_amount_to_human(asset: OptionsAssets, amount: Nat) -> f64 {
    match asset {
        OptionsAssets::ETH => match biguint_to_u128_func(&amount.0) {
            Ok(res) => {
                let eth: String = format_units(res, "ether").unwrap();

                eth.parse::<f64>().unwrap()
            }
            Err(err) => {
                ic_cdk::println!("Error: {}", err);
                0.00 as f64
            }
        },
        OptionsAssets::ICRC(crate::api::interfaces::options_assets::OptionsAssetsIcrc::CKETH) => {
            match biguint_to_u128_func(&amount.0) {
                Ok(res) => {
                    let eth: String = format_units(res, "ether").unwrap();

                    eth.parse::<f64>().unwrap()
                }
                Err(err) => {
                    ic_cdk::println!("Error: {}", err);
                    0.00 as f64
                }
            }
        }
        OptionsAssets::ICRC(crate::api::interfaces::options_assets::OptionsAssetsIcrc::CKUSDT) => {
            match biguint_to_u128_func(&amount.0) {
                Ok(res) => {
                    let eth: String = format_units(res, "ether").unwrap();

                    eth.parse::<f64>().unwrap()
                }
                Err(err) => {
                    ic_cdk::println!("Error: {}", err);
                    0.00 as f64
                }
            }
        }
        // _ => amount / Nat::from(BASE_OF_EIGHT as u64),
        _ => match biguint_to_u128_func(&amount.0) {
            Ok(res) => (res as f64) / BASE_OF_EIGHT as f64,
            Err(err) => {
                ic_cdk::println!("Error: {}", err);
                0.00 as f64
            }
        },
    }
    // Nat::from(0 as u32)
}

pub fn convert_asset_amount_to_non_human(asset: OptionsAssets, value: f64) -> Nat {
    match asset {
        OptionsAssets::ETH => Nat::from(f64_to_biguint((value * (BASE_OF_EIGHTEEN as f64)).trunc()).unwrap()),
        OptionsAssets::ICRC(crate::api::interfaces::options_assets::OptionsAssetsIcrc::CKETH) => {
            Nat::from(f64_to_biguint((value * (BASE_OF_EIGHTEEN as f64)).trunc()).unwrap())
        }
        OptionsAssets::ICRC(crate::api::interfaces::options_assets::OptionsAssetsIcrc::CKUSDT) => {
            Nat::from(f64_to_biguint((value * (BASE_OF_EIGHTEEN as f64)).trunc()).unwrap())
        }
        _ => Nat::from(f64_to_biguint((value * (BASE_OF_EIGHT as f64)).trunc()).unwrap()),
    }
}

pub fn convert_premium_amount_to_non_humans(__asset: OptionsAssets, premium: f64) -> Nat {
    let res: U256 = parse_units(premium, "wei").unwrap().into();

    Nat::from(saturating_to_u64(res))
}

// Saturating conversion to u64 (caps at u64::MAX)
pub fn saturating_to_u64(val: U256) -> u64 {
    if val.0[1] != 0 || val.0[2] != 0 || val.0[3] != 0 {
        u64::MAX
    } else {
        val.0[0]
    }
}

pub fn u256_to_biguint(val: U256) -> BigUint {
    let mut biguint = BigUint::from(0u64);

    // Iterate through the 64-bit words and add them to the BigUint, considering their positions
    for (i, &word) in val.0.iter().enumerate() {
        let part = BigUint::from(word);
        biguint += part << (i * 64); // Shift by the position in bits
    }

    biguint
}

pub fn convert_xrc_human_to_non_humans(value: f64) -> Nat {
    Nat::from(f64_to_biguint(value * BASE_OF_XRC as f64).unwrap())
}

pub fn convert_xrc_non_human_to_human(value: Nat) -> f64 {
    biguint_to_u128_func(&value.0).unwrap() as f64 / BASE_OF_XRC as f64
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn convert_asset_amount_to_human_test() {
        let res = convert_asset_amount_to_human(OptionsAssets::ETH, Nat::from(600000000000000000 as u64));
        assert_eq!(res, 0.6);
    }

    #[test]
    fn convert_asset_amount_to_non_human_test() {
        let res = convert_asset_amount_to_non_human(OptionsAssets::ETH, 0.6 as f64);
        assert_eq!(res, Nat::from(600000000000000000 as u64));
    }
}
