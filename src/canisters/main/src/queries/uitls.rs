use candid::Principal;
use ic_cdk::query;

#[query]
fn principal_to_eth_address(principal: candid::Principal) -> String {
    ic_utils::principal_to_eth_address(principal)
}

#[query]
pub fn principal_to_subaccount(principal_id: Principal) -> [u8; 32] {
    ic_utils::principal_to_subaccount(&principal_id)
}

#[query]
pub fn convert_u32_to_subaccount(num: u32) -> [u8; 32] {
    ic_utils::convert_u32_to_subaccount(num)
}

#[query]
fn get_current_timestamp() -> u64 {
    ic_cdk::api::time()
}
