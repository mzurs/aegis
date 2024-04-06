use std::collections::HashMap;

use candid::Principal;

use crate::{
    guard::_if_account_exist,
    ledgers::{
        services::ledger::Result_,
        types::{CKBTCMinter, ICRCLedgerType, Ledger},
    },
    memory::STATE,
    methods::principal_to_subaccount,
    types::{
        interfaces::account::CkBtc2BtcErr,
        states::{Account, StableStates},
    },
    utils::{_generate_random_number, increment_user_count},
};

impl Account {
    pub fn get_account() -> Option<Self> {
        STATE.with(|s| {
            s.borrow().stable_state.user_accounts.get(&ic_cdk::caller())
            // USER_ACCOUNTS.with(|accounts| accounts.borrow().get(&ic_cdk::caller()))
        })
    }

    pub async fn create_account() -> Result<bool, String> {
        let user_id: u64 = match _generate_random_number().await {
            Ok(id) => id,
            Err(err) => return Err(err),
        };

        let account_args: Account = Account {
            user_id,
            principal: ic_cdk::caller(),
            user_name: Option::None,
        };

        STATE.with(|s| {
            Self::create_account_impl(
                &mut s.borrow_mut().stable_state,
                ic_cdk::caller(),
                account_args,
            )
        });

        increment_user_count();

        Ok(true)
    }

    fn create_account_impl(
        state: &mut StableStates,
        principal: Principal,
        account_args: Account,
    ) -> Option<Account> {
        let res: Option<Account> = state.user_accounts.insert(principal, account_args);
        res
    }
    pub fn update_account_user_name(user_name: String) -> Result<(), String> {
        let principal: &Principal = &ic_cdk::caller();

        if _if_account_exist(*principal) {
            let _account: Account = Self::get_account().unwrap();

            STATE.with(|s| {
                s.borrow_mut().stable_state.user_accounts.insert(
                    *principal,
                    Account {
                        user_name: Option::Some(user_name),
                        .._account
                    },
                );
            })
        } else {
            return Result::Err(String::from("Account Not Exists"));
        }

        Ok(())
    }

    pub async fn ckbtc_to_btc_from_account(
        amount: u64,
        btc_address: String,
    ) -> Result<bool, CkBtc2BtcErr> {
        let minter: CKBTCMinter = CKBTCMinter::new();

        let withdrawl_account = minter.get_withdrawal_account().await;

        match withdrawl_account {
            Ok(_) => (),
            Err(err) => return Err(CkBtc2BtcErr::ErrMessage(err.1)),
        };

        let ckbtc: Ledger = Ledger::new(crate::ledgers::types::ICRCLedgerType::CKBTC);
        let ckbtc_transfer_result = ckbtc
            .icrc1_transfer(
                Option::Some(principal_to_subaccount(ic_cdk::caller())),
                withdrawl_account.unwrap().0,
                amount.into(),
            )
            .await;

        match ckbtc_transfer_result {
            crate::ledgers::services::ledger::Result_::Ok(_) => (),
            crate::ledgers::services::ledger::Result_::Err(err) => {
                return Err(CkBtc2BtcErr::TransferError(err))
            }
        }

        let retrieve_btc_result = minter.retrieve_btc(btc_address, amount).await;

        match retrieve_btc_result {
            Ok(_) => (),
            Err(err) => return Err(CkBtc2BtcErr::ErrMessage(err.1)),
        }

        Ok(true)
    }

    pub async fn transfer_from_account(amount: u64, asset_type: ICRCLedgerType) -> Result_ {
        let asset_map: HashMap<ICRCLedgerType, Principal> = HashMap::from([
            (
                ICRCLedgerType::ICP,
                STATE.with_borrow(|c| {
                    let state: &StableStates = &c.stable_state;
                    state.constants.get().ledger_ids.icp_ledger_id
                }), // Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
            ),
            (
                ICRCLedgerType::CKETH,
                STATE.with_borrow(|c| {
                    let state: &StableStates = &c.stable_state;
                    state.constants.get().ledger_ids.cketh_ledger_id
                }),
            ),
            (
                ICRCLedgerType::CKBTC,
                STATE.with_borrow(|c| {
                    let state: &StableStates = &c.stable_state;
                    state.constants.get().ledger_ids.ckbtc_ledger_id
                }),
            ),
        ]);
        let ledger = Ledger(*asset_map.get(&asset_type).unwrap());
        let to: icrc_ledger_types::icrc1::account::Account =
            icrc_ledger_types::icrc1::account::Account {
                owner: ic_cdk::caller(),
                subaccount: None,
            };
        let res: Result_ = ledger
            .icrc1_transfer(
                Option::Some(principal_to_subaccount(ic_cdk::caller())),
                to,
                candid::Nat::from(amount),
            )
            .await;
        res
    }
}
