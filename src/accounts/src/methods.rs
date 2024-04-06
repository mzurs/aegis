use candid::{Nat, Principal};
use ic_cdk::{
    api::{call::CallResult, management_canister::bitcoin::BitcoinNetwork},
    export_candid, init, query, update,
};

use crate::{
    enums::{Metric, MetricValues},
    guard,
    ledgers::{
        services::{
            ledger::Result_,
            minter::{RetrieveBtcRet, UpdateBalanceRet},
        },
        types::ICRCLedgerType,
    },
    memory::init_stable_states,
    types::states::{
        initialization_configs::InitArgs, Account, Constants, LedgerIds, MinterIds, State,
    },
    utils::{self, _principal_to_subaccount},
};
use crate::{memory::STATE, types::states::StableStates};
use guard::{account_exists, caller_is_admin_controller};

#[init]
fn init(args: InitArgs) -> () {
    STATE.with(|s| {
        {
            *s.borrow_mut() = State {
                stable_state: init_stable_states(),
            }
        }
    });

    let _ = STATE.with(|s| {
        let res: &mut StableStates = &mut s.borrow_mut().stable_state;
        res.init.set(args)
    });
}

#[query]
fn principal_to_hex(principal: Principal) -> String {
    utils::principal_to_hex(principal)
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

#[query]
pub fn principal_to_subaccount(principal_id: Principal) -> [u8; 32] {
    _principal_to_subaccount(&principal_id)
}

#[query]
/// Get the constants for Account Canister
pub fn get_constants() -> Constants {
    let constants = STATE.with(|c| c.borrow().stable_state.constants.get().to_owned());
    constants
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

/// Set the new Ledger Canister IDs
#[update(guard = "caller_is_admin_controller")]
fn set_ledger_ids(ids: LedgerIds) -> () {
    Constants::set_ledger_ids(ids)
}

/// Set the new Minter Canister IDs
#[update(guard = "caller_is_admin_controller")]
fn set_minter_ids(ids: MinterIds) -> () {
    Constants::set_minter_ids(ids)
}

#[update]
async fn transfer_from_account(amount: u64, asset_type: ICRCLedgerType) -> Result_ {
    Account::transfer_from_account(amount, asset_type).await
}
/*
     Macro to generate Candid Interface for the Account Canister
*/

export_candid!();
