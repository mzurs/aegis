use std::{borrow::Cow, collections::HashMap};

use candid::{CandidType, Decode, Encode, Principal};
use ic_ledger_utils::{
    services::{TransferError, TransferResult},
    Ledger,
};
use ic_stable_structures::{storable::Bound, Storable};
use ic_utils::{generate_random_number, principal_to_subaccount};
use icrc_ledger_types::icrc1::account::Account;
use minter_utils::ckbtc::CkBTCMinter;

use crate::{
    api::interfaces::{
        account::{AegisAccount, AegisAccountInfo},
        ledger::ICRCLedgerType,
        state::StableStates,
    },
    mutate_state, read_state,
    utils::increment_user_count::increment_user_count,
};

#[derive(CandidType)]
pub enum CkBtc2BtcErr {
    ErrMessage(String),
    TransferError(TransferError),
}

impl Storable for AegisAccount {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for AegisAccount {
    fn default() -> Self {
        Self::new()
    }
}

impl AegisAccount {
    pub fn new() -> Self {
        Self(ic_cdk::caller())
    }

    pub fn is_account_exists(&self) -> bool {
        read_state(|acc| acc.stable_state.aegis_account.contains_key(self))
    }

    pub fn get_account(&self) -> Option<AegisAccountInfo> {
        // STATE.with(|s| s.borrow().stable_state.aegis_account.get(&self))
        read_state(|s| s.stable_state.aegis_account.get(self))
    }

    pub async fn create_account(&self) -> Result<bool, String> {
        let user_id: u64 = match generate_random_number().await {
            Ok(id) => id,
            Err(err) => return Err(err),
        };

        mutate_state(|ss| {
            ss.stable_state.aegis_account.insert(
                AegisAccount(ic_cdk::caller()),
                AegisAccountInfo {
                    user_id,
                    user_name: Option::None,
                },
            )
        });
        increment_user_count();

        Ok(true)
    }

    pub fn update_account_user_name(&self, user_name: String) -> Result<(), String> {
        let principal: &Principal = &ic_cdk::caller();

        if self.is_account_exists() {
            let _account: AegisAccountInfo = self.get_account().unwrap();

            mutate_state(|s| {
                s.stable_state.aegis_account.insert(
                    AegisAccount(*principal),
                    AegisAccountInfo {
                        user_name: Option::Some(user_name),
                        .._account
                    },
                );
            })
        } else {
            return Result::Err(String::from("AegisAccount Not Exists"));
        }

        Ok(())
    }

    pub async fn ckbtc_to_btc_from_account(amount: u64, btc_address: String) -> Result<bool, CkBtc2BtcErr> {
        let minter_id: Principal = read_state(|s| s.stable_state.constants.get().minter_ids.ckbtc_minter_id);
        let minter: CkBTCMinter = CkBTCMinter::new(minter_id);

        let withdrawl_account_opt = minter.get_withdrawal_account().await;

        let withdrawl_account: Account = match withdrawl_account_opt {
            Ok(acc) => acc.0,
            Err(err) => return Err(CkBtc2BtcErr::ErrMessage(err.1)),
        };

        let ledger_id: Principal = read_state(|s| s.stable_state.constants.get().ledger_ids.ckbtc_ledger_id);

        let ckbtc: Ledger = Ledger::new(ledger_id);
        let ckbtc_transfer_result = ckbtc
            .icrc1_transfer(
                Option::Some(principal_to_subaccount(&ic_cdk::caller())),
                withdrawl_account,
                amount.into(),
            )
            .await;

        match ckbtc_transfer_result {
            TransferResult::Ok(_) => (),
            TransferResult::Err(err) => return Err(CkBtc2BtcErr::TransferError(err)),
        }

        let retrieve_btc_result = minter.retrieve_btc(btc_address, amount).await;

        match retrieve_btc_result {
            Ok(_) => (),
            Err(err) => return Err(CkBtc2BtcErr::ErrMessage(err.1)),
        }

        Ok(true)
    }

    pub async fn transfer_from_account(amount: u64, asset_type: ICRCLedgerType) -> TransferResult {
        let asset_map: HashMap<ICRCLedgerType, Principal> = HashMap::from([
            (
                ICRCLedgerType::ICP,
                mutate_state(|c| {
                    let state: &StableStates = &c.stable_state;
                    state.constants.get().ledger_ids.icp_ledger_id
                }), // Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
            ),
            (
                ICRCLedgerType::CKETH,
                mutate_state(|c| {
                    let state: &StableStates = &c.stable_state;
                    state.constants.get().ledger_ids.cketh_ledger_id
                }),
            ),
            (
                ICRCLedgerType::CKBTC,
                mutate_state(|c| {
                    let state: &StableStates = &c.stable_state;
                    state.constants.get().ledger_ids.ckbtc_ledger_id
                }),
            ),
        ]);
        let ledger = Ledger(*asset_map.get(&asset_type).unwrap());
        let to: icrc_ledger_types::icrc1::account::Account = icrc_ledger_types::icrc1::account::Account {
            owner: ic_cdk::caller(),
            subaccount: None,
        };
        let res: TransferResult = ledger
            .icrc1_transfer(
                Option::Some(principal_to_subaccount(&ic_cdk::caller())),
                to,
                candid::Nat::from(amount),
            )
            .await;
        res
    }
}
