use crate::ONE;
use rust_decimal::prelude::*;

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
/// use rust_decimal_macros::*;
///
/// let beginning_balance = dec!(1000);
/// let ending_balance = dec!(2000);
/// let n = dec!(5);
///
/// cagr(beginning_balance, ending_balance, n);
/// ```
pub fn cagr(beginning_balance: Decimal, ending_balance: Decimal, n: Decimal) -> Decimal {
    (ending_balance / beginning_balance).powd(ONE / n) - ONE
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_cagr() {
        let beginning_balance = dec!(1000);
        let ending_balance = dec!(500);
        let n = dec!(5);
        let result = cagr(beginning_balance, ending_balance, n);
        let expected = dec!(-0.12945);
        assert!(
            (result - expected).abs() < dec!(1e-5),
            "Failed on case: {}. Expected: {}, Result: {}",
            "Beginning balance of $1000, ending balance of $500 after 5 years",
            expected,
            result
        );
    }
}
