use ic_cdk::caller;

use crate::api::interfaces::account::AegisAccount;

/// Return true if AegisAccount of a `ic_cdk::caller()` exist
pub fn account_exist() -> Result<(), String> {
    let aegis_account: AegisAccount = AegisAccount::new();

    if aegis_account.is_account_exists() {
        return Err(String::from("AegisAccount Already Exist"));
    }
    return Ok(());
}

/// Check if the caller is an Admin of a AegisAccount Canister
pub fn caller_is_admin_controller() -> Result<(), String> {
    if ic_cdk::api::is_controller(&caller()) {
        Ok(())
    } else {
        Err(String::from("Caller is not a Controller of the Acoount Canister"))
    }
}
