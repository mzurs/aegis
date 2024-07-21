use ic_cdk::update;

use crate::api::interfaces::{
    constants::{IcrcAsset, StakeAsset},
    icrc_stake::{IcrcSetRewardsDurationArgs, StakeIcrc},
    stake::Stake,
};

#[update]
fn set_rewards_duration(args: StakeAsset, duration_in_secs: u64) {
    match args {
        StakeAsset::ICRC(icrc) => {
            let mut set_rewards_args: IcrcSetRewardsDurationArgs = IcrcSetRewardsDurationArgs {
                duration_secs: duration_in_secs.clone(),
                icrc_asset: IcrcAsset::AEGIS,
            };
            match icrc {
                IcrcAsset::AEGIS => {
                    set_rewards_args.icrc_asset = IcrcAsset::AEGIS;
                    StakeIcrc::set_rewards_duration(set_rewards_args);
                }
                IcrcAsset::ICP => {
                    set_rewards_args.icrc_asset = IcrcAsset::ICP;
                    StakeIcrc::set_rewards_duration(set_rewards_args);
                }
                IcrcAsset::CKBTC => {
                    set_rewards_args.icrc_asset = IcrcAsset::CKBTC;
                    StakeIcrc::set_rewards_duration(set_rewards_args);
                }
                IcrcAsset::CKETH => {
                    set_rewards_args.icrc_asset = IcrcAsset::CKETH;
                    StakeIcrc::set_rewards_duration(set_rewards_args);
                }
            }
        }
        StakeAsset::BTC => (),
        StakeAsset::ETH => (),
    };
}
