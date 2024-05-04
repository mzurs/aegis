use crate::{
    api::interfaces::{account_metrics::AccountMetrics, state::StableStates},
    mutate_state,
};

/// Increment the total no of user account by one
pub fn increment() {
    mutate_state(|s| {
        let state: &mut StableStates = &mut s.stable_state;

        let get_metrics: &AccountMetrics = state.account_metrics.get();

        let _ = state.account_metrics.set(AccountMetrics {
            user_counts: get_metrics.user_counts + 1,
            ..*get_metrics
        });
    })
}
