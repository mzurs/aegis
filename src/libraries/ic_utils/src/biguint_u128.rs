use num_bigint::BigUint;

pub fn biguint_to_u128_func(value: &BigUint) -> Result<u128, &'static str> {
    if *value > u128::MAX.into() {
        Err("BigUint value too large to fit in u128")
    } else {
        Ok(value.try_into().unwrap()) // Unwrap is safe here as we've checked for overflow
    }
}
