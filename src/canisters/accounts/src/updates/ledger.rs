use candid::Nat;
use ic_cdk::{query, update};

use crate::api::interfaces::{
    account::AegisAccount,
    constants::CanisterName,
    ledger::RetrieveBtcResult,
};

#[query]
async fn get_balance(ledger_id: CanisterName) -> Nat {
    let account = AegisAccount::new();
    account.get_balance(ledger_id).await
}

#[update]
async fn get_btc_address() -> String {
    let account = AegisAccount::new();
    account.get_btc_address().await
}

#[update]
async fn retrieve_btc(btc_address: String, amount: Nat) -> RetrieveBtcResult {
    AegisAccount::retrieve_btc(btc_address, amount).await
}


