use ic_cdk::update;

use crate::{
    api::interfaces::{
        constants::IcrcAsset,
        icrc_stake::{ExecuteUnstakeAmountRes, StakeIcrc, StakeIcrcArgs, StakeIcrcInitArgs, StakeIcrcRes, UnStakeIcrcArgs, UnStakeIcrcRes},
        stake::Stake,
    },
    guard::restrict_anonymous_identity,
};

#[update(guard = "restrict_anonymous_identity")]
async fn icrc_stake_tokens(asset: IcrcAsset, args: StakeIcrcArgs) -> StakeIcrcRes {
    let icrc_args: StakeIcrcInitArgs = StakeIcrcInitArgs {
        principal: ic_cdk::caller(),
        ledger: asset,
    };

    let icrc: StakeIcrc = Stake::new(icrc_args);

    icrc.stake(args).await
}

#[update(guard = "restrict_anonymous_identity")]
async fn icrc_unstake_tokens(asset: IcrcAsset, args: UnStakeIcrcArgs) -> UnStakeIcrcRes {
    let icrc_args: StakeIcrcInitArgs = StakeIcrcInitArgs {
        principal: ic_cdk::caller(),
        ledger: asset,
    };

    let icrc: StakeIcrc = Stake::new(icrc_args);

    icrc.unstake(args).await
}

#[update(guard = "restrict_anonymous_identity")]
async fn icrc_unstake_tokens_manual(asset: IcrcAsset) -> ExecuteUnstakeAmountRes {
    let icrc_args: StakeIcrcInitArgs = StakeIcrcInitArgs {
        principal: ic_cdk::caller(),
        ledger: asset,
    };

    let icrc: StakeIcrc = Stake::new(icrc_args);

    icrc.execute_unstake_amount_manual().await
}
