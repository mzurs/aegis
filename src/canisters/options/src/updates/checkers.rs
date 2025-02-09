use candid::Nat;
use ic_cdk::update;
use management_canister::ManagementCanister;

use crate::{
    api::{
        interfaces::{
            constants::CanisterName,
            exchange::Ticker,
            options::OptionsType,
            options_assets::OptionsAssets,
            premium::{EuropeanOptions, EuropeanOptionsCalculatePremiumArgs, EuropeanOptionsCalculatePremiumRes, Premium},
        },
        utils::constants::get_canister_id,
    },
    queries::checkers::convert_xrc_non_human_to_human,
};

#[update]
pub async fn get_exchange_rate(asset: OptionsAssets) -> Result<u64, String> {
    let mgmt: ManagementCanister = ManagementCanister::new();
    mgmt.xrc(Into::<Ticker>::into(asset).0, get_canister_id(CanisterName::ExchangeRate))
        .await
}

#[update]
pub async fn calculate_premium(
    strike_price: Nat,
    option_type: OptionsType,
    contract_expiry: u64,
    asset: OptionsAssets,
) -> EuropeanOptionsCalculatePremiumRes {
    EuropeanOptions::calculate_premium(EuropeanOptionsCalculatePremiumArgs {
        option_type,
        strike_price: convert_xrc_non_human_to_human(strike_price),
        contract_expiry,
        asset,
    })
    .await
}
