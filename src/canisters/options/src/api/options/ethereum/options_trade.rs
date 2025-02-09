use candid::Nat;
use ic_utils::generate_random_number_u64;

use crate::api::interfaces::{
    options::{CreateOptionArgs, CreateOptionRes, Options},
    options_assets::OptionsAssetsEth,
    trade::TradeOptions,
};

impl TradeOptions<OptionsAssetsEth> for Options {
    type Args = CreateOptionArgs;
    type Res = CreateOptionRes;
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

        let _timer_id = Self::set_contract_execute_timer(ledger, args.contract_expiry, id);

        Ok(String::from(format!("Option is successfully created with Id {}", id)))
    }

    async fn trade(_ledger: OptionsAssetsEth, _args: Self::TradeArgs) -> Self::TradeRes {
        todo!()
    }

    async fn execute(_ledger: OptionsAssetsEth, _args: Self::ExecuteArgs) -> Self::ExecuteRes {
        todo!()
    }

    async fn execute_offer(_ledger: OptionsAssetsEth, _args: Self::ExecuteArgs) -> Self::ExecuteRes {
        todo!()
    }
}
