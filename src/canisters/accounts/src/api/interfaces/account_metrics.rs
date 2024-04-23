use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub enum Metric {
    UserCounts,
}

#[derive(CandidType, Deserialize)]
pub enum MetricValues {
    UserCounts(u64),
}

#[derive(CandidType, Deserialize)]
pub struct AccountMetrics {
    pub user_counts: u64,
    pub active_users: u64,
}
