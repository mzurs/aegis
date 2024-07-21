use candid::Nat;
use ic_cdk::query;

use crate::api::{
    interfaces::{
        constants::IcrcAsset,
        icrc_stake::{StakeIcrc, StakeIcrcInitArgs},
        stake::Stake,
    },
    stake::stake::get_min_stake_delay,
};

#[query]
async fn icrc_get_staked_amount_by_principal(asset: IcrcAsset) -> Nat {
    let icrc_args: StakeIcrcInitArgs = StakeIcrcInitArgs {
        principal: ic_cdk::caller(),
        ledger: asset,
    };

    let icrc: StakeIcrc = Stake::new(icrc_args);

    icrc.get_staked_amount().await
}

#[query]
fn get_staked_timestamp(asset: IcrcAsset) -> u64 {
    let icrc_args: StakeIcrcInitArgs = StakeIcrcInitArgs {
        principal: ic_cdk::caller(),
        ledger: asset,
    };

    let icrc: StakeIcrc = Stake::new(icrc_args);
    icrc.get_staked_timestamp()
}

#[query]
fn get_unstaked_timestamp(asset: IcrcAsset) -> u64 {
    let icrc_args: StakeIcrcInitArgs = StakeIcrcInitArgs {
        principal: ic_cdk::caller(),
        ledger: asset,
    };

    let icrc: StakeIcrc = Stake::new(icrc_args);
    icrc.get_unstaked_timestamp()
}

#[query]
fn if_min_delay_over(asset: IcrcAsset) -> Result<String, String> {
    let icrc_args: StakeIcrcInitArgs = StakeIcrcInitArgs {
        principal: ic_cdk::caller(),
        ledger: asset,
    };
    let icrc: StakeIcrc = Stake::new(icrc_args);

    let unstake_timestamp = icrc.get_unstaked_timestamp();

    match icrc.if_min_delay_over() {
        Ok(_) => Ok(format!(
            "Min unstake delay reached   {} - {} <= {}",
            ic_cdk::api::time(),
            unstake_timestamp,
            get_min_stake_delay()
        )),
        Err(err) => Err(err),
    }
}
