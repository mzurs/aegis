use api::interface::{
    inflation_points::Country,
    insurance::{
        BuyInsuranceArgs, BuyInsuranceRes, ExecuteInsuranceContractRes, Insurance, InsuranceAssets, InsuranceContractInitArgs,
        InsuranceInitRes, SellInsuranceArgs, SellInsuranceRes,
    },
    lifecycle::InsuranceInitArgs,
    state::{
        InsuranceActiveListKey, InsuranceBuyersKey, InsuranceContractExecutionLogsKeys, InsuranceSellersKey, State,
        UserInsuranceListHistoryKey,
    },
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

canister_state!(State);

// type  a=Nat
export_candid!();
