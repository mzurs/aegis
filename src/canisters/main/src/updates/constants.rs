use candid::Principal;
use ic_cdk::update;

use crate::api::stake::stake::{self};
use crate::api::{constants, interfaces::constants::IcrcAsset};
use crate::guard::caller_is_admin_controller;

// Set the new Minter Canister IDs
#[update(guard = "caller_is_admin_controller")]
fn set_canister_id(key: IcrcAsset, value: Principal) {
    constants::set_ledger_canister_id(key, value);
}

#[update]
fn set_min_staking_delay(timestamp_nanos: Option<u64>) -> u64 {
    stake::set_min_staking_delay(timestamp_nanos)
}
