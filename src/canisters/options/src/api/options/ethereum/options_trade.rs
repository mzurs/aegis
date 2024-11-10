use candid::Nat;
use ic_utils::generate_random_number_u64;

use crate::api::interfaces::{
    options::{CreateOptionArgs, CreateOptionRes, Options},
    options_assets::OptionsAssetsEth,
    trade::TradeOptions,
};

impl TradeOptions<OptionsAssetsEth> for Options {
    type Args = CreateOptionArgs;
    // type PutArgs = Nat;
    // type CallArgs = Nat;
    type Res = CreateOptionRes;
    // type PutRes = Nat;
    // type CallRes = Nat;
    type ExecuteArgs = u64;
    type ExecuteRes = Nat;
    type TradeArgs = Nat;
    type TradeRes = Nat;

    async fn new(ledger: OptionsAssetsEth, args: Self::Args) -> Self::Res {
        //
        // generating random option id
        let id: u64 = match generate_random_number_u64().await {
            Ok(id) => id,
            Err(err) => return Err(err),
        };

        // timestamp at which the option created
        // let timestamp: u64 = ic_cdk::api::time();

        let _timer_id = Self::set_contract_execute_timer(ledger, args.contract_expiry, id);

        // let ledger_id: Principal = fetch_icrc_ledger_ids(ledger);

        // let icrc: IcrcLedger = IcrcLedger::new(ledger_id);

        // let transfer_args: TransferFromArgs = TransferFromArgs {
        //     spender_subaccount: Option::None,

        //     from: Account {
        //         owner: ic_cdk::caller(),
        //         subaccount: Option::None,
        //     },

        //     to: Account {
        //         owner: ic_cdk::id(),
        //         subaccount: Some(convert_u64_to_subaccount(id)),
        //     },

        //     amount: args.asset_amount.to_owned(),

        //     fee: Option::None,

        //     memo: Option::None,

        //     created_at_time: Option::None,
        // };

        // match icrc.transfer_from(transfer_args).await {
        //     IcrcTransferFromResult::TransferFromSuccess(_) => (),
        //     IcrcTransferFromResult::TransferFromErrorMessage(transfer_from_error) => {
        //         return Err(transfer_from_error.to_string())
        //     }
        //     IcrcTransferFromResult::TransferFromErrorString(err_str) => return Err(err_str),
        // }

        // // transfer and create options
        // // todo.....

        // // Store the option contract in stable memory
        // Self::create_options(
        //     id,
        //     ic_cdk::caller(),
        //     args.contract_state,
        //     args.asset.clone(),
        //     args.asset_amount,
        //     args.contract_expiry,
        //     args.options_type.clone(),
        //     timestamp,
        // );

        // // Add Option to Active List to display users to trade
        // Self::add_option_to_active_list(id, args.options_type.clone(), args.asset.clone().into(), timestamp);

        // // Check if the contract is PUT or CALL and insert the option data to memory respectively
        // match args.options_type {
        //     OptionsType::PUT => {
        //         // Add Option to Active List of Put Option Contract that are traded By Principal
        //         Self::add_option_to_put_active_list_by_principal(id, ic_cdk::caller());
        //     }
        //     OptionsType::CALL => {
        //         // Add Option to Active List of Call Option Contract that are traded By Principal
        //         Self::add_option_to_call_active_list_by_principal(id, ic_cdk::caller());
        //     }
        // }

        Ok(String::from(format!("Option is successfully created with Id {}", id)))
    }

    // async fn put(&self, _args: Self::PutArgs) -> Self::PutRes {
    //     todo!()
    // }

    // async fn call(&self, _args: Self::CallArgs) -> Self::CallRes {
    //     todo!()
    // }

    async fn trade(&self, _args: Self::TradeArgs) -> Self::TradeRes {
        todo!()
    }

    async fn execute(_ledger: OptionsAssetsEth, _args: Self::ExecuteArgs) -> Self::ExecuteRes {
        todo!()
    }

    async fn execute_offer(_ledger: OptionsAssetsEth, _args: Self::ExecuteArgs) -> Self::ExecuteRes {
        todo!()
    }
}
