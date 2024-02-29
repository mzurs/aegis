use account::{_create_account, _get_account, _update_account_user_name};
use enums::{Metric, MetricValues};
use guard::caller_is_admin_controller;
use ic_cdk_macros::{export_candid, query, update};

pub mod account;
pub mod enums;
pub mod guard;
pub mod impls;
pub mod memory;
pub mod types;
pub mod utils;

use types::states::Account;
use utils::_get_metrics;

/// Function to get User Account Details

#[query]
fn get_account() -> Option<Account> {
    _get_account()
}

/// Get the Account Canister Metrics

#[query(guard = "caller_is_admin_controller")]
fn get_metrics(args: Metric) -> MetricValues {
    _get_metrics(args)
}


// =======================================================================================================================

/// Function to create User Account

#[update]
async fn create_account() -> Result<bool, String> {
    _create_account().await
}

/// Function to update user account name

#[update]
fn update_account_user_name(user_name: String) -> Result<(), String> {
    _update_account_user_name(user_name)
}

export_candid!();
