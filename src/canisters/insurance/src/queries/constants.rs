use candid::Principal;
use ic_cdk::update;

use crate::api::interface::insurance::InsuranceAssets;

#[update]
pub fn get_ledger_canister_id(asset: InsuranceAssets) -> Principal {
    crate::api::utils::constants::get_ledger_canister_id(asset)
}
