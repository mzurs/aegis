use crate::services::ckbtc::{RetrieveBtcArgs, RetrieveBtcRet, UpdateBalanceArg, UpdateBalanceRet};
use candid::Principal;
use ic_cdk::{api::call::CallResult, call, id};
use ic_utils::principal_to_subaccount;
use icrc_ledger_types::icrc1::account::Account;

#[derive(Debug, Copy, Clone)]
pub struct CkBTCMinter(pub Principal);

impl CkBTCMinter {
    pub fn new(principal: Principal) -> Self {
        Self(principal)
    }

    /// Function to get new btc address for a given id
    pub async fn get_btc_address(self, principal: Principal) -> String {
        let account: Account = Account {
            owner: id(),
            subaccount: Option::Some(principal_to_subaccount(&principal)),
        };

        let address: CallResult<(String,)> = call(self.0, "get_btc_address", (account,)).await;

        match address {
            Ok(addr) => addr.0,
            Err(err) => err.1,
        }
    }

    /// Function to Update BTC Balance for a given details
    pub async fn update_balance(self, principal: Principal) -> CallResult<(UpdateBalanceRet,)> {
        let update_args: UpdateBalanceArg = UpdateBalanceArg {
            owner: Option::Some(ic_cdk::id()),
            subaccount: Option::Some(principal_to_subaccount(&principal)),
        };

        let result: CallResult<(UpdateBalanceRet,)> = call(self.0, "update_balance", (update_args,)).await;

        result
    }

    /// To get the current Bitcoin Network Fee
    pub async fn get_deposit_fee(self) -> u64 {
        let fee: CallResult<(u64,)> = call(self.0, "get_deposit_fee", ()).await;

        fee.unwrap().0
    }

    /// Retrieve Bitcoin from a Account Canister to a given BTC Address
    pub async fn retrieve_btc(self, btc_address: String, amount: u64) -> CallResult<(RetrieveBtcRet,)> {
        let retrieve_args: RetrieveBtcArgs = RetrieveBtcArgs {
            address: btc_address,
            amount,
        };
        let result: CallResult<(RetrieveBtcRet,)> = call(self.0, "retrieve_btc", (retrieve_args,)).await;

        result
    }

    pub async fn get_withdrawal_account(&self) -> CallResult<(Account,)> {
        let result: CallResult<(Account,)> = call(self.0, "get_withdrawal_account", ()).await;

        result
    }
}
