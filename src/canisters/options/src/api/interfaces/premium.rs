pub trait Premium<T, U> {
    fn calculate_premium(args: T) -> U;
}

pub struct EuropeanOptions;

pub struct EuropeanOptionsCalculatePremiumArgs {}

pub type EuropeanOptionsCalculatePremiumRes = f64;
