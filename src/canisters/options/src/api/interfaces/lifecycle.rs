use candid::CandidType;
use serde::Deserialize;

/// Accounts Canister Intialization Arguments
#[derive(CandidType, Deserialize, Clone)]
pub struct InitArgs {}
