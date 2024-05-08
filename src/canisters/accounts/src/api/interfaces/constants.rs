use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum CanisterName {
    ICP,
    CKBTC,
    CKETH,
    CKBTCMINTER,
    CKETHMINTER,
}
