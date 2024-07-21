use candid::{CandidType, Nat};
use serde::Deserialize;

use super::state::IcrcTotalValueLockedType;

pub struct MetricsType {
    pub icrc_metrics: IcrcMetricsType,
}

pub struct IcrcMetricsType {
    pub total_value_locked: IcrcTotalValueLockedType,
}

#[derive(CandidType, Deserialize)]
pub enum TotalValueLockedRes {
    ICRC(Nat),
    BTC(u64),
    ETH(u64),
}
