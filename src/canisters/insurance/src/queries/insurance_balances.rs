use candid::Nat;
use ic_cdk::query;

use crate::api::{insurance::insurance_balances, interface::insurance::Insurance};

#[query]
pub async fn get_pool_balance_by_insurance_id(insurance_id: u32) -> Nat {
    match Insurance::get_insurance_by_id(insurance_id) {
        Some(res) => insurance_balances::get_pool_balance_by_insurance_id(insurance_id, res.insurance_asset).await,
        None => Nat::from(0 as u64),
    }
}

#[query]
pub async fn get_premium_pool_balance_by_insurance_id(insurance_id: u32) -> Nat {
    match Insurance::get_insurance_by_id(insurance_id) {
        Some(res) => insurance_balances::get_premium_pool_balance_by_insurance_id(insurance_id, res.insurance_asset).await,
        None => Nat::from(0 as u64),
    }
}