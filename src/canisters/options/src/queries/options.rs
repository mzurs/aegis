use ic_cdk::query;

use crate::api::interfaces::{
    options::{
        Options, OptionsActiveListKey, OptionsContractState, OptionsType, TradedOptionsContractsKey,
        TradedOptionsContractsValue,
    },
    options_assets::OptionsAssetsByNames,
};

#[query]
fn get_put_options_by_asset(asset: OptionsAssetsByNames) -> Vec<(OptionsActiveListKey, ())> {
    Options::get_all_active_options_by_options_type_and_asset(OptionsType::PUT, asset)
}

#[query]
fn get_call_options_by_asset(asset: OptionsAssetsByNames) -> Vec<(OptionsActiveListKey, ())> {
    Options::get_all_active_options_by_options_type_and_asset(OptionsType::CALL, asset)
}

#[query]
fn get_all_options() -> Vec<Options> {
    Options::get_all_options()
}

#[query]
fn get_all_options_ids() -> Vec<u64> {
    Options::get_all_options_ids()
}

#[query]
fn get_options_trade_history_by_principal(
    state: OptionsContractState,
) -> Vec<(TradedOptionsContractsKey, TradedOptionsContractsValue)> {
    Options::get_trade_history_of_options_contract_by_principal(ic_cdk::caller(), state)
}

#[query]
fn get_option_by_id(option_id: u64) -> Result<Options, String> {
    Options::get_options_by_id(option_id)
}
