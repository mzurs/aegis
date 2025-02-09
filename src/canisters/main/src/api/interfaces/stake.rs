use candid::CandidType;
use serde::Deserialize;

use super::{constants::StakeAsset, state::StakeExecutionLogs};

pub trait Stake {
    // stake functions arguments
    type NewArgs;

    type StakeArgs;
    type StakeRes;

    // unstake functions arguments
    type UnstakeArgs;
    type UnStakeRes;

    // claim functions arguments
    type ClaimArgs;
    type ClaimRes;

    // Timer Rewards Args
    type SetRewardsDurationArgs;
    type SetRewardsDurationRes;

    type DistributeRewardsArgs;
    type DistributeRewardsRes;

    fn new(args: Self::NewArgs) -> Self;

    fn stake(&self, args: Self::StakeArgs) -> impl std::future::Future<Output = Self::StakeRes> + Send;

    fn unstake(&self, args: Self::UnstakeArgs) -> impl std::future::Future<Output = Self::UnStakeRes> + Send;

    fn claim(&self, args: Self::ClaimArgs) -> Self::ClaimRes;

    fn set_rewards_duration(args: Self::SetRewardsDurationArgs) -> Self::SetRewardsDurationRes;

    fn distribute_rewards(
        args: Self::DistributeRewardsArgs,
    ) -> impl std::future::Future<Output = Self::DistributeRewardsRes> + Send;
}

#[derive(CandidType, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum StakeTransactionType {
    Stake,
    UnStake,
    ClaimRewards,
}

#[derive(CandidType, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct StakeExecutionLogsKeys {
    pub asset_type: StakeAsset,
    pub transaction_type: StakeTransactionType,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct StakeExecutionLogsValue {
    pub message: String,
}

pub struct StakeType {
    pub stake_execution_logs: StakeExecutionLogs,
}
