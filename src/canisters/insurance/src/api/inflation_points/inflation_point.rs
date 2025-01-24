use candid::Nat;
use ic_cdk::{
    api::management_canister::http_request::{
        http_request, CanisterHttpRequestArgument, HttpMethod, HttpResponse, TransformArgs, TransformContext,
    },
    query,
};

use crate::api::{
    interface::inflation_points::{Country, FredInflationData, InfationData},
    utils::convert::encode_url,
};

impl InfationData {
    pub fn new(country: Country, date: Option<String>) -> Self {
        Self { country, date }
    }

    /// Return cuurent inflation data point for a given `Counttry`abc123
    pub(crate) async fn get_inflation_points(&self) -> Result<f32, String> {
        match Self::request_provider(&self).await {
            Ok(res) => Ok(res),
            Err(err) => Err(err),
        }
    }

    async fn request_provider(&self) -> Result<f32, String> {
        self.provider_fred().await
    }

    async fn provider_fred(&self) -> Result<f32, String> {
        //Example URL of the request
        //https://api.stlouisfed.org/fred/series/observations?series_id=CPIAUCSL&api_key=15e62224856a1ef86749639d67a04aea&units=pc1&file_type=json&vintage_dates=2024-06-16&sort_order=desc&limit=1&frequency=m

        let host: &str = "api.stlouisfed.org";
        let series_id: &str = "CPIAUCSL";
        let api_key: &str = "15e62224856a1ef86749639d67a04aea";
        let file_type: &str = "json";
        let units: &str = "pc1";
        let sort_order: &str = "desc";
        let limit: &str = "1";
        let frequency: &str = "m";

        let query_params: String = match &self.date {
            Some(vintage_dates) => format!(
                "https://{}/fred/series/observations?series_id={}&api_key={}&file_type={}&units={}&sort_order={}&limit={}&frequency={}&vintage_dates={}",
                host, series_id, api_key, file_type, units, sort_order, limit, frequency, vintage_dates
            ),
            None => format!(
                "https://{}/fred/series/observations?series_id={}&api_key={}&file_type={}&units={}&sort_order={}&limit={}&frequency={}",
                host, series_id, api_key, file_type, units, sort_order, limit, frequency
            ),
        };

        let request_headers = vec![];
        let base_url = "https://74dvalxfnhjxsuoxnvbqavqm7y0pcxah.lambda-url.us-east-2.on.aws/";

        let request = CanisterHttpRequestArgument {
            // url: url.to_string(),
            url: match encode_url(base_url, &query_params) {
                Ok(res) => res,
                Err(err) => return Err(err.to_string()),
            },
            method: HttpMethod::GET,
            body: None,               //optional for request
            max_response_bytes: None, //optional for request
            transform: Some(TransformContext::from_name("transform_fred".to_string(), vec![])),
            headers: request_headers,
        };

        // ic_cdk::println!("PARAMS: {}", request.to_owned());

        match http_request(request, 50_000_000_000).await {
            Ok((response,)) => {
                let str_body: String = String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.");

                let json_body: FredInflationData = match serde_json::from_str(&str_body) {
                    Ok(res) => res,
                    Err(err) => return Err(err.to_string()),
                };

                Ok(json_body.observations[0].value.parse::<f32>().unwrap())
            }
            Err((r, m)) => {
                let message: String = format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

                Err(message)
            }
        }
    }

    pub(crate) async fn should_insurance_contract_exercise(&self, target_points: f32) -> Result<bool, String> {
        // Get the inflation data points
        // let inflation = InfationData::new(Country::US, "date".to_owned());
        let data_points: f32 = match self.get_inflation_points().await {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        if data_points <= target_points {
            return Ok(false);
        }
        Ok(true)

        // Condition to check whether the  insurance exercised in favor of Buyer
    }
}

// Strips all data that is not needed from the original response.
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
