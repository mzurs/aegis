use byteorder::{BigEndian, ByteOrder};
use candid::Principal;
use ic_cdk::api::{call::RejectionCode, management_canister::main::raw_rand};
use icrc_ledger_types::icrc1::account::Subaccount;

pub fn principal_to_subaccount(principal_id: &Principal) -> [u8; 32] {
    let mut subaccount: [u8; 32] = [0; std::mem::size_of::<Subaccount>()];
    let principal_id = principal_id.as_slice();
    subaccount[0] = principal_id.len().try_into().unwrap();
    subaccount[1..1 + principal_id.len()].copy_from_slice(principal_id);

    subaccount
}

/// Convert the principal an Ethereum Address
pub fn principal_to_eth_address(principal: Principal) -> String {
    let n = principal.as_slice().len();
    assert!(n <= 29);
    let mut fixed_bytes = [0u8; 32];
    fixed_bytes[0] = n as u8;
    fixed_bytes[1..=n].copy_from_slice(principal.as_slice());
    format!("0x{}", hex::encode(fixed_bytes))
}

/**
    Implementation of Random Number Generator using Management Canister
*/
pub async fn generate_random_number() -> Result<u64, String> {
    let random_bytes: Result<(Vec<u8>,), (RejectionCode, String)> = raw_rand().await;

    let random_number: u64 = match random_bytes {
        Ok(rand_bytes) => BigEndian::read_u64(rand_bytes.0.as_slice()),
        Err(err) => return Err(err.1),
    };

    Ok(random_number)
}

pub fn convert_u64_to_subaccount(num: u64) -> [u8; 32] {
    let mut network_bytes: [u8; 32] = [0; 32];
    network_bytes[..8].copy_from_slice(&num.to_ne_bytes());

    // Little-endian byte order
    let mut little_endian_bytes: [u8; 32] = [0; 32];
    little_endian_bytes[..8].copy_from_slice(&num.to_le_bytes());
    little_endian_bytes
}
