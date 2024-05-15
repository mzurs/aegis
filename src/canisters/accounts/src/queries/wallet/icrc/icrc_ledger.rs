use candid::Nat;
use ic_cdk::query;

use crate::api::interfaces::{account::AegisAccount, constants::CanisterName};

#[query]
async fn icrc_get_balance(ledger_id: CanisterName) -> Nat {
    let account = AegisAccount::new();
    account.get_balance(ledger_id).await
}
