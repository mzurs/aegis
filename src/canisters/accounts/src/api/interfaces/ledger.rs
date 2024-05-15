use candid::{CandidType, Nat};
use ic_ledger_utils::types::icrc_types::IcrcTransferResult;
use minter_utils::services::ckbtc::{RetrieveBtcError, RetrieveBtcOk};
use serde::Deserialize;

use super::constants::CanisterName;

#[derive(CandidType, Deserialize, PartialEq, Eq, Hash)]
pub enum ICRCLedgerType {
    ICP,
    CKBTC,
    CKETH,
}

#[derive(CandidType)]
pub enum ConvertCkBTCResult {
    ErrMessage(String),
    IcrcTransferResult(IcrcTransferResult),
    ConvertSuccess(Nat),
    RetrieveBtcError(RetrieveBtcError),
}

#[derive(CandidType)]
pub enum RetrieveBtcResult {
    RetrieveBtcOk(RetrieveBtcOk),
    RetrieveBtcError(RetrieveBtcError),
    RetrieveBtcString(String),
}

#[derive(CandidType, Deserialize)]
pub enum Ledger {
    ICRC(CanisterName),
    ETH,
    BTC,
}

impl Ledger {
    pub fn get_canister_name(&self) -> Option<CanisterName> {
        match self {
            Ledger::ICRC(canister_name) => Some(canister_name.clone()),
            _ => None,
        }
    }
}