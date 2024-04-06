use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(CandidType, Deserialize, PartialEq, Eq, Hash)]
pub enum ICRCLedgerType {
    ICP,
    CKBTC,
    CKETH,
}

#[derive(Debug, Copy, Clone)]
pub struct Ledger(pub Principal);

#[derive(Debug, Copy, Clone)]
pub struct CKBTCMinter(pub Principal);
