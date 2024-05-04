pub mod api;
pub mod guard;
pub mod lifecycle;
pub mod memory;
pub mod queries;
pub mod updates;
pub mod utils;

use candid::Principal;
use ic_cdk::{
    api::{call::CallResult, management_canister::bitcoin::BitcoinNetwork},
    export_candid,
};

use api::interfaces::{
    account::AegisAccountInfo,
    account_metrics::{Metric, MetricValues},
    constants::{Constants, LedgerIds, MinterIds},
    state::State,
};
use canister_state_macro::canister_state;
use icrc_ledger_types::icrc1::account::Account;
use minter_utils::services::ckbtc::{RetrieveBtcRet, UpdateBalanceRet};

use crate::api::lifecycle::init::InitArgs;

canister_state!(State);

export_candid!();
