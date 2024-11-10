use std::collections::BTreeMap;

use candid::Principal;
use ic_stable_structures::{StableBTreeMap, StableCell};
use serde::{Deserialize, Serialize};

use crate::{api::state::init_stable_states, memory::Memory};

use super::{
    lifecycle::InitArgs,
    options::{
        CallOptionsActiveListByPrincipalKey, ContractTimestampsKey, ContractTimestampsValue, OfferDurationTimestampsKey,
        Options, OptionsActiveListKey, OptionsId, PutOptionsActiveListByPrincipalKey, TradedOptionsContractsKey,
        TradedOptionsContractsValue,
    },
    options_assets::OptionsAssetsIcrc,
};

// stable variable to store all init arguments
pub type InitType = StableCell<InitArgs, Memory>;

// Map that stores all options with a given key ( OptionsId ) of any status
pub type OptionsDataType = StableBTreeMap<OptionsId, Options, Memory>;

// List of Options that are currently active and are disaply to every users to trade options
pub type OptionsActiveList = StableBTreeMap<OptionsActiveListKey, (), Memory>;

// List of Put Traded Options that are active by principal
pub type PutOptionsActiveListByPrincipal = StableBTreeMap<PutOptionsActiveListByPrincipalKey, (), Memory>;

// List of Call Traded Options that are active by principal
pub type CallOptionsActiveListByPrincipal = StableBTreeMap<CallOptionsActiveListByPrincipalKey, (), Memory>;

// List of Call Traded Options that are active by principal
pub type ContractTmestamps = StableBTreeMap<ContractTimestampsKey, ContractTimestampsValue, Memory>;

// List of Call Traded Options that are active by principal
pub type ContractOfferDurationTmestamps = StableBTreeMap<OfferDurationTimestampsKey, u64, Memory>;

// List of all Options that are traded and executed in any of the state by a Principal
pub type TradedOptionsByPrincipal = StableBTreeMap<TradedOptionsContractsKey, TradedOptionsContractsValue, Memory>;

// // List of Options that are either completed or cancelled
// pub type UserOptionsListHistory = StableBTreeMap<UserOptionsListHistoryKey, u64, Memory>;

// // List of all timers used to invoke Options contract
// pub type OptionsContractTimer = StableVec<(u64, u32), Memory>;

// pub type OptionsBuyers = StableBTreeMap<OptionsBuyersKey, OptionsAmount, Memory>;

// List of Seler Options amount
// pub type OptionsSellers = StableBTreeMap<OptionsSellersKey, OptionsAmount, Memory>;

// Logs of execution of Options contract
// pub type OptionsContractExecutionLogs = StableBTreeMap<OptionsContractExecutionLogsKeys, (), Memory>;

// #[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct OptionsContractExecutionLogsKeys {
//     pub options_id: OptionsId,
//     pub timestamp: u64,
//     pub message: String,
// }

// #[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct OptionsContractExecutionLogsValues {
//     pub message: Option<String>,
//     pub is_transfer_successfull: Option<bool>,
// }

// // List of Buyer Options amount
// #[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct OptionsBuyersKey {
//     pub options_id: u32,
//     pub principal: Principal,
//     pub time_stamp: u64,
// }

// // List of Buyer Options amount
// #[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
// pub struct OptionsSellersKey {
//     pub options_id: u32,
//     pub principal: Principal,
//     pub time_stamp: u64,
// }

// #[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct UserOptionsListHistoryKey {
//     pub principal: Principal,
//     pub options_id: u32,
// }

// #[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct OptionsActiveListKey {
//     pub principal: Principal,
//     pub options_id: u32,
// }

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip, default = "init_stable_states")]
    pub stable_state: StableStates,
    pub heap_state: HeapStates,
}

pub struct StableStates {
    pub init: InitType,
    pub options: OptionsDataType,
    pub options_active_list: OptionsActiveList,
    pub put_options_active_list_by_principal: PutOptionsActiveListByPrincipal,
    pub call_options_active_list_by_principal: CallOptionsActiveListByPrincipal,
    pub contract_timestamps: ContractTmestamps,
    pub contract_offer_duration_timestamps: ContractOfferDurationTmestamps,
    pub principal_trade_history: TradedOptionsByPrincipal,
    // pub Options_contract_timer: OptionsContractTimer,
    // pub Options_buyers: OptionsBuyers,
    // pub Options_sellers: OptionsSellers,
    // pub Options_contract_execution_logs: OptionsContractExecutionLogs,
    // pub user_trade_history: UserOptionsListHistory,
    // pub options_active_list: OptionsActiveList,
}

#[derive(Serialize, Deserialize)]
pub struct HeapStates {
    // pub canister_ids: BTreeMap<CanisterName, Principal>,
    pub icrc_ledger_ids: BTreeMap<OptionsAssetsIcrc, Principal>,
}
