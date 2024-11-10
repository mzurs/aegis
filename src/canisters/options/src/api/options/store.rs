use candid::{Nat, Principal};
use ic_utils::timestamp_to_date::convert_to_date;

use crate::{
    api::{
        interfaces::{
            options::{
                CallOptionsActiveListByPrincipalKey, Options, OptionsActiveListKey, OptionsContractState, OptionsId,
                OptionsType, PutOptionsActiveListByPrincipalKey, TradedOptionsContractsKey, TradedOptionsContractsValue,
            },
            options_assets::{OptionsAssets, OptionsAssetsByNames},
        },
        utils::amount_conversion::convert_asset_amount_to_human,
    },
    mutate_state,
};

impl Options {
   pub(crate)  fn create_option_name(asset: OptionsAssets, asset_amount: Nat, contract_expiry: u64, options_type: OptionsType) -> String {
        format!(
            "{}-{}-{}-{}",
            Into::<String>::into(asset.to_owned()),
            convert_to_date(contract_expiry),
            convert_asset_amount_to_human(asset, asset_amount),
            Into::<String>::into(options_type).chars().next().unwrap()
        )
    }
    ///
    ///  Store the newly created Options contract to StableMemory
    ///
    pub(super) fn create_options(
        id: OptionsId,
        seller: Principal,
        contract_state: OptionsContractState,
        asset: OptionsAssets,
        asset_amount: Nat,
        contract_expiry: u64,
        options_type: OptionsType,
        timestamp: u64,
        offer_duration: u64,
    ) {
        let option: Options = Options {
            seller,
            contract_state,
            asset: asset.to_owned(),
            asset_amount: asset_amount.to_owned(),
            contract_expiry,
            buyer: Option::None,
            options_type: options_type.to_owned(),
            timestamp,
            // BTC-241108-75000-C
            name: Self::create_option_name(asset, asset_amount, contract_expiry, options_type),
            offer_duration,
        };

        mutate_state(|s| s.stable_state.options.insert(id, option));
    }

    ///
    ///  Update the Options contract to StableMemory
    ///
    pub(super) fn update_options(
        id: OptionsId,
        Options {
            name,
            seller,
            contract_state,
            asset,
            asset_amount,
            contract_expiry,
            buyer,
            options_type,
            timestamp,
            offer_duration,
        }: Options,
    ) {
        let option: Options = Options {
            name,
            seller,
            contract_state,
            asset,
            asset_amount,
            contract_expiry,
            buyer,
            options_type,
            timestamp,
            offer_duration,
        };

        mutate_state(|s| s.stable_state.options.insert(id, option));
    }

    ///
    /// insert the option to trade active list
    ///
    pub(crate) fn add_option_to_active_list(
        id: OptionsId,
        options_type: OptionsType,
        options_asset: OptionsAssetsByNames,
        timestamp: u64,
        offer_duration: u64,
    ) {
        let obj: OptionsActiveListKey = OptionsActiveListKey {
            id,
            options_type,
            options_asset: options_asset.into(),
            timestamp,
            offer_duration,
        };

        mutate_state(|s| s.stable_state.options_active_list.insert(obj, ()));
    }

    ///
    /// insert the currently active traded put options contract by principal
    ///  
    pub(crate) fn add_option_to_put_active_list_by_principal(id: OptionsId, principal: Principal) {
        mutate_state(|s| {
            s.stable_state
                .put_options_active_list_by_principal
                .insert(PutOptionsActiveListByPrincipalKey { id, principal }, ())
        });
    }

    ///
    /// remove the currently active traded put options contract by principal
    ///  
    pub fn remove_option_from_put_active_list_by_principal(id: OptionsId, principal: Principal) {
        mutate_state(|s| {
            s.stable_state
                .put_options_active_list_by_principal
                .remove(&PutOptionsActiveListByPrincipalKey { id, principal })
        });
    }

    ///
    /// insert the currently active traded call options contract by principal
    ///  
    pub fn add_option_to_call_active_list_by_principal(id: OptionsId, principal: Principal) {
        mutate_state(|s| {
            s.stable_state
                .call_options_active_list_by_principal
                .insert(CallOptionsActiveListByPrincipalKey { id, principal }, ())
        });
    }

    ///
    /// remove the currently active traded call options contract by principal
    ///  
    pub fn remove_option_from_call_active_list_by_principal(id: OptionsId, principal: Principal) {
        mutate_state(|s| {
            s.stable_state
                .call_options_active_list_by_principal
                .remove(&CallOptionsActiveListByPrincipalKey { id, principal })
        });
    }

    ///
    /// insert the option to an option trade history for a given principal
    ///
    pub(crate) fn add_option_to_trade_history_by_principal(
        principal: Principal,
        contract_state: String,
        timestamp: u64,
        id: OptionsId,
        options_name: String,
        options_type: String,
        trade_timestamp: u64,
    ) {
        mutate_state(|s| {
            s.stable_state.principal_trade_history.insert(
                TradedOptionsContractsKey {
                    principal,
                    contract_state,
                    timestamp,
                    id,
                },
                TradedOptionsContractsValue {
                    options_name,
                    options_type,
                    trade_timestamp,
                },
            )
        });
    }

    ///
    /// Update the option traded history  
    ///
    pub(crate) fn update_option_trade_history_by_principal(
        principal: Principal,
        prev_contract_state: String,
        new_contract_state: String,
        timestamp: u64,
        id: OptionsId,
        options_type: String,
        options_name: String,
        trade_timestamp: u64,
    ) {
        mutate_state(|s| {
            s.stable_state.principal_trade_history.remove(&TradedOptionsContractsKey {
                principal,
                contract_state: prev_contract_state,
                timestamp,
                id,
            })
        });

        mutate_state(|s| {
            s.stable_state.principal_trade_history.insert(
                TradedOptionsContractsKey {
                    principal,
                    contract_state: new_contract_state,
                    timestamp,
                    id,
                },
                TradedOptionsContractsValue {
                    options_name,
                    options_type,
                    trade_timestamp,
                },
            )
        });
    }

    ///
    /// remove option from an active list
    ///
    pub(crate) fn remove_option_from_active_list(
        id: OptionsId,
        Options {
            asset,
            options_type,
            timestamp,
            offer_duration,
            ..
        }: Options,
    ) {
        mutate_state(|s| {
            s.stable_state.options_active_list.remove(&OptionsActiveListKey {
                options_type,
                options_asset: asset.into(),
                timestamp,
                id,
                offer_duration,
            })
        });
    }
}
