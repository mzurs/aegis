use ic_cdk::query;

use crate::api::{
    interfaces::stake::{StakeExecutionLogsKeys, StakeExecutionLogsValue},
    stake::stake::{self, get_execution_logs},
};

#[query]
pub fn get_min_stake_delay_() -> u64 {
    stake::get_min_stake_delay()
}

#[query]
pub fn get_stake_execution_logs() -> Vec<(StakeExecutionLogsKeys, StakeExecutionLogsValue)> {
    get_execution_logs()
}
