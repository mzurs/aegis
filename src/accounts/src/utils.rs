use byteorder::{BigEndian, ByteOrder};
use ic_cdk::api::call::RejectionCode;

use crate::{
    enums::{Metric, MetricValues},
    memory::ACCOUNT_METRICS,
    types::states::AccountMetrics,
};

/**
    @title: Implementation of Random Number Generator using Management Canister
*/
pub async fn _generate_random_number() -> Result<u64, String> {
    let random_bytes: Result<(Vec<u8>,), (RejectionCode, String)> =
        ic_cdk::api::management_canister::main::raw_rand().await;

    let random_number: u64 = match random_bytes {
        Ok(rand_bytes) => BigEndian::read_u64(rand_bytes.0.as_slice()),
        Err(err) => return Err(err.1),
    };

    Ok(random_number)
}

// Increment the total no of user account by one
pub fn increment_user_count() -> () {
    ACCOUNT_METRICS.with_borrow_mut(|m| {
        let get_metrics: &AccountMetrics = m.get();

        let _ = m.set(AccountMetrics {
            // ..m
            user_counts: get_metrics.user_counts + 1,
            ..*get_metrics
        });
    });
    ()
}

pub fn _get_metrics(metric: Metric) -> MetricValues {
    match metric {
        Metric::UserCounts => {
            MetricValues::UserCounts(ACCOUNT_METRICS.with_borrow(|m| m.get().user_counts))
        }
        Metric::ActiveUsers => MetricValues::ActiveUsers(0),
    }
}
