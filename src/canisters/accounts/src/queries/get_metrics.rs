use crate::api::interfaces::account_metrics::{Metric, MetricValues};
use crate::api::interfaces::state::StableStates;
use crate::guard::caller_is_admin_controller;
use crate::read_state;
use ic_cdk::query;

/// Get the AegisAccount Canister Metrics
#[query(guard = "caller_is_admin_controller")]
fn get_metrics(args: Metric) -> MetricValues {
    match args {
        Metric::UserCounts => MetricValues::UserCounts(read_state(|m| {
            let state: &StableStates = &m.stable_state;

            state.account_metrics.get().user_counts
        })),
    }
}
