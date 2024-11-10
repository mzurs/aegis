use std::time::Duration;

use candid::Nat;
use ic_cdk::println;
use ic_cdk_timers::set_timer;
use ic_ledger_utils::{
    icrc::IcrcLedger,
    types::icrc_types::{IcrcFee, IcrcTransferResult},
};
use ic_utils::{convert_u32_to_subaccount, principal_to_subaccount};
use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};

use crate::{
    api::{
        constants::get_ledger_canister_id,
        interfaces::{
            constants::{IcrcAsset, IcrcAssetValue, StakeAsset},
            icrc_stake::{
                ExecuteUnstakeAmountRes, IcrcDistributeRewardsArgs, IcrcStakeKey, IcrcStakeTimeStampKey,
                IcrcStakeTimeStampValue, IcrcStakeValue, IcrcUnStakeTimeStampKey, IcrcUnStakeTimeStampValue, StakeIcrc,
            },
            stake::StakeTransactionType,
        },
        stake::stake::{add_execution_logs, get_min_stake_delay},
    },
    mutate_state, read_state,
    utils::convert::{biguint_f64::f64_to_biguint, biguint_u128::biguint_to_u128_func},
};

use super::icrc_balances::get_staking_account_balance;

impl StakeIcrc {
    /// Return the amount staked inside main canister for a given principal
    pub async fn get_staked_amount(&self) -> Nat {
        let key: IcrcStakeKey = IcrcStakeKey {
            principal: ic_cdk::caller(),
            icrc_asset: self.ledger.clone(),
        };

        match read_state(|s| s.stable_state.icrc.icrc_stake.get(&key)) {
            Some(res) => res.amount,
            None => Nat::from(0 as u64),
        };

        let ledger: IcrcLedger = ic_ledger_utils::icrc::IcrcLedger::new(get_ledger_canister_id(self.ledger.clone()));

        ledger
            .balance(Account {
                owner: ic_cdk::api::id(),
                subaccount: Some(principal_to_subaccount(&ic_cdk::caller())),
            })
            .await
    }

    pub fn get_staked_tvl(ledger: IcrcAsset) -> IcrcAssetValue {
        let key: &IcrcAsset = &ledger;

        match read_state(|s| s.stable_state.metrics.icrc_metrics.total_value_locked.get(key)) {
            Some(res) => res,
            None => IcrcAssetValue(Nat::from(0 as u32)),
        }
    }

    pub fn if_min_delay_over(&self) -> Result<(), String> {
        let key = &IcrcUnStakeTimeStampKey {
            principal: self.principal,
            icrc_asset: self.ledger.clone(),
        };

        let unstake_timestamp = match read_state(|s| s.stable_state.icrc.icrc_unstake_ts.get(key)) {
            Some(res) => res,
            None => {
                return Err("No value found".to_owned());
            }
        };

        if unstake_timestamp.timestamp == 0 {
            return Err("Unstake Timestamp set to zero".to_owned());
        } else if ic_cdk::api::time() - unstake_timestamp.timestamp <= get_min_stake_delay() {
            return Err(format!(
                "Min unstake delay not reached yet {} - {} <= {}",
                ic_cdk::api::time(),
                unstake_timestamp.timestamp,
                get_min_stake_delay()
            ));
        } else {
            Ok(())
        }
    }

    pub fn get_staked_timestamp(&self) -> u64 {
        match read_state(|s| {
            s.stable_state.icrc.icrc_stake_ts.get(&IcrcStakeTimeStampKey {
                principal: self.principal.clone(),
                icrc_asset: self.ledger.clone(),
            })
        }) {
            Some(res) => res.timestamp,
            None => 0,
        }
    }

    pub fn get_unstaked_timestamp(&self) -> u64 {
        match read_state(|s| {
            s.stable_state.icrc.icrc_unstake_ts.get(&IcrcUnStakeTimeStampKey {
                principal: self.principal.clone(),
                icrc_asset: self.ledger.clone(),
            })
        }) {
            Some(res) => res.timestamp,
            None => 0,
        }
    }

    /// Reset the principal to min_delay for a given asset
    pub fn set_stake_delay_to_zero(&self) {
        let key: IcrcStakeTimeStampKey = IcrcStakeTimeStampKey {
            principal: ic_cdk::caller(),
            icrc_asset: self.ledger.clone(),
        };

        let value: IcrcStakeTimeStampValue = IcrcStakeTimeStampValue { timestamp: 0 };
        mutate_state(|s| s.stable_state.icrc.icrc_stake_ts.insert(key, value));
    }

    /// Reset the principal to min_delay for a given asset
    pub fn add_timestamp_to_stake_tokens(&self) {
        let min_delay: u64 = read_state(|s| s.heap_state.min_staking_delay);

        let key: IcrcStakeTimeStampKey = IcrcStakeTimeStampKey {
            principal: ic_cdk::caller(),
            icrc_asset: self.ledger.clone(),
        };

        // let timestamp = ic_cdk::api::time();

        let value: IcrcStakeTimeStampValue = IcrcStakeTimeStampValue { timestamp: min_delay };
        mutate_state(|s| s.stable_state.icrc.icrc_stake_ts.insert(key, value));

        //-----------------

        let key: IcrcStakeKey = IcrcStakeKey {
            principal: ic_cdk::caller(),
            icrc_asset: self.ledger.clone(),
        };

        let mut ts = match read_state(|s| s.stable_state.icrc.icrc_stake.get(&key)) {
            Some(res) => res,
            None => IcrcStakeValue {
                amount: Nat::from(0 as u32),
                remaining_time_unstake: 0,
                stake_recent_timestamp: min_delay,
            },
        };

        ts.stake_recent_timestamp = min_delay;
        mutate_state(|s| s.stable_state.icrc.icrc_stake.insert(key, ts));
    }

    /// Add Current Timestamp to unstake the tokens
    pub fn add_timestamp_to_unstake_tokens(&self, timestamp: u64) {
        let key: IcrcUnStakeTimeStampKey = IcrcUnStakeTimeStampKey {
            principal: ic_cdk::caller(),
            icrc_asset: self.ledger.clone(),
        };

        let value: IcrcUnStakeTimeStampValue = IcrcUnStakeTimeStampValue { timestamp };

        mutate_state(|s| s.stable_state.icrc.icrc_unstake_ts.insert(key, value));
    }

    /// Reset the principal to min_delay for a given asset
    pub fn set_unstake_timestamp_to_zero(&self) {
        let key: IcrcUnStakeTimeStampKey = IcrcUnStakeTimeStampKey {
            principal: ic_cdk::caller(),
            icrc_asset: self.ledger.clone(),
        };

        let value: IcrcUnStakeTimeStampValue = IcrcUnStakeTimeStampValue { timestamp: 0 };
        mutate_state(|s| s.stable_state.icrc.icrc_unstake_ts.insert(key, value));
    }

    /// insert the new stake amount
    pub async fn insert_principal_amount(&self) {
        let key: IcrcStakeKey = IcrcStakeKey {
            principal: ic_cdk::caller(),
            icrc_asset: self.ledger.clone(),
        };

        let value: IcrcStakeValue = IcrcStakeValue {
            amount: self.get_staked_amount().await,
            remaining_time_unstake: 0,
            stake_recent_timestamp: ic_cdk::api::time(),
        };

        mutate_state(|s| s.stable_state.icrc.icrc_stake.insert(key, value));
    }

    /// delete the new stake amount
    pub async fn delete_principal_amount(&self) {
        let key: IcrcStakeKey = IcrcStakeKey {
            principal: ic_cdk::caller(),
            icrc_asset: self.ledger.clone(),
        };

        mutate_state(|s| s.stable_state.icrc.icrc_stake.remove(&key));

        mutate_state(|s| {
            s.stable_state.icrc.icrc_stake_ts.remove(&IcrcStakeTimeStampKey {
                principal: ic_cdk::caller(),
                icrc_asset: self.ledger.clone(),
            })
        });

        mutate_state(|s| {
            s.stable_state.icrc.icrc_unstake_ts.remove(&IcrcUnStakeTimeStampKey {
                principal: ic_cdk::caller(),
                icrc_asset: self.ledger.clone(),
            })
        });
    }

    pub fn set_unstake_timer(&self) {
        let args: StakeIcrc = Self {
            principal: self.principal,
            ledger: self.ledger.clone(),
        };
        let _timer_id = set_timer(Duration::from_nanos(get_min_stake_delay()), move || {
            let args_copy: StakeIcrc = args;

            ic_cdk::spawn(async move {
                Self::execute_unstake_amount(&args_copy).await;
            })
        });
    }

    pub(crate) async fn execute_unstake_amount(&self) {
        add_execution_logs(
            StakeAsset::ICRC(self.ledger.clone()),
            StakeTransactionType::UnStake,
            format!("Execute Unstake Function Invoked at {}", ic_cdk::api::time()),
        );

        let current_staked_amount = self.get_staked_amount().await;

        if current_staked_amount.clone() <= Nat::from(0 as u32) {
            let message: String = "Amount should be greater than 0 to unstake".to_owned();

            add_execution_logs(
                StakeAsset::ICRC(self.ledger.clone()),
                StakeTransactionType::UnStake,
                message.clone(),
            );

            println!("{}", &message[..]);

            return;
        };

        if self.get_staked_timestamp() != 0 {
            let message: String = "Current Timestamp is not zero".to_owned();

            add_execution_logs(
                StakeAsset::ICRC(self.ledger.clone()),
                StakeTransactionType::UnStake,
                message.clone(),
            );

            println!("Current Timestamp is not zero");

            return;
        }

        match self.if_min_delay_over() {
            Ok(_) => (),
            Err(err) => {
                println!("{}", err);

                return;
            }
        }

        let ledger: IcrcLedger = ic_ledger_utils::icrc::IcrcLedger::new(get_ledger_canister_id(self.ledger.clone()));

        let fee: Nat = match ledger.fee().await {
            IcrcFee::Fee(f) => f,
            IcrcFee::ErrorMessage(err) => {
                ic_cdk::println!("Fee error {}", err);
                return;
            }
        };

        let new_amount: Nat = if current_staked_amount.clone() > fee.clone() {
            Nat::from(current_staked_amount.clone()) - Nat::from(fee.clone())
        } else {
            ic_cdk::print(format!(
                "Staked Amount- Fee {} - {} ",
                current_staked_amount.clone(),
                fee.clone()
            ));
            return;
        };
        let transfer_args: TransferArg = TransferArg {
            from_subaccount: Some(principal_to_subaccount(&self.principal)),

            to: Account {
                owner: self.principal,
                subaccount: None,
            },

            amount: new_amount,

            fee: Option::None,

            memo: Option::None,

            created_at_time: Option::None,
        };

        let transfer_res: IcrcTransferResult = ledger.transfer(transfer_args).await;

        match transfer_res {
            IcrcTransferResult::TransferSuccess(_) => {
                self.set_unstake_timestamp_to_zero();

                // subtract the amount to TVL
                self.decrement_amount_in_tvl(current_staked_amount.clone());

                self.delete_principal_amount().await;

                let message = format!("Unstake Transfer Successfull for Principal {}", self.principal);

                add_execution_logs(
                    StakeAsset::ICRC(self.ledger.clone()),
                    StakeTransactionType::UnStake,
                    message.clone(),
                );

                println!("Unstake Transfer Successfull for Principal {}", self.principal)
            }
            _ => {
                let message = format!("Transfer Error ");

                add_execution_logs(
                    StakeAsset::ICRC(self.ledger.clone()),
                    StakeTransactionType::UnStake,
                    message.clone(),
                );

                return;
            }
        };

        ()
    }

    pub async fn execute_unstake_amount_manual(&self) -> ExecuteUnstakeAmountRes {
        let current_staked_amount: Nat = self.get_staked_amount().await;

        if current_staked_amount.clone() <= Nat::from(0 as u32) {
            println!("Amount should be greater than 0 to unstake");

            return ExecuteUnstakeAmountRes::ErrorMessage("Amount should be greater than 0 to unstake".to_owned());
        };

        if self.get_staked_timestamp() != 0 {
            println!("Current Timestamp is not zero");

            return ExecuteUnstakeAmountRes::ErrorMessage("Current Timestamp is not zero".to_owned());
        }

        match self.if_min_delay_over() {
            Ok(_) => (),
            Err(err) => return ExecuteUnstakeAmountRes::ErrorMessage(err),
        }

        let ledger: IcrcLedger = ic_ledger_utils::icrc::IcrcLedger::new(get_ledger_canister_id(self.ledger.clone()));

        let fee = match ledger.fee().await {
            IcrcFee::Fee(fee) => fee,
            IcrcFee::ErrorMessage(err) => {
                ic_cdk::println!("Fee error {}", err);
                return ExecuteUnstakeAmountRes::ErrorMessage(format!("Fee error {}", err));
            }
        };

        add_execution_logs(
            StakeAsset::ICRC(self.ledger.clone()),
            StakeTransactionType::UnStake,
            format!("Staked Amount- Fee {} - {} ", current_staked_amount.clone(), fee.clone()),
        );

        let new_amount: Nat = if current_staked_amount.clone() > fee.clone() {
            Nat::from(current_staked_amount.clone()) - Nat::from(fee.clone())
        } else {
            return ExecuteUnstakeAmountRes::ErrorMessage(format!(
                "Staked Amount- Fee {} - {} ",
                current_staked_amount.clone(),
                fee.clone()
            ));
        };

        add_execution_logs(
            StakeAsset::ICRC(self.ledger.clone()),
            StakeTransactionType::UnStake,
            format!(
                "Staked Amount- Fee {} - {}  new amount {}",
                current_staked_amount.clone(),
                fee.clone(),
                new_amount.clone()
            ),
        );

        let transfer_args: TransferArg = TransferArg {
            from_subaccount: Some(principal_to_subaccount(&self.principal)),

            to: Account {
                owner: self.principal,
                subaccount: None,
            },

            amount: new_amount.to_owned(), //Nat::from(999_800_000 as u64),

            fee: None,

            memo: Option::None,

            created_at_time: Option::None,
        };

        add_execution_logs(
            StakeAsset::ICRC(self.ledger.clone()),
            StakeTransactionType::UnStake,
            format!(
                "Staked Amount- Fee {} - {}  new amount {}",
                current_staked_amount.clone(),
                fee.clone(),
                transfer_args.clone().amount
            ),
        );

        let transfer_res: IcrcTransferResult = ledger.transfer(transfer_args).await;

        match transfer_res {
            IcrcTransferResult::TransferSuccess(_) => {
                self.set_unstake_timestamp_to_zero();

                // subtract the amount to TVL
                self.decrement_amount_in_tvl(current_staked_amount.clone());

                self.delete_principal_amount().await;

                println!("Unstake Transfer Successfull for Principal {}", self.principal);

                let message: String = format!("Unstake Transfer Successfull for Principal {}", self.principal);

                add_execution_logs(
                    StakeAsset::ICRC(self.ledger.clone()),
                    StakeTransactionType::UnStake,
                    message.clone(),
                );

                return ExecuteUnstakeAmountRes::Success;
            }
            IcrcTransferResult::TransferErrorMessage(err) => {
                let message = format!("Transfer Error ");

                add_execution_logs(
                    StakeAsset::ICRC(self.ledger.clone()),
                    StakeTransactionType::UnStake,
                    message.clone(),
                );
                return ExecuteUnstakeAmountRes::TransferError(err.to_owned());
            }
            IcrcTransferResult::TransferErrorString(err) => {
                let message: String = format!("Transfer Error ");

                add_execution_logs(
                    StakeAsset::ICRC(self.ledger.clone()),
                    StakeTransactionType::UnStake,
                    message.clone(),
                );
                return ExecuteUnstakeAmountRes::ErrorMessage(err.to_owned());
            }
        };
    }

    /// Increment the Amount in TVL
    pub fn increment_amount_in_tvl(&self, amount: Nat) {
        let key: &IcrcAsset = &self.ledger;

        let mut tvl: Nat = Self::get_staked_tvl(self.ledger.to_owned()).0;

        tvl = tvl.clone() + amount.to_owned();

        mutate_state(|s| {
            s.stable_state
                .metrics
                .icrc_metrics
                .total_value_locked
                .insert(key.clone(), IcrcAssetValue(tvl))
        });
    }

    /// Drecrement the Amount in TVL
    pub fn decrement_amount_in_tvl(&self, amount: Nat) {
        let key: &IcrcAsset = &self.ledger;

        let mut tvl: Nat = Self::get_staked_tvl(self.ledger.clone()).0;

        add_execution_logs(
            StakeAsset::ICRC(self.ledger.clone()),
            StakeTransactionType::UnStake,
            format!("TVL Amount- AMount {} - {}  ", tvl.clone(), amount.clone(),),
        );
        if tvl.clone() >= amount.to_owned() {
            tvl = tvl.clone() - amount.to_owned();

            mutate_state(|s| {
                s.stable_state
                    .metrics
                    .icrc_metrics
                    .total_value_locked
                    .insert(key.clone(), IcrcAssetValue(tvl.to_owned()))
            });
        }

        println!("TVL {}  is less than amount {}", tvl.clone(), amount.clone())
    }

    pub async fn icrc_distribute_rewards(args: IcrcDistributeRewardsArgs) {
        ic_cdk::println!(" icrc_distribute_rewards start");

        let ledger: IcrcLedger = ic_ledger_utils::icrc::IcrcLedger::new(get_ledger_canister_id(args.icrc_asset.clone()));
        let asset = args.clone().icrc_asset.clone();
        let asset_fee: Nat = match ledger.fee().await {
            ic_ledger_utils::types::icrc_types::IcrcFee::Fee(res) => res,
            ic_ledger_utils::types::icrc_types::IcrcFee::ErrorMessage(err) => {
                ic_cdk::println!("Fee Error: {}", err);
                return;
            }
        };
        let staking_account_balance: f64 =
            match biguint_to_u128_func(&get_staking_account_balance(args.clone().icrc_asset.clone()).await.0) {
                Ok(res) => res,
                Err(err) => {
                    ic_cdk::println!("Error: {}", err);
                    0u128
                }
            } as f64;

        ic_cdk::println!(" staking_account_balance {}", staking_account_balance.clone());

        let min_stake_delay: u64 = get_min_stake_delay();

        ic_cdk::println!("min_stake_delay: {}", min_stake_delay);

        let tvl: IcrcAssetValue = Self::get_staked_tvl(args.clone().icrc_asset.clone());
        ic_cdk::println!("tvl: {:?}", tvl);

        if mutate_state(|s| s.stable_state.icrc.icrc_stake.is_empty()) {
            ic_cdk::println!("Stake User Map is Empty:");

            return;
        }

        mutate_state(|s| {
            s.stable_state
                .icrc
                .icrc_stake
                .iter()
                .filter(|(_k, v)| v.stake_recent_timestamp.clone() >= min_stake_delay.clone())
                .for_each(|(key, value)| {
                    let tvl_f64: f64 = match biguint_to_u128_func(&tvl.0 .0) {
                        Ok(res) => res,
                        Err(err) => {
                            ic_cdk::println!("Error: {}", err);
                            0u128
                        }
                    } as f64;

                    ic_cdk::println!("amount: {}", value.to_owned().amount);

                    let amount_f64: f64 = match biguint_to_u128_func(&value.amount.0) {
                        Ok(res) => res,
                        Err(err) => {
                            ic_cdk::println!("Error: {}", err);
                            0u128
                        }
                    } as f64;
                    ic_cdk::println!("amount_f64: {}", amount_f64);

                    let staker_share: f64 = amount_f64 / tvl_f64;
                    ic_cdk::println!("staker_share: {}", staker_share);

                    let payout_amount_f64 = staker_share * staking_account_balance;
                    ic_cdk::println!("payout_amount_f64: {}", payout_amount_f64);

                    let payout_amount: Nat = match f64_to_biguint(payout_amount_f64) {
                        Some(res) => Nat::from(res),
                        None => {
                            ic_cdk::println!("Error in conversion of  Payout Amount {} ", payout_amount_f64.clone());
                            Nat::from(0 as u64)
                        }
                    };
                    ic_cdk::println!("payout_amount: {}", payout_amount.clone());

                    let transfer_args: TransferArg = TransferArg {
                        from_subaccount: Some(convert_u32_to_subaccount(1)),
                        to: Account {
                            owner: key.principal.clone(),
                            subaccount: Option::None,
                        },
                        fee: None,
                        created_at_time: None,
                        memo: None,
                        amount: payout_amount.clone() - asset_fee.clone(),
                    };

                    ic_cdk::spawn(async move {
                        let transfer_res: IcrcTransferResult = ledger.transfer(transfer_args.to_owned()).await;

                        match transfer_res {
                            IcrcTransferResult::TransferSuccess(_) => {
                                let message = format!(
                                    "Distribute Rewards Transfer Successfull for Principal {}",
                                    transfer_args.to_owned().to.owner.to_owned()
                                );

                                add_execution_logs(
                                    StakeAsset::ICRC(asset.clone()),
                                    StakeTransactionType::ClaimRewards,
                                    message.clone(),
                                );

                                ic_cdk::println!("Successfuly Distributed Rewards to Principal {} ", key.principal.to_owned(),);
                            }
                            IcrcTransferResult::TransferErrorMessage(err) => {
                                let message = format!(
                                    "Distribute Rewards Transfer Error for Principal {} {:?}",
                                    transfer_args.to_owned().to.owner.to_owned(),
                                    err
                                );

                                add_execution_logs(
                                    StakeAsset::ICRC(asset.clone()),
                                    StakeTransactionType::ClaimRewards,
                                    message.clone(),
                                );

                                ic_cdk::println!(" TransferErrorMessage Error Distributed Rewards \n{}", err);
                            }
                            IcrcTransferResult::TransferErrorString(err) => {
                                let message = format!(
                                    "Distribute Rewards Transfer Error for Principal {} {}",
                                    transfer_args.to_owned().to.owner.to_owned(),
                                    err
                                );

                                add_execution_logs(
                                    StakeAsset::ICRC(asset.clone()),
                                    StakeTransactionType::ClaimRewards,
                                    message.clone(),
                                );

                                ic_cdk::println!(" TransferErrorString Error Distributed Rewards \n{}", err);
                            }
                        }
                    })
                })
        });
    }
}
