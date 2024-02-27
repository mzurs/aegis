use candid::{Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

use crate::{
    account::_add_account,
    guard::_if_account_exist,
    types::states::{Account, AccountMetrics},
    utils::_generate_random_number,
};

impl Account {
    pub async fn new() -> Result<bool, String> {
        if _if_account_exist(ic_cdk::caller()) {
            Err(String::from("Account Already Exists"))
        } else {
            let user_id: u64 = match _generate_random_number().await {
                Ok(id) => id,
                Err(err) => return Err(err),
            };

            let account_args: Account = Account {
                user_id,
                principal: ic_cdk::caller(),
                user_name: Option::None,
            };
            match _add_account(account_args) {
                Ok(res) => Ok(res),
                Err(err) => Err(String::from(err)),
            }
        }
    }
}

impl Storable for Account {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

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
