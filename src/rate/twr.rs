use crate::{rate::pct_change, ONE};
use rust_decimal::prelude::*;

/// Time Weighted Return (TWR)
///
/// This function calculates the Time Weighted Rate of Return (TWR) for a series of asset values
/// and cash flows.
///
/// The TWR is a measure of the compound growth rate of an investment portfolio over time,
/// eliminating the impact of cash flows (deposits and withdrawals) on the return. It is useful
/// as a performance measure on the underlying investment strategy of the portfolio.
///
/// # Arguments
/// * `values` - A slice of tuples containing the asset value at the end of the period and the net
/// cash flow during the period. Cash flows are from the perspective of the asset, i.e. net contibutions
/// are positive and withdrawals/distributions are negative. For example, at the end of the year the
/// asset value is $1000 and there was a net contribution of $100, the tuple would be (1000, 100).
/// * `annualization_period` (optional) - The number of annual periods to annualize the TWR.
/// If `None`, the TWR is not annualized. For example, if you have 6 months of data, you would
/// pass `Some(0.5)` to annualize the TWR. If you have 6 quarters of data, you would pass
/// `Some(1.5)` to annualize the TWR.
///
/// # Returns
/// * The Time Weighted Rate of Return (TWR) as a `Decimal`.
///
/// # Example
/// ```
/// use rust_decimal_macros::dec;
/// use rust_finprim::rate::twr;
///
/// let values = vec![
///    (dec!(1000), dec!(1000)), // Initially we bought an asset for $1000
///    (dec!(1600), dec!(400)), // In Q1'24 we had a net contribution of $400, ending value is $1600
///    (dec!(1450), dec!(-200)), // In Q2'24 we had a net withdrawal of $200, ending value is $1450
///    (dec!(1700), dec!(200)), // In Q3'24 we had a net contribution of $200, ending value is $1700
///    (dec!(2200), dec!(300)), // In Q4'24 we had a net contribution of $300, ending value is $2200
///    (dec!(2500), dec!(0)), // In Q1'25 we had no cash flows, ending value is $2500
///    (dec!(3000), dec!(-300)), // In Q2'25 we had a net withdrawal of $300, ending value is $3000
///    (dec!(1700), dec!(-1500)), // In Q3'25 we had a net withdrawal of $1500, ending value is $1700
///    (dec!(0), dec!(2000)), // In Q4'25 the asset was liquidated and we had a net withdrawal of $2000
/// ];
/// let twr = twr(&values, Some(dec!(2))); // Annualize over 2 years
/// ```
pub fn twr(values: &[(Decimal, Decimal)], annualization_period: Option<Decimal>) -> Decimal {
    let total_return = values
        .windows(2)
        .map(|window| {
            let (start_value, _) = window[0];
            let (end_value, end_cashflow) = window[1];
            let adjusted_end = end_value - end_cashflow;
            pct_change(start_value, adjusted_end)
                .map(|pct| ONE + pct)
                .expect("TWR: pct_change failed, (ending value - cash flow) should not be zero")
        })
        .product::<Decimal>();

    annualization_period
        .map(|period| (total_return).powd(ONE / period) - ONE)
        .unwrap_or(total_return - ONE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::*;
    #[cfg(not(feature = "std"))]
    extern crate std;
    #[cfg(not(feature = "std"))]
    use std::assert;
    #[cfg(not(feature = "std"))]
    use std::prelude::v1::*;

    #[test]
    fn test_twr() {
        let values = vec![
            (dec!(1000), dec!(0)),
            (dec!(1600), dec!(400)),
            (dec!(1450), dec!(-200)),
            (dec!(1700), dec!(200)),
            (dec!(2200), dec!(300)),
        ];
        // Assume the periods are quarterly spanning 1 year
        let twr_qtr = twr(&values, None);
        let expected_qtr = dec!(0.43078093);
        assert!((twr_qtr - expected_qtr).abs() < dec!(1e-5));
        // Assume the periods are annual spanning 4 years
        let twr_yr = twr(&values, Some(dec!(4)));
        let expected_yr = dec!(0.093688);
        assert!((twr_yr - expected_yr).abs() < dec!(1e-5));

        let values_bankruptcy = vec![
            (dec!(1000), dec!(0)),
            (dec!(1600), dec!(400)),
            (dec!(1450), dec!(-200)),
            (dec!(1700), dec!(200)),
            (dec!(2200), dec!(300)),
            (dec!(2500), dec!(0)),
            (dec!(3000), dec!(-300)),
            (dec!(1700), dec!(-1500)),
            (dec!(0), dec!(0)),
        ];
        let twr_bankruptcy = twr(&values_bankruptcy, Some(dec!(2)));
        println!("TWR with bankruptcy: {}", twr_bankruptcy);
        let expected_bankruptcy = dec!(-1);
        assert_eq!(twr_bankruptcy, expected_bankruptcy);

        let values_6qtr = vec![
            (dec!(1000), dec!(0)),
            (dec!(1600), dec!(400)),
            (dec!(1450), dec!(-200)),
            (dec!(1700), dec!(200)),
            (dec!(2200), dec!(300)),
            (dec!(2500), dec!(0)),
            (dec!(3000), dec!(-300)),
        ];
        let twr_6qtr = twr(&values_6qtr, Some(dec!(1.5)));
        let expected_6qtr = dec!(0.663832);
        assert!((twr_6qtr - expected_6qtr).abs() < dec!(1e-5));

        // Just 2 periods
        let values_2 = vec![(dec!(1000), dec!(0)), (dec!(1600), dec!(400))];
        let twr_2 = twr(&values_2, None);
        assert_eq!(twr_2, dec!(0.2));
    }
}
