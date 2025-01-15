use std::f64::consts::{E, PI};

use crate::api::interfaces::options::OptionsType;

/// Calculates the cumulative standard normal distribution function (N(x))
fn cumulative_normal_distribution(x: f64) -> f64 {
    let l = x.abs();
    let k = 1.0 / (1.0 + 0.2316419 * l);
    let k_series = k * (k * (k * (k * k * 0.319381530 - 0.356563782) + 1.781477937) - 1.821255978) + 1.330274429;

    let cnd = 1.0 - (1.0 / ((2.0 * PI).sqrt())) * E.powf(-0.5 * l * l) * k_series;

    if x < 0.0 {
        1.0 - cnd
    } else {
        cnd
    }
}

// /// Represents the type of option (Call or Put)
// #[derive(Debug, Clone, Copy)]
// enum OptionType {
//     Call,
//     Put,
// }

/// Calculates the price of a European option using Black-Scholes-Merton model
pub fn black_scholes_merton(
    option_type: OptionsType,
    stock_price: f64,
    strike_price: f64,
    risk_free_rate: f64,
    volatility: f64,
    time_to_maturity: f64,
) -> f64 {
    // Calculate d1 and d2 parameters
    let d1: f64 = (stock_price.ln() - strike_price.ln() + (risk_free_rate + 0.5 * volatility.powi(2)) * time_to_maturity)
        / (volatility * time_to_maturity.sqrt());

    let d2: f64 = d1 - volatility * time_to_maturity.sqrt();

    // Calculate option price based on option type
    match option_type {
        OptionsType::CALL => {
            stock_price * cumulative_normal_distribution(d1)
                - strike_price * E.powf(-risk_free_rate * time_to_maturity) * cumulative_normal_distribution(d2)
        }
        OptionsType::PUT => {
            strike_price * E.powf(-risk_free_rate * time_to_maturity) * cumulative_normal_distribution(-d2)
                - stock_price * cumulative_normal_distribution(-d1)
        }
    }
}

// /// Example usage of the Black-Scholes-Merton option pricing function
// fn main() {
//     // Example parameters
//     let stock_price = 100.0;     // Current stock price
//     let strike_price = 100.0;    // Option strike price
//     let risk_free_rate = 0.05;   // Annual risk-free interest rate
//     let volatility = 0.20;       // Stock price volatility
//     let time_to_maturity = 1.0;  // Time to option expiration (in years)

//     // Calculate Call option price
//     let call_price = black_scholes_merton(
//         OptionType::Call,
//         stock_price,
//         strike_price,
//         risk_free_rate,
//         volatility,
//         time_to_maturity
//     );

//     // Calculate Put option price
//     let put_price = black_scholes_merton(
//         OptionType::Put,
//         stock_price,
//         strike_price,
//         risk_free_rate,
//         volatility,
//         time_to_maturity
//     );

//     println!("Call Option Price: ${:.2}", call_price);
//     println!("Put Option Price: ${:.2}", put_price);
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn calculate() {
        // Example parameters
        let stock_price = 100042.000000000; // Current stock price
        let strike_price = 95000.0000000; // Option strike price
        let risk_free_rate = 0.000000000; // Annual risk-free interest rate
        let volatility = 0.200000000; // Stock price volatility
        let time_to_maturity = 0.00040991005; // Time to option expiration (in years)

        // Calculate Put option price
        let put_price = black_scholes_merton(
            OptionsType::CALL,
            stock_price,
            strike_price,
            risk_free_rate,
            volatility,
            time_to_maturity,
        );

        print!("put_price {}", put_price)
    }
}
