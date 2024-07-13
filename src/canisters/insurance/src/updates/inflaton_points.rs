use ic_cdk::update;

use crate::api::interface::inflation_points::{Country, InfationData};

#[update]
async fn get_inflation_data(country: Country, date: Option<String>) -> Result<f32, String> {
    let inflation_data = InfationData::new(country, date);
    inflation_data.get_inflation_points().await
}
