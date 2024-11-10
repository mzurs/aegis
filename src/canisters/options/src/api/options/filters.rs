use std::{ops::RangeBounds, u64};

use candid::Principal;

use crate::api::interfaces::{
    options::{ContractTimestampsKey, OptionsActiveListKey, OptionsContractState, OptionsType, TradedOptionsContractsKey},
    options_assets::OptionsAssetsByNames,
};

///
/// Filter the active options collection based on asset and option type
///
pub fn filter_active_options(option_type: OptionsType, asset: OptionsAssetsByNames) -> impl RangeBounds<OptionsActiveListKey> {
    let start_key: OptionsActiveListKey = OptionsActiveListKey {
        id: u64::MIN,
        options_type: option_type,
        options_asset: asset.clone(),
        timestamp: u64::MIN,
        offer_duration: u64::MIN,
    };

    let end_key: OptionsActiveListKey = OptionsActiveListKey {
        id: u64::MAX,
        options_type: option_type,
        options_asset: asset.clone(),
        timestamp: u64::MAX,
        offer_duration: u64::MAX,
    };

    start_key..end_key
}

///
/// Filter contract timestamps collection based on time and id
///
pub fn filter_contract_timestamps(
    min: ContractTimestampsKey,
    max: ContractTimestampsKey,
) -> impl RangeBounds<ContractTimestampsKey> {
    min..max
}

///
/// Filter traded options contract by principal first and then by timestamps
///
pub fn filter_traded_options_by_principal(
    principal: Principal,
    option_state: OptionsContractState,
) -> impl RangeBounds<TradedOptionsContractsKey> {
    TradedOptionsContractsKey {
        principal,
        contract_state: option_state.to_owned().into(),
        timestamp: u64::MAX,
        id: u64::MIN,
    }..TradedOptionsContractsKey {
        principal,
        contract_state: option_state.into(),
        timestamp: u64::MIN,
        id: u64::MAX,
    }
}
