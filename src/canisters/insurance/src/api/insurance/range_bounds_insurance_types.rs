use std::ops::RangeBounds;

use candid::Principal;

use crate::api::interface::state::{
    InsuranceActiveListKey, InsuranceBuyersKey, InsuranceContractExecutionLogsKeys, InsuranceSellersKey,
    UserInsuranceListHistoryKey,
};

pub fn filter_insurance_buyer_amount(insurance_id: u32) -> impl RangeBounds<InsuranceBuyersKey> {
    pub const PRINCIPAL_MIN: Principal = Principal::from_slice(&[]);
    pub const PRINCIPAL_MAX: Principal = Principal::from_slice(&[255; 29]);

    let start: InsuranceBuyersKey = InsuranceBuyersKey {
        insurance_id,

        principal: PRINCIPAL_MIN,
        time_stamp: u64::MIN,
    };
    let end: InsuranceBuyersKey = InsuranceBuyersKey {
        insurance_id,
        principal: PRINCIPAL_MAX,
        time_stamp: u64::MAX,
    };

    start..end
}

pub fn filter_insurance_seller_amount(insurance_id: u32) -> impl RangeBounds<InsuranceSellersKey> {
    pub const PRINCIPAL_MIN: Principal = Principal::from_slice(&[]);
    pub const PRINCIPAL_MAX: Principal = Principal::from_slice(&[255; 29]);

    let start: InsuranceSellersKey = InsuranceSellersKey {
        insurance_id,
        principal: PRINCIPAL_MIN,
        time_stamp: u64::MIN,
    };
    let end: InsuranceSellersKey = InsuranceSellersKey {
        insurance_id,
        principal: PRINCIPAL_MAX,
        time_stamp: u64::MAX,
    };

    start..=end
}

pub fn filter_insruance_trade_history_by_user(principal: Principal) -> impl RangeBounds<UserInsuranceListHistoryKey> {
    let start: UserInsuranceListHistoryKey = UserInsuranceListHistoryKey {
        principal,
        insurance_id: u32::MIN,
    };

    let end: UserInsuranceListHistoryKey = UserInsuranceListHistoryKey {
        principal,
        insurance_id: u32::MAX,
    };

    start..end
}

pub fn filter_iinsurance_active_list_to_remove_by_insurance_id(insurance_id: u32) -> impl RangeBounds<InsuranceActiveListKey> {
    pub const PRINCIPAL_MIN: Principal = Principal::from_slice(&[]);
    pub const PRINCIPAL_MAX: Principal = Principal::from_slice(&[255; 29]);

    let start: InsuranceActiveListKey = InsuranceActiveListKey {
        insurance_id,
        principal: PRINCIPAL_MIN,
    };
    let end: InsuranceActiveListKey = InsuranceActiveListKey {
        insurance_id,
        principal: PRINCIPAL_MAX,
    };

    start..end
}

pub fn filter_contract_execution_logs_by_insurance_id(
    insurance_id: u32,
) -> impl RangeBounds<InsuranceContractExecutionLogsKeys> {
    pub const _PRINCIPAL_MIN: Principal = Principal::from_slice(&[]);
    pub const _PRINCIPAL_MAX: Principal = Principal::from_slice(&[255; 29]);

    let start: InsuranceContractExecutionLogsKeys = InsuranceContractExecutionLogsKeys {
        insurance_id,
        timestamp: u64::MIN,
        message: "".to_string(),
    };
    let end: InsuranceContractExecutionLogsKeys = InsuranceContractExecutionLogsKeys {
        insurance_id,
        timestamp: u64::MAX,
        message: "".to_string(),
    };

    start..end
}

pub fn filter_buyer_insurance_contract_by_principal(principal: Principal) -> impl RangeBounds<InsuranceBuyersKey> {
    let start: InsuranceBuyersKey = InsuranceBuyersKey {
        principal,
        insurance_id: u32::MIN,
        time_stamp: u64::MIN,
    };
    let end: InsuranceBuyersKey = InsuranceBuyersKey {
        principal,
        insurance_id: u32::MAX,
        time_stamp: u64::MAX,
    };

    start..end
}

pub fn filter_seller_insurance_contract_by_principal(principal: Principal) -> impl RangeBounds<InsuranceSellersKey> {
    let start: InsuranceSellersKey = InsuranceSellersKey {
        principal,
        insurance_id: u32::MIN,
        time_stamp: u64::MIN,
    };
    let end: InsuranceSellersKey = InsuranceSellersKey {
        principal,
        insurance_id: u32::MAX,
        time_stamp: u64::MAX,
    };

    start..=end
}
