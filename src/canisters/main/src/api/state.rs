use ic_stable_structures::{StableBTreeMap, StableCell};

use crate::memory::{
    get_memory, ICRC_STAKE_MEMORY, ICRC_STAKE_TIMESTAMP_MEMORY, ICRC_TOTAL_VALUE_LOCKED_MEMORY, ICRC_UNSTAKE_TIMESTAMP_MEMORY,
    INIT_MEMORY, STAKE_EXECUTION_LOGS_MEMORY,
};

use super::{
    constants::init_ledger_ids,
    interfaces::{
        icrc_stake::IcrcStakeStableStateType,
        lifecycle::InitArgs,
        metrics::{IcrcMetricsType, MetricsType},
        stake::StakeType,
        state::{HeapStates, StableStates, State},
    },
};

impl Default for State {
    fn default() -> Self {
        ic_cdk::println!("Default State");

        Self {
            stable_state: init_stable_states(),
            heap_state: init_heap_state(),
        }
    }
}

pub(crate) fn init_stable_states() -> StableStates {
    StableStates {
        init: StableCell::init(get_memory(INIT_MEMORY), InitArgs::default()).unwrap(),
        icrc: IcrcStakeStableStateType {
            icrc_stake: StableBTreeMap::init(get_memory(ICRC_STAKE_MEMORY)),
            icrc_stake_ts: StableBTreeMap::init(get_memory(ICRC_STAKE_TIMESTAMP_MEMORY)),
            icrc_unstake_ts: StableBTreeMap::init(get_memory(ICRC_UNSTAKE_TIMESTAMP_MEMORY)),
        },
        metrics: MetricsType {
            icrc_metrics: IcrcMetricsType {
                total_value_locked: StableBTreeMap::init(get_memory(ICRC_TOTAL_VALUE_LOCKED_MEMORY)),
            },
        },
        stake: StakeType {
            stake_execution_logs: StableBTreeMap::init(get_memory(STAKE_EXECUTION_LOGS_MEMORY)),
        },
    }
}

pub(crate) fn init_heap_state() -> HeapStates {
    HeapStates {
        ledger_ids: init_ledger_ids(),
        min_staking_delay: 60 * 1_000_000_000,
    }
}
