use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};

use super::options_assets::{OptionsAssets, OptionsAssetsByNames};

#[derive(CandidType, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum OptionsType {
    PUT,
    CALL,
}
#[derive(CandidType, Deserialize, Clone, PartialEq, Debug)]
pub enum OptionsContractState {
    ///
    /// The offer phase is close and contract execution will take place soon
    ///
    OPEN,
    ///
    /// The option contract is closed without execution and closed by seller
    ///
    CLOSED,
    ///
    /// The contract is already executed
    ///
    EXECUTED,
    ///
    /// The option contract expired worthless
    ///
    EXPIRED,
    ///
    /// The option contract is currently in offer phase and will transition into `OPEN` phase
    ///
    OFFER,
}

/// Id to uniquely identify an option contract.
pub type OptionsId = u64;

/// Data structure to represent the properties of single Option Contract.
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Options {
    pub name: String,
    pub seller: Principal,
    pub contract_state: OptionsContractState,
    pub asset: OptionsAssets,
    pub asset_amount: Nat,
    pub strike_price: Nat,
    pub contract_expiry: u64,
    pub buyer: Option<Principal>,
    pub options_type: OptionsType,
    pub timestamp: u64,
    pub offer_duration: u64,
}

/// Keys of Active Options contracts list that display to every users
#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OptionsActiveListKey {
    pub options_type: OptionsType,
    pub options_asset: OptionsAssetsByNames,
    pub timestamp: u64,
    pub id: OptionsId,
    pub offer_duration: u64,
}

/// Keys of Active Put Options contracts list by principal
#[derive(CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PutOptionsActiveListByPrincipalKey {
    pub principal: Principal,
    pub id: OptionsId,
}

/// Keys of Active Call Options contracts list by principal
#[derive(CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CallOptionsActiveListByPrincipalKey {
    pub id: OptionsId,
    pub principal: Principal,
}

/// Create Options Args
#[derive(CandidType, Deserialize, Clone)]
pub struct CreateOptionArgs {
    pub asset: OptionsAssets,
    pub asset_amount: Nat,
    pub contract_expiry: u64,
    pub offer_duration: u64,
    pub options_type: OptionsType,
    pub contract_state: OptionsContractState,
    pub use_exchange_account: bool,
    pub strike_price: Nat,
}

pub type CreateOptionRes = Result<String, String>;

/// Keys of Contract Timestamp
#[derive(CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ContractTimestampsKey {
    pub id: u64,
}

/// Values of Contract Timestamp
#[derive(CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ContractTimestampsValue {
    pub timestamp: u64,
}

/// Keys of Contract Offer Duration Timestamp
#[derive(CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct OfferDurationTimestampsKey {
    pub id: u64,
}
pub type ExecuteOptionRes = Result<(), String>;

/// Keys of Traded options Contract for user User Trade History
#[derive(CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TradedOptionsContractsKey {
    pub principal: Principal,
    pub contract_state: String,
    pub timestamp: u64,
    pub id: u64,
}

/// Values of Traded options Contract for user User Trade History
#[derive(CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TradedOptionsContractsValue {
    pub options_name: String,
    pub options_type: String,
    pub trade_timestamp: u64,
}

/// Create Options Args
#[derive(CandidType, Deserialize, Clone)]
pub struct TradeOptionArgs {
    pub id: u64,
    pub use_exchange_account: bool,
}

pub type TradeOptionRes = Result<String, String>;
