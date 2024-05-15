use candid::{Nat, Principal};
use ic_ledger_utils::{
    icrc::IcrcLedger,
    types::icrc_types::{IcrcTransferFromResult, IcrcTransferResult},
};
use ic_utils::principal_to_subaccount;
use icrc_ledger_types::{
    icrc1::{account::Account, transfer::TransferArg},
    icrc2::transfer_from::TransferFromArgs,
};

use crate::api::{
    constants::get_canister_id,
    interfaces::{account::AegisAccount, constants::CanisterName},
};

impl AegisAccount {
    /// Get the account(aegis canister) balance of a user(caller)  
    pub async fn get_balance(&self, ledger: CanisterName) -> Nat {
        let ledger_id: Principal = get_canister_id(ledger);

        let ledger: IcrcLedger = IcrcLedger::new(ledger_id);

        let account: Account = Account {
            owner: self.0,
            subaccount: None,
        };

        ledger.balance(account).await
    }

    /// Transfer ICRC Ledger funds from User(caller) Canister Based Account to other account
    pub(crate) async fn icrc_transfer_from_wallet(
        &self,
        ledger: CanisterName,
        to: Option<Account>,
        amount: Nat,
    ) -> IcrcTransferResult {
        let ledger_id: Principal = get_canister_id(ledger);

        let ledger: IcrcLedger = IcrcLedger::new(ledger_id);

        let from_subaccount: Option<[u8; 32]> = Option::Some(principal_to_subaccount(&ic_cdk::caller()));

        let to: Account = match to {
            Some(acc) => acc,
            None => Account {
                owner: ic_cdk::caller(),
                subaccount: None,
            },
        };

        let args: TransferArg = TransferArg {
            from_subaccount,
            to,
            amount,
            fee: Option::None,
            created_at_time: Option::None,
            memo: Option::None,
        };
        ledger.transfer(args).await
    }

    /// Transfer ICRC Ledger funds to User(caller) Canister Based Account from other account
    pub(crate) async fn icrc_transfer_to_wallet(&self, ledger: CanisterName, amount: Nat) -> IcrcTransferFromResult {
        let ledger_id: Principal = get_canister_id(ledger);

        let ledger: IcrcLedger = IcrcLedger::new(ledger_id);

        let from = Account {
            owner: ic_cdk::caller(),
            subaccount: Option::None,
        };

        let to: Account = Account {
            owner: ic_cdk::id(),
            subaccount: Some(principal_to_subaccount(&ic_cdk::caller())),
        };

        let args: TransferFromArgs = TransferFromArgs {
            spender_subaccount: Option::None,
            from,
            to,
            amount,
            fee: Option::None,
            memo: Option::None,
            created_at_time: Option::None,
        };
        ledger.transfer_from(args).await
    }
}
