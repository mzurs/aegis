
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, CandidType, Deserialize, Clone, Debug, )]
pub enum OptionsAssetsIcrc {
    ICP,
    CKBTC,
    CKETH,
    CKUSDT,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, CandidType, Deserialize, Clone)]
pub enum OptionsAssetsEth {
    ETH,
    LINK,
    UNI,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, CandidType, Deserialize, Clone, Debug)]
pub enum OptionsAssets {
    ICRC(OptionsAssetsIcrc),
    ETH,
    BTC,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, CandidType, Deserialize, Clone, Hash)]
pub enum OptionsAssetsByNames {
    ICP,
    CKBTC,
    CKETH,
    ETH,
    BTC,
    USDT,
}
