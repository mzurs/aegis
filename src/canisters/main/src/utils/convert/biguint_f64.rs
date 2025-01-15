// use num_bigint::BigUint;

// pub fn f64_to_biguint(f: f64) -> Option<BigUint> {
//     if f < 0.0 {
//         return None; // Handle negative values as needed
//     }

//     let u128_value: u128 = f as u128; // Potential loss of precision and overflow

//     // Check for overflow
//     if u128_value as f64 != f {
//         return None; // Overflow occurred
//     }

//     Some(BigUint::from(u128_value))
// }
