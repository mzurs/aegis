use std::collections::BTreeMap;

use candid::{CandidType, Principal};
use ic_stable_structures::{StableBTreeMap, StableCell, StableVec};
use serde::{Deserialize, Serialize};

use crate::{api::state::init_stable_states, memory::Memory};

use super::{
    insurance::{Insurance, InsuranceAmount, InsuranceAssets, InsuranceId},
    lifecycle::InsuranceInitArgs,
};

// stable variable to store all init arguments
pub type InitType = StableCell<InsuranceInitArgs, Memory>;

// Map that stores all insurance with a given key ( InsuranceId ) of any status
pub type InsuranceDataType = StableBTreeMap<InsuranceId, Insurance, Memory>;

// List of insurance that are currently active
pub type InsuranceActiveList = StableBTreeMap<InsuranceActiveListKey, (), Memory>;

// List of insurance that are either completed or cancelled
pub type UserInsuranceListHistory = StableBTreeMap<UserInsuranceListHistoryKey, u64, Memory>;

// List of all timers used to invoke insurance contract
pub type InsuranceContractTimer = StableVec<(u64, u32), Memory>;

pub type InsuranceBuyers = StableBTreeMap<InsuranceBuyersKey, InsuranceAmount, Memory>;

// List of Seler Insurance amount
pub type InsuranceSellers = StableBTreeMap<InsuranceSellersKey, InsuranceAmount, Memory>;

// Logs of execution of insurance contract
pub type InsuranceContractExecutionLogs = StableBTreeMap<InsuranceContractExecutionLogsKeys, (), Memory>;

#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InsuranceContractExecutionLogsKeys {
    pub insurance_id: InsuranceId,
    pub timestamp: u64,
    pub message: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InsuranceContractExecutionLogsValues {
    pub message: Option<String>,
    pub is_transfer_successfull: Option<bool>,
}

// List of Buyer Insurance amount
#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InsuranceBuyersKey {
    pub insurance_id: u32,
    pub principal: Principal,
    pub time_stamp: u64,
}

// List of Buyer Insurance amount
#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct InsuranceSellersKey {
    pub insurance_id: u32,
    pub principal: Principal,
    pub time_stamp: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserInsuranceListHistoryKey {
    pub principal: Principal,
    pub insurance_id: u32,
}

#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InsuranceActiveListKey {
    pub principal: Principal,
    pub insurance_id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip, default = "init_stable_states")]
    pub stable_state: StableStates,
    pub heap_state: HeapStates,
}

pub struct StableStates {
    pub init: InitType,
    pub insurance: InsuranceDataType,
    pub insurance_contract_timer: InsuranceContractTimer,
    pub insurance_buyers: InsuranceBuyers,
    pub insurance_sellers: InsuranceSellers,
    pub insurance_contract_execution_logs: InsuranceContractExecutionLogs,
    pub user_trade_history: UserInsuranceListHistory,
    pub insurance_active_list: InsuranceActiveList,
}

#[derive(Serialize, Deserialize)]
pub struct HeapStates {
    // pub canister_ids: BTreeMap<CanisterName, Principal>,
    pub ledger_ids: BTreeMap<InsuranceAssets, Principal>,
}
