// use blackscholes::OptionType;
use serde::Deserialize;

use super::{options::OptionsType, options_assets::OptionsAssets};

pub trait Premium<T, U> {
    fn calculate_premium(args: T) -> impl std::future::Future<Output = U> + Send;
}

pub struct EuropeanOptions;

pub struct EuropeanOptionsCalculatePremiumArgs {
    pub option_type: OptionsType,
    pub strike_price: f64,
    pub contract_expiry: u64,
    pub asset: OptionsAssets,
}

pub type EuropeanOptionsCalculatePremiumRes = Result<f64, String>;

/**
option_type - The type of option to be priced.

s - The current price of the underlying asset.

k - The strike price of the option.

p - The dividend yield of the underlying asset.

r - The risk-free interest rate.

q - The dividend yield of the underlying asset.

t - The time to maturity of the option in years.

sigma - The volatility of the underlying asset.

 */
pub struct CalculatePremiumUsingBlackScholesArgs {
    pub option_type: OptionsType,
    pub current_price: f32,
    pub strike_price: f32,
    pub risk_free_interest_rate: f32,
    pub implied_volatility: f64,
    pub time_to_maturity_in_years: f32,
}

pub type CalculatePremiumUsingBlackScholesRes = Result<f32, String>;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct DerbitVolatilityIndex {
    pub jsonrpc: String,
    pub result: DerbitVolatilityIndexResult,
    pub usIn: u64,
    pub usOut: u64,
    pub usDiff: u32,
    pub testnet: bool,
}

#[derive(Deserialize)]
pub struct DerbitVolatilityIndexResult {
    pub data: Vec<DerbitVolatilityIndexData>,
    pub continuation: u64,
}

#[derive(Deserialize)]
pub struct DerbitVolatilityIndexData(
    pub u64, // Timestamp
    pub f64, // Open price
    pub f64, // High price
    pub f64, // Low price
    pub f64, // Close price
);
