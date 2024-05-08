use candid::Principal;
use ic_cdk::{api::management_canister::bitcoin::BitcoinNetwork, query};

use crate::{api::interfaces::state::StableStates, read_state};

#[query]
fn principal_to_eth_address(principal: candid::Principal) -> String {
    ic_utils::principal_to_eth_address(principal)
}

#[query]
pub fn principal_to_subaccount(principal_id: Principal) -> [u8; 32] {
    ic_utils::principal_to_subaccount(&principal_id)
}

#[query]
pub fn get_bitcoin_network() -> BitcoinNetwork {
    read_state(|n| {
        let state: &StableStates = &n.stable_state;

        state.init.get().bitcoin_network
    })
}
