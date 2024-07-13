use candid::{CandidType, Nat, Principal};
use ic_ledger_utils::types::icrc_types::{IcrcTransferFromResult, IcrcTransferResult};
use serde::{Deserialize, Serialize};

use super::inflation_points::Country;

pub type InsuranceId = u32;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct InsuranceAmount(pub Nat);

/// All supported assets to create an Insurance Contract
#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Ord, Eq, Copy, Debug)]
pub enum InsuranceAssets {
    ICP,
    CKBTC,
    CKETH,
}
#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum InsuranceContractStatus {
    OPEN,
    CLOSED,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum InsuranceRewardsMultiplier {
    M2X = 2,
    M3X = 3,
    M4X = 4,
}
/// Insurance Datastructure of an Insurance Contract
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Insurance {
    pub title: String,
    pub description: String,
    pub issuer: Principal,
    pub is_muliple_seller_allowed: bool,
    pub insurance_asset: InsuranceAssets,
    pub min_premium_amount: Nat,
    pub min_share_amount: Option<Nat>,
    pub expiry_date: u64,
    pub category: InsuranceCategory,
    pub multiplier: InsuranceRewardsMultiplier,
    pub status: InsuranceContractStatus,
    pub last_executed_time: u64,
    pub last_pool_balance: Nat,
    pub last_premium_balance: Nat,
}

/// Insurance Category
#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum InsuranceCategory {
    InflationBasedInsurance(InflationBasedInsurance),
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct InflationBasedInsurance {
    pub country: Country,
    pub inflation_target: f32,
    pub target_expiry: u64,
}
/// Create Inusrance Args
#[derive(CandidType, Deserialize, Clone)]
pub struct InsuranceContractInitArgs {
    pub title: String,
    pub description: String,
    pub insurance_asset: InsuranceAssets,
    pub amount: Nat,
    pub min_premium_amount: Nat,
    pub min_share_amount: Option<Nat>,
    pub expiry_date: u64,
    pub multiplier: InsuranceRewardsMultiplier,
    pub is_muliple_seller_allowed: bool,
    pub category: InsuranceCategory,
}

/// Create Insurance Response
#[derive(CandidType, Deserialize)]
pub enum InsuranceInitRes {
    Success(u32),
    ErrorMessage(String),
    TransferError(IcrcTransferFromResult),
}

/// Buy Insurance Args
#[derive(CandidType, Deserialize, Clone)]
pub struct BuyInsuranceArgs {
    pub insurance_id: u32,
    pub premium: Nat,
}

/// Buy Insurance Response
#[derive(CandidType, Deserialize)]
pub enum BuyInsuranceRes {
    Success,
    ErrorMessage(String),
    TransferError(IcrcTransferFromResult),
}

/// Sell Insurance Args
#[derive(CandidType, Deserialize, Clone)]
pub struct SellInsuranceArgs {
    pub insurance_id: u32,
    pub amount: Nat,
}

/// Sell InsuranceResponse
#[derive(CandidType, Deserialize)]
pub enum SellInsuranceRes {
    Success,
    ErrorMessage(String),
    TransferError(IcrcTransferFromResult),
}

// Execute Inusurance Contract Args
#[derive(CandidType, Clone)]
pub struct ExecuteInsuranceContractArgs {
    pub insurance_id: u32,
}

#[derive(CandidType)]
pub enum ExecuteInsuranceContractRes {
    Success,
    ErrorMessage(String),
    TransferError(IcrcTransferResult),
}
