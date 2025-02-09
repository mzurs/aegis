pub mod api;
pub mod guard;
pub mod lifecycle;
pub mod memory;
pub mod queries;
pub mod updates;

use candid::{Nat, Principal};
use ic_cdk::{api::management_canister::bitcoin::BitcoinNetwork, export_candid};

use api::interfaces::{
    account::AegisAccountInfo,
    account_metrics::{Metric, MetricValues},
    constants::CanisterName,
    ledger::{ConvertCkBTCResult, RetrieveBtcResult},
    state::State,
};
use canister_state_macro::canister_state;
use ic_ledger_utils::types::icrc_types::{IcrcTransferFromResult, IcrcTransferResult};
use icrc_ledger_types::icrc1::account::Account;

use crate::api::lifecycle::init::InitArgs;

canister_state!(State);

export_candid!();
