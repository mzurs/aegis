use candid::{Nat, Principal};
use ic_cdk::{api::call::CallResult, call};
use icrc_ledger_types::{
    icrc1::{account::Account, transfer::TransferArg},
    icrc2::transfer_from::TransferFromArgs,
};

use crate::types::icrc_types::{IcrcFee, IcrcTransferFromResult, IcrcTransferResult, TransferFromResult, TransferResult};

pub struct IcrcLedger(Principal);

impl IcrcLedger {
    pub fn new(canister_id: Principal) -> Self {
        Self(canister_id)
    }

    pub async fn balance(&self, account: Account) -> Nat {
        let (balance,): (Nat,) = call(self.0, "icrc1_balance_of", (account,)).await.unwrap();

        balance
    }

    pub async fn fee(&self) -> IcrcFee {
        let ledger_fee: CallResult<(Nat,)> = call(self.0, "icrc1_fee", ((),)).await;
        return match ledger_fee {
            Ok(fee) => IcrcFee::Fee(fee.0),
            Err(err) => IcrcFee::ErrorMessage(err.1),
        };
    }

    pub async fn transfer(&self, from_subaccount: Option<[u8; 32]>, to: Account, amount: Nat) -> IcrcTransferResult {
        let fee: Nat = match Self::fee(&self).await {
            IcrcFee::Fee(res) => res,
            IcrcFee::ErrorMessage(err) => return IcrcTransferResult::TransferErrorString(err),
        };

        let args: TransferArg = TransferArg {
            from_subaccount,
            to,
            fee: Some(fee),
            created_at_time: Option::Some(ic_cdk::api::time()),
            memo: Option::None,
            amount,
        };

        let transfer_result: CallResult<(TransferResult,)> = call(self.0, "icrc1_transfer", (args,)).await;

        match transfer_result {
            Ok(res) => {
                return match res.0 {
                    TransferResult::Ok(res1) => IcrcTransferResult::TransferSuccess(res1),
                    TransferResult::Err(err) => IcrcTransferResult::TransferErrorMessage(err),
                }
            }
            Err(err) => IcrcTransferResult::TransferErrorString(err.1),
        }
    }

    pub async fn transfer_from(
        &self,
        spender_subaccount: Option<[u8; 32]>,
        from: Account,
        to: Account,
        amount: Nat,
    ) -> IcrcTransferFromResult {
        let fee: Nat = match Self::fee(&self).await {
            IcrcFee::Fee(res) => res,
            IcrcFee::ErrorMessage(err) => return IcrcTransferFromResult::TransferFromErrorString(err),
        };

        let args: TransferFromArgs = TransferFromArgs {
            spender_subaccount,
            from,
            to,
            amount,
            fee: Some(fee),
            memo: Option::None,
            created_at_time: Option::Some(ic_cdk::api::time()),
        };

        let transfer_from_result: CallResult<(TransferFromResult,)> = call(self.0, "icrc2_transfer_from", (args,)).await;

        match transfer_from_result {
            Ok(res) => {
                return match res.0 {
                    TransferFromResult::Ok(res1) => IcrcTransferFromResult::TransferFromSuccess(res1),
                    TransferFromResult::Err(err) => IcrcTransferFromResult::TransferFromErrorMessage(err),
                }
            }
            Err(err) => IcrcTransferFromResult::TransferFromErrorString(err.1),
        }
    }
}
