use candid::Principal;
use ic_cdk::caller;

/// Check if the caller is an Admin of a AegisAccount Canister
pub fn caller_is_admin_controller() -> Result<(), String> {
    if ic_cdk::api::is_controller(&caller()) {
        Ok(())
    } else {
        Err(String::from("Caller is not a Controller of the Acoount Canister"))
    }
}

pub fn restrict_anonymous_identity() -> Result<(), String> {
    // Check Anonymous Identity
    if ic_cdk::caller() == Principal::from_text("2vxsx-fae").unwrap() {
        Err("Anonymous Principal not allowed".to_owned())
    } else {
        Ok(())
    }
}
