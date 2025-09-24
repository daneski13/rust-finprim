use crate::FloatLike;

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
pub fn wacc_prime_de<T: FloatLike>(r_e: T, r_d: T, de_ratio: T, tax: T) -> T {
    -(tax * r_d + r_e - r_d) / (de_ratio + T::one()).powf(T::two())
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
pub fn wacc_prime2_de<T: FloatLike>(r_e: T, r_d: T, de_ratio: T, tax: T) -> T {
    T::two() * (tax * r_d + r_e - r_d) / (de_ratio + T::one()).powf(T::from_u16(3u16))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(feature = "std"))]
    extern crate std;
    #[cfg(not(feature = "std"))]
    use std::assert;

    #[test]
    fn test_wacc_prime() {
        let r_e = 0.05;
        let r_d = 0.07;
        let de_ratio = 0.7;
        let tax = 0.25;

        let result = wacc_prime_de(r_e, r_d, de_ratio, tax);
        let expected: f64 = 0.00086;
        assert!(
            (result - expected).abs() < 1e-5,
            "Failed on case: {}. Expected{}, Result: {}",
            "Cost of Equity 5%, Cost of Debt 7%, D/E 0.7, Tax Rate 25%",
            expected,
            result
        );
    }
}
