use std::time::Duration;

use candid::Nat;
use ic_cdk_timers::set_timer_interval;
use ic_ledger_utils::{icrc::IcrcLedger, types::icrc_types::IcrcTransferFromResult};
use icrc_ledger_types::{icrc1::account::Account, icrc2::transfer_from::TransferFromArgs};

use crate::{
    api::{
        constants::get_ledger_canister_id,
        interfaces::{
            constants::StakeAsset,
            icrc_stake::{
                IcrcDistributeRewardsArgs, IcrcSetRewardsDurationArgs, StakeIcrc, StakeIcrcArgs, StakeIcrcInitArgs,
                StakeIcrcRes, UnStakeIcrcArgs, UnStakeIcrcRes,
            },
            stake::{Stake, StakeTransactionType},
        },
        stake::stake::add_execution_logs,
    },
    queries::uitls::principal_to_subaccount,
};

use super::icrc_balances::get_staking_account_balance;

impl Stake for StakeIcrc {
    type NewArgs = StakeIcrcInitArgs;

    type StakeArgs = StakeIcrcArgs;
    type StakeRes = StakeIcrcRes;

    type UnstakeArgs = UnStakeIcrcArgs;
    type UnStakeRes = UnStakeIcrcRes;

    type ClaimArgs = ();
    type ClaimRes = ();

    type SetRewardsDurationArgs = IcrcSetRewardsDurationArgs;
    type SetRewardsDurationRes = ();

    type DistributeRewardsArgs = IcrcDistributeRewardsArgs;
    type DistributeRewardsRes = ();

    fn new(args: Self::NewArgs) -> Self {
        Self {
            principal: ic_cdk::caller(),
            ledger: args.ledger,
        }
    }

    async fn stake(&self, args: Self::StakeArgs) -> StakeIcrcRes {
        let ledger: IcrcLedger = ic_ledger_utils::icrc::IcrcLedger::new(get_ledger_canister_id(self.ledger.clone()));

        let transfer_args: TransferFromArgs = TransferFromArgs {
            spender_subaccount: Option::None,

            from: Account {
                owner: ic_cdk::caller(),
                subaccount: Option::None,
            },

            to: Account {
                owner: ic_cdk::id(),
                subaccount: Some(principal_to_subaccount(self.principal)),
            },

            amount: args.amount.to_owned(),

            fee: Option::None,

            memo: Option::None,

            created_at_time: Option::None,
        };

        let transfer_res: IcrcTransferFromResult = ledger.transfer_from(transfer_args).await;

        match transfer_res {
            IcrcTransferFromResult::TransferFromSuccess(_) => {
                // commit principal amount to stablememory
                self.insert_principal_amount().await;

                // add the amount to TVL
                self.increment_amount_in_tvl(args.amount.clone());
            }
            _ => {
                return StakeIcrcRes::TransferError(transfer_res);
            }
        };

        //reset the timestamp
        self.set_unstake_timestamp_to_zero();

        self.add_timestamp_to_stake_tokens();

        StakeIcrcRes::Success
    }

    async fn unstake(&self, args: UnStakeIcrcArgs) -> UnStakeIcrcRes {
        let staked_amount: Nat = self.get_staked_amount().await;
        let _unstake_amount = args.amount;

        if staked_amount <= Nat::from(0 as u32) {
            let message = "Amount should be greater than 0 to unstake".to_owned();
            add_execution_logs(
                StakeAsset::ICRC(self.ledger.clone()),
                StakeTransactionType::UnStake,
                message.clone(),
            );

            return UnStakeIcrcRes::ErrorMessage(message);
        }

        self.set_stake_delay_to_zero();

        let timestamp = ic_cdk::api::time();

        ic_cdk::println!("Unstake TimeStamp recorded {}", timestamp);

        add_execution_logs(
            StakeAsset::ICRC(self.ledger.clone()),
            StakeTransactionType::UnStake,
            format!("Unstake TimeStamp recorded {}", timestamp),
        );

        // set timestamp to start un staking tokens
        self.add_timestamp_to_unstake_tokens(timestamp);

        // self.set_unstake_timer();

        UnStakeIcrcRes::Success
    }

    fn claim(&self, _args: Self::ClaimArgs) -> Self::ClaimRes {
        ()
    }

    fn set_rewards_duration(args: Self::SetRewardsDurationArgs) -> Self::SetRewardsDurationRes {
        set_timer_interval(Duration::from_secs(args.duration_secs), move || {
            let args: IcrcDistributeRewardsArgs = IcrcDistributeRewardsArgs {
                icrc_asset: args.icrc_asset.clone(),
            };
            ic_cdk::spawn(async move { Self::distribute_rewards(args).await })
        });
    }

    async fn distribute_rewards(args: Self::DistributeRewardsArgs) -> Self::DistributeRewardsRes {
        ic_cdk::println!(" distribute rewards start");

        if get_staking_account_balance(args.icrc_asset.clone()).await >= Nat::from(100_000_000_000 as u64) {
            Self::icrc_distribute_rewards(args).await
        } else {
            ic_cdk::println!("Minimum Threshold to distribute rewards not reached yet");

            return;
        }
    }
}
