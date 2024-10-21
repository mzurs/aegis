use super::interfaces::state::{HeapStates, StableStates, State};

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
        // init: StableCell::init(get_memory(INIT_MEMORY), InsuranceInitArgs::default()).unwrap(),
        // insurance: StableBTreeMap::init(get_memory(INSURANCE_DATA_MEMORY)),
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
        // ledger_ids: init_ledger_ids(),
    }
}
