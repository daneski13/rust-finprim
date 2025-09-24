use crate::rate::pct_change;
use crate::{FinPrimError, FloatLike};

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
/// * The Time Weighted Rate of Return (TWR).
///
/// # Example
/// ```
/// use rust_finprim::rate::twr;
///
/// let values = vec![
///    (1000.0, 1000.0), // Initially we bought an asset for $1000
///    (1600.0, 400.0), // In Q1'24 we had a net contribution of $400, ending value is $1600
///    (1450.0, -200.0), // In Q2'24 we had a net withdrawal of $200, ending value is $1450
///    (1700.0, 200.0), // In Q3'24 we had a net contribution of $200, ending value is $1700
///    (2200.0, 300.0), // In Q4'24 we had a net contribution of $300, ending value is $2200
///    (2500.0, 0.0), // In Q1'25 we had no cash flows, ending value is $2500
///    (3000.0, -300.0), // In Q2'25 we had a net withdrawal of $300, ending value is $3000
///    (1700.0, -1500.0), // In Q3'25 we had a net withdrawal of $1500, ending value is $1700
///    (0.0, 2000.0), // In Q4'25 the asset was liquidated and we had a net withdrawal of $2000
/// ];
/// let twr = twr(&values, Some(2.0)); // Annualize over 2 years
/// ```
pub fn twr<T: FloatLike>(values: &[(T, T)], annualization_period: Option<T>) -> Result<T, FinPrimError<T>> {
    let total_return = values.windows(2).try_fold(T::one(), |acc, window| {
        let (start_value, _) = window[0];
        let (end_value, end_cashflow) = window[1];
        let adjusted_end = end_value - end_cashflow;
        pct_change(start_value, adjusted_end)
            .map(|pct| acc * (pct + T::one()))
            .map_err(|_| FinPrimError::DivideByZero)
    })?;
    // .map(|window| {
    //     let (start_value, _) = window[0];
    //     let (end_value, end_cashflow) = window[1];
    //     let adjusted_end = end_value - end_cashflow;
    //     pct_change(start_value, adjusted_end)
    //         .map(|pct| pct + T::one())
    //         .map_err(|_| return FinPrimError::DivideByZero)
    // })
    // .product::<T>();

    Ok(annualization_period
        .map(|period| (total_return).powf(T::one() / period) - T::one())
        .unwrap_or(total_return - T::one()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(feature = "std"))]
    extern crate std;
    #[cfg(not(feature = "std"))]
    use std::vec;

    #[test]
    fn test_twr() {
        let values = vec![
            (1000.0, 0.0),
            (1600.0, 400.0),
            (1450.0, -200.0),
            (1700.0, 200.0),
            (2200.0, 300.0),
        ];
        // Assume the periods are quarterly spanning 1 year
        let twr_qtr = twr(&values, None).unwrap();
        let expected_qtr: f64 = 0.43078093;
        assert!((twr_qtr - expected_qtr).abs() < 1e-5);
        // Assume the periods are annual spanning 4 years
        let twr_yr = twr(&values, Some(4.0)).unwrap();
        let expected_yr = 0.093688;
        assert!((twr_yr - expected_yr).abs() < 1e-5);

        let values_bankruptcy = vec![
            (1000.0, 0.0),
            (1600.0, 400.0),
            (1450.0, -200.0),
            (1700.0, 200.0),
            (2200.0, 300.0),
            (2500.0, 0.0),
            (3000.0, -300.0),
            (1700.0, -1500.0),
            (0.0, 0.0),
        ];
        let twr_bankruptcy = twr(&values_bankruptcy, Some(2.0)).unwrap();
        let expected_bankruptcy = -1.0;
        assert_eq!(twr_bankruptcy, expected_bankruptcy);

        let values_6qtr = vec![
            (1000.0, 0.0),
            (1600.0, 400.0),
            (1450.0, -200.0),
            (1700.0, 200.0),
            (2200.0, 300.0),
            (2500.0, 0.0),
            (3000.0, -300.0),
        ];
        let twr_6qtr = twr(&values_6qtr, Some(1.5)).unwrap();
        let expected_6qtr = 0.663832;
        assert!((twr_6qtr - expected_6qtr).abs() < 1e-5);

        // Just 2 periods
        let values_2 = vec![(1000.0, 0.0), (1600.0, 400.0)];
        let twr_2 = twr(&values_2, None).unwrap();
        assert!(twr_2 - 0.2 < 1e-5);
    }
}
