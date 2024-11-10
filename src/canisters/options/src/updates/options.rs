use ic_cdk::update;

use crate::api::interfaces::{
    options::{CreateOptionArgs, CreateOptionRes, Options},
    options_assets::OptionsAssetsIcrc,
    trade::TradeOptions,
};

#[update]
async fn create_icrc_options(ledger: OptionsAssetsIcrc, args: CreateOptionArgs) -> CreateOptionRes {
    Options::new(ledger, args).await
}
