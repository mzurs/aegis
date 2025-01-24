use blackscholes::{Inputs, OptionType, Pricing};
// use blackscholes::{Inputs, OptionType, Pricing};
use candid::Nat;
use ic_cdk::{
    api::management_canister::http_request::{
        http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs, TransformContext,
    },
    query,
};
use ic_utils::{generate_random_number_u64, time::remaining_time_in_years};
use management_canister::ManagementCanister;

use crate::api::{
    interfaces::{
        constants::CanisterName,
        exchange::Ticker,
        options::OptionsType,
        premium::{
            CalculatePremiumUsingBlackScholesArgs, DerbitVolatilityIndex, EuropeanOptions, EuropeanOptionsCalculatePremiumArgs,
            EuropeanOptionsCalculatePremiumRes, Premium,
        },
    },
    utils::{amount_conversion::convert_xrc_non_human_to_human, constants::get_canister_id},
};

use super::encode_url;

// use super::model::black_scholes_merton::black_scholes_merton;

impl Premium<EuropeanOptionsCalculatePremiumArgs, EuropeanOptionsCalculatePremiumRes> for EuropeanOptions {
    ///
    /// Calculate the option premium of assest with respect to option European Style Option Contract
    ///
    async fn calculate_premium(
        EuropeanOptionsCalculatePremiumArgs {
            option_type,
            strike_price,
            contract_expiry,
            asset,
        }: EuropeanOptionsCalculatePremiumArgs,
    ) -> EuropeanOptionsCalculatePremiumRes {
        let implied_volatility = match Self::fetch_options_datapoints(Into::<Ticker>::into(asset.to_owned()), None, None).await
        {
            Ok(res) => (res / (100.000 as f32)) as f64,
            Err(err) => return Err(err),
        };

        let time_to_maturity_in_years = remaining_time_in_years(ic_cdk::api::time(), contract_expiry);

        let risk_free_interest_rate = 0.0;
        let strike_price: f32 = strike_price as f32;
        // match biguint_u128::biguint_to_u128_func(&strike_price.0.to_owned()) {
        //     Ok(value) => value,
        //     Err(error) => return Err(error.to_owned()),
        // } as f64 as f32;

        // let option_type: OptionType = match option_type {
        //     crate::api::interfaces::options::OptionsType::PUT => OptionType::Put,
        //     crate::api::interfaces::options::OptionsType::CALL => OptionType::Call,
        // };

        let mgmt: ManagementCanister = ManagementCanister::new();

        let current_price: u64 = match mgmt
            .xrc(
                Into::<Ticker>::into(asset.to_owned()).0,
                get_canister_id(CanisterName::ExchangeRate),
            )
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        ic_cdk::println!("{}", current_price);

        let mut res: f64 = Self::calculate_premium_using_black_scholes_model(CalculatePremiumUsingBlackScholesArgs {
            option_type,
            current_price: convert_xrc_non_human_to_human(Nat::from(current_price)) as f32,
            // biguint_to_u128_func(&convert_xrc_non_human_to_human(asset.to_owned(), current_price.into()).0)
            //     .unwrap() as u64 as f64 as f32,
            strike_price,
            risk_free_interest_rate,
            implied_volatility,
            time_to_maturity_in_years,
        });

        ic_cdk::println!("Premium without Condition {}", res);

        if res < 1.0 {
            ic_cdk::println!("Options Premium is too low!");

            res = convert_xrc_non_human_to_human(Nat::from(current_price)) * 0.001
        } else if res > convert_xrc_non_human_to_human(Nat::from(current_price)) {
            ic_cdk::println!("Options Premium is too high!");

            res = res * 0.010 as f64
        } else {
            ic_cdk::println!("Options Premium is neither too high nor too low!");
        }

        Ok(res)
    }
}

impl EuropeanOptions {
    fn get_start_end_timestamps() -> (u64, u64) {
        let milli_second: u64 = 1_000_000;
        let seven_days_nanos: u64 = 7 * 24 * 60 * 60 * 1_000_000_000;

        let mut end_timestamp: u64 = ic_cdk::api::time();

        // Subtract 7 days from the current time
        let seven_days_ago_nanos = end_timestamp.saturating_sub(seven_days_nanos);

        // Convert the result to milliseconds
        let start_timestamp = seven_days_ago_nanos / milli_second;
        end_timestamp = end_timestamp / milli_second;

        (start_timestamp, end_timestamp)
    }

    async fn fetch_options_datapoints_exchange_derbit(ticker: Ticker) -> Result<f32, String> {
        let currency = ticker.0;

        let (start_timestamp, end_timestamp) = Self::get_start_end_timestamps();

        let test_host_url: String = "https://test.deribit.com/api/v2/public/".to_owned();

        let method: String = "get_volatility_index_data".to_owned();

        let resolution: String = "1".to_owned();

        let query_params: String = format!(
            "{}{}?currency={}&end_timestamp={}&resolution={}&start_timestamp={}",
            test_host_url, method, currency, end_timestamp, resolution, start_timestamp
        );

        let request_headers: Vec<ic_cdk::api::management_canister::http_request::HttpHeader> = vec![
            HttpHeader {
                name: "Idempotency-Key".to_string(),
                value: format!(
                    "{}{}",
                    "UUID-".to_string(),
                    match generate_random_number_u64().await {
                        Ok(res) => res.to_string(),
                        Err(err) => return Err(err.to_owned()),
                    }
                ),
            },
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
        ];
        let base_url = "https://74dvalxfnhjxsuoxnvbqavqm7y0pcxah.lambda-url.us-east-2.on.aws/";

        let request: CanisterHttpRequestArgument = CanisterHttpRequestArgument {
            url: match encode_url(base_url, &query_params) {
                Ok(res) => res,
                Err(err) => return Err(err.to_string()),
            },
            // url: url.to_string(),
            method: HttpMethod::GET,
            body: None,               //optional for request
            max_response_bytes: None, //optional for request
            transform: Some(TransformContext::from_name("transform_fred".to_string(), vec![])),
            headers: request_headers,
        };

        match http_request(request, 50_000_000_000).await {
            Ok((response,)) => {
                let str_body: String = String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.");

                let json_body: DerbitVolatilityIndex = match serde_json::from_str(&str_body) {
                    Ok(res) => res,
                    Err(err) => return Err(err.to_string()),
                };

                let vol: f64 = json_body.result.data[0].1;

                Ok(vol as f32)
            }

            Err((r, m)) => {
                let message: String = format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

                Err(message)
            }
        }
    }

    async fn fetch_options_datapoints(
        ticker: Ticker,
        _exchange: Option<String>,
        _method: Option<String>,
    ) -> Result<f32, String> {
        Self::fetch_options_datapoints_exchange_derbit(ticker).await
    }

    fn calculate_premium_using_black_scholes_model(
        CalculatePremiumUsingBlackScholesArgs {
            option_type,
            current_price,
            strike_price,
            risk_free_interest_rate,
            time_to_maturity_in_years,
            implied_volatility,
        }: CalculatePremiumUsingBlackScholesArgs,
    ) -> f64 {
        // let inputs: Inputs = Inputs::new(
        //     option_type,
        //     current_price,
        //     strike_price,
        //     None,
        //     risk_free_interest_rate,
        //     0.0,
        //     time_to_maturity_in_years,
        //     Some(implied_volatility as f32),
        // );

        ic_cdk::println!(
            "current_price: {} \n strike_price: {} \n risk_free_interest_rate: {} \n implied_volatility: {} \n time_to_maturity_in_years: {} \n",
            current_price,strike_price,risk_free_interest_rate,implied_volatility,time_to_maturity_in_years
        );
        let inputs_call_otm: Inputs = Inputs {
            option_type: match option_type {
                OptionsType::CALL => OptionType::Call,
                OptionsType::PUT => OptionType::Put,
            },
            s: current_price as f64,
            k: strike_price as f64,
            p: None,
            r: risk_free_interest_rate as f64,
            q: 0.00,
            t: time_to_maturity_in_years as f64, //20.0 / 365.25,
            sigma: Some(implied_volatility),
        };

        inputs_call_otm.calc_price().unwrap()

        // black_scholes_merton(
        //     option_type,
        //     current_price as f64,
        //     strike_price as f64,
        //     risk_free_interest_rate as f64,
        //     implied_volatility as f64,
        //     time_to_maturity_in_years as f64,
        // )
    }
}

// let seller_share_amount: f64 = match biguint_u128::biguint_to_u128_func(&v.0.to_owned().0) {
//     Ok(value) => value,
//     Err(error) => {
//         ic_cdk::println!("Error: {}", error);
//         0u128
//     }
// } as f64;

// Strips all data that is not needed from the original response.
// #[query]
// pub fn transform_fred(raw: TransformArgs) -> HttpResponse {
//     let headers = vec![];

//     let mut res: HttpResponse = HttpResponse {
//         status: raw.response.status.clone(),
//         body: raw.response.body.clone(),
//         headers,
//         ..Default::default()
//     };

//     let success_status: Nat = Nat::try_from(200 as u64).unwrap();

//     if res.status == success_status {
//         let str_body: String = String::from_utf8(res.body).expect("Transformed response is not UTF-8 encoded.");

//         let json_body: DerbitVolatilityIndex = match serde_json::from_str(&str_body) {
//             Ok(res) => res,
//             Err(err) => ic_cdk::trap(&err.to_string()),
//         };
//         res.body = match serde_json::to_vec(serde_json::to_string(&json_body.result.data[0])) {
//             Ok(vec) => vec,
//             Err(err) => ic_cdk::trap(&err.to_string()),
//         };

//         res.body = raw.response.body;
//     } else {
//         ic_cdk::api::print(format!("Received an error: err = {:?}", raw));
//     }
//     res
// }

#[query]
pub fn transform_fred(raw: TransformArgs) -> HttpResponse {
    let headers = vec![];

    let mut res: HttpResponse = HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
        ..Default::default()
    };

    let success_status: Nat = Nat::try_from(200 as u64).unwrap();

    if res.status == success_status {
        res.body = raw.response.body;
    } else {
        ic_cdk::api::print(format!("Received an error: err = {:?}", raw));
    }
    res
}
