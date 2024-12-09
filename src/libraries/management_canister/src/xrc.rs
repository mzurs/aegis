use candid::Principal;
use candid_xrc::{Asset, AssetClass, GetExchangeRateRequest, GetExchangeRateResult};
use ic_cdk::api::call::call_with_payment;

use crate::ManagementCanister;

mod candid_xrc {
    // This is an experimental feature to generate Rust binding from Candid.
    // You may want to manually adjust some of the types.
    #![allow(dead_code, unused_imports)]
    use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
    use ic_cdk::api::call::CallResult as Result;

    #[derive(CandidType, Deserialize)]
    pub enum AssetClass {
        Cryptocurrency,
        FiatCurrency,
    }

    #[derive(CandidType, Deserialize)]
    pub struct Asset {
        pub class: AssetClass,
        pub symbol: String,
    }

    #[derive(CandidType, Deserialize)]
    pub struct GetExchangeRateRequest {
        pub timestamp: Option<u64>,
        pub quote_asset: Asset,
        pub base_asset: Asset,
    }

    #[derive(CandidType, Deserialize)]
    pub struct ExchangeRateMetadata {
        pub decimals: u32,
        pub forex_timestamp: Option<u64>,
        pub quote_asset_num_received_rates: u64,
        pub base_asset_num_received_rates: u64,
        pub base_asset_num_queried_sources: u64,
        pub standard_deviation: u64,
        pub quote_asset_num_queried_sources: u64,
    }

    #[derive(CandidType, Deserialize)]
    pub struct ExchangeRate {
        pub metadata: ExchangeRateMetadata,
        pub rate: u64,
        pub timestamp: u64,
        pub quote_asset: Asset,
        pub base_asset: Asset,
    }

    #[derive(CandidType, Deserialize)]
    pub enum ExchangeRateError {
        AnonymousPrincipalNotAllowed,
        CryptoQuoteAssetNotFound,
        FailedToAcceptCycles,
        ForexBaseAssetNotFound,
        CryptoBaseAssetNotFound,
        StablecoinRateTooFewRates,
        ForexAssetsNotFound,
        InconsistentRatesReceived,
        RateLimited,
        StablecoinRateZeroRate,
        Other { code: u32, description: String },
        ForexInvalidTimestamp,
        NotEnoughCycles,
        ForexQuoteAssetNotFound,
        StablecoinRateNotFound,
        Pending,
    }

    #[derive(CandidType, Deserialize)]
    pub enum GetExchangeRateResult {
        Ok(ExchangeRate),
        Err(ExchangeRateError),
    }

    impl From<ExchangeRateError> for String {
        fn from(error: ExchangeRateError) -> Self {
            match error {
                ExchangeRateError::AnonymousPrincipalNotAllowed => "AnonymousPrincipalNotAllowed".to_string(),
                ExchangeRateError::CryptoQuoteAssetNotFound => "CryptoQuoteAssetNotFound".to_string(),
                ExchangeRateError::FailedToAcceptCycles => "FailedToAcceptCycles".to_string(),
                ExchangeRateError::ForexBaseAssetNotFound => "ForexBaseAssetNotFound".to_string(),
                ExchangeRateError::CryptoBaseAssetNotFound => "CryptoBaseAssetNotFound".to_string(),
                ExchangeRateError::StablecoinRateTooFewRates => "StablecoinRateTooFewRates".to_string(),
                ExchangeRateError::ForexAssetsNotFound => "ForexAssetsNotFound".to_string(),
                ExchangeRateError::InconsistentRatesReceived => "InconsistentRatesReceived".to_string(),
                ExchangeRateError::RateLimited => "RateLimited".to_string(),
                ExchangeRateError::StablecoinRateZeroRate => "StablecoinRateZeroRate".to_string(),
                ExchangeRateError::Other { code, description } => {
                    format!("Other (code: {}, description: {})", code, description)
                }
                ExchangeRateError::ForexInvalidTimestamp => "ForexInvalidTimestamp".to_string(),
                ExchangeRateError::NotEnoughCycles => "NotEnoughCycles".to_string(),
                ExchangeRateError::ForexQuoteAssetNotFound => "ForexQuoteAssetNotFound".to_string(),
                ExchangeRateError::StablecoinRateNotFound => "StablecoinRateNotFound".to_string(),
                ExchangeRateError::Pending => "Pending".to_string(),
            }
        }
    }
}

impl ManagementCanister {
    // pub fn get_xrc_canister_id() -> Principal {
    //     return Principal::from_text("uf6dk-hyaaa-aaaaq-qaaaq-cai").unwrap();
    // }
    pub async fn xrc(&self, asset: String, xrc_canister_id: Principal) -> Result<u64, String> {
        let base_asset: String = asset;

        let quote_asset = String::from("USDT");

        let xrc_canister_cycles_cost: u64 = 1_000_000_000;

        let xrc_args: GetExchangeRateRequest = GetExchangeRateRequest {
            timestamp: Option::None,
            quote_asset: Asset {
                class: AssetClass::Cryptocurrency,
                symbol: quote_asset,
            },
            base_asset: Asset {
                class: AssetClass::Cryptocurrency,
                symbol: base_asset,
            },
        };
        let (res,): (GetExchangeRateResult,) =
            call_with_payment(xrc_canister_id, "get_exchange_rate", (xrc_args,), xrc_canister_cycles_cost)
                .await
                .unwrap();

        let rate = match res {
            GetExchangeRateResult::Ok(exchange_rate) => exchange_rate.rate,
            GetExchangeRateResult::Err(exchange_rate_error) => return Err(exchange_rate_error.into()),
            // (GetExchangeRateResult::Ok(result),) => result,
            // (GetExchangeRateResult::Err(err),) => return Err(enum_exchange_rate_error(err)),
        };
        Ok(rate)
    }
}
