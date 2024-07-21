use crate::{
    api::interfaces::{
        constants::StakeAsset,
        stake::{StakeExecutionLogsKeys, StakeExecutionLogsValue, StakeTransactionType},
    },
    mutate_state, read_state,
};

pub fn get_min_stake_delay() -> u64 {
    read_state(|s| s.heap_state.min_staking_delay)
}

pub(crate) fn set_min_staking_delay(delay_in_secs: Option<u64>) -> u64 {
    let one_day: u64 = 24;
    let one_hour: u64 = 60;
    let one_minute: u64 = 60;
    let one_second: u64 = 1_000_000_000;
    let min_days = 7 as u64;

    // 7 days * 24 hours/day * 60 minutes/hour * 60 seconds/minute * 1,000,000,000 nanoseconds/second
    //  = 604,800,000,000,000 nanoseconds

    let delay = match delay_in_secs {
        Some(delay) => delay * one_second,
        None => min_days * one_day * one_hour * one_minute * one_second,
    };

    mutate_state(|s| s.heap_state.min_staking_delay = delay);

    read_state(|s| s.heap_state.min_staking_delay)
}

pub fn add_execution_logs(asset_type: StakeAsset, transaction_type: StakeTransactionType, message: String) {
    let key: StakeExecutionLogsKeys = StakeExecutionLogsKeys {
        asset_type,
        transaction_type,
        timestamp: ic_cdk::api::time(),
    };
    let value: StakeExecutionLogsValue = StakeExecutionLogsValue { message };

    mutate_state(|s| s.stable_state.stake.stake_execution_logs.insert(key, value));
}

/// Return the List of Stake Execution Operations
pub fn get_execution_logs() -> Vec<(StakeExecutionLogsKeys, StakeExecutionLogsValue)> {
    read_state(|s| s.stable_state.stake.stake_execution_logs.iter().collect::<Vec<_>>())
}
