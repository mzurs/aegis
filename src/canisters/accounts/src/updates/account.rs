use candid::Nat;
use ic_cdk::update;
use ic_ledger_utils::types::icrc_types::IcrcTransferResult;
use icrc_ledger_types::icrc1::account::Account;

use crate::api::interfaces::account::AegisAccount;
use crate::api::interfaces::constants::CanisterName;
use crate::api::interfaces::ledger::ConvertCkBTCResult;
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
async fn transfer_from_account(ledger: CanisterName, to: Option<Account>, amount: Nat) -> IcrcTransferResult {
    let account: AegisAccount = AegisAccount::new();

    account.transfer_from_account(ledger, to, amount).await
}

#[update]
async fn convert_ckbtc(btc_address: String, amount: Nat) -> ConvertCkBTCResult {
    let account: AegisAccount = AegisAccount::new();
    account.convert_ckbtc(btc_address, &amount).await
}