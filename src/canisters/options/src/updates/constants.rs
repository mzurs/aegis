use candid::Principal;
use ic_cdk::update;

use crate::api::{
    interfaces::{constants::CanisterName, options_assets::OptionsAssetsIcrc},
    utils::constants,
};

#[update]
pub fn set_ledger_canister_id(asset: OptionsAssetsIcrc, principal: Principal) {
    constants::set_icrc_ledger_canister_id(asset, principal)
}

#[update]
pub fn set_canister_id(canister: CanisterName, principal: Principal) {
    constants::set_canister_id(canister, principal)
}
