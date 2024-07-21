use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd, Debug, Hash,Copy)]
pub enum IcrcAsset {
    AEGIS,
    ICP,
    CKBTC,
    CKETH,
}

#[derive(CandidType, Deserialize,Debug)]
pub struct IcrcAssetValue(pub Nat);

#[derive(CandidType, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum StakeAsset {
    ICRC(IcrcAsset),
    BTC,
    ETH,
}
