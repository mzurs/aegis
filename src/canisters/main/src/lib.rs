pub mod api;
pub mod guard;
pub mod lifecycle;
pub mod memory;
pub mod queries;
pub mod updates;

use candid::{Nat, Principal};
use ic_cdk::export_candid;

use api::interfaces::{
    constants::{IcrcAsset, StakeAsset},
    icrc_stake::{ExecuteUnstakeAmountRes, StakeIcrcArgs, StakeIcrcRes, UnStakeIcrcArgs, UnStakeIcrcRes},
    lifecycle::InitArgs,
    metrics::TotalValueLockedRes,
    stake::{StakeExecutionLogsKeys, StakeExecutionLogsValue},
    state::State,
};
use canister_state_macro::canister_state;

canister_state!(State);

export_candid!();
