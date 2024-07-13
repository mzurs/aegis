use candid::Nat;
use ic_cdk::query;

use crate::api::utils::convert::{biguint_f64, biguint_u128};

#[query]
pub fn nat_to_f64(value: Nat) -> f64 {
    let f64_value = match biguint_u128::biguint_to_u128_func(&value.0) {
        Ok(value) => value,
        Err(error) => {
            ic_cdk::println!("Error: {}", error);
            0u128
        }
    } as f64;

    f64_value
}

#[query]
pub fn f64_to_biguint(value: f64) -> Nat {
    match biguint_f64::f64_to_biguint(value) {
        Some(res) => Nat::from(res),
        None => Nat::from(0 as u64),
    }
}
