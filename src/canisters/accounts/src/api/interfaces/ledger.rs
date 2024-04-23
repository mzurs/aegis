use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, PartialEq, Eq, Hash)]
pub enum ICRCLedgerType {
    ICP,
    CKBTC,
    CKETH,
}
