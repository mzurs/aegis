
// /// Init Type of Main Canister
// pub type InitType = StableCell<InitArgs, Memory>;

// /// Icrc Tokens Stake Type
// pub type IcrcStakeType = StableBTreeMap<IcrcStakeKey, IcrcStakeValue, Memory>;

// /// Icrc Tokens Stake TimeStamp Type
// pub type IcrcStakeTimeStampType = StableBTreeMap<IcrcStakeTimeStampKey, IcrcStakeTimeStampValue, Memory>;

// /// Icrc Tokens UnStake TimeStamp Type
// pub type IcrcUnStakeTimeStampType = StableBTreeMap<IcrcUnStakeTimeStampKey, IcrcUnStakeTimeStampValue, Memory>;

// /// icrc Tokens TVL metrics
// pub type IcrcTotalValueLockedType = StableBTreeMap<IcrcAsset, IcrcAssetValue, Memory>;

// pub type StakeExecutionLogs = StableBTreeMap<StakeExecutionLogsKeys, StakeExecutionLogsValue, Memory>;

use serde::{Deserialize, Serialize};
use crate::api::state::init_stable_states;


#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip, default = "init_stable_states")]
    pub stable_state: StableStates,
    pub heap_state: HeapStates,
}

pub struct StableStates {
    // pub init: InitType,
    // pub icrc: IcrcStakeStableStateType,
    // pub stake:StakeType,
    // pub metrics: MetricsType,
}

#[derive(Serialize, Deserialize)]
pub struct HeapStates {
    // pub ledger_ids: BTreeMap<IcrcAsset, Principal>,
    // pub min_staking_delay: u64,
}
