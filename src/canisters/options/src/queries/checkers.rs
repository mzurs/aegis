use candid::Nat;
use ic_cdk::query;

use crate::api::utils::amount_conversion;

#[query]
pub fn remaining_time_in_years(current_timestamp_ns: u64) -> f32 {
    let past_timestamp_ns: u64 = ic_cdk::api::time();

    if current_timestamp_ns - past_timestamp_ns <= 0 {
        return 0.00;
    }
    const NANOS_IN_A_YEAR: f64 = 365.25 * 24.0 * 60.0 * 60.0 * 1_000_000_000.0;
    //31_536_000_000_000_000; // Approximate number of nanoseconds in a year

    ic_cdk::println!("{}", NANOS_IN_A_YEAR);

    // Calculate the difference in nanoseconds
    let duration_ns: u64 = current_timestamp_ns.saturating_sub(past_timestamp_ns);
    ic_cdk::println!("{}", current_timestamp_ns);
    ic_cdk::println!("{}", past_timestamp_ns);

    ic_cdk::println!("{}", duration_ns);

    // Convert nanoseconds to years
    (duration_ns as f64 / NANOS_IN_A_YEAR) as f32
}

#[query]
pub fn convert_premium_amount_to_non_humans(asset: crate::api::interfaces::options_assets::OptionsAssets, premium: f64) -> Nat {
    amount_conversion::convert_premium_amount_to_non_humans(asset, premium)
}

#[query]
pub fn convert_asset_amount_to_human(asset: crate::api::interfaces::options_assets::OptionsAssets, amount: Nat) -> f64 {
    amount_conversion::convert_asset_amount_to_human(asset, amount)
}

#[query]
pub fn convert_asset_amount_to_non_human(asset: crate::api::interfaces::options_assets::OptionsAssets, amount: f64) -> Nat {
    amount_conversion::convert_asset_amount_to_non_human(asset, amount)
}

#[query]
pub fn convert_xrc_human_to_non_humans(value: f64) -> Nat {
    amount_conversion::convert_xrc_human_to_non_humans(value)
}

#[query]
pub fn convert_xrc_non_human_to_human(value: Nat) -> f64 {
    amount_conversion::convert_xrc_non_human_to_human(value)
}
