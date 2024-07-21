use candid::Principal;
use ic_cdk::query;

use crate::{
    api::{constants, interfaces::constants::IcrcAsset, stake::stake},
    guard::caller_is_admin_controller,
};

// Get the cosntants
#[query(guard = "caller_is_admin_controller")]
fn get_canister_id(key: IcrcAsset) -> Principal {
    constants::get_ledger_canister_id(key)
}

#[query]
fn get_min_staking_delay() -> u64 {
    stake::get_min_stake_delay()
}
