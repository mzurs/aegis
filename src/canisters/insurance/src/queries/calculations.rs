use candid::Nat;
use ic_cdk::query;

use crate::api::utils::convert::biguint_u128;

#[query]
pub fn division(a: Nat, b: Nat) -> f64 {
    let a_value = match biguint_u128::biguint_to_u128_func(&a.0) {
        Ok(value) => value,
        Err(error) => {
            ic_cdk::println!("Error: {}", error);
            0u128
        }
    } as f64;

    let b_value = match biguint_u128::biguint_to_u128_func(&b.0) {
        Ok(value) => value,
        Err(error) => {
            ic_cdk::println!("Error: {}", error);
            0u128
        }
    } as f64;

    a_value / b_value
}

#[query]
pub fn multiplication(a: f64, b: f64) -> f64 {
    a * b
}
