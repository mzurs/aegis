use candid::Nat;
use ic_ledger_utils::icrc::IcrcLedger;
use ic_utils::convert_u32_to_subaccount;
use icrc_ledger_types::icrc1::account::Account;

use crate::api::{constants::get_ledger_canister_id, interfaces::constants::IcrcAsset};

pub async fn get_staking_account_balance(asset: IcrcAsset) -> Nat {
    let ledger: IcrcLedger = ic_ledger_utils::icrc::IcrcLedger::new(get_ledger_canister_id(asset));

    ledger
        .balance(Account {
            owner: ic_cdk::id(),
            subaccount: Some(convert_u32_to_subaccount(1)),
        })
        .await
}
