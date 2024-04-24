use ic_cdk::update;

use crate::api::interfaces::constants::{Constants, LedgerIds, MinterIds};
use crate::guard::caller_is_admin_controller;

// Set the new Ledger Canister IDs
#[update(guard = "caller_is_admin_controller")]
fn set_ledger_ids(ids: LedgerIds) {
    Constants::set_ledger_ids(ids)
}

// Set the new Minter Canister IDs
#[update(guard = "caller_is_admin_controller")]
fn set_minter_ids(ids: MinterIds) {
    Constants::set_minter_ids(ids)
}
