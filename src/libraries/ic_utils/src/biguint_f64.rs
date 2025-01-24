use num_bigint::BigUint;

pub fn f64_to_biguint(f: f64) -> Option<BigUint> {
    ic_cdk::println!("f {}", f);

    if f < 0.0 {
        return None; // Handle negative values as needed
    }

    let u128_value: u128 = f as u128; // Potential loss of precision and overflow

    ic_cdk::println!("u128_value {}", u128_value);

    // // Check for overflow
    // if u128_value as f64 != f {
    //     return None; // Overflow occurred
    // }

    Some(BigUint::from(u128_value))
}
