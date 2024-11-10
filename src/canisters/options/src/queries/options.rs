use ic_cdk::query;

use crate::api::interfaces::{
    options::{Options, OptionsActiveListKey, OptionsType},
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
