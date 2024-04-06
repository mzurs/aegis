use candid::Principal;
use ic_cdk::caller;

use crate::{
    memory::STATE,
    types::states::{Account, StableStates},
};

/// Check if Account Exist
pub fn _if_account_exist(principal: Principal) -> bool {
    STATE.with(|s| if_account_exists_impl(principal, &s.borrow().stable_state))
}
pub fn if_account_exists_impl(principal: Principal, state: &StableStates) -> bool {
    let user: Option<Account> = state.user_accounts.get(&principal);
    match user {
        Some(_user) => true,
        None => false,
    }
}
/// Check if User Account Exists of the caller
pub fn account_exists() -> Result<(), String> {
    STATE.with(|s| account_exists_impl(ic_cdk::caller(), &s.borrow().stable_state))
}

pub fn account_exists_impl(principal: Principal, state: &StableStates) -> Result<(), String> {
    let user: Option<Account> = state.user_accounts.get(&principal);
    match user {
        Some(_user) => Err(String::from("Account Already Exist")),
        None => Ok(()),
    }
}
/// Check if the caller is an Admin of a Account Canister
pub fn caller_is_admin_controller() -> Result<(), String> {
    if ic_cdk::api::is_controller(&caller()) {
        Ok(())
    } else {
        Err(String::from(
            "Caller is not a Controller of the Acoount Canister",
        ))
    }
}
