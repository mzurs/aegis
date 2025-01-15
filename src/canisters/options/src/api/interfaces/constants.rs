use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, PartialOrd, Ord, CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum CanisterName {
    ExchangeRate,
}
