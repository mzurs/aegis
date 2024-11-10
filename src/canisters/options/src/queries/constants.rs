use candid::Principal;
use ic_cdk::update;

use crate::api::{interfaces::options_assets::OptionsAssetsIcrc, utils::constants};

#[update]
pub fn get_ledger_canister_id(asset: OptionsAssetsIcrc) -> Principal {
    constants::get_icrc_ledger_canister_id(asset)
}
