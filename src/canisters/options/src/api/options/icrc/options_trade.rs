use candid::{Nat, Principal};
use ic_cdk_timers::{clear_timer, TimerId};
use ic_ledger_utils::{
    icrc::IcrcLedger,
    types::icrc_types::{IcrcFee, IcrcTransferFromResult, IcrcTransferResult},
};
use ic_utils::{convert_u64_to_subaccount, generate_random_number_u64};
use icrc_ledger_types::{
    icrc1::{account::Account, transfer::TransferArg},
    icrc2::transfer_from::TransferFromArgs,
};

use crate::api::{
    interfaces::{
        options::{
            CreateOptionArgs, CreateOptionRes, ExecuteOptionRes, Options, OptionsContractState, OptionsId, OptionsType,
            TradeOptionArgs, TradeOptionRes,
        },
        options_assets::OptionsAssetsIcrc,
        trade::TradeOptions,
    },
    utils::get_icrc_ledger_ids::fetch_icrc_ledger_ids,
};

impl TradeOptions<OptionsAssetsIcrc> for Options {
    type Args = CreateOptionArgs;
    type Res = CreateOptionRes;
    type TradeArgs = TradeOptionArgs;
    type TradeRes = TradeOptionRes;
    type ExecuteArgs = u64;
    type ExecuteRes = ExecuteOptionRes;

    async fn new(ledger: OptionsAssetsIcrc, args: Self::Args) -> Self::Res {
        //
        // generating random option id
        let id: u64 = match generate_random_number_u64().await {
            Ok(id) => id,
            Err(err) => return Err(err),
        };

        // to check contract expiry should be greater than offer duration
        if args.offer_duration > args.contract_expiry {
            return Err("Offer Duration should be within Contract Expiry".to_owned());
        }

        // timestamp at which the option created
        let timestamp: u64 = ic_cdk::api::time();

        let execute_contract_timer_id: TimerId =
            match Self::set_contract_execute_timer(ledger.clone(), args.contract_expiry, id) {
                Ok(res) => res,
                Err(err) => return Err(err),
            };

        let execute_offer_timer_id: TimerId =
            match Self::set_contract_offer_duration_timer(ledger.clone(), args.offer_duration, id) {
                Ok(res) => res,
                Err(err) => return Err(err),
            };

        let ledger_id: Principal = fetch_icrc_ledger_ids(ledger);

        let icrc: IcrcLedger = IcrcLedger::new(ledger_id);

        let transfer_args: TransferFromArgs = TransferFromArgs {
            spender_subaccount: Option::None,

            from: Account {
                owner: ic_cdk::caller(),
                subaccount: Option::None,
            },

            to: Account {
                owner: ic_cdk::id(),
                subaccount: Some(convert_u64_to_subaccount(id)),
            },

            amount: args.asset_amount.to_owned(),

            fee: Option::None,

            memo: Option::None,

            created_at_time: Option::None,
        };

        match icrc.transfer_from(transfer_args).await {
            IcrcTransferFromResult::TransferFromSuccess(_) => (),
            IcrcTransferFromResult::TransferFromErrorMessage(transfer_from_error) => {
                clear_timer(execute_contract_timer_id);
                clear_timer(execute_offer_timer_id);

                return Err(transfer_from_error.to_string());
            }
            IcrcTransferFromResult::TransferFromErrorString(err_str) => {
                clear_timer(execute_contract_timer_id);
                clear_timer(execute_offer_timer_id);

                return Err(err_str);
            }
        }

        // Add timestamp to stable memory with id
        Self::add_options_contract_timestamp(args.contract_expiry, id);

        // Store the option contract in stable memory
        Self::create_options(
            id,
            ic_cdk::caller(),
            args.contract_state,
            args.asset.clone(),
            args.asset_amount.to_owned(),
            args.contract_expiry,
            args.options_type.clone(),
            timestamp,
            args.offer_duration,
        );

        // Add Option to Active List to display users to trade
        Self::add_option_to_active_list(
            id,
            args.options_type.clone(),
            args.asset.clone().into(),
            timestamp,
            args.offer_duration,
        );

        // Add option to user trade history
        Self::add_option_to_trade_history_by_principal(
            ic_cdk::caller(),
            OptionsContractState::OFFER.into(),
            timestamp,
            id,
            Self::create_option_name(
                args.asset,
                args.asset_amount.to_owned(),
                args.contract_expiry,
                args.options_type,
            ),
            args.options_type.to_owned().into(),
            timestamp,
        );

        // Check if the contract is PUT or CALL and insert the option data to memory respectively
        match args.options_type {
            OptionsType::PUT => {
                // Add Option to Active List of Put Option Contract that are traded By Principal
                Self::add_option_to_put_active_list_by_principal(id, ic_cdk::caller());
            }
            OptionsType::CALL => {
                // Add Option to Active List of Call Option Contract that are traded By Principal
                Self::add_option_to_call_active_list_by_principal(id, ic_cdk::caller());
            }
        }

        Ok(String::from(format!("Option is successfully created with Id {}", id)))
    }

    async fn trade(&self, TradeOptionArgs { id, .. }: Self::TradeArgs) -> Self::TradeRes {
        if !Self::if_option_contract_is_active(id) {
            return Err("Contract State is not in Offer phase".to_owned());
        }

        let mut option: Options = match Self::get_options_by_id(id) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        // check to protect seller from buying its own option
        if option.seller.to_text() == ic_cdk::caller().to_text() {
            return Err(format!("Seller Cannot be a Buyer"));
        }

        let premium: Nat = Nat::from(0 as u32);

        match Self::transfer_premium_to_seller(option.seller.to_owned(), premium).await {
            Ok(_) => {
                let trade_timestamp: u64 = ic_cdk::api::time();

                option.contract_state = OptionsContractState::OPEN;

                option.buyer = Some(ic_cdk::caller());

                Self::update_options(id, option.to_owned());

                Self::remove_option_from_active_list(id, option.to_owned());

                // update seller history
                Self::update_option_trade_history_by_principal(
                    option.seller.to_owned(),
                    OptionsContractState::OFFER.into(),
                    OptionsContractState::OPEN.into(),
                    option.timestamp,
                    id,
                    option.options_type.into(),
                    option.name.to_owned(),
                    option.timestamp,
                );

                // add option to buyer history
                Self::add_option_to_trade_history_by_principal(
                    ic_cdk::caller(),
                    OptionsContractState::OPEN.into(),
                    option.timestamp,
                    id,
                    option.name.to_owned(),
                    option.options_type.to_owned().into(),
                    trade_timestamp,
                );

                Ok(format!("{} Option Purchased!", {
                    Into::<String>::into(option.options_type.to_owned())
                }))
            }
            Err(err) => return Err(err),
        }
    }

    async fn execute(ledger: OptionsAssetsIcrc, id: Self::ExecuteArgs) -> Self::ExecuteRes {
        // let trade_timestamp = ic_cdk::api::time();

        let mut option: Options = match Self::get_options_by_id(id) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let contract_state: OptionsContractState = option.contract_state.clone();

        // check the contract state whether it is OPEN state or not
        match option.contract_state.clone() {
            OptionsContractState::OPEN => (),

            OptionsContractState::OFFER => (),

            _ => {
                return Err(format!(
                    "The state of Option Already Changed and the current state is {}",
                    Into::<String>::into(option.contract_state)
                ));
            }
        }

        // Check if buyer purchase the option contract
        let buyer: Principal = match option.buyer.clone() {
            Some(buyer) => buyer,
            None => {
                match Self::transfer_from_contract(ledger, id, option.seller, None).await {
                    Err(err) => return Err(err),
                    _ => {
                        option.contract_state = OptionsContractState::CLOSED;

                        Self::update_options(id, option.to_owned());
                        Self::remove_contract_offer_timestamps(id);

                        Self::update_option_trade_history_by_principal(
                            option.seller.to_owned(),
                            contract_state.to_owned().into(),
                            option.contract_state.to_owned().into(),
                            option.timestamp,
                            id,
                            option.options_type.to_owned().into(),
                            option.name.to_owned(),
                            option.timestamp,
                        );
                    }
                }
                return Ok(());
            }
        };

        // changing the option contract state
        option.contract_state = OptionsContractState::EXECUTED;

        // commiting the state changes

        Self::update_options(id, option.to_owned());

        let transfer_amount = Nat::from(0 as u32);

        let if_option_exercise_in_favor_of_buyer: bool = true;

        if if_option_exercise_in_favor_of_buyer {
            match Self::transfer_from_contract(ledger.clone(), id, buyer, Some(transfer_amount.to_owned())).await {
                Err(err) => return Err(err),
                _ => {
                    Self::update_option_trade_history_by_principal(
                        buyer,
                        contract_state.to_owned().into(),
                        option.contract_state.to_owned().into(),
                        option.timestamp,
                        id,
                        option.options_type.to_owned().into(),
                        option.name.to_owned(),
                        option.timestamp,
                    );
                }
            }
        }

        match Self::transfer_from_contract(ledger, id, option.seller.to_owned(), None).await {
            Err(err) => return Err(err),
            _ => {
                Self::update_option_trade_history_by_principal(
                    option.seller.to_owned(),
                    contract_state.to_owned().into(),
                    option.contract_state.to_owned().into(),
                    option.timestamp,
                    id,
                    option.options_type.to_owned().into(),
                    option.name.to_owned(),
                    option.timestamp,
                );
            }
        }

        Self::remove_execute_contract_timestamps(id);

        Ok(())
    }

    ///
    /// Execute the Offer State for an Option Contract
    ///
    async fn execute_offer(ledger: OptionsAssetsIcrc, id: Self::ExecuteArgs) -> Self::ExecuteRes {
        // timestamp the state change
        let trade_timestamp: u64 = ic_cdk::api::time();

        let mut option: Options = match Self::get_options_by_id(id) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        //
        // check if buyer participated in an option contract
        //
        match option.buyer {
            Some(_) => {
                Self::remove_contract_offer_timestamps(id);
            }
            None => {
                match Self::transfer_from_contract(ledger.clone(), id, option.seller, None).await {
                    Ok(_) => {
                        // changing the option contract state
                        option.contract_state = OptionsContractState::CLOSED;

                        // commiting the state changes

                        Self::update_options(id, option.to_owned());

                        Self::update_option_trade_history_by_principal(
                            option.seller,
                            OptionsContractState::OFFER.into(),
                            option.contract_state.to_owned().into(),
                            option.timestamp,
                            id,
                            option.options_type.to_owned().into(),
                            option.name.to_owned(),
                            trade_timestamp,
                        );

                        Self::remove_contract_offer_timestamps(id);
                        Self::remove_execute_contract_timestamps(id);
                    }
                    Err(err) => return Err(err),
                }
            }
        }
        Ok(())
    }
}

impl Options {
    ///
    /// Transfer the ICRC tokens from an option contract to provided principal
    ///
    async fn transfer_from_contract(
        ledger: OptionsAssetsIcrc,
        id: OptionsId,
        owner: Principal,
        amount: Option<Nat>,
    ) -> ExecuteOptionRes {
        let ledger_id: Principal = fetch_icrc_ledger_ids(ledger);

        let icrc: IcrcLedger = IcrcLedger::new(ledger_id);

        // fetching fee for an icrc ledger
        let fee: Nat = match icrc.fee().await {
            IcrcFee::Fee(fees) => fees,
            IcrcFee::ErrorMessage(err) => return Err(err),
        };

        // fetching balance for a given option contract
        let balance: Nat = icrc
            .balance({
                Account {
                    owner: ic_cdk::caller(),
                    subaccount: Some(convert_u64_to_subaccount(id)),
                }
            })
            .await;

        if balance.clone()
            - match amount.clone() {
                Some(res) => res,
                None => Nat::from(0 as u32),
            }
            - fee.clone()
            <= Nat::from(0 as u32)
        {
            return Err(format!("No minimum balance in an option contract with Id {}", id));
        }

        match icrc
            .transfer(TransferArg {
                from_subaccount: Some(convert_u64_to_subaccount(id)),
                to: Account {
                    owner,
                    subaccount: Option::None,
                },

                amount: match amount {
                    Some(amount) => amount,
                    None => balance,
                } - fee,

                fee: Option::None,

                memo: Option::None,

                created_at_time: Option::None,
            })
            .await
        {
            IcrcTransferResult::TransferSuccess(_) => {}
            IcrcTransferResult::TransferErrorMessage(transfer_from_error) => {
                return Err(transfer_from_error.to_string());
            }
            IcrcTransferResult::TransferErrorString(err_str) => {
                return Err(err_str);
            }
        }

        Ok(())
    }

    async fn transfer_premium_to_seller(seller: Principal, amount: Nat) -> TradeOptionRes {
        let ledger_id: Principal = fetch_icrc_ledger_ids(OptionsAssetsIcrc::CKUSDT);

        let icrc: IcrcLedger = IcrcLedger::new(ledger_id);

        match icrc
            .transfer_from(TransferFromArgs {
                spender_subaccount: None,
                from: Account {
                    owner: ic_cdk::caller(),
                    subaccount: None,
                },
                to: Account {
                    owner: seller.to_owned(),
                    subaccount: None,
                },
                amount,
                fee: None,
                memo: None,
                created_at_time: None,
            })
            .await
        {
            IcrcTransferFromResult::TransferFromSuccess(_) => return Ok(format!("Premium Amount Sent to Seller {}", seller)),
            IcrcTransferFromResult::TransferFromErrorMessage(transfer_from_error) => {
                return Err(transfer_from_error.to_string());
            }
            IcrcTransferFromResult::TransferFromErrorString(err_str) => {
                return Err(err_str);
            }
        }
    }
}
