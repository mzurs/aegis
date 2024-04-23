use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(CandidType, Deserialize, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub struct AegisAccount(pub Principal);

#[derive(CandidType, Deserialize)]
pub struct AegisAccountInfo {
    pub user_id: u64,
    pub user_name: Option<String>,
}
