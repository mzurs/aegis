use candid::Principal;
use ic_cdk::query;

use crate::{
    api::{constants, interfaces::constants::CanisterName},
    guard::caller_is_admin_controller,
};

// Get the cosntants
#[query(guard = "caller_is_admin_controller")]
fn get_canister_id(key: CanisterName) -> Principal {
    constants::get_canister_id(key)
}
