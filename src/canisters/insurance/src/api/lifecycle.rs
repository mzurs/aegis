use std::borrow::Cow;

use candid::{Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};

use super::interface::lifecycle::InsuranceInitArgs;

impl Storable for InsuranceInitArgs {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for InsuranceInitArgs {
    fn default() -> Self {
        Self {}
    }
}
