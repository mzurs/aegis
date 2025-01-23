use ic_cdk::update;

use crate::api::utils::convert::encode_url;

pub mod constants;
pub mod inflaton_points;
pub mod insurance;

#[update]
async fn test_https_outcalls(base_url: String, query_params: String) -> Result<String, String> {
    match encode_url(&base_url, &query_params) {
        Ok(encoded_url) => Ok(format!("Encoded URL: {}", encoded_url)),
        Err(e) => Err(format!("Error: {}", e.to_string())),
    }
}
