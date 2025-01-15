use crate::api::interfaces::lifecycle::InitArgs;
use api::interfaces::{
    constants::CanisterName,
    options::{
        CreateOptionArgs, CreateOptionRes, ExecuteOptionRes, Options, OptionsActiveListKey, OptionsContractState, OptionsType,
        TradeOptionRes, TradedOptionsContractsKey, TradedOptionsContractsValue,
    },
    options_assets::{OptionsAssets, OptionsAssetsByNames, OptionsAssetsIcrc},
    premium::EuropeanOptionsCalculatePremiumRes,
    state::State,
};
use candid::{Nat, Principal};
use canister_state_macro::canister_state;
use ic_cdk::{
    api::management_canister::http_request::{HttpResponse, TransformArgs},
    export_candid,
};

pub mod api;
pub mod guard;
pub mod lifecycle;
pub mod memory;
pub mod queries;
pub mod updates;

#[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown"))]
/// A getrandom implementation that always fails
pub fn always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}

#[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown"))]
getrandom::register_custom_getrandom!(always_fail);

canister_state!(State);

export_candid!();

type _A = TradedOptionsContractsKey;
