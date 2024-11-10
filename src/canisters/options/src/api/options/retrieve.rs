use candid::Principal;

use crate::{
    api::interfaces::{
        options::{
            Options, OptionsActiveListKey, OptionsContractState, OptionsType, TradedOptionsContractsKey,
            TradedOptionsContractsValue,
        },
        options_assets::OptionsAssetsByNames,
    },
    read_state,
};

use super::filters::{filter_active_options, filter_traded_options_by_principal};

impl Options {
    ///
    /// Check whether the option with specific id present in the active option list and in offer phase?
    ///
    pub(crate) fn if_option_contract_is_active(id: u64) -> bool {
        Some(OptionsContractState::OFFER) == read_state(|s| s.stable_state.options.get(&id).map(|x| x.contract_state))
    }

    ///
    /// Get trade history of options contract by principal
    ///
    pub fn get_trade_history_of_options_contract_by_principal(
        principal: Principal,
        option_state: OptionsContractState,
    ) -> Vec<(TradedOptionsContractsKey, TradedOptionsContractsValue)> {
        read_state(|s| {
            s.stable_state
                .principal_trade_history
                .range(filter_traded_options_by_principal(principal, option_state))
                .collect()
        })
    }

    ///
    /// Get the option contract by Id
    ///
    pub fn get_options_by_id(id: u64) -> Result<Options, String> {
        match read_state(|s| s.stable_state.options.get(&id)) {
            Some(res) => Ok(res),
            None => Err(String::from("No Option Contract Found.")),
        }
    }

    ///
    /// Get the all options contracts
    ///
    pub fn get_all_options() -> Vec<Options> {
        read_state(|s| {
            s.stable_state
                .options
                .iter()
                .map(|(_x, y)| y.clone()) // Clone each `y` to avoid lifetime issues
                .collect()
        })
    }

    ///
    /// Get the all options contract Ids
    ///
    pub fn get_all_options_ids() -> Vec<u64> {
        read_state(|s| {
            s.stable_state
                .options
                .iter()
                .map(|(x, _y)| x) // Clone each `y` to avoid lifetime issues
                .collect()
        })
    }

    ///
    /// Get all active Put Options Contracts
    ///
    pub fn get_all_active_options_by_options_type_and_asset(
        options_type: OptionsType,
        asset: OptionsAssetsByNames,
    ) -> Vec<(OptionsActiveListKey, ())> {
        read_state(|s| {
            s.stable_state
                .options_active_list
                .range(filter_active_options(options_type, asset))
                .collect()
        })
    }
}
