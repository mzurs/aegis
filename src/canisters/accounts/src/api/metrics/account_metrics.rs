use std::borrow::Cow;

use candid::{Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};

use crate::api::interfaces::account_metrics::AccountMetrics;

impl Default for AccountMetrics {
    fn default() -> Self {
        Self {
            user_counts: 0,
            active_users: 1,
        }
    }
}

impl Storable for AccountMetrics {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
