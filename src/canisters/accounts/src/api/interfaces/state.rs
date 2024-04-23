use ic_stable_structures::{StableBTreeMap, StableCell};
use serde::{Deserialize, Serialize};

use crate::{
    api::lifecycle::init::InitArgs,
    memory::{get_memory, Memory, ACCOUNT_METRICS_MEMORY, CONSTANTS_MEMORY, INIT_MEMORY, USER_ACCOUNTS_MEMORY},
};

use super::{
    account::{AegisAccount, AegisAccountInfo},
    account_metrics::AccountMetrics,
    constants::Constants,
};

pub type AegisAccountsType = StableBTreeMap<AegisAccount, AegisAccountInfo, Memory>;
pub type ConstantsType = StableCell<Constants, Memory>;
pub type InitType = StableCell<InitArgs, Memory>;
pub type AccountMetricsType = StableCell<AccountMetrics, Memory>;

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip, default = "init_stable_states")]
    pub stable_state: StableStates,
}

pub struct StableStates {
    pub aegis_account: AegisAccountsType,
    pub constants: ConstantsType,
    pub init: InitType,
    pub account_metrics: AccountMetricsType,
}

impl Default for State {
    fn default() -> Self {
        Self {
            stable_state: init_stable_states(),
        }
    }
}

pub(crate) fn init_stable_states() -> StableStates {
    StableStates {
        aegis_account: StableBTreeMap::init(get_memory(USER_ACCOUNTS_MEMORY)),
        constants: StableCell::init(get_memory(CONSTANTS_MEMORY), Constants::default()).unwrap(),
        init: StableCell::init(get_memory(INIT_MEMORY), InitArgs::default()).unwrap(),
        account_metrics: StableCell::init(get_memory(ACCOUNT_METRICS_MEMORY), AccountMetrics::default()).unwrap(),
    }
}
