use ic_cdk::query;

use crate::api::interfaces::account::{AegisAccount, AegisAccountInfo};

#[query]
fn get_account() -> Option<AegisAccountInfo> {
    let aegis_account: AegisAccount = AegisAccount::new();

    aegis_account.get_account()
}

