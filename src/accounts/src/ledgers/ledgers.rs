use candid::{Nat, Principal};
use ic_cdk::{
    api::call::{CallResult, RejectionCode},
    call, id,
};
use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};

use crate::{memory::CONSTANTS, utils::_principal_to_subaccount};

use super::{
    services::{
        ledger::Result_,
        minter::{RetrieveBtcArgs, RetrieveBtcRet, UpdateBalanceArg, UpdateBalanceRet},
    },
    types::{CKBTCMinter, ICRCLedgerType, Ledger},
};

impl Ledger {
    pub fn new(ledger: ICRCLedgerType) -> Self {
        Self(match ledger {
            ICRCLedgerType::ICP => CONSTANTS.with(|c| c.borrow().get().ledger_ids.icp_ledger_id),
            ICRCLedgerType::CKBTC => {
                CONSTANTS.with(|c| c.borrow().get().ledger_ids.ckbtc_ledger_id)
            }
            ICRCLedgerType::CKETH => {
                CONSTANTS.with(|c| c.borrow().get().ledger_ids.cketh_ledger_id)
            }
        })
    }

    /// get the fee for any ledger for a given canister id
    pub async fn icrc1_fee(self: Self) -> Nat {
        let (fee,): (Nat,) = call(self.0, "icrc1_fee", ()).await.unwrap();
        fee
    }

    /// transfer any icrc ledger tokens to Principal for a given canister id
    pub async fn icrc1_transfer(self: Self, to: Principal, amount: u64) {
        let transfer_args: TransferArg = TransferArg {
            from_subaccount: Option::None,
            to: Account {
                owner: to,
                subaccount: Option::None,
            },
            fee: Option::Some(self.icrc1_fee().await),
            created_at_time: Option::Some(ic_cdk::api::time()),
            memo: Option::None,
            amount: Nat::from(amount),
        };

        // result of transfer operation
        let _transfer_result: Result<(Result_,), (RejectionCode, String)> =
            ic_cdk::call(self.0, "icrc1_transfer", (transfer_args,)).await;
    }

    pub async fn icrc1_balance_of(self: Self, principal: Principal) -> Nat {
        let account: Account = Account {
            owner: ic_cdk::id(),
            subaccount: Option::Some(_principal_to_subaccount(&principal)),
        };

        let (balance,): (Nat,) = ic_cdk::call(self.0, "icrc1_balance_of", (account,))
            .await
            .unwrap();

        return balance;
    }
}

impl CKBTCMinter {
    pub fn new() -> Self {
        Self(CONSTANTS.with(|c| c.borrow().get().minter_ids.ckbtc_minter_id))
    }

    /// Function to get new btc address for a given id
    pub async fn get_btc_address(self: Self, principal: Principal) -> String {
        let account: Account = Account {
            owner: id(),
            subaccount: Option::Some(_principal_to_subaccount(&principal)),
        };

        let address: CallResult<(String,)> = call(self.0, "get_btc_address", (account,)).await;

        return match address {
            Ok(addr) => addr.0,
            Err(err) => err.1,
        };
    }

    /// Function to Update BTC Balance for a given details
    pub async fn update_balance(
        self: Self,
        principal: Principal,
    ) -> CallResult<(UpdateBalanceRet,)> {
        let update_args: UpdateBalanceArg = UpdateBalanceArg {
            owner: Option::Some(ic_cdk::id()),
            subaccount: Option::Some(_principal_to_subaccount(&principal)),
        };

        let result: CallResult<(UpdateBalanceRet,)> =
            call(self.0, "update_balance", (update_args,)).await;

        result
    }

    /// To get the current Bitcoin Network Fee
    pub async fn get_deposit_fee(self: Self) -> u64 {
        let fee: CallResult<(u64,)> = call(self.0, "get_deposit_fee", ()).await;

        fee.unwrap().0
    }

    /// Retrieve Bitcoin from a Account Canister to a given BTC Address
    pub async fn retrieve_btc(
        self: Self,
        btc_address: String,
        amount: u64,
    ) -> CallResult<(RetrieveBtcRet,)> {
        let retrieve_args: RetrieveBtcArgs = RetrieveBtcArgs {
            address: btc_address,
            amount,
        };
        let result: CallResult<(RetrieveBtcRet,)> =
            call(self.0, "retrieve_btc", (retrieve_args,)).await;

        result
    }
}
