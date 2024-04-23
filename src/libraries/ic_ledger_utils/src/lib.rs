pub mod services;

use candid::{Nat, Principal};
use ic_cdk::api::call::CallResult;
use ic_utils::principal_to_subaccount;
use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};
use services::TransferResult;
use ic_cdk::call;

#[derive(Debug, Copy, Clone)]
pub struct Ledger(pub Principal);

impl Ledger {
    pub fn new(ledger_id: Principal) -> Self {
        Self(ledger_id)
    }

    /// get the fee for any ledger for a given canister id
    pub async fn icrc1_fee(self: Self) -> Nat {
        let (fee,): (Nat,) = call(self.0, "icrc1_fee", ()).await.unwrap();
        fee
    }

    /// transfer any icrc ledger tokens to Principal for a given canister id
    pub async fn icrc1_transfer(self: Self, from_subaccount: Option<[u8; 32]>, to: Account, amount: Nat) -> TransferResult {
        let transfer_args: TransferArg = TransferArg {
            from_subaccount,
            to: Account { ..to },
            fee: Option::Some(self.icrc1_fee().await),
            created_at_time: Option::Some(ic_cdk::api::time()),
            memo: Option::None,
            amount: (amount),
        };

        // result of transfer operation
        let transfer_result: CallResult<(TransferResult,)> = call(self.0, "icrc1_transfer", (transfer_args,)).await;

        transfer_result.unwrap().0
    }

    pub async fn icrc1_balance_of(self: Self, principal: Principal) -> Nat {
        let account: Account = Account {
            owner: ic_cdk::id(),
            subaccount: Option::Some(principal_to_subaccount(&principal)),
        };

        let (balance,): (Nat,) = call(self.0, "icrc1_balance_of", (account,)).await.unwrap();

        return balance;
    }
}
