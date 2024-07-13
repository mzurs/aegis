use ic_cdk::update;

use crate::api::interface::insurance::{
    BuyInsuranceArgs, BuyInsuranceRes, ExecuteInsuranceContractRes, Insurance, InsuranceContractInitArgs, InsuranceInitRes,
    SellInsuranceArgs, SellInsuranceRes,
};
use crate::guard::restrict_anonymous_identity;

#[update(guard=restrict_anonymous_identity)]
pub async fn create_insurance_contract(args: InsuranceContractInitArgs) -> InsuranceInitRes {
    Insurance::new(args).await
}

#[update(guard=restrict_anonymous_identity)]
pub async fn buy_insurance_contract(args: BuyInsuranceArgs) -> BuyInsuranceRes {
    Insurance::buy_insurance_contract(args).await
}

#[update(guard=restrict_anonymous_identity)]
pub async fn sell_insurance_contract(args: SellInsuranceArgs) -> SellInsuranceRes {
    Insurance::sell_insurance_contract(args).await
}

#[update]
pub async fn execute_insurance_contract_manual(insurance_id: u32) -> ExecuteInsuranceContractRes {
    Insurance::execute_insurance_contract_manual(insurance_id).await
}
