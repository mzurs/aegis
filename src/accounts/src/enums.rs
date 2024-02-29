use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub enum Metric {
    UserCounts,
    ActiveUsers,
}

#[derive(CandidType, Deserialize)]
pub enum MetricValues {
    UserCounts(u64),
    ActiveUsers(u64),
}
