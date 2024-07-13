use candid::Nat;
use ic_ledger_utils::icrc::IcrcLedger;
use ic_utils::convert_u32_to_subaccount;
use icrc_ledger_types::icrc1::account::Account;

use crate::api::{interface::insurance::InsuranceAssets, utils::constants::get_ledger_canister_id};

/// Get the balance of a insurance pool for a given `insurance_id` and `asset`
pub async fn get_pool_balance_by_insurance_id(insurance_id: u32, insurance_asset: InsuranceAssets) -> Nat {
    // get the balance
    let ledger: IcrcLedger = ic_ledger_utils::icrc::IcrcLedger::new(get_ledger_canister_id(insurance_asset));

    // get the balance of a insurance contract liquiduty pool
    let insurance_pool_balance: Nat = ledger
        .balance(Account {
            owner: ic_cdk::api::id(),
            subaccount: Some(convert_u32_to_subaccount(insurance_id)),
        })
        .await;

    insurance_pool_balance
}

/// Get the balance of a insurance premium pool for a given `insurance_id` and `asset`
pub async fn get_premium_pool_balance_by_insurance_id(insurance_id: u32, insurance_asset: InsuranceAssets) -> Nat {
    // get the balance
    let ledger: IcrcLedger = ic_ledger_utils::icrc::IcrcLedger::new(get_ledger_canister_id(insurance_asset));

    // get the balance of a insurance contract liquiduty pool
    let insurance_premium_balance: Nat = ledger
        .balance(Account {
            owner: ic_cdk::api::id(),
            subaccount: Some(convert_u32_to_subaccount(insurance_id + 1)),
        })
        .await;
    insurance_premium_balance
}
