use candid::{Nat, Principal};
use ic_ledger_utils::types::icrc_types::IcrcTransferResult;
use icrc_ledger_types::icrc1::account::Account;
use minter_utils::{ckbtc::CkBTCMinter, services::ckbtc::RetrieveBtcRet};

use crate::api::{
    constants::get_canister_id,
    interfaces::{
        account::AegisAccount,
        constants::CanisterName,
        ledger::{ConvertCkBTCResult, RetrieveBtcResult},
    },
};

impl AegisAccount {
    pub async fn get_btc_address(&self) -> String {
        let minter_id: Principal = get_canister_id(CanisterName::CKBTCMINTER);

        let minter: CkBTCMinter = CkBTCMinter::new(minter_id);

        minter.get_btc_address(self.0).await
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
            .icrc_transfer_from_wallet(CanisterName::CKBTCMINTER, Some(withdrawal_account), amount.to_owned())
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

    pub(crate) async fn retrieve_btc(btc_address: String, amount: Nat) -> RetrieveBtcResult {
        let minter_id: Principal = get_canister_id(CanisterName::CKBTCMINTER);

        let minter: CkBTCMinter = CkBTCMinter::new(minter_id);

        let result = minter
            .retrieve_btc(btc_address, Nat::to_string(&amount).parse::<u64>().unwrap())
            .await;

        match result {
            Ok(res) => match res.0 {
                RetrieveBtcRet::Ok(res) => RetrieveBtcResult::RetrieveBtcOk(res),
                RetrieveBtcRet::Err(err) => RetrieveBtcResult::RetrieveBtcError(err),
            },
            Err(err) => RetrieveBtcResult::RetrieveBtcString(err.1),
        }
    }
}
