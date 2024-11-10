use crate::api::interfaces::premium::{
    EuropeanOptions, EuropeanOptionsCalculatePremiumArgs, EuropeanOptionsCalculatePremiumRes, Premium,
};

impl Premium<EuropeanOptionsCalculatePremiumArgs, EuropeanOptionsCalculatePremiumRes> for EuropeanOptions {
    ///
    /// Calculate the option premium of assest with respect to option European Style Option Contract
    ///
    fn calculate_premium(_args: EuropeanOptionsCalculatePremiumArgs) -> EuropeanOptionsCalculatePremiumRes {
        1.0
    }
}
