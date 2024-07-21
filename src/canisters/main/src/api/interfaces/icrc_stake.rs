use candid::{CandidType, Nat, Principal};
use ic_ledger_utils::types::icrc_types::IcrcTransferFromResult;
use icrc_ledger_types::icrc1::transfer::TransferError;
use serde::Deserialize;

use super::{
    constants::IcrcAsset,
    state::{IcrcStakeTimeStampType, IcrcStakeType, IcrcUnStakeTimeStampType},
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Ord, Eq, Deserialize, CandidType)]
pub struct IcrcStakeKey {
    pub principal: Principal,
    pub icrc_asset: IcrcAsset,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Ord, Eq, Deserialize, CandidType)]
pub struct IcrcStakeValue {
    pub amount: Nat,
    pub remaining_time_unstake: u64,
    pub stake_recent_timestamp:u64
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Ord, Eq, Deserialize, CandidType)]
pub struct IcrcStakeTimeStampKey {
    pub principal: Principal,
    pub icrc_asset: IcrcAsset,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Ord, Eq, Deserialize, CandidType)]
pub struct IcrcStakeTimeStampValue {
    pub timestamp: u64,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Ord, Eq, Deserialize, CandidType)]
pub struct IcrcUnStakeTimeStampKey {
    pub principal: Principal,
    pub icrc_asset: IcrcAsset,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Ord, Eq, Deserialize, CandidType)]
pub struct IcrcUnStakeTimeStampValue {
    pub timestamp: u64,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct StakeIcrc {
    pub principal: Principal,
    pub ledger: IcrcAsset,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct StakeIcrcInitArgs {
    pub principal: Principal,
    pub ledger: IcrcAsset,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct StakeIcrcArgs {
    pub use_account: bool,
    pub amount: Nat,
}

#[derive(Debug, CandidType, Deserialize)]
pub enum StakeIcrcRes {
    Success,
    ErrorMessage(String),
    TransferError(IcrcTransferFromResult),
}

#[derive(Debug, CandidType, Deserialize)]
pub struct UnStakeIcrcArgs {
    pub to_account: bool,
    pub amount: Nat,
}

#[derive(Debug, CandidType, Deserialize)]
pub enum UnStakeIcrcRes {
    Success,
    ErrorMessage(String),
    TransferError(IcrcTransferFromResult),
}

pub struct IcrcStakeStableStateType {
    pub icrc_stake: IcrcStakeType,
    pub icrc_stake_ts: IcrcStakeTimeStampType,
    pub icrc_unstake_ts: IcrcUnStakeTimeStampType,
}

#[derive(Debug, CandidType, Deserialize)]
pub enum ExecuteUnstakeAmountRes {
    TransferError(TransferError),
    Success,
    ErrorMessage(String),
}

#[derive(Debug, CandidType, Deserialize)]
pub struct IcrcSetRewardsDurationArgs {
    pub duration_secs: u64,
    pub icrc_asset: IcrcAsset,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct IcrcDistributeRewardsArgs {
    pub icrc_asset: IcrcAsset,
}
