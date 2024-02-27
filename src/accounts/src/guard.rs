use candid::Principal;
use ic_cdk::caller;

use crate::memory::USER_ACCOUNTS;

/// Check if Account Exist in Memory
pub fn _if_account_exist(principal: Principal) -> bool {
    let user = USER_ACCOUNTS.with(|accounts| accounts.borrow().get(&principal));
    match user {
        Some(_user) => true,
        None => false,
    }
}

pub fn caller_is_admin_controller() -> Result<(), String> {
    if ic_cdk::api::is_controller(&caller()) {
        Ok(())
    } else {
        Err(String::from(
            "Caller is not a Controller of the Acoount Canister",
        ))
    }
}
