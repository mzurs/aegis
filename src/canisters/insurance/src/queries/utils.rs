use ic_cdk::query;

use crate::api::utils;

#[query]
pub fn convert_u64_to_date(time_stamp: Option<u64>) -> String {
    let time_stamp = match time_stamp {
        Some(t) => t,
        None => ic_cdk::api::time(),
    };
    utils::u64_to_date::convert_timestamp_to_date(time_stamp)
}
