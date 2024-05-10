use crate::{
    api::{constants, interfaces::constants::CanisterName},
    guard::caller_is_admin_controller,
};
use candid::Principal;
use ic_cdk::update;

// Set the new Minter Canister IDs
#[update(guard = "caller_is_admin_controller")]
fn set_canister_id(key: CanisterName, value: Principal) {
    constants::set_canister_id(key, value);
}
