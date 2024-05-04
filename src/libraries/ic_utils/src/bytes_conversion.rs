use candid::Principal;

use crate::constants::{SERIALIZED_PRINCIPAL_LENGTH, SERIALIZED_STRING_LENGTH};

// Source: https://forum.dfinity.org/t/convert-principal-to-vec-29-bytes-length/22468/3
pub fn principal_to_bytes(p: &Principal) -> [u8; SERIALIZED_PRINCIPAL_LENGTH] {
    let mut bytes: [u8; SERIALIZED_PRINCIPAL_LENGTH] = [0; SERIALIZED_PRINCIPAL_LENGTH];
    let p_bytes: &[u8] = p.as_slice();
    bytes[0] = p_bytes.len() as u8;
    bytes[1..p_bytes.len() + 1].copy_from_slice(p_bytes);
    bytes
}

pub fn bytes_to_principal(bytes: &[u8; SERIALIZED_PRINCIPAL_LENGTH]) -> Principal {
    Principal::from_slice(&bytes[1..1 + bytes[0] as usize])
}

pub fn string_to_bytes(s: &String) -> [u8; SERIALIZED_STRING_LENGTH] {
    let mut bytes: [u8; SERIALIZED_STRING_LENGTH] = [0; SERIALIZED_STRING_LENGTH];
    let p_bytes: &[u8] = s.as_bytes();
    bytes[0] = p_bytes.len() as u8;
    bytes[1..p_bytes.len() + 1].copy_from_slice(p_bytes);
    bytes
}

pub fn bytes_to_string(bytes: &[u8; SERIALIZED_STRING_LENGTH]) -> String {
    String::from_utf8(bytes[1..1 + bytes[0] as usize].to_vec()).expect("Failed to convert bytes to string")
}
