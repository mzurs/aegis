use candid::Principal;
use ic_cdk::update;

use crate::api::interface::insurance::InsuranceAssets;

#[update]
pub fn set_ledger_canister_id(asset: InsuranceAssets, principal: Principal) {
    crate::api::utils::constants::set_ledger_canister_id(asset, principal)
}
