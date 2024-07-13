// pub async fn _get_exchange_rate(asset: Assets) -> Result<ExchangeRate, String> {
//     let base_asset: String = _get_asset_str(asset);

//     let quote_asset = String::from("USDT");
//     let xrc_canister_id: Principal = get_xrc_canister_id();
//     let xrc_canister_cycles_cost = 1_000_000_000;

//     let xrc_args: GetExchangeRateRequest = GetExchangeRateRequest {
//         timestamp: Option::None,
//         quote_asset: Asset {
//             class: AssetClass::Cryptocurrency,
//             symbol: quote_asset,
//         },
//         base_asset: Asset {
//             class: AssetClass::Cryptocurrency,
//             symbol: base_asset,
//         },
//     };
//     let (res): (GetExchangeRateResult,) = call_with_payment(
//         xrc_canister_id,
//         "get_exchange_rate",
//         (xrc_args,),
//         xrc_canister_cycles_cost,
//     )
//     .await
//     .unwrap();

//     let rate = match res {
//         (GetExchangeRateResult::Ok(result),) => result,
//         (GetExchangeRateResult::Err(err),) => return Err(enum_exchange_rate_error(err)),
//     };
//     Ok(rate)
// }
