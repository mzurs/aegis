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
use management_canister::ManagementCanister;

use crate::api::{
    interfaces::{
        constants::CanisterName,
        exchange::Ticker,
        options::{
            CreateOptionArgs, CreateOptionRes, ExecuteOptionRes, Options, OptionsContractState, OptionsId, OptionsType,
            TradeOptionArgs, TradeOptionRes,
        },
        options_assets::{OptionsAssets, OptionsAssetsIcrc},
        premium::{EuropeanOptions, EuropeanOptionsCalculatePremiumArgs, Premium},
        trade::TradeOptions,
    },
    utils::{
        amount_conversion::{
            convert_asset_amount_to_human, convert_asset_amount_to_non_human, convert_premium_amount_to_non_humans,
            convert_xrc_non_human_to_human,
        },
        constants::{get_canister_id, get_icrc_ledger_canister_id},
        get_icrc_ledger_ids::fetch_icrc_ledger_ids,
    },
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

        // let ledger_id: Principal = fetch_icrc_ledger_ids(ledger);

        let icrc: IcrcLedger = IcrcLedger::new(match args.options_type.to_owned() {
            OptionsType::PUT => get_icrc_ledger_canister_id(OptionsAssetsIcrc::CKUSDT),
            OptionsType::CALL => fetch_icrc_ledger_ids(ledger),
        });

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

            amount: match args.options_type.to_owned() {
                OptionsType::PUT => {
                    let price_f64: f64 = convert_xrc_non_human_to_human(args.strike_price.to_owned());

                    let amount_f64: f64 = convert_asset_amount_to_human(args.asset.to_owned(), args.asset_amount.to_owned()); //biguint_to_u128_func(&args.asset_amount.to_owned().0).unwrap() as f64;

                    ic_cdk::println!("New Option Put price_f64  {}", price_f64.to_owned());

                    ic_cdk::println!("New Option Put Amount f64 {}", amount_f64.to_owned());

                    ic_cdk::println!("New Option Put Amount f64 x Price_f64 {}", amount_f64.to_owned() * price_f64);

                    let amount: Nat = convert_asset_amount_to_non_human(OptionsAssets::ETH, amount_f64 * price_f64);

                    ic_cdk::println!("New Option Put Amount {}", amount.to_owned());

                    amount
                }
                OptionsType::CALL => args.asset_amount.to_owned(),
            },

            fee: Option::None,

            memo: Option::None,

            created_at_time: Option::None,
        };

        match icrc.transfer_from(transfer_args).await {
            IcrcTransferFromResult::TransferFromSuccess(_) => (),
            IcrcTransferFromResult::TransferFromErrorMessage(transfer_from_error) => {
                clear_timer(execute_contract_timer_id);
                clear_timer(execute_offer_timer_id);

                return Err(format!("Transfer Error: {:?}", transfer_from_error));
            }
            IcrcTransferFromResult::TransferFromErrorString(err_str) => {
                clear_timer(execute_contract_timer_id);
                clear_timer(execute_offer_timer_id);

                return Err(err_str);
            }
        }
        ic_cdk::println!("add_options_contract_timestamp");
        // Add timestamp to stable memory with id
        Self::add_options_contract_timestamp(args.contract_expiry, id);

        ic_cdk::println!("create_options");
        // Store the option contract in stable memory
        Self::create_options(
            id,
            ic_cdk::caller(),
            OptionsContractState::OFFER,
            args.asset.clone(),
            args.asset_amount.to_owned(),
            args.contract_expiry,
            args.options_type.clone(),
            timestamp,
            args.strike_price.to_owned(),
            args.offer_duration,
        );

        ic_cdk::println!("add_option_to_active_list");
        // Add Option to Active List to display users to trade
        Self::add_option_to_active_list(
            id,
            args.options_type.clone(),
            args.asset.clone().into(),
            timestamp,
            args.offer_duration,
            args.strike_price,
            args.contract_expiry,
            args.asset_amount.to_owned(),
        );

        ic_cdk::println!("add_option_to_trade_history_by_principal");

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
                ic_cdk::println!("add_option_to_put_active_list_by_principal");

                // Add Option to Active List of Put Option Contract that are traded By Principal
                Self::add_option_to_put_active_list_by_principal(id, ic_cdk::caller());
            }
            OptionsType::CALL => {
                ic_cdk::println!("add_option_to_call_active_list_by_principal");

                // Add Option to Active List of Call Option Contract that are traded By Principal
                Self::add_option_to_call_active_list_by_principal(id, ic_cdk::caller());
            }
        }

        ic_cdk::println!("Option Created");

        Ok(String::from(format!("Option is successfully created with Id {}", id)))
    }

    async fn trade(_ledger: OptionsAssetsIcrc, TradeOptionArgs { id, .. }: Self::TradeArgs) -> Self::TradeRes {
        ic_cdk::println!("",);

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

        let premium_: f64 = EuropeanOptions::calculate_premium(EuropeanOptionsCalculatePremiumArgs {
            option_type: option.options_type.clone(),
            strike_price: convert_xrc_non_human_to_human(option.strike_price.clone()), //Nat::from(f64_to_biguint(convert_xrc_non_human_to_human(option.strike_price.clone())).unwrap()),
            contract_expiry: option.contract_expiry,
            asset: option.asset.clone(),
        })
        .await
        .unwrap();

        ic_cdk::println!("premium_f64 {}", premium_);

        let premium: Nat = convert_premium_amount_to_non_humans(option.asset.clone(), premium_);
        ic_cdk::println!("premium {}", premium);

        let asset_amount_f64: f64 = convert_asset_amount_to_human(option.asset.to_owned(), option.asset_amount.to_owned());
        ic_cdk::println!("asset_amount_f64 {}", asset_amount_f64);

        let amount_f64: f64 = asset_amount_f64 * premium_;
        ic_cdk::println!("amount_f64 {}", amount_f64);

        let amount: Nat = convert_asset_amount_to_non_human(OptionsAssets::ETH, amount_f64);
        ic_cdk::println!("amount {}", amount);

        match Self::transfer_premium_to_seller(option.seller.to_owned(), amount).await {
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
        ic_cdk::println!("Option Execution with Id {}  starts", id);
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
                ic_cdk::println!("No buyer purchase the option contract");

                match Self::transfer_from_contract(
                    match option.options_type {
                        OptionsType::PUT => OptionsAssetsIcrc::CKUSDT,
                        OptionsType::CALL => ledger,
                    },
                    id,
                    option.seller,
                    None,
                )
                .await
                {
                    Err(err) => return Err(err),
                    _ => {
                        Self::remove_option_from_active_list(id, option.to_owned());

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

        ic_cdk::println!("Buyer Purchased the Option Contract");

        let (ledger_id, if_option_exercise_in_favor_of_buyer, amount): (OptionsAssetsIcrc, bool, Nat) = {
            let mgmt: ManagementCanister = ManagementCanister::new();

            let current_price: Nat = match mgmt
                .xrc(
                    Into::<Ticker>::into(option.asset.to_owned()).0,
                    get_canister_id(CanisterName::ExchangeRate),
                )
                .await
            {
                Ok(res) => Nat::from(res),
                Err(err) => return Err(err),
            };

            ic_cdk::println!(
                "Current Price {} \nStrike Price {}\n",
                current_price,
                option.strike_price.to_owned()
            );

            match option.options_type.to_owned() {
                OptionsType::PUT => {
                    if (current_price) < option.strike_price.to_owned() {
                        let price_difference: f64 =
                            convert_xrc_non_human_to_human(option.strike_price.to_owned() - current_price.to_owned())
                                / convert_xrc_non_human_to_human(option.strike_price.to_owned());
                        ic_cdk::println!("price_difference {}", price_difference);

                        let amount: f64 =
                            convert_asset_amount_to_human(option.asset.to_owned(), option.asset_amount.to_owned())
                                * price_difference;

                        ic_cdk::println!("amount {}", amount);

                        let amount_nat: Nat = convert_asset_amount_to_non_human(OptionsAssets::ETH, amount);

                        ic_cdk::println!("amount_nat {}", amount_nat);

                        ((OptionsAssetsIcrc::CKUSDT), true, amount_nat)
                    } else {
                        ((OptionsAssetsIcrc::CKUSDT), false, Nat::from(0 as u32))
                    }
                }
                OptionsType::CALL => {
                    if current_price > option.strike_price.to_owned() {
                        let price_difference =
                            convert_xrc_non_human_to_human(current_price.to_owned() - option.strike_price.to_owned())
                                / convert_xrc_non_human_to_human(current_price.to_owned());
                        ic_cdk::println!("price_difference {}", price_difference);

                        let amount: f64 =
                            convert_asset_amount_to_human(option.asset.to_owned(), option.asset_amount.to_owned())
                                * price_difference;
                        ic_cdk::println!("amount {}", amount);
                        let amount_nat = convert_asset_amount_to_non_human(option.asset.to_owned(), amount);

                        ic_cdk::println!("amount_nat {}", amount_nat);

                        (ledger.clone(), true, amount_nat)
                    } else {
                        (ledger.clone(), false, Nat::from(0 as u32))
                    }
                }
            }
        };

        if if_option_exercise_in_favor_of_buyer {
            ic_cdk::println!("Buyer won the option contract of Amount {}", amount);
            match Self::transfer_from_contract(ledger_id.to_owned(), id, buyer.to_owned(), Some(amount.to_owned())).await {
                Err(err) => return Err(err),
                _ => {
                    Self::remove_option_from_active_list(id, option.to_owned());

                    // changing the option contract state
                    option.contract_state = OptionsContractState::EXECUTED;

                    // commiting the state changes

                    Self::update_options(id, option.to_owned());

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
        } else {
            ic_cdk::println!("Seller won the option contract");
        }

        match Self::transfer_from_contract(ledger_id, id, option.seller.to_owned(), None).await {
            Err(err) => return Err(err),
            _ => {
                Self::remove_option_from_active_list(id, option.to_owned());

                // changing the option contract state
                option.contract_state = OptionsContractState::EXECUTED;

                // commiting the state changes

                Self::update_options(id, option.to_owned());

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

                match option.buyer.to_owned() {
                    None => (),
                    Some(buyer) => {
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
        }

        Self::remove_execute_contract_timestamps(id);

        ic_cdk::println!("Option Execution with Id {}  ended", id);

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
        // Remove Option from Active List
        //
        Self::remove_option_from_active_list(id, option.to_owned());

        //
        // check if buyer participated in an option contract
        //
        match option.buyer {
            Some(_) => {
                // changing the option contract state
                option.contract_state = OptionsContractState::OPEN;

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
    /// Execute the Offer State for an Option Contract
    ///
    pub(crate) async fn execute_manual(ledger: OptionsAssetsIcrc, id: u64) -> ExecuteOptionRes {
        let option: Options = match Self::get_options_by_id(id) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        match option.contract_state {
            OptionsContractState::OPEN => (),
            OptionsContractState::OFFER => {
                if ic_cdk::caller().to_text() != option.seller.to_text() {
                    return Err(format!(
                        "Option Cannot Execute from Offer Phase because the caller is not the owner(seller)"
                    ));
                }
            }
            _ => return Err(format!("The option state is not OPEN")),
        }

        if option.contract_expiry >= ic_cdk::api::time() {
            return Err(format!("Option Contract Expiry Didn't reached"));
        }

        Options::execute(ledger, id).await
    }

    ///
    /// Transfer the ICRC tokens from an option contract to provided principal
    ///
    async fn transfer_from_contract(
        ledger: OptionsAssetsIcrc,
        id: OptionsId,
        owner: Principal,
        amount: Option<Nat>,
    ) -> ExecuteOptionRes {
        let ledger_id: Principal = fetch_icrc_ledger_ids(ledger.to_owned());

        ic_cdk::println!("ledger_id {}", ledger_id.to_owned());
        ic_cdk::println!("Ledger {:?}", ledger.to_owned());
        ic_cdk::println!(
            "Amount {}",
            match amount.to_owned() {
                Some(res) => res,
                None => Nat::from(0 as u64),
            }
        );

        let icrc: IcrcLedger = IcrcLedger::new(ledger_id);

        // fetching fee for an icrc ledger
        let fee: Nat = match icrc.fee().await {
            IcrcFee::Fee(fees) => fees,
            IcrcFee::ErrorMessage(err) => return Err(err),
        };
        ic_cdk::println!("fee {}", fee);

        // fetching balance for a given option contract
        let balance: Nat = icrc
            .balance({
                Account {
                    owner: ic_cdk::api::id(),
                    subaccount: Some(convert_u64_to_subaccount(id)),
                }
            })
            .await;

        ic_cdk::println!("option contract balance {}", balance);

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
                    Some(amount) => {
                        if amount >= fee {
                            amount - fee
                        } else {
                            amount
                        }
                    }
                    None => balance - fee,
                },

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
                return Err(format!("Transfer Error: {:?}", transfer_from_error));
            }
            IcrcTransferFromResult::TransferFromErrorString(err_str) => {
                return Err(err_str);
            }
        }
    }
}
