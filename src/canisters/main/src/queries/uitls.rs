use candid::Principal;
use ic_cdk::query;

//
// This function converts a principal to an Ethereum address.
//
#[query]
fn principal_to_eth_address(principal: candid::Principal) -> String {
    ic_utils::principal_to_eth_address(principal)
}

//
// This function converts a principal to a subaccount.
//
#[query]
pub fn principal_to_subaccount(principal_id: Principal) -> [u8; 32] {
    ic_utils::principal_to_subaccount(&principal_id)
}

//
// This function converts a u32 to a subaccount.
//
#[query]
pub fn convert_u32_to_subaccount(num: u32) -> [u8; 32] {
    ic_utils::convert_u32_to_subaccount(num)
}

//
// This is the current timestamp in milliseconds since the UNIX epoch.
//
#[query]
fn get_current_timestamp() -> u64 {
    ic_cdk::api::time()
}
