use candid::CandidType;
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use serde::Deserialize;

/// Accounts Canister Intialization Arguments
#[derive(CandidType, Deserialize, Clone)]
pub struct InitArgs {
    pub bitcoin_network: BitcoinNetwork,
}
