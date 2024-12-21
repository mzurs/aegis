use crate::api::interfaces::{exchange::Ticker, options_assets::OptionsAssets};

impl From<OptionsAssets> for Ticker {
    fn from(value: OptionsAssets) -> Self {
        Ticker(match value {
            OptionsAssets::ICRC(options_assets_icrc) => match options_assets_icrc {
                crate::api::interfaces::options_assets::OptionsAssetsIcrc::ICP => "ICP".to_string(),
                crate::api::interfaces::options_assets::OptionsAssetsIcrc::CKBTC => "BTC".to_string(),
                crate::api::interfaces::options_assets::OptionsAssetsIcrc::CKETH => "ETH".to_string(),
                crate::api::interfaces::options_assets::OptionsAssetsIcrc::CKUSDT => "USDT".to_string(),
            },
            OptionsAssets::ETH => "ETH".to_string(),
            OptionsAssets::BTC => "BTC".to_string(),
        })
    }
}
