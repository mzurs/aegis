use candid::Principal;
use ic_cdk::caller;

use crate::memory::USER_ACCOUNTS;

/// Check if Account Exist
pub fn _if_account_exist(principal: Principal) -> bool {
    let user: Option<crate::types::states::Account> =
        USER_ACCOUNTS.with(|accounts| accounts.borrow().get(&principal));
    match user {
        Some(_user) => true,
        None => false,
    }
}

/// Check if User Account Exists of the caller
pub fn account_exists() -> Result<(), String> {
    let user: Option<crate::types::states::Account> =
        USER_ACCOUNTS.with(|accounts| accounts.borrow().get(&caller()));
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
