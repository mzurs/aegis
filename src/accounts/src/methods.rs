use candid::{Nat, Principal};
use ic_cdk::{
    api::{call::CallResult, management_canister::bitcoin::BitcoinNetwork},
    export_candid, init, query, update,
};

use crate::{
    enums::{Metric, MetricValues},
    guard,
    ledgers::services::minter::{RetrieveBtcRet, UpdateBalanceRet},
    memory::{CONSTANTS, INIT_ARGS},
    types::states::{initialization_configs::InitArgs, Account, Constants},
    utils::{self, _principal_to_subaccount},
};
use guard::{account_exists, caller_is_admin_controller};

#[init]
fn init(args: InitArgs) -> () {
    let _ = INIT_ARGS
        .with(|_args| _args.borrow_mut().set(args))
        .expect("Unable to insert initialize args to stable memory");
}

#[query]
pub fn get_bitcoin_network() -> BitcoinNetwork {
    utils::get_bitcoin_network()
}

/// Function to get User Account Details
#[query]
fn get_account() -> Option<Account> {
    Account::get_account()
}

/// Get the Account Canister Metrics
#[query(guard = "caller_is_admin_controller")]
fn get_metrics(args: Metric) -> MetricValues {
    utils::get_metrics(args)
}

// ==========================================================================================================================

/// Function to create User Account
#[update(guard = "account_exists")]
async fn create_account() -> Result<bool, String> {
    Account::create_account().await
}

/// Function to update user account name
#[update]
fn update_account_user_name(user_name: String) -> Result<(), String> {
    Account::update_account_user_name(user_name)
}

#[query]
pub fn principal_to_subaccount(principal_id: Principal) -> [u8; 32] {
    _principal_to_subaccount(&principal_id)
}

#[query]
/// Get the constants for Account Canister
pub fn get_constants() -> Constants {
    let constants = CONSTANTS.with(|c| c.borrow().get().to_owned());
    constants
}

/*
     Macro to generate Candid Interface for the Account Canister
*/

export_candid!();
