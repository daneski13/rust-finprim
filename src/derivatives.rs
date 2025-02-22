//! This module contains various derivatives of financial calculations.
//! Useful for sensitivity analysis and optimization problems.

use crate::ONE;
use rust_decimal::prelude::*;

/// PV'(r) - Derivative of the present value of a cash flow with respect to the rate.
///
/// # Arguments
/// * `rate` - The discount rate per period
/// * `n` - The nth period
/// * `cash_flow` - The cash flow at period n
///
/// # Returns
/// * The derivative of the present value (PV) with respect to the rate
///
/// Via the sum and difference property, this can be used for finding the derivative of
/// an NPV calculation with respect to the rate.
pub fn pv_prime_r(rate: Decimal, n: Decimal, cash_flow: Decimal) -> Decimal {
    -cash_flow * n / (rate + ONE).powd(n + ONE)
}

/// PV''(r) - Second derivative of the present value of a cash flow with respect to the rate.
///
/// # Arguments
/// * `rate` - The discount rate per period
/// * `n` - The nth period
/// * `cash_flow` - The cash flow at period n
///
/// # Returns
/// * The second derivative of the present value (PV) with respect to the rate
///
/// Via the sum and difference property, this can be used for finding the 2nd derivative of
/// an NPV calculation with respect to the rate.
pub fn pv_prime2_r(rate: Decimal, n: Decimal, cash_flow: Decimal) -> Decimal {
    cash_flow * n * (n + ONE) / (rate + ONE).powd(n + Decimal::TWO)
}

/// WACC'(D/E) - First derivative of WACC with respect to the debt to equity ratio.
///
/// # Arguments
/// * `r_e` - The Cost of Equity
/// * `r_d` - The Cost of Debt
/// * `de_ratio` - The Debt to Equity Ratio (market value where D + E = V)
/// * `tax` - The tax rate
///
/// # Returns
/// * The first derivative of the WACC with respect to the D/E ratio.
///
/// # WACC Formula
/// $$\mathrm{WACC} = R_e (\frac{1}{1+D/E}) + R_d (\frac{D/E}{1+D/E})(1-T_C)$$
///
/// Where:
/// * \\(R_e\\) = Cost of Equity
/// * \\(R_d\\) = Cost of Debt
/// * \\(D/E\\) = The Debt to Equity Ratio (market value)
/// * \\(T_C)\\) = The tax rate
pub fn wacc_prime_de(r_e: Decimal, r_d: Decimal, de_ratio: Decimal, tax: Decimal) -> Decimal {
    -(tax * r_d + r_e - r_d) / (de_ratio + ONE).powd(Decimal::TWO)
}

/// WACC''(D/E) - Second derivative of WACC with respect to the debt to equity ratio.
///
/// # Arguments
/// * `r_e` - The Cost of Equity
/// * `r_d` - The Cost of Debt
/// * `de_ratio` - The Debt to Equity Ratio (market value where D + E = V)
/// * `tax` - The tax rate
///
/// # Returns
/// * The first derivative of the WACC with respect to the D/E ratio.
///
/// # WACC Formula
/// $$\mathrm{WACC} = R_e (\frac{1}{1+D/E}) + R_d (\frac{D/E}{1+D/E})(1-T_C)$$
///
/// Where:
/// * \\(R_e\\) = Cost of Equity
/// * \\(R_d\\) = Cost of Debt
/// * \\(D/E\\) = The Debt to Equity Ratio
/// * \\(T_C)\\) = The tax rate
pub fn wacc_prime2_de(r_e: Decimal, r_d: Decimal, de_ratio: Decimal, tax: Decimal) -> Decimal {
    Decimal::TWO * (tax * r_d + r_e - r_d) / (de_ratio + ONE).powd(Decimal::from_u8(3u8).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(feature = "std"))]
    extern crate std;
    use rust_decimal_macros::dec;
    #[cfg(not(feature = "std"))]
    use std::assert;
    #[cfg(not(feature = "std"))]
    use std::prelude::v1::*;

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

    #[test]
    fn test_wacc_prime() {
        let r_e = dec!(0.05);
        let r_d = dec!(0.07);
        let de_ratio = dec!(0.7);
        let tax = dec!(0.25);

        let result = wacc_prime_de(r_e, r_d, de_ratio, tax);
        let expected = dec!(0.00086);
        assert!(
            (result - expected).abs() < dec!(1e-5),
            "Failed on case: {}. Expected{}, Result: {}",
            "Cost of Equity 5%, Cost of Debt 7%, D/E 0.7, Tax Rate 25%",
            expected,
            result
        );
    }
}
