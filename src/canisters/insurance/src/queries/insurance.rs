use candid::Nat;
use ic_cdk::query;

use crate::api::interface::{
    insurance::Insurance,
    state::{
        InsuranceActiveListKey, InsuranceBuyersKey, InsuranceContractExecutionLogsKeys, InsuranceSellersKey,
        UserInsuranceListHistoryKey,
    },
};

#[query]
pub fn list_insurance_contract() -> Vec<(InsuranceActiveListKey, ())> {
    Insurance::get_all_insurance_contracts()
}

#[query]
pub fn get_buy_insurance_contract_list_by_princicpal() -> Vec<InsuranceBuyersKey> {
    Insurance::get_buy_insurance_contract_list_by_princicpal()
}

#[query]
pub fn get_seller_insurance_contract_list_by_princicpal() -> Vec<InsuranceSellersKey> {
    Insurance::get_seller_insurance_contract_list_by_princicpal()
}

#[query]
pub fn get_contract_execution_logs_by_insurance_id(insurance_id: u32) -> Vec<InsuranceContractExecutionLogsKeys> {
    Insurance::get_contract_execution_logs_by_insurance_id(insurance_id)
}

#[query]
pub fn get_insurance_by_id(insurance_id: u32) -> Option<Insurance> {
    Insurance::get_insurance_by_id(insurance_id)
}

#[query]
pub fn get_all_contract_execution_logs() -> Vec<InsuranceContractExecutionLogsKeys> {
    Insurance::get_all_contract_execution_logs()
}

#[query]
pub fn get_user_insurance_history_by_principal() -> Vec<(UserInsuranceListHistoryKey, u64)> {
    Insurance::get_user_insurance_history_by_principal(ic_cdk::caller())
}

#[query]
pub async fn calculate_buy_insurance_contract_premium(insurance_id: u32) -> Nat {
    let insurance: Insurance = match Insurance::get_insurance_by_id(insurance_id) {
        Some(res) => res,
        None => return Nat::from(0 as u32),
    };

    Insurance::calculate_buy_insurance_contract_premium(insurance_id, &insurance).await
}
