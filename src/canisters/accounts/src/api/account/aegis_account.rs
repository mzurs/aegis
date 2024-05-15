use std::{borrow::Cow, u64};

use candid::{Decode, Encode, Principal};

use ic_stable_structures::{storable::Bound, Storable};
use ic_utils::generate_random_number;

use crate::{
    api::{
        interfaces::account::{AegisAccount, AegisAccountInfo},
        metrics::increment_user_count::increment,
    },
    mutate_state, read_state,
};

impl Storable for AegisAccount {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for AegisAccount {
    fn default() -> Self {
        Self::new()
    }
}

impl AegisAccount {
    /// Returns the AegisAccount object of the caller
    pub fn new() -> Self {
        Self(ic_cdk::caller())
    }

    /// Returns true if user account(caller) account exists in Stable Memory
    pub fn is_account_exists(&self) -> bool {
        read_state(|acc| acc.stable_state.aegis_account.contains_key(self))
    }

    /// Retrieve User Account(caller) from memory
    pub fn get_account(&self) -> Option<AegisAccountInfo> {
        read_state(|s| s.stable_state.aegis_account.get(self))
    }

    /// Function to create User AegisAccount
    pub(crate) async fn create_account(&self) -> Result<bool, String> {
        let user_id: u64 = match generate_random_number().await {
            Ok(id) => id,
            Err(err) => return Err(err),
        };

        mutate_state(|ss| {
            ss.stable_state.aegis_account.insert(
                AegisAccount(ic_cdk::caller()),
                AegisAccountInfo {
                    user_id,
                    user_name: Option::None,
                },
            )
        });
        increment();

        Ok(true)
    }

    /// Function to update user account name
    pub(crate) fn update_account_user_name(&self, user_name: String) -> Result<(), String> {
        let principal: &Principal = &ic_cdk::caller();

        if self.is_account_exists() {
            let account: AegisAccountInfo = self.get_account().unwrap();

            mutate_state(|s| {
                s.stable_state.aegis_account.insert(
                    AegisAccount(*principal),
                    AegisAccountInfo {
                        user_name: Option::Some(user_name),
                        ..account
                    },
                );
            })
        } else {
            return Result::Err(String::from("AegisAccount Not Exists"));
        }

        Ok(())
    }
}
