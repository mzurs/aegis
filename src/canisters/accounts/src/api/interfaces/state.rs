use std::collections::BTreeMap;

use candid::Principal;
use ic_stable_structures::{StableBTreeMap, StableCell};
use serde::{Deserialize, Serialize};

use crate::{
    api::{constants::init_canister_ids, lifecycle::init::InitArgs},
    memory::{get_memory, Memory, ACCOUNT_METRICS_MEMORY, INIT_MEMORY, USER_ACCOUNTS_MEMORY},
};

use super::{
    account::{AegisAccount, AegisAccountInfo},
    account_metrics::AccountMetrics,
    constants::CanisterName,
};

pub type AegisAccountsType = StableBTreeMap<AegisAccount, AegisAccountInfo, Memory>;
pub type InitType = StableCell<InitArgs, Memory>;
pub type AccountMetricsType = StableCell<AccountMetrics, Memory>;

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip, default = "init_stable_states")]
    pub stable_state: StableStates,
    pub heap_state: HeapStates,
}

pub struct StableStates {
    pub aegis_account: AegisAccountsType,
    pub init: InitType,
    pub account_metrics: AccountMetricsType,
}

#[derive(Serialize, Deserialize)]
pub struct HeapStates {
    pub canister_ids: BTreeMap<CanisterName, Principal>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            stable_state: init_stable_states(),
            heap_state: init_heap_state(),
        }
    }
}

pub(crate) fn init_stable_states() -> StableStates {
    StableStates {
        aegis_account: StableBTreeMap::init(get_memory(USER_ACCOUNTS_MEMORY)),
        init: StableCell::init(get_memory(INIT_MEMORY), InitArgs::default()).unwrap(),
        account_metrics: StableCell::init(get_memory(ACCOUNT_METRICS_MEMORY), AccountMetrics::default()).unwrap(),
    }
}

pub(crate) fn init_heap_state() -> HeapStates {
    HeapStates {
        canister_ids: init_canister_ids(),
    }
}
