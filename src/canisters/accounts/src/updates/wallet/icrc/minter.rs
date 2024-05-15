use candid::Nat;
use ic_cdk::update;

use crate::api::interfaces::{
    account::AegisAccount,
    ledger::{ConvertCkBTCResult, RetrieveBtcResult},
};

#[update]
async fn convert_ckbtc(btc_address: String, amount: Nat) -> ConvertCkBTCResult {
    let account: AegisAccount = AegisAccount::new();

    account.convert_ckbtc(btc_address, &amount).await
}

#[update]
async fn get_btc_address() -> Box<String> {
    let account = AegisAccount::new();
    Box::new(account.get_btc_address().await)
}

#[update]
async fn retrieve_btc(btc_address: String, amount: Nat) -> RetrieveBtcResult {
    AegisAccount::retrieve_btc(btc_address, amount).await
}
