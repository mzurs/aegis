use crate::api::interfaces::lifecycle::InitArgs;
use api::interfaces::{
    options::{CreateOptionArgs, CreateOptionRes, Options, OptionsActiveListKey},
    options_assets::{OptionsAssetsByNames, OptionsAssetsIcrc},
    state::State,
};
use candid::Principal;
use canister_state_macro::canister_state;
use ic_cdk::export_candid;

pub mod api;
pub mod guard;
pub mod lifecycle;
pub mod memory;
pub mod queries;
pub mod updates;

canister_state!(State);

export_candid!();

// type a = OptionsActiveListKey;
