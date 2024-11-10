use candid::Principal;
use ic_cdk::update;

use crate::api::{interfaces::options_assets::OptionsAssetsIcrc, utils::constants};

#[update]
pub fn set_ledger_canister_id(asset: OptionsAssetsIcrc, principal: Principal) {
    constants::set_icrc_ledger_canister_id(asset, principal)
}
