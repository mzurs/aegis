use ic_cdk::update;

use crate::api::interfaces::{
    options::{CreateOptionArgs, CreateOptionRes, ExecuteOptionRes, Options, TradeOptionArgs, TradeOptionRes},
    options_assets::OptionsAssetsIcrc,
    trade::TradeOptions,
};

#[update]
async fn create_icrc_options(ledger: OptionsAssetsIcrc, args: CreateOptionArgs) -> CreateOptionRes {
    Options::new(ledger, args).await
}

#[update]
async fn trade_icrc_options(ledger: OptionsAssetsIcrc, id: u64) -> TradeOptionRes {
    Options::trade(
        ledger,
        TradeOptionArgs {
            id,
            use_exchange_account: false,
        },
    )
    .await
}

#[update]
async fn execute_manual(ledger: OptionsAssetsIcrc, id: u64) -> ExecuteOptionRes {
    Options::execute_manual(ledger, id).await
}
