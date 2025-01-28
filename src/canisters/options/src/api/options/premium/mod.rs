pub mod conversions;
pub mod european_options;
pub mod model;

use url::Url;

#[allow(deprecated)]
pub fn encode_url(base_url: &str, query_params: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Parse the base URL
    let mut url = Url::parse(base_url)?;

    // Encode the query parameters
    let encoded_query = urlencoding::encode(query_params);

    // Add the encoded query to the base URL
    url.set_query(Some(&format!("url={}", encoded_query)));

    Ok(url.into_string())
}
