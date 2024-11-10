use candid::Nat;
use ethers_core::utils::format_units;
use ic_utils::biguint_u128::biguint_to_u128_func;

use crate::api::interfaces::options_assets::OptionsAssets;

const BASE_OF_EIGHT: u64 = 100_000_000;
pub const BASE_OF_EIGHTEEN: u128 = 1_000_000_000_000_000_000;

pub fn convert_asset_amount_to_human(asset: OptionsAssets, amount: Nat) -> Nat {
    match asset {
        OptionsAssets::ETH => (format_units(biguint_to_u128_func(&amount.0).unwrap(), "ether").unwrap())
            .parse()
            .expect("Not a Valid Amount"),
        _ => amount / Nat::from(BASE_OF_EIGHT as u64),
    }
}
