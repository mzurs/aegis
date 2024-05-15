use candid::Nat;
use ic_cdk::update;
use ic_ledger_utils::types::icrc_types::{IcrcTransferFromResult, IcrcTransferResult};
use icrc_ledger_types::icrc1::account::Account;

use crate::api::interfaces::{account::AegisAccount, constants::CanisterName};

#[update]
async fn icrc_transfer_to_account(ledger: CanisterName, amount: Nat) -> IcrcTransferFromResult {
    let account: AegisAccount = AegisAccount::new();

    account.icrc_transfer_to_wallet(ledger, amount).await
}

#[update]
async fn icrc_transfer_from_account(ledger: CanisterName, to: Option<Account>, amount: Nat) -> IcrcTransferResult {
    let account: AegisAccount = AegisAccount::new();

    account.icrc_transfer_from_wallet(ledger, to, amount).await
}
