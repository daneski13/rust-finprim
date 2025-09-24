use crate::FloatLike;

/// CAGR - Compound Annual Growth Rate
///
/// The compound annual growth rate (CAGR) is the rate of return that would be required for an investment
/// to grow from its beginning balance to its ending balance, assuming the profits were reinvested at the
/// end of each period of the investment’s life span.
///
/// # Arguments
/// * `beginning_balance` - The initial investment or balance
/// * `ending_balance` - The final investment or balance
/// * `n` - The number of years
///
/// # Returns
/// * The compound annual growth rate (CAGR)
///
/// # Example
/// * Beginning balance of $1000, ending balance of $2000 after 5 years
/// ```
/// use rust_finprim::rate::cagr;
///
/// let beginning_balance = 1000.0;
/// let ending_balance = 2000.0;
/// let n = 5.0;
///
/// cagr(beginning_balance, ending_balance, n);
/// ```
pub fn cagr<T: FloatLike>(beginning_balance: T, ending_balance: T, n: T) -> T {
    (ending_balance / beginning_balance).powf(T::one() / n) - T::one()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(feature = "std"))]
    extern crate std;
    #[cfg(not(feature = "std"))]
    use std::assert;

    #[test]
    fn test_cagr() {
        let beginning_balance = 1000.0;
        let ending_balance = 500.0;
        let n = 5.0;
        let result = cagr(beginning_balance, ending_balance, n);
        let expected: f64 = -0.12945;
        assert!(
            (result - expected).abs() < 1e-5,
            "Failed on case: {}. Expected: {}, Result: {}",
            "Beginning balance of $1000, ending balance of $500 after 5 years",
            expected,
            result
        );
    }
}
