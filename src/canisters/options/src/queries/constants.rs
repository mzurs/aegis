use candid::Principal;
use ic_cdk::query;

use crate::api::{
    interfaces::{constants::CanisterName, options_assets::OptionsAssetsIcrc},
    utils::constants,
};

#[query]
pub fn get_ledger_canister_id(asset: OptionsAssetsIcrc) -> Principal {
    constants::get_icrc_ledger_canister_id(asset)
}

#[query]
pub fn get_canister_id(canister: CanisterName) -> Principal {
    constants::get_canister_id(canister)
}
