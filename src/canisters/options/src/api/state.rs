use ic_stable_structures::{StableBTreeMap, StableCell};

use crate::memory::{
    get_memory, CALL_OPTIONS_ACTIVE_LIST_BY_PRINCIPAL_MEMORY, CONTRACT_OFFER_DURATION_MEMORY, CONTRACT_TIMESTAMPS_MEMORY,
    INIT_MEMORY, OPTIONS_ACTIVE_LIST_MEMORY, OPTIONS_MEMORY, PUT_OPTIONS_ACTIVE_LIST_BY_PRINCIPAL_MEMORY,
};

use super::{
    interfaces::{
        lifecycle::InitArgs,
        state::{HeapStates, StableStates, State},
    },
    utils::constants::init_icrc_ledger_ids,
};

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
        init: StableCell::init(get_memory(INIT_MEMORY), InitArgs::default()).unwrap(),
        options: StableBTreeMap::init(get_memory(OPTIONS_MEMORY)),
        options_active_list: StableBTreeMap::init(get_memory(OPTIONS_ACTIVE_LIST_MEMORY)),
        put_options_active_list_by_principal: StableBTreeMap::init(get_memory(PUT_OPTIONS_ACTIVE_LIST_BY_PRINCIPAL_MEMORY)),
        call_options_active_list_by_principal: StableBTreeMap::init(get_memory(CALL_OPTIONS_ACTIVE_LIST_BY_PRINCIPAL_MEMORY)),
        contract_timestamps: StableBTreeMap::init(get_memory(CONTRACT_TIMESTAMPS_MEMORY)),
        contract_offer_duration_timestamps: StableBTreeMap::init(get_memory(CONTRACT_OFFER_DURATION_MEMORY)),
        principal_trade_history: StableBTreeMap::init(get_memory(CONTRACT_TIMESTAMPS_MEMORY)),
        // insurance_contract_timer: StableVec::init(get_memory(INSURANCE_TIMERS_MEMORY)).unwrap(),
        // insurance_buyers: StableBTreeMap::init(get_memory(INSURANCE_BUYERS_MEMORY)),
        // insurance_sellers: StableBTreeMap::init(get_memory(INSURANCE_SELLERS_MEMORY)),
        // insurance_contract_execution_logs: StableBTreeMap::init(get_memory(INSURANCE_CONTRACT_EXECUTION_LOGS)),
        // user_trade_history: StableBTreeMap::init(get_memory(USER_INSURANCE_LIST_HISTORY)),
        // insurance_active_list: StableBTreeMap::init(get_memory(INSURANCE_ACTIVE_CONTRACT_LIST)),
    }
}

pub(crate) fn init_heap_state() -> HeapStates {
    HeapStates {
        icrc_ledger_ids: init_icrc_ledger_ids(),
    }
}
