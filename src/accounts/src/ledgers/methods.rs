use candid::{Nat, Principal};
use ic_cdk::{api::call::CallResult, query, update};

use crate::mgmt::MgmtCanister;

use super::{
    services::minter::{RetrieveBtcRet, UpdateBalanceRet},
    types::{CKBTCMinter, ICRCLedgerType, Ledger},
};

#[query]
async fn get_deposit_fee() -> u64 {
    let minter: CKBTCMinter = CKBTCMinter::new();

    minter.get_deposit_fee().await
}

#[query]
async fn get_user_balance() -> Nat {
    let ledger: Ledger = Ledger::new(ICRCLedgerType::CKBTC);
    ledger.icrc1_balance_of(ic_cdk::caller()).await
}

#[query]
fn get_id() -> Principal {
    let minter = CKBTCMinter::new();
    minter.0
}

#[update]
async fn get_btc_balance(addr: String) -> u64 {
    let mgmt: MgmtCanister = MgmtCanister::new();

    mgmt.get_balance(addr, true).await
}

#[update]
async fn get_btc_address() -> String {
    let minter: CKBTCMinter = CKBTCMinter::new();

    minter.get_btc_address(ic_cdk::caller()).await
}

#[update]
async fn update_btc_balance() -> CallResult<(UpdateBalanceRet,)> {
    let minter: CKBTCMinter = CKBTCMinter::new();

    minter.update_balance(ic_cdk::caller()).await
}

#[update]
async fn retrieve_btc(btc_address: String, amount: u64) -> CallResult<(RetrieveBtcRet,)> {
    let minter: CKBTCMinter = CKBTCMinter::new();

    minter.retrieve_btc(btc_address, amount).await
}
