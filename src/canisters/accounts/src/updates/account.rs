use ic_cdk::update;
use ic_ledger_utils::services::TransferResult;

use crate::api::interfaces::account::AegisAccount;
use crate::api::interfaces::ledger::ICRCLedgerType;
use crate::guard::account_exist;
 
// Function to create User AegisAccount
#[update(guard = "account_exist")]
async fn create_account() -> Result<bool, String> {
    let aegis_account: AegisAccount = AegisAccount::new();
    aegis_account.create_account().await
}

// Function to update user account name
#[update]
fn update_account_user_name(user_name: String) -> Result<(), String> {
    let aegis_account: AegisAccount = AegisAccount::new();

    aegis_account.update_account_user_name(user_name)
}

#[update]
async fn transfer_from_account(amount: u64, asset_type: ICRCLedgerType) -> TransferResult {
    AegisAccount::transfer_from_account(amount, asset_type).await
}
