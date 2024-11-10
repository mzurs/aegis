use crate::api::interfaces::{
    options::{OptionsContractState, OptionsType},
    options_assets::{OptionsAssets, OptionsAssetsByNames, OptionsAssetsIcrc},
};

impl From<OptionsAssets> for OptionsAssetsByNames {
    fn from(asset: OptionsAssets) -> Self {
        match asset {
            OptionsAssets::ICRC(icrc_asset) => match icrc_asset {
                OptionsAssetsIcrc::ICP => OptionsAssetsByNames::ICP,
                OptionsAssetsIcrc::CKBTC => OptionsAssetsByNames::CKBTC,
                OptionsAssetsIcrc::CKETH => OptionsAssetsByNames::CKETH,
                OptionsAssetsIcrc::CKUSDT => OptionsAssetsByNames::USDT,
            },
            OptionsAssets::ETH => OptionsAssetsByNames::ETH,
            OptionsAssets::BTC => OptionsAssetsByNames::BTC,
        }
    }
}

impl From<OptionsAssets> for String {
    fn from(asset: OptionsAssets) -> Self {
        match asset {
            OptionsAssets::ICRC(icrc_asset) => match icrc_asset {
                OptionsAssetsIcrc::ICP => format!("ICP"),
                OptionsAssetsIcrc::CKBTC => format!("CKBTC"),
                OptionsAssetsIcrc::CKETH => format!("CKETH"),
                OptionsAssetsIcrc::CKUSDT => format!("CKUSDT"),
            },
            OptionsAssets::ETH => format!("ETH"),
            OptionsAssets::BTC => format!("BTC"),
        }
    }
}

impl From<OptionsContractState> for String {
    fn from(value: OptionsContractState) -> Self {
        match value {
            OptionsContractState::OPEN => format!("OPEN"),
            OptionsContractState::CLOSED => format!("CLOSED"),
            OptionsContractState::EXECUTED => format!("EXECUTED"),
            OptionsContractState::EXPIRED => format!("EXPIRED"),
            OptionsContractState::OFFER => format!("OFFER"),
        }
    }
}

impl From<OptionsType> for String {
    fn from(value: OptionsType) -> Self {
        match value {
            OptionsType::PUT => format!("PUT"),
            OptionsType::CALL => format!("CALL"),
        }
    }
}
