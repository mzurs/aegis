use ic_cdk::update;

use crate::api::interfaces::account::AegisAccount;
use crate::guard::account_exist;

#[update(guard = "account_exist")]
async fn create_account() -> Result<bool, String> {
    let aegis_account: AegisAccount = AegisAccount::new();

    aegis_account.create_account().await
}

#[update]
fn update_account_user_name(user_name: String) -> Result<(), String> {
    let aegis_account: AegisAccount = AegisAccount::new();

    aegis_account.update_account_user_name(user_name)
}
