use candid::{Nat, Principal};
use ic_cdk::{api::call::CallResult, query, update};
use ic_ledger_utils::Ledger;
use icrc_ledger_types::icrc1::account::Account;
use minter_utils::{
    ckbtc::CkBTCMinter,
    services::ckbtc::{RetrieveBtcRet, UpdateBalanceRet},
};

use crate::read_state;

#[query]
async fn get_deposit_fee() -> u64 {
    let minter_id: Principal = read_state(|s| s.stable_state.constants.get().minter_ids.ckbtc_minter_id);

    let minter: CkBTCMinter = CkBTCMinter::new(minter_id);

    minter.get_deposit_fee().await
}

#[query]
async fn get_user_balance() -> Nat {
    let ledger_id: Principal = read_state(|s| s.stable_state.constants.get().ledger_ids.ckbtc_ledger_id);

    let ledger: Ledger = Ledger::new(ledger_id);
    ledger.icrc1_balance_of(ic_cdk::caller()).await
}

#[query]
fn get_id() -> Principal {
    let minter_id: Principal = read_state(|s| s.stable_state.constants.get().minter_ids.ckbtc_minter_id);

    let minter: CkBTCMinter = CkBTCMinter::new(minter_id);

    minter.0
}

#[update]
async fn get_btc_address() -> String {
    let minter_id: Principal = read_state(|s| s.stable_state.constants.get().minter_ids.ckbtc_minter_id);

    let minter: CkBTCMinter = CkBTCMinter::new(minter_id);

    minter.get_btc_address(ic_cdk::caller()).await
}

#[update]
async fn update_btc_balance() -> CallResult<(UpdateBalanceRet,)> {
    let minter_id: Principal = read_state(|s| s.stable_state.constants.get().minter_ids.ckbtc_minter_id);

    let minter: CkBTCMinter = CkBTCMinter::new(minter_id);

    minter.update_balance(ic_cdk::caller()).await
}

#[update]
async fn retrieve_btc(btc_address: String, amount: u64) -> CallResult<(RetrieveBtcRet,)> {
    let minter_id: Principal = read_state(|s| s.stable_state.constants.get().minter_ids.ckbtc_minter_id);

    let minter: CkBTCMinter = CkBTCMinter::new(minter_id);

    minter.retrieve_btc(btc_address, amount).await
}

#[update]
async fn get_withdrawal_account() -> CallResult<(Account,)> {
    let minter_id: Principal = read_state(|s| s.stable_state.constants.get().minter_ids.ckbtc_minter_id);

    let minter: CkBTCMinter = CkBTCMinter::new(minter_id);
    minter.get_withdrawal_account().await
}
