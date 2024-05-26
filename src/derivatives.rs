//! This module contains functions for calculating derivatives of various financial functions.
//!
//! This may be useful for sensitivity analysis and optimization problems.

use crate::ONE;
use rust_decimal::prelude::*;

/// PV'(r) - Derivative of the present value of a cash flow with respect to the rate.
///
/// # Arguments
/// * `rate` - The interest rate per period
/// * `n` - The nth period
/// * `cash_flow` - The cash flow at period n
///
/// # Returns
/// * The derivative of the present value (PV) with respect to the rate
pub fn pv_prime_r(rate: Decimal, n: Decimal, cash_flow: Decimal) -> Decimal {
    -cash_flow * n / (rate + ONE).powd(n + ONE)
}

/// PV''(r) - Second derivative of the present value of a cash flow with respect to the rate.
///
/// # Arguments
/// * `rate` - The interest rate per period
/// * `n` - The nth period
/// * `cash_flow` - The cash flow at period n
///
/// # Returns
/// * The second derivative of the present value (PV) with respect to the rate
pub fn pv_prime2_r(rate: Decimal, n: Decimal, cash_flow: Decimal) -> Decimal {
    cash_flow * n * (n + ONE) / (rate + ONE).powd(n + Decimal::TWO)
}

mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_pv_prime() {
        let rate = dec!(0.05);
        let n = dec!(5);
        let cash_flow = dec!(1000);

        let result = pv_prime_r(rate, n, cash_flow);
        let expected = dec!(-3731.07698);
        assert!(
            (result - expected).abs() < dec!(1e-5),
            "Failed on case: {}. Expected: {}, Result: {}",
            "Rate of 5%, 5th period, cash flow of $1000",
            expected,
            result
        );
    }

    #[test]
    fn test_pv_double_prime() {
        let rate = dec!(0.05);
        let n = dec!(5);
        let cash_flow = dec!(1000);

        let result = pv_prime2_r(rate, n, cash_flow);
        let expected = dec!(21320.43990);
        assert!(
            (result - expected).abs() < dec!(1e-5),
            "Failed on case: {}. Expected: {}, Result: {}",
            "Rate of 5%, 5th period, cash flow of $1000",
            expected,
            result
        );
    }
}
