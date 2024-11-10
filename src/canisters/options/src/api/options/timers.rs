use std::time::Duration;

use ic_cdk_timers::{set_timer, TimerId};

use crate::{
    api::interfaces::{
        options::{ContractTimestampsKey, ContractTimestampsValue, OfferDurationTimestampsKey, Options, OptionsId},
        trade::TradeOptions,
    },
    mutate_state, read_state,
};

use super::filters::filter_contract_timestamps;

impl Options {
    ///
    /// Get timestamps from stable memory
    ///  
    pub fn get_options_contract_timestamp(
        min: ContractTimestampsKey,
        max: ContractTimestampsKey,
    ) -> Vec<ContractTimestampsKey> {
        read_state(|s| {
            s.stable_state
                .contract_timestamps
                .range(filter_contract_timestamps(min, max))
                .map(|(k, _)| k)
                .collect()
        })
    }

    ///
    /// Set a timer for a given options contract for option execution
    ///
    pub(crate) fn set_contract_execute_timer<T>(
        ledger: T,
        contract_expiry: u64,
        options_id: <Self as TradeOptions<T>>::ExecuteArgs,
    ) -> Result<TimerId, String>
    where
        Self: TradeOptions<T>,
        T: Clone + Send + 'static,
    {
        let current_time: u64 = ic_cdk::api::time();
        if contract_expiry <= current_time {
            return Err("Contract expiry time cannot be in the past".to_owned());
        }

        let duration: Duration = Duration::from_secs(contract_expiry - current_time);

        // Spawn the timer to call `execute`
        let timer_id: TimerId = set_timer(duration, move || {
            let ledger_clone = ledger.clone();
            ic_cdk::spawn(async move {
                Options::execute(ledger_clone, options_id).await;
            });
        });

        Ok(timer_id)
    }

    ///
    /// Set a timer for a given options contract for offer duration
    ///
    pub(crate) fn set_contract_offer_duration_timer<T>(
        ledger: T,
        offer_duration: u64,
        options_id: <Self as TradeOptions<T>>::ExecuteArgs,
    ) -> Result<TimerId, String>
    where
        Self: TradeOptions<T>,
        T: Clone + Send + 'static,
    {
        let current_time: u64 = ic_cdk::api::time();
        if offer_duration <= current_time {
            return Err("Contract expiry time cannot be in the past".to_owned());
        }

        let duration: Duration = Duration::from_secs(offer_duration - current_time);

        // Spawn the timer to call `execute`
        let timer_id: TimerId = set_timer(duration, move || {
            let ledger_clone: T = ledger.clone();
            ic_cdk::spawn(async move {
                Options::execute_offer(ledger_clone, options_id).await;
            });
        });

        Ok(timer_id)
    }

    ///
    /// Add timestamp to stable memory
    ///  
    pub(crate) fn add_options_contract_timestamp(timestamp: u64, id: u64) {
        mutate_state(|s| {
            s.stable_state
                .contract_timestamps
                .insert(ContractTimestampsKey { id }, ContractTimestampsValue { timestamp })
        });
    }

    ///
    /// Remove the offer duration timer from stable memory
    ///
    pub(crate) fn remove_contract_offer_timestamps(id: OptionsId) {
        mutate_state(|s| {
            s.stable_state
                .contract_offer_duration_timestamps
                .remove(&OfferDurationTimestampsKey { id });
        })
    }

    ///
    /// Remove the exectute contract timer from stable memory
    ///
    pub(crate) fn remove_execute_contract_timestamps(id: OptionsId) {
        mutate_state(|s| {
            s.stable_state.contract_timestamps.remove(&ContractTimestampsKey { id });
        })
    }
}

pub mod timers_candid {
    use std::borrow::Cow;

    use candid::{Decode, Encode};
    use ic_stable_structures::{storable::Bound, Storable};

    use crate::api::interfaces::options::{ContractTimestampsKey, ContractTimestampsValue, OfferDurationTimestampsKey};

    impl Storable for ContractTimestampsKey {
        fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
            Cow::Owned(Encode!(self).unwrap())
        }

        fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
            Decode!(bytes.as_ref(), Self).unwrap()
        }

        const BOUND: Bound = Bound::Unbounded;
    }

    impl Storable for OfferDurationTimestampsKey {
        fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
            Cow::Owned(Encode!(self).unwrap())
        }

        fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
            Decode!(bytes.as_ref(), Self).unwrap()
        }

        const BOUND: Bound = Bound::Unbounded;
    }

    impl Storable for ContractTimestampsValue {
        fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
            Cow::Owned(Encode!(self).unwrap())
        }

        fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
            Decode!(bytes.as_ref(), Self).unwrap()
        }

        const BOUND: Bound = Bound::Unbounded;
    }
}
