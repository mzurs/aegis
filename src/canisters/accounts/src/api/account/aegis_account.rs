use std::{borrow::Cow, u64};

use candid::{Decode, Encode, Nat, Principal};

use ic_ledger_utils::{icrc::IcrcLedger, types::icrc_types::IcrcTransferResult};
use ic_stable_structures::{storable::Bound, Storable};
use ic_utils::{generate_random_number, principal_to_subaccount};
use icrc_ledger_types::icrc1::account::Account;
use minter_utils::{ckbtc::CkBTCMinter, services::ckbtc::RetrieveBtcRet};

use crate::{
    api::{
        constants::get_canister_id,
        interfaces::{
            account::{AegisAccount, AegisAccountInfo},
            constants::CanisterName,
            ledger::{ConvertCkBTCResult, RetrieveBtcResult},
        },
        metrics::increment_user_count::increment,
    },
    mutate_state, read_state,
};

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
    /// Returns the AegisAccount object of the caller
    pub fn new() -> Self {
        Self(ic_cdk::caller())
    }

    /// Returns true if user account(caller) account exists in Stable Memory
    pub fn is_account_exists(&self) -> bool {
        read_state(|acc| acc.stable_state.aegis_account.contains_key(self))
    }

    /// Retrieve User Account(caller) from memory
    pub fn get_account(&self) -> Option<AegisAccountInfo> {
        read_state(|s| s.stable_state.aegis_account.get(self))
    }

    /// Function to create User AegisAccount
    pub(crate) async fn create_account(&self) -> Result<bool, String> {
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
        increment();

        Ok(true)
    }

    /// Function to update user account name
    pub(crate) fn update_account_user_name(&self, user_name: String) -> Result<(), String> {
        let principal: &Principal = &ic_cdk::caller();

        if self.is_account_exists() {
            let account: AegisAccountInfo = self.get_account().unwrap();

            mutate_state(|s| {
                s.stable_state.aegis_account.insert(
                    AegisAccount(*principal),
                    AegisAccountInfo {
                        user_name: Option::Some(user_name),
                        ..account
                    },
                );
            })
        } else {
            return Result::Err(String::from("AegisAccount Not Exists"));
        }

        Ok(())
    }

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

    /// Convert CKBTC to BTC from a User(Caller) Canister Based Account
    pub(crate) async fn convert_ckbtc(&self, btc_address: String, amount: &Nat) -> ConvertCkBTCResult {
        let minter_id: Principal = get_canister_id(CanisterName::CKBTCMINTER);

        let minter: CkBTCMinter = CkBTCMinter::new(minter_id);

        let withdrawal_account: Account = match minter.get_withdrawal_account().await {
            Ok(acc) => acc.0,
            Err(err) => return ConvertCkBTCResult::ErrMessage(err.1),
        };

        let transfer_result: IcrcTransferResult = self
            .icrc_transfer_from_account(CanisterName::CKBTCMINTER, Some(withdrawal_account), amount.to_owned())
            .await;

        match transfer_result {
            IcrcTransferResult::TransferSuccess(_) => (),
            IcrcTransferResult::TransferErrorMessage(err) => {
                return ConvertCkBTCResult::IcrcTransferResult(IcrcTransferResult::TransferErrorMessage(err))
            }
            IcrcTransferResult::TransferErrorString(err) => {
                return ConvertCkBTCResult::IcrcTransferResult(IcrcTransferResult::TransferErrorString(err))
            }
        }

        let retrieve_btc_result: RetrieveBtcResult = AegisAccount::retrieve_btc(btc_address, amount.clone()).await;

        match retrieve_btc_result {
            RetrieveBtcResult::RetrieveBtcOk(res) => ConvertCkBTCResult::ConvertSuccess(Nat::from(res.block_index)),
            RetrieveBtcResult::RetrieveBtcError(err) => ConvertCkBTCResult::RetrieveBtcError(err),
            RetrieveBtcResult::RetrieveBtcString(msg) => ConvertCkBTCResult::ErrMessage(msg),
        }
    }

    /// Transfer ICRC Ledger funds from User(caller) Canister Based Account to other account
    pub(crate) async fn icrc_transfer_from_account(
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

        ledger.transfer(from_subaccount, to, amount).await
    }

    // pub(crate) async fn transfer_to_account(ledger: CanisterName,)->IcrcTransferFromResult{
    //     let ledger_id: Principal = get_canister_id(ledger);

    //     IcrcTransferFromResult::TransferFromErrorString("()".to_string())
    // }
    pub async fn get_btc_address(&self) -> String {
        let minter_id: Principal = get_canister_id(CanisterName::CKBTCMINTER);

        let minter: CkBTCMinter = CkBTCMinter::new(minter_id);

        minter.get_btc_address(self.0).await
    }

    pub(crate) async fn retrieve_btc(btc_address: String, amount: Nat) -> RetrieveBtcResult {
        let minter_id: Principal = get_canister_id(CanisterName::CKBTCMINTER);

        let minter: CkBTCMinter = CkBTCMinter::new(minter_id);

        let result = minter
            .retrieve_btc(btc_address, Nat::to_string(&amount).parse::<u64>().unwrap())
            .await;

        return match result {
            Ok(res) => match res.0 {
                RetrieveBtcRet::Ok(res) => RetrieveBtcResult::RetrieveBtcOk(res),
                RetrieveBtcRet::Err(err) => RetrieveBtcResult::RetrieveBtcError(err),
            },
            Err(err) => RetrieveBtcResult::RetrieveBtcString(err.1),
        };
    }
}
