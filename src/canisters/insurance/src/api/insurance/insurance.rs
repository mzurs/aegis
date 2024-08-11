use std::time::Duration;

use candid::{Nat, Principal};
use ic_cdk_timers::{clear_timer, set_timer, TimerId};
use ic_ledger_utils::{
    icrc::IcrcLedger,
    types::icrc_types::{IcrcTransferFromResult, IcrcTransferResult},
};
use ic_utils::{convert_u32_to_subaccount, generate_random_number_u32};
use icrc_ledger_types::{
    icrc1::{account::Account, transfer::TransferArg},
    icrc2::transfer_from::TransferFromArgs,
};

use crate::{
    api::{
        interface::{
            inflation_points::{Country, InfationData},
            insurance::{
                BuyInsuranceArgs, BuyInsuranceRes, ExecuteInsuranceContractArgs, ExecuteInsuranceContractRes, Insurance,
                InsuranceAmount, InsuranceAssets, InsuranceCategory, InsuranceContractInitArgs, InsuranceContractStatus,
                InsuranceInitRes, InsuranceRewardsMultiplier, SellInsuranceArgs, SellInsuranceRes,
            },
            state::{
                InsuranceActiveListKey, InsuranceBuyersKey, InsuranceContractExecutionLogsKeys, InsuranceSellersKey,
                UserInsuranceListHistoryKey,
            },
        },
        utils::{
            constants::get_ledger_canister_id,
            convert::{
                biguint_f64::f64_to_biguint,
                biguint_u128::{self, biguint_to_u128_func},
            },
        },
    },
    mutate_state, read_state,
};

use super::insurance_balances::{get_pool_balance_by_insurance_id, get_premium_pool_balance_by_insurance_id};

impl Insurance {
    /**
       Create a new Insurance Contract for a given Asset
    */
    pub(crate) async fn new(args: InsuranceContractInitArgs) -> InsuranceInitRes {
        // Generate a random u64
        let insurance_id: u32 = match generate_random_number_u32().await {
            Ok(id) => id,
            Err(_) => return InsuranceInitRes::ErrorMessage(String::from("Unable to generate insruance id")),
        };

        // Insurance Object initialization
        let insurance: Insurance = Insurance {
            title: args.title,
            description: args.description.clone(),
            issuer: ic_cdk::caller(),
            is_muliple_seller_allowed: args.is_muliple_seller_allowed,
            min_premium_amount: args.min_premium_amount,
            insurance_asset: args.insurance_asset,
            expiry_date: args.expiry_date,
            multiplier: args.multiplier.clone(),
            category: args.category,
            status: InsuranceContractStatus::OPEN,
            last_executed_time: 0,
            min_share_amount: args.min_share_amount,
            last_pool_balance: Nat::from(0 as u32),
            last_premium_balance: Nat::from(0 as u32),
        };

        let timer_id: TimerId = match Self::set_contract_timer(args.expiry_date, insurance_id) {
            Ok(res) => res,
            Err(err) => return InsuranceInitRes::ErrorMessage(err),
        };
        // Transfer the funds from writer wallet to insurance contract
        let ledger: IcrcLedger = ic_ledger_utils::icrc::IcrcLedger::new(get_ledger_canister_id(args.insurance_asset.clone()));

        let transfer_args: TransferFromArgs = TransferFromArgs {
            spender_subaccount: Option::None,

            from: Account {
                owner: ic_cdk::caller(),
                subaccount: Option::None,
            },

            to: Account {
                owner: ic_cdk::id(),
                subaccount: Some(convert_u32_to_subaccount(insurance_id)),
            },

            amount: args.amount.to_owned(),

            fee: Option::None,

            memo: Option::None,

            created_at_time: Option::None,
        };

        let transfer_res: IcrcTransferFromResult = ledger.transfer_from(transfer_args).await;

        match transfer_res {
            IcrcTransferFromResult::TransferFromSuccess(_) => (),
            _ => {
                clear_timer(timer_id);

                return InsuranceInitRes::TransferError(transfer_res);
            }
        };

        let contract_timer: &(u64, u32) = &(args.expiry_date, insurance_id);

        // save the new insurance in stable memory
        mutate_state(|s| s.stable_state.insurance.insert(insurance_id, insurance));

        // insert the insurance liquidity amount of the writer(seller) in stable memory
        mutate_state(|s| {
            let key: InsuranceSellersKey = InsuranceSellersKey {
                insurance_id,
                principal: ic_cdk::caller(),
                time_stamp: ic_cdk::api::time(),
            };
            s.stable_state
                .insurance_sellers
                .insert(key, InsuranceAmount(args.amount.to_owned()))
        });

        mutate_state(|s| {
            s.stable_state.insurance_active_list.insert(
                InsuranceActiveListKey {
                    principal: ic_cdk::caller(),
                    insurance_id,
                },
                (),
            )
        });

        let _ = mutate_state(|s| s.stable_state.insurance_contract_timer.push(contract_timer));

        // add  logs
        Self::add_contract_exection_logs(insurance_id, format!("Insurance with {} successfully created", insurance_id));

        InsuranceInitRes::Success(insurance_id)
    }

    /// Get the insurance with a given id
    pub fn get_insurance_by_id(insurance_id: u32) -> Option<Insurance> {
        mutate_state(|s| s.stable_state.insurance.get(&insurance_id))
    }

    /// Get the Buyer Insurance Contract by Principal(caller id)
    pub fn get_buy_insurance_contract_list_by_princicpal() -> Vec<InsuranceBuyersKey> {
        read_state(|s| {
            s.stable_state
                .insurance_buyers
                .iter()
                .filter(|(key, _)| key.principal == ic_cdk::caller())
                .map(|(key, _)| key)
                .collect()
        })
    }

    /// Get the Seller Insurance Contract by Principal(caller id)
    pub fn get_seller_insurance_contract_list_by_princicpal() -> Vec<InsuranceSellersKey> {
        read_state(|s| {
            s.stable_state
                .insurance_sellers
                .iter()
                .filter(|(key, _)| key.principal == ic_cdk::caller())
                .map(|(key, _)| key)
                .collect()
        })
    }
    /// Returns all active insurance contract
    pub fn get_all_insurance_contracts() -> Vec<(InsuranceActiveListKey, ())> {
        let list: Vec<(InsuranceActiveListKey, ())> = read_state(|s| {
            s.stable_state
                .insurance_active_list
                .iter()
                // .take(chunk_size as usize)
                // .step_by(steps.try_into().unwrap())
                // .map(|&(key, _)| key)  // Destructure to get key only
                .collect::<Vec<_>>()
        });

        list
    }
    /// Get the history of User Traded Insurance Contract
    pub(crate) fn get_user_insurance_history_by_principal(principal: Principal) -> Vec<(UserInsuranceListHistoryKey, u64)> {
        read_state(|s| {
            s.stable_state
                .user_trade_history
                .iter()
                .filter(|(key, _)| key.principal == principal)
                .collect()
        })
    }

    /// Get the contract execution logs By insurance Id
    pub fn get_contract_execution_logs_by_insurance_id(insurance_id: u32) -> Vec<InsuranceContractExecutionLogsKeys> {
        read_state(|s| {
            s.stable_state
                .insurance_contract_execution_logs
                .iter()
                .filter(|(key, _)| key.insurance_id == insurance_id)
                .map(|(k, _)| k)
                .collect()
        })
    }

    /// Get all contract execution logs  
    pub fn get_all_contract_execution_logs() -> Vec<InsuranceContractExecutionLogsKeys> {
        read_state(|s| {
            s.stable_state
                .insurance_contract_execution_logs
                .iter()
                // .filter(|(key, _)| key.insurance_id == insurance_id)
                .map(|(k, _)| k)
                .collect()
        })
    }

    /// Remove insurance contract from active list
    pub fn remove_insuurance_contract_from_active_list(insurance_id: u32) {
        read_state(|state| {
            // Shorten lifetime to match iterator
            let insurance_active_list = &state.stable_state.insurance_active_list;
            insurance_active_list
                .iter()
                .filter(|(key, _)| key.insurance_id == insurance_id)
                .for_each(move |(k, _)| {
                    mutate_state(|s| s.stable_state.insurance_active_list.remove(&k));

                    //add logs
                    Self::add_contract_exection_logs(
                        insurance_id,
                        format!("Principal {} removed from active list", k.principal),
                    );
                });
        });
    }
    /// Delete the insurance contract from stable memory
    #[allow(dead_code)]
    fn delete_insurance_contract(insurance_id: u32) {
        mutate_state(|s| s.stable_state.insurance.remove(&insurance_id));
    }

    /// Remove the insurance id from buyers memory
    fn remove_buyer_from_insurance_buyers_list(key: &InsuranceBuyersKey) {
        mutate_state(|s| s.stable_state.insurance_buyers.remove(key));
    }

    /// Remove the insurance id from sellers memory
    fn remove_seller_from_insurance_sellers_list(key: &InsuranceSellersKey) {
        mutate_state(|s| s.stable_state.insurance_sellers.remove(key));
    }

    /// Change the Insurance Contract Status
    #[allow(dead_code)]
    fn change_insurance_contract_status(self, insurance_id: u32, status: InsuranceContractStatus) {
        let mut insurance: Insurance = self;

        insurance.status = status.to_owned();

        mutate_state(|s| s.stable_state.insurance.insert(insurance_id, insurance));

        //add logs
        Self::add_contract_exection_logs(insurance_id, format!("Insurance Contract Status chaged to {:?}", status));
    }

    /// Insert Contract Executon Logs in a stable memory
    fn add_contract_exection_logs(insurance_id: u32, message: String) {
        let log_keys: InsuranceContractExecutionLogsKeys = InsuranceContractExecutionLogsKeys {
            insurance_id,
            timestamp: ic_cdk::api::time(),
            message,
        };

        mutate_state(|s| s.stable_state.insurance_contract_execution_logs.insert(log_keys, ()));
    }

    /// Buy a share of insruance contract by depositing premiums
    pub async fn buy_insurance_contract(args: BuyInsuranceArgs) -> BuyInsuranceRes {
        // check if insurance of a given id exist
        let insurance: Insurance = match Self::get_insurance_by_id(args.insurance_id) {
            Some(res) => res,
            None => return BuyInsuranceRes::ErrorMessage(format!("Insurance with Id {} not found", args.insurance_id)),
        };

        // check the buyer of an insrance contract should not be issuer
        if insurance.issuer.clone() == ic_cdk::caller() {
            return BuyInsuranceRes::ErrorMessage("Issuer Can Not be Buyer".to_owned());
        }

        // check whether insurance contract alreay expired or not
        if insurance.expiry_date.clone() < ic_cdk::api::time() {
            return BuyInsuranceRes::ErrorMessage(format!("Insurance Contract with Id {} already expired", args.insurance_id));
        }

        // the amount of premium should be greater than min_premium_amount
        if args.premium < insurance.min_premium_amount.clone() {
            return BuyInsuranceRes::ErrorMessage(format!(
                "Min Premium Amount to Participate in Inusrance is {} ",
                insurance.min_premium_amount
            ));
        }
        // Check the current max allocation of a premium amount for a buyer
        let curr_premium: Nat = Self::calculate_buy_insurance_contract_premium(args.insurance_id, &insurance).await;

        ic_cdk::println!("curr_premium {}", curr_premium);
        // check if a premium cannot exceed the remaining max premium amount
        if curr_premium < args.premium {
            return {
                ic_cdk::println!(
                    "Premium amount {} exceeds the current premium amount {} or premium pool threshold reached",
                    args.premium,
                    curr_premium
                );

                BuyInsuranceRes::ErrorMessage(format!(
                    "Premium amount {} exceeds the current premium amount {} or premium pool threshold reached",
                    args.premium, curr_premium
                ))
            };
        }

        // Transfer the funds from buyer wallet to insurance contract
        let ledger: IcrcLedger = ic_ledger_utils::icrc::IcrcLedger::new(get_ledger_canister_id(insurance.insurance_asset));

        let transfer_args: TransferFromArgs = TransferFromArgs {
            spender_subaccount: Option::None,

            from: Account {
                owner: ic_cdk::caller(),
                subaccount: Option::None,
            },

            to: Account {
                owner: ic_cdk::id(),
                subaccount: Some(convert_u32_to_subaccount(args.insurance_id + 1)),
            },

            amount: args.premium.clone(),

            fee: Option::None,

            memo: Option::None,

            created_at_time: Option::None,
        };

        let transfer_res: IcrcTransferFromResult = ledger.transfer_from(transfer_args).await;

        match transfer_res {
            IcrcTransferFromResult::TransferFromSuccess(_) => (),
            _ => return BuyInsuranceRes::TransferError(transfer_res),
        };

        let amount: Nat = args.to_owned().premium;

        // insert the insurance premium amount of the buyer in stable memory
        mutate_state(|s| {
            let key: InsuranceBuyersKey = InsuranceBuyersKey {
                insurance_id: args.insurance_id,
                principal: ic_cdk::caller(),
                time_stamp: ic_cdk::api::time(),
            };

            s.stable_state.insurance_buyers.insert(key, InsuranceAmount(amount))
        });

        // add log
        Self::add_contract_exection_logs(
            args.insurance_id,
            format!(
                "Buy order for Insurance Contract successfully executed for Principal {} with transfer detail {:?}",
                ic_cdk::caller(),
                transfer_res
            ),
        );

        BuyInsuranceRes::Success
    }

    /// Sell a insurance contract by increasing liquidity of a contract
    pub async fn sell_insurance_contract(args: SellInsuranceArgs) -> SellInsuranceRes {
        // check if insurance of a given id exist
        let insurance: Insurance = match Self::get_insurance_by_id(args.insurance_id) {
            Some(res) => res,
            None => return SellInsuranceRes::ErrorMessage(format!("Insurance with Id {} not found", args.insurance_id)),
        };

        // check whether insurance contract alreay expired or not
        if insurance.expiry_date.clone() < ic_cdk::api::time() {
            return SellInsuranceRes::ErrorMessage(format!("Insurance Contract with Id {} already expired", args.insurance_id));
        }

        // check if multiple sellets are allowed by the issuer
        if !insurance.is_muliple_seller_allowed.clone() {
            return SellInsuranceRes::ErrorMessage(String::from("Participation Not Allowed for Sellers"));
        }

        if insurance.min_share_amount.clone() > Some(args.amount.clone()) {
            return SellInsuranceRes::ErrorMessage(format!(
                "Insurance Minimum Share Amount {:?}",
                insurance.min_share_amount.clone()
            ));
        }

        // Transfer the funds from writer wallet to insurance contract
        let ledger: IcrcLedger = ic_ledger_utils::icrc::IcrcLedger::new(get_ledger_canister_id(insurance.insurance_asset));

        let transfer_args: TransferFromArgs = TransferFromArgs {
            spender_subaccount: Option::None,

            from: Account {
                owner: ic_cdk::caller(),
                subaccount: Option::None,
            },

            to: Account {
                owner: ic_cdk::id(),
                subaccount: Some(convert_u32_to_subaccount(args.insurance_id)),
            },

            amount: args.amount.clone(),

            fee: Option::None,

            memo: Option::None,

            created_at_time: Option::None,
        };

        let transfer_res: IcrcTransferFromResult = ledger.transfer_from(transfer_args).await;

        match transfer_res {
            IcrcTransferFromResult::TransferFromSuccess(_) => (),
            _ => return SellInsuranceRes::TransferError(transfer_res),
        };

        let amount: Nat = args.amount.clone();

        // insert the insurance premium amount of the seller in stable memory
        mutate_state(|s| {
            let key: InsuranceSellersKey = InsuranceSellersKey {
                insurance_id: args.insurance_id,
                principal: ic_cdk::caller(),
                time_stamp: ic_cdk::api::time(),
            };
            s.stable_state.insurance_sellers.insert(key, InsuranceAmount(amount))
        });

        // add log
        Self::add_contract_exection_logs(
            args.insurance_id,
            format!(
                "Sell order for Insurance Contract successfully executed for Principal {} with transfer detail {:?}",
                ic_cdk::caller(),
                transfer_res
            ),
        );
        SellInsuranceRes::Success
    }

    /// Execute Insurance Contract for a given contract id
    async fn execute_insurance_contract(args: ExecuteInsuranceContractArgs) -> ExecuteInsuranceContractRes {
        //add logs
        Self::add_contract_exection_logs(args.insurance_id, format!("Contract Execution Start"));
        // check if insurance of a given id exist
        let insurance: Insurance = match Self::get_insurance_by_id(args.insurance_id) {
            Some(res) => res,
            None => {
                //add logs
                Self::add_contract_exection_logs(
                    args.insurance_id,
                    format!("Insurance Id Not Found while executing the contract"),
                );

                return ExecuteInsuranceContractRes::ErrorMessage(format!("Insurance with Id {} not found", args.insurance_id));
            }
        };

        mutate_state(|s| {
            // Add Last_executed time for a insurance contract
            let mut insurance_mut: Insurance = insurance.clone();
            insurance_mut.last_executed_time = ic_cdk::api::time();
            s.stable_state.insurance.insert(args.insurance_id, insurance_mut)
        });

        // At the time of invokation of a insurance contract the present time should equal
        // or greater than insurance expiry time otherwise the error message will generate
        if ic_cdk::api::time() < insurance.expiry_date.clone() {
            //add logs
            Self::add_contract_exection_logs(
                args.insurance_id,
                format!("Expiry time {} for a contract is not reached", insurance.expiry_date.clone(),),
            );

            // this entry can not stored in logs stable memory
            return ExecuteInsuranceContractRes::ErrorMessage(format!(
                "Expiry time {} for a contract is not reached",
                insurance.expiry_date.clone(),
            ));
        }

        //add logs
        Self::add_contract_exection_logs(args.insurance_id, format!("Contract Expiry Date Verified"));
        // get the balance of a insurance contract liquiduty pool
        let insurance_pool_balance: Nat =
            get_pool_balance_by_insurance_id(args.insurance_id, insurance.insurance_asset.clone()).await;

        ic_cdk::println!("insurance_pool_balance{}", insurance_pool_balance);

        // get the balance of a insurance contract premium pool
        let insurance_premium_balance: Nat =
            get_premium_pool_balance_by_insurance_id(args.insurance_id, insurance.insurance_asset.clone()).await;

        ic_cdk::println!("insurance_premium_balance {}", insurance_premium_balance);

        // commit balances before execution
        if insurance.last_pool_balance.clone() == Nat::from(0 as u64)
            && insurance.last_premium_balance.clone() == Nat::from(0 as u64)
        {
            mutate_state(|s| {
                // Add Last_executed time for a insurance contract
                let mut insurance_mut: Insurance = insurance.clone();
                insurance_mut.last_pool_balance = insurance_pool_balance.clone();
                insurance_mut.last_premium_balance = insurance_premium_balance.clone();

                s.stable_state.insurance.insert(args.insurance_id, insurance_mut)
            });
        }

        // check whether the buyers participated in insurance contract
        if insurance_premium_balance < insurance.min_premium_amount.clone() {
            //add logs
            Self::add_contract_exection_logs(
                args.insurance_id,
                format!(
                    "Insurance Premium Pool Balance {} is less min_premium_amount {}",
                    insurance_premium_balance,
                    insurance.min_premium_amount.clone()
                ),
            );

            Self::repay_insurance_amount_to_sellers(args.insurance_id, insurance.insurance_asset.clone()).await;

            Self::change_insurance_contract_status(insurance.clone(), args.insurance_id, InsuranceContractStatus::CLOSED);

            Self::add_contract_exection_logs(args.insurance_id, "None of the buyers participated".to_owned());

            // Self::remove_insuurance_contract_from_active_list(args.insurance_id);

            return ExecuteInsuranceContractRes::Success;
        }

        //add logs
        Self::add_contract_exection_logs(
            args.insurance_id,
            format!(
                "Insurance Premium Pool Balance {} is greater min_premium_amount {}",
                insurance_premium_balance.clone(),
                insurance.min_premium_amount.clone()
            ),
        );

        ic_cdk::println!(
            "Insurance Premium Pool Balance {} is greater min_premium_amount {}",
            insurance_premium_balance.clone(),
            insurance.min_premium_amount.clone()
        );

        let contract_decision: bool = match Self::select_category_with_contract_decision(insurance.clone()).await {
            Ok(res) => res,
            Err(err) => return ExecuteInsuranceContractRes::ErrorMessage(err),
        };

        ic_cdk::println!("contract_decision {}", contract_decision);

        if contract_decision {
            ic_cdk::println!("Buyers won the contract");
            //add logs
            Self::add_contract_exection_logs(args.insurance_id, format!("Buyers won the contract",));
            Self::execute_insurance_contract_for_buyers(args.insurance_id, insurance.clone()).await;

            if get_pool_balance_by_insurance_id(args.insurance_id, insurance.insurance_asset.clone()).await
                > Nat::from(0 as u64)
            {
                Self::repay_excessive_amount_to_sellers(
                    args.insurance_id,
                    insurance.insurance_asset.clone(),
                    insurance_premium_balance.clone(),
                    insurance_pool_balance.clone(),
                )
                .await;
            }
            Self::send_premium_amount_to_sellers(
                args.insurance_id,
                insurance.clone(),
                insurance_pool_balance.clone(),
                insurance_premium_balance.clone(),
            )
            .await;
        } else {
            ic_cdk::println!("Sellers won the contract");

            //add logs
            Self::add_contract_exection_logs(args.insurance_id, format!("Sellers won the contract",));

            Self::execute_insurance_contract_for_sellers(
                args.insurance_id,
                insurance.clone(),
                insurance_pool_balance,
                insurance_premium_balance,
            )
            .await;

            Self::repay_insurance_amount_to_sellers(args.insurance_id, insurance.insurance_asset.clone()).await;
        }

        //add logs
        Self::add_contract_exection_logs(args.insurance_id, format!("Contract Execution End"));
        ExecuteInsuranceContractRes::Success
    }

    /// Repay excessive Amount to Sellers
    pub async fn repay_excessive_amount_to_sellers(
        insurance_id: u32,
        insurance_asset: InsuranceAssets,
        last_insurance_premium_balance: Nat,
        last_insurance_pool_balance: Nat,
    ) -> ExecuteInsuranceContractRes {
        ic_cdk::print("Repay Excessive Amount to Sellers Start");

        let ledger: IcrcLedger = ic_ledger_utils::icrc::IcrcLedger::new(get_ledger_canister_id(insurance_asset.clone()));

        let insurance_asset_fee: Nat = match ledger.fee().await {
            ic_ledger_utils::types::icrc_types::IcrcFee::Fee(res) => res,
            ic_ledger_utils::types::icrc_types::IcrcFee::ErrorMessage(err) => {
                return ExecuteInsuranceContractRes::ErrorMessage(err)
            }
        };

        let insurance_pool_balance_nat = get_pool_balance_by_insurance_id(insurance_id, insurance_asset.clone()).await;
        let insurance_pool_balance: f64 = match biguint_to_u128_func(&insurance_pool_balance_nat.clone().0) {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error: {}", err);
                0u128
            }
        } as f64;
        ic_cdk::println!("Current insurance_pool_balance: {}", insurance_pool_balance);

        let last_insurance_premium_balance: f64 = match biguint_to_u128_func(&last_insurance_premium_balance.clone().0) {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error: {}", err);
                0u128
            }
        } as f64;
        ic_cdk::println!("last_insurance_premium_balance: {}", last_insurance_premium_balance);

        let last_insurance_pool_balance: f64 = match biguint_to_u128_func(&last_insurance_pool_balance.clone().0) {
            Ok(res) => res,
            Err(err) => {
                ic_cdk::println!("Error: {}", err);
                0u128
            }
        } as f64;
        ic_cdk::println!("insurance_pool_balance: {}", insurance_pool_balance);

        // let sellers_amount_map: Vec<(InsuranceSellersKey, InsuranceAmount)> =
        mutate_state(|s| {
            s.stable_state
                .insurance_sellers
                .iter()
                .filter(|(key, _)| key.insurance_id == insurance_id)
                .for_each(|(k, v)| {
                    let seller_amount: f64 = match biguint_to_u128_func(&v.0.clone().0) {
                        Ok(res) => res,
                        Err(err) => {
                            ic_cdk::println!("Error: {}", err);
                            0u128
                        }
                    } as f64;

                    let seller_share: f64 = seller_amount / last_insurance_pool_balance.clone();
                    let seller_share_amount_f64: f64 = seller_share * insurance_pool_balance;
                    let seller_share_amount: Nat = match f64_to_biguint(seller_share_amount_f64) {
                        Some(res) => Nat::from(res),
                        None => {
                            ic_cdk::println!("Error in conversion of  seller_share_amount {} ", seller_share_amount_f64);
                            Nat::from(0 as u64)
                        }
                    };
                    ic_cdk::println!("seller_share_amount {}", seller_share_amount);

                    let transfer_args: TransferArg = TransferArg {
                        from_subaccount: Some(convert_u32_to_subaccount(insurance_id)),

                        to: Account {
                            owner: k.principal.clone(),
                            subaccount: None,
                        },

                        amount: seller_share_amount - insurance_asset_fee.to_owned(),

                        fee: Option::None,

                        memo: Option::None,

                        created_at_time: Option::None,
                    };

                    ic_cdk::spawn(async move {
                        let transfer_res: IcrcTransferResult = ledger.transfer(transfer_args.to_owned()).await;

                        match transfer_res {
                            IcrcTransferResult::TransferSuccess(_) => {
                                Self::remove_seller_from_insurance_sellers_list(&k);

                                ic_cdk::println!("Repay Excessive Tranfer Success {}", transfer_args.to.owner);


                                // add log
                                Self::add_contract_exection_logs(
                                    insurance_id,
                                    format!(
                                        "Repay excessive Amount to Seller Principal {} with transfer details {:?}",
                                        transfer_args.to.owner, transfer_res
                                    ),
                                );
                            }

                            IcrcTransferResult::TransferErrorMessage(_msg) => {
                                ic_cdk::println!(
                                    "Error Occured while transfer(repay excessive) fund to Principal {} with Transfer Error {}",
                                    transfer_args.to.owner,
                                    _msg
                                );

                                Self::add_contract_exection_logs(
                                    insurance_id,
                                    format!(
                                        "Error Occured while transfer(repay excessive) fund to Principal {} with Transfer Error {}",
                                        transfer_args.to.owner, _msg
                                    ),
                                );
                            }

                            IcrcTransferResult::TransferErrorString(_msg_str) => {
                                ic_cdk::println!(
                                    "Error Occured while transfer(repay excessive) fund to Principal {} with Transfer Error {}",
                                    transfer_args.to.owner,
                                    _msg_str
                                );

                                // add log
                                Self::add_contract_exection_logs(
                                    insurance_id,
                                    format!(
                                        "Error Occured while transfer(repay excessive) fund to Principal {} with Transfer Error {}",
                                        transfer_args.to.owner, _msg_str
                                    ),
                                );
                            }
                        }
                    })
                })
        });

        ic_cdk::print("Repay Excessive Amount to Sellers End");

        ExecuteInsuranceContractRes::Success
    }

    /// Repay the amount to Insurance Contract Sellers
    async fn repay_insurance_amount_to_sellers(
        insurance_id: u32,
        insurance_asset: InsuranceAssets,
    ) -> ExecuteInsuranceContractRes {
        ic_cdk::print("Repay to Sellers Start");

        let mut i = 0;

        let ledger: IcrcLedger = ic_ledger_utils::icrc::IcrcLedger::new(get_ledger_canister_id(insurance_asset.clone()));

        let insurance_asset_fee: Nat = match ledger.fee().await {
            ic_ledger_utils::types::icrc_types::IcrcFee::Fee(res) => res,
            ic_ledger_utils::types::icrc_types::IcrcFee::ErrorMessage(err) => {
                return ExecuteInsuranceContractRes::ErrorMessage(err)
            }
        };

        // let sellers_amount_map: Vec<(InsuranceSellersKey, InsuranceAmount)> =
        mutate_state(|s| {
            s.stable_state
                .insurance_sellers
                .iter()
                .filter(|(key, _)| key.insurance_id == insurance_id)
                .for_each(|(k, v)| {
                    i += 1;

                    let repay_amount: Nat = v.0.to_owned() - insurance_asset_fee.clone();
                    ic_cdk::println!("repay_amount {}", repay_amount);

                    let key: InsuranceSellersKey = k.to_owned();

                    // Transfer funds to sellers from insurance contract pool of a given insurance_id
                    let transfer_args: TransferArg = TransferArg {
                        from_subaccount: Some(convert_u32_to_subaccount(insurance_id)),

                        to: Account {
                            owner: key.principal,
                            subaccount: None,
                        },

                        amount: repay_amount,

                        fee: Option::None,

                        memo: Option::None,

                        created_at_time: Option::None,
                    };

                    ic_cdk::spawn(async move {
                        let transfer_res: IcrcTransferResult = ledger.transfer(transfer_args.to_owned()).await;

                        match transfer_res {
                            IcrcTransferResult::TransferSuccess(_) => {
                                Self::remove_seller_from_insurance_sellers_list(&key);

                                ic_cdk::println!("Repay Tranfer Success {}", transfer_args.to.owner);

                                Self::add_insurance_details_to_user_history(key.insurance_id, key.principal);

                                // add log
                                Self::add_contract_exection_logs(
                                    insurance_id,
                                    format!(
                                        "Repay Amount to Seller Principal {} with transfer details {:?}",
                                        transfer_args.to.owner, transfer_res
                                    ),
                                );
                            }

                            IcrcTransferResult::TransferErrorMessage(_msg) => {
                                ic_cdk::println!(
                                    "Error Occured while transfer(repay) fund to Principal {} with Transfer Error {}",
                                    transfer_args.to.owner,
                                    _msg
                                );

                                Self::add_contract_exection_logs(
                                    insurance_id,
                                    format!(
                                        "Error Occured while transfer(repay) fund to Principal {} with Transfer Error {}",
                                        transfer_args.to.owner, _msg
                                    ),
                                );
                            }

                            IcrcTransferResult::TransferErrorString(_msg_str) => {
                                ic_cdk::println!(
                                    "Error Occured while transfer(repay) fund to Principal {} with Transfer Error {}",
                                    transfer_args.to.owner,
                                    _msg_str
                                );

                                // add log
                                Self::add_contract_exection_logs(
                                    insurance_id,
                                    format!(
                                        "Error Occured while transfer(repay) fund to Principal {} with Transfer Error {}",
                                        transfer_args.to.owner, _msg_str
                                    ),
                                );
                            }
                        }
                    })
                })
        });
        ic_cdk::println!("Total Repay Iteration {}", i);

        ic_cdk::print("Repay to Seller End");

        ExecuteInsuranceContractRes::Success
    }

    /// Executes  the given insurance contract in favor of sellers
    pub async fn execute_insurance_contract_for_sellers(
        insurance_id: u32,
        insurance: Insurance,
        insurance_pool_balance: Nat,
        insurance_premium_balance: Nat,
    ) -> ExecuteInsuranceContractRes {
        let i: i32 = 0;

        let ledger: IcrcLedger = ic_ledger_utils::icrc::IcrcLedger::new(get_ledger_canister_id(insurance.insurance_asset));

        let insurance_asset_fee: Nat = match ledger.fee().await {
            ic_ledger_utils::types::icrc_types::IcrcFee::Fee(res) => res,
            ic_ledger_utils::types::icrc_types::IcrcFee::ErrorMessage(err) => {
                return ExecuteInsuranceContractRes::ErrorMessage(err)
            }
        };
        ic_cdk::println!("insurance_asset_fee {}", insurance_asset_fee);

        mutate_state(|s| {
            s.stable_state
                .insurance_sellers
                .iter()
                .filter(|(key, _)| key.insurance_id == insurance_id)
                .for_each(|(k, v)| {
                    let insurance_premium_balance =
                        match biguint_u128::biguint_to_u128_func(&insurance_premium_balance.to_owned().0) {
                            Ok(value) => value,
                            Err(error) => {
                                ic_cdk::println!("Error: {}", error);
                                0u128
                            }
                        } as f64;
                    ic_cdk::println!("insurance_premium_balance {}", insurance_premium_balance);

                    let insurance_pool_balance: f64 =
                        match biguint_u128::biguint_to_u128_func(&insurance_pool_balance.to_owned().0) {
                            Ok(value) => value,
                            Err(error) => {
                                ic_cdk::println!("Error: {}", error);
                                0u128
                            }
                        } as f64;
                    ic_cdk::println!("insurance_pool_balance {}", insurance_pool_balance);

                    let seller_share_amount: f64 = match biguint_u128::biguint_to_u128_func(&v.0.to_owned().0) {
                        Ok(value) => value,
                        Err(error) => {
                            ic_cdk::println!("Error: {}", error);
                            0u128
                        }
                    } as f64;
                    ic_cdk::println!("seller_share_amount {}", seller_share_amount);

                    let seller_pool_share: f64 = seller_share_amount / insurance_pool_balance;
                    ic_cdk::println!("seller_pool_share {}", seller_pool_share);

                    let total_amount: f64 = insurance_premium_balance * seller_pool_share;
                    ic_cdk::println!("total_amount {}", total_amount);

                    let payout_amount_f64: f64 = total_amount;

                    ic_cdk::println!("payout_amount_f64 {}", payout_amount_f64);

                    let payout_amount = match f64_to_biguint(payout_amount_f64) {
                        Some(res) => Nat::from(res),
                        None => {
                            ic_cdk::println!("Error in conversion of  Payout Amount {} ", payout_amount_f64);
                            Nat::from(0 as u64)
                        }
                    };
                    ic_cdk::println!("payout_amount {}", payout_amount);

                    let key: InsuranceSellersKey = k.to_owned();

                    // Transfer funds to buyers from insurance contract pool of a given insurance_id
                    let transfer_args: TransferArg = TransferArg {
                        from_subaccount: Some(convert_u32_to_subaccount(insurance_id + 1)),

                        to: Account {
                            owner: key.principal,
                            subaccount: None,
                        },

                        amount: payout_amount - insurance_asset_fee.to_owned(),

                        fee: Option::None,

                        memo: Option::None,

                        created_at_time: Option::None,
                    };

                    ic_cdk::spawn(async move {
                        let transfer_res: IcrcTransferResult = ledger.transfer(transfer_args.to_owned()).await;

                        match transfer_res {
                            IcrcTransferResult::TransferSuccess(_) => {
                                ic_cdk::println!("Sending Premium to Amount to {}", transfer_args.to.owner);

                                Self::add_contract_exection_logs(
                                    insurance_id,
                                    format!("Sending Premium to Amount to {}", transfer_args.to.owner),
                                );
                            }

                            IcrcTransferResult::TransferErrorMessage(_msg) => {
                                ic_cdk::println!(
                                    "Error Occured while transfer fund to Seller Principal {} with Transfer Error {}",
                                    transfer_args.to.owner,
                                    _msg.clone()
                                );

                                Self::add_contract_exection_logs(
                                    insurance_id,
                                    format!(
                                        "Error Occured while transfer fund to Seller Principal {} with Transfer Error {}",
                                        transfer_args.to.owner, _msg
                                    ),
                                );
                            }

                            IcrcTransferResult::TransferErrorString(_msg_str) => {
                                ic_cdk::println!(
                                    "Error Occured while transfer  fund to Selller Principal {} with Transfer Error {}",
                                    transfer_args.to.owner,
                                    _msg_str.clone()
                                );

                                // add log
                                Self::add_contract_exection_logs(
                                    insurance_id,
                                    format!(
                                        "Error Occured while transfer  fund to Selller Principal {} with Transfer Error {}",
                                        transfer_args.to.owner, _msg_str
                                    ),
                                );
                            }
                        }
                    })
                })
        });

        ic_cdk::println!("Total Sellers Iterations {}", i);

        ExecuteInsuranceContractRes::Success
    }

    /// Executes  the given insurance contract in favor of sellers
    pub async fn send_premium_amount_to_sellers(
        insurance_id: u32,
        insurance: Insurance,
        insurance_pool_balance: Nat,
        insurance_premium_balance: Nat,
    ) -> ExecuteInsuranceContractRes {
        let i: i32 = 0;

        let ledger: IcrcLedger = ic_ledger_utils::icrc::IcrcLedger::new(get_ledger_canister_id(insurance.insurance_asset));

        let insurance_asset_fee: Nat = match ledger.fee().await {
            ic_ledger_utils::types::icrc_types::IcrcFee::Fee(res) => res,
            ic_ledger_utils::types::icrc_types::IcrcFee::ErrorMessage(err) => {
                return ExecuteInsuranceContractRes::ErrorMessage(err)
            }
        };

        ic_cdk::println!("insurance_asset_fee {}", insurance_asset_fee);

        mutate_state(|s| {
            s.stable_state
                .insurance_sellers
                .iter()
                .filter(|(key, _)| key.insurance_id == insurance_id)
                .for_each(|(k, v)| {
                    let insurance_premium_balance =
                        match biguint_u128::biguint_to_u128_func(&insurance_premium_balance.to_owned().0) {
                            Ok(value) => value,
                            Err(error) => {
                                ic_cdk::println!("Error: {}", error);
                                0u128
                            }
                        } as f64;
                    ic_cdk::println!("insurance_premium_balance {}", insurance_premium_balance);

                    let insurance_pool_balance: f64 =
                        match biguint_u128::biguint_to_u128_func(&insurance_pool_balance.to_owned().0) {
                            Ok(value) => value,
                            Err(error) => {
                                ic_cdk::println!("Error: {}", error);
                                0u128
                            }
                        } as f64;
                    ic_cdk::println!("insurance_pool_balance {}", insurance_pool_balance);

                    let seller_share_amount: f64 = match biguint_u128::biguint_to_u128_func(&v.0.to_owned().0) {
                        Ok(value) => value,
                        Err(error) => {
                            ic_cdk::println!("Error: {}", error);
                            0u128
                        }
                    } as f64;
                    ic_cdk::println!("seller_share_amount {}", seller_share_amount);

                    let seller_pool_share: f64 = seller_share_amount / insurance_pool_balance;
                    ic_cdk::println!("seller_pool_share {}", seller_pool_share);

                    let total_amount: f64 = insurance_premium_balance * seller_pool_share;
                    ic_cdk::println!("total_amount {}", total_amount);

                    let payout_amount_f64: f64 = total_amount;

                    ic_cdk::println!("payout_amount_f64 {}", payout_amount_f64);

                    let payout_amount = match f64_to_biguint(payout_amount_f64) {
                        Some(res) => Nat::from(res),
                        None => {
                            ic_cdk::println!("Error in conversion of  Payout Amount {} ", payout_amount_f64);
                            Nat::from(0 as u64)
                        }
                    };
                    ic_cdk::println!("payout_amount {}", payout_amount);

                    let key: InsuranceSellersKey = k.to_owned();

                    // Transfer funds to buyers from insurance contract pool of a given insurance_id
                    let transfer_args: TransferArg = TransferArg {
                        from_subaccount: Some(convert_u32_to_subaccount(insurance_id + 1)),

                        to: Account {
                            owner: key.principal,
                            subaccount: None,
                        },

                        amount: payout_amount - insurance_asset_fee.to_owned(),

                        fee: Option::None,

                        memo: Option::None,

                        created_at_time: Option::None,
                    };

                    ic_cdk::spawn(async move {
                        let transfer_res: IcrcTransferResult = ledger.transfer(transfer_args.to_owned()).await;

                        match transfer_res {
                            IcrcTransferResult::TransferSuccess(_) => {
                                ic_cdk::println!("Sending Premium to Amount to {}", transfer_args.to.owner);

                                Self::remove_seller_from_insurance_sellers_list(&key);

                                ic_cdk::println!("Premium Tranfer Success {}", transfer_args.to.owner);

                                Self::add_insurance_details_to_user_history(key.insurance_id, key.principal);

                                Self::add_contract_exection_logs(
                                    insurance_id,
                                    format!("Sending Premium to Amount to {}", transfer_args.to.owner),
                                );
                            }

                            IcrcTransferResult::TransferErrorMessage(_msg) => {
                                ic_cdk::println!(
                                    "Error Occured while transfer fund to Seller Principal {} with Transfer Error {}",
                                    transfer_args.to.owner,
                                    _msg.clone()
                                );

                                Self::add_contract_exection_logs(
                                    insurance_id,
                                    format!(
                                        "Error Occured while transfer fund to Seller Principal {} with Transfer Error {}",
                                        transfer_args.to.owner, _msg
                                    ),
                                );
                            }

                            IcrcTransferResult::TransferErrorString(_msg_str) => {
                                ic_cdk::println!(
                                    "Error Occured while transfer  fund to Selller Principal {} with Transfer Error {}",
                                    transfer_args.to.owner,
                                    _msg_str.clone()
                                );

                                // add log
                                Self::add_contract_exection_logs(
                                    insurance_id,
                                    format!(
                                        "Error Occured while transfer  fund to Selller Principal {} with Transfer Error {}",
                                        transfer_args.to.owner, _msg_str
                                    ),
                                );
                            }
                        }
                    })
                })
        });

        ic_cdk::println!("Total Sellers Premiums Transaction Iterations {}", i);

        ExecuteInsuranceContractRes::Success
    }

    /// Execute the given insurance contract in favor of Buyers
    pub async fn execute_insurance_contract_for_buyers(insurance_id: u32, insurance: Insurance) -> ExecuteInsuranceContractRes {
        ic_cdk::println!("Execute Buyer Insurance Contract Start");

        let i = 0;

        let ledger: IcrcLedger = ic_ledger_utils::icrc::IcrcLedger::new(get_ledger_canister_id(insurance.insurance_asset));

        let insurance_asset_fee: Nat = match ledger.fee().await {
            ic_ledger_utils::types::icrc_types::IcrcFee::Fee(res) => res,
            ic_ledger_utils::types::icrc_types::IcrcFee::ErrorMessage(err) => {
                return ExecuteInsuranceContractRes::ErrorMessage(err)
            }
        };

        mutate_state(|s| {
            s.stable_state
                .insurance_buyers
                .iter()
                .filter(|(key, _)| key.insurance_id == insurance_id)
                .for_each(|(k, v)| {
                    let buyer_share_amount = v.0.to_owned();
                    ic_cdk::println!("buyer_share_amount {}", buyer_share_amount);

                    let multiplier: Nat = Self::get_insurance_by_id_multiplier_in_nat(insurance.multiplier.to_owned());
                    let payout_amount: Nat = (buyer_share_amount * multiplier) - insurance_asset_fee.to_owned();
                    ic_cdk::println!("payout_amount {}", payout_amount);

                    let key: InsuranceBuyersKey = k.to_owned();

                    // Transfer funds to buyers from insurance contract pool of a given insurance_id
                    let transfer_args: TransferArg = TransferArg {
                        from_subaccount: Some(convert_u32_to_subaccount(insurance_id)),

                        to: Account {
                            owner: key.principal,
                            subaccount: None,
                        },

                        amount: payout_amount,

                        fee: Option::None,

                        memo: Option::None,

                        created_at_time: Option::None,
                    };

                    ic_cdk::spawn(async move {
                        let transfer_res: IcrcTransferResult = ledger.transfer(transfer_args.to_owned()).await;

                        match transfer_res {
                            IcrcTransferResult::TransferSuccess(_) => {
                                Self::remove_buyer_from_insurance_buyers_list(&key);

                                // Add User Trade to History
                                Self::add_insurance_details_to_user_history(key.insurance_id, key.principal);

                                ic_cdk::println!(
                                    "Transfer Amount {} to Buyer Principal {} with transfer detail {:?}",
                                    transfer_args.to_owned().amount,
                                    transfer_args.to.owner,
                                    transfer_res
                                );

                                Self::add_contract_exection_logs(
                                    insurance_id,
                                    format!(
                                        "Transfer Amount to Buyer Principal {} with transfer detail {}",
                                        transfer_args.to.owner, transfer_args.to
                                    ),
                                );
                            }

                            IcrcTransferResult::TransferErrorMessage(_msg) => {
                                ic_cdk::println!(
                                    "Error Occured while transfer fund to Buyer Principal {} with Transfer Error {}",
                                    transfer_args.to.owner,
                                    _msg.clone()
                                );

                                // add log
                                Self::add_contract_exection_logs(
                                    insurance_id,
                                    format!(
                                        "Error Occured while transfer fund to Buyer Principal {} with Transfer Error {}",
                                        transfer_args.to.owner, _msg
                                    ),
                                );
                            }

                            IcrcTransferResult::TransferErrorString(_msg_str) => {
                                ic_cdk::println!(
                                    "Error Occured while transfer fund to Buyer Principal {} with Transfer Error {}",
                                    transfer_args.to.owner,
                                    _msg_str.clone()
                                );

                                // add log
                                Self::add_contract_exection_logs(
                                    insurance_id,
                                    format!(
                                        "Error Occured while transfer fund to Buyer Principal {} with Transfer Error {}",
                                        transfer_args.to.owner, _msg_str
                                    ),
                                );
                            }
                        }
                    })
                })
        });

        ic_cdk::println!("Total Buyers Iterations {}", i);

        ic_cdk::println!("Execute Buyer Insurance Contract End");

        ExecuteInsuranceContractRes::Success
    }

    fn add_insurance_details_to_user_history(insurance_id: u32, principal: Principal) {
        let key: UserInsuranceListHistoryKey = UserInsuranceListHistoryKey { principal, insurance_id };

        mutate_state(|s| s.stable_state.user_trade_history.insert(key, ic_cdk::api::time()));
    }

    /// Set a timer for a given insurance contract
    pub(crate) fn set_contract_timer(contract_expiry: u64, insurance_id: u32) -> Result<TimerId, String> {
        let current_time: u64 = ic_cdk::api::time();
        if contract_expiry - current_time < 0 as u64 {
            return Err("Time can not be negative".to_owned());
        }

        let timer_id = set_timer(Duration::from_nanos(contract_expiry - current_time), move || {
            ic_cdk::spawn(async move {
                let insurance_id_copy: u32 = insurance_id;

                let execute_insurance_contract_args: ExecuteInsuranceContractArgs = ExecuteInsuranceContractArgs {
                    insurance_id: insurance_id_copy,
                };

                Self::execute_insurance_contract(execute_insurance_contract_args).await;
            })
        });
        Ok(timer_id)
    }

    /// Calculate the premium amount for a buyer to purchase the contract
    pub async fn calculate_buy_insurance_contract_premium(insurance_id: u32, insurance: &Insurance) -> Nat {
        let ledger = ic_ledger_utils::icrc::IcrcLedger::new(get_ledger_canister_id(insurance.insurance_asset));

        // get the balance of a insurance contract liquiduty pool
        let insurance_pool_balance: Nat = ledger
            .balance(Account {
                owner: ic_cdk::api::id(),
                subaccount: Some(convert_u32_to_subaccount(insurance_id)),
            })
            .await;

        // get the balance of a insurance contract premium pool
        let premium_balance: Nat = ledger
            .balance(Account {
                owner: ic_cdk::api::id(),
                subaccount: Some(convert_u32_to_subaccount(insurance_id + 1)),
            })
            .await;

        let multplier: u64 = insurance.multiplier.clone() as u64;

        let max_allowed_premium: Nat = insurance_pool_balance / Nat::from(multplier);

        let max_current_premium: Nat = max_allowed_premium - premium_balance;

        max_current_premium
    }

    /// This function select the insurance category and returns the decision
    ///  either in favor of buyer or seller defined in the contract condition
    ///
    /// True -> Buyers are eligible to exercise the contract
    ///
    /// False -> Insurance Contract will expire worthless
    async fn select_category_with_contract_decision(self) -> Result<bool, String> {
        match self.category {
            InsuranceCategory::InflationBasedInsurance(inflation_category) => {
                let country: Country = inflation_category.country.clone();

                let inflation: InfationData = InfationData::new(country, Option::None);

                return inflation
                    .should_insurance_contract_exercise(inflation_category.inflation_target)
                    .await;
            }
        }
    }

    fn get_insurance_by_id_multiplier_in_nat(multiplier: InsuranceRewardsMultiplier) -> Nat {
        match multiplier {
            InsuranceRewardsMultiplier::M2X => Nat::from(2 as u64),
            InsuranceRewardsMultiplier::M3X => Nat::from(3 as u64),
            InsuranceRewardsMultiplier::M4X => Nat::from(4 as u64),
        }
    }

    pub async fn execute_insurance_contract_manual(insurance_id: u32) -> ExecuteInsuranceContractRes {
        //add logs
        Self::add_contract_exection_logs(insurance_id, format!("Contract Execution Start"));
        // check if insurance of a given id exist
        let insurance: Insurance = match Self::get_insurance_by_id(insurance_id) {
            Some(res) => res.to_owned(),
            None => {
                //add logs
                Self::add_contract_exection_logs(insurance_id, format!("Insurance Id Not Found while executing the contract"));

                return ExecuteInsuranceContractRes::ErrorMessage(format!("Insurance with Id {} not found", insurance_id));
            }
        };

        // At the time of invokation of a insurance contract the present time should equal
        // or greater than insurance expiry time otherwise the error message will generate
        if ic_cdk::api::time() < insurance.expiry_date.clone() {
            // this entry can not stored in logs stable memory
            return ExecuteInsuranceContractRes::ErrorMessage(format!(
                "Expiry time {} for a contract is not reached",
                insurance.expiry_date,
            ));
        }

        // get the balance of a insurance contract liquiduty pool
        let insurance_pool_balance: Nat =
            get_pool_balance_by_insurance_id(insurance_id, insurance.insurance_asset.clone()).await;
        ic_cdk::println!("insurance_pool_balance {}", insurance_pool_balance);

        // get the balance of a insurance contract premium pool
        let insurance_premium_balance: Nat =
            get_premium_pool_balance_by_insurance_id(insurance_id, insurance.insurance_asset.clone()).await;
        ic_cdk::println!("insurance_premium_balance {}", insurance_premium_balance);

        // check whether the buyers participated in insurance contract
        if insurance_premium_balance < insurance.min_premium_amount.clone() {
            ic_cdk::println!("Insurance Premium Pool Balance is less min_premium_amount");

            Self::repay_insurance_amount_to_sellers(insurance_id, insurance.insurance_asset.clone()).await;

            // Self::change_insurance_contract_status(&insurance, insurance_id, InsuranceContractStatus::CLOSED);

            // Self::remove_insuurance_contract_from_active_list(insurance_id);

            // Self::add_contract_exection_logs(insurance_id, "None of the buyers participated".to_owned());
            ic_cdk::print("None of the buyers participated".to_owned());

            return ExecuteInsuranceContractRes::ErrorMessage("None of the buyers participated".to_owned());
        }

        //add logs
        Self::add_contract_exection_logs(
            insurance_id,
            format!(
                "Insurance Premium Pool Balance {} is greater min_premium_amount {}",
                insurance_premium_balance, insurance.min_premium_amount
            ),
        );

        let contract_decision: bool = match Self::select_category_with_contract_decision(insurance.clone()).await {
            Ok(res) => res,
            Err(err) => return ExecuteInsuranceContractRes::ErrorMessage(err),
        };

        if contract_decision {
            //add logs
            Self::add_contract_exection_logs(insurance_id, format!("Buyers won the contract {}", contract_decision));
            Self::execute_insurance_contract_for_buyers(insurance_id, insurance.clone()).await;

            Self::repay_insurance_amount_to_sellers(insurance_id, insurance.insurance_asset.clone()).await;
        } else {
            //add logs
            Self::add_contract_exection_logs(insurance_id, format!("Sellers won the contract",));

            Self::execute_insurance_contract_for_sellers(
                insurance_id,
                insurance.clone(),
                insurance_pool_balance,
                insurance_premium_balance,
            )
            .await;

            Self::repay_insurance_amount_to_sellers(insurance_id, insurance.insurance_asset.clone()).await;
        }

        //add logs
        Self::add_contract_exection_logs(insurance_id, format!("Contract Execution End"));
        ExecuteInsuranceContractRes::Success
    }
}
