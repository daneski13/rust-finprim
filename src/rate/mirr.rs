use crate::rate::cagr;
use crate::tvm::{fv, pv};
use crate::FloatLike;

/// MIRR - Modified Internal Rate of Return
///
/// The modified internal rate of return (MIRR) is a financial metric that adjusts the
/// internal rate of return (IRR) to account for a different cost of capital and reinvestment rate.
/// Similar behavior and usage to the `MIRR` function in Excel.
///
/// The MIRR assumes that positive cash flows are reinvested at a reinvestment rate, and
/// any negative cash flows are financed at the cost of capital.
///
/// # Arguments
/// * `cash_flows` - A slice of values representing the cash flows of the investment
/// * `finance_rate` - The cost of capital (interest rate) for financing
/// * `reinvest_rate` - The reinvestment rate for positive cash flows
///
/// # Returns
/// * The modified internal rate of return (MIRR)
///
/// # Example
/// * Cash flows of $-100, $50, $40, $30, $20, finance rate of 0.1, reinvestment rate of 0.05
/// ```
/// use rust_finprim::rate::mirr;
///
/// let cash_flows = vec![-100.0, 50.0, 40.0, 30.0, 20.0];
/// let finance_rate = 0.1;
/// let reinvest_rate = 0.05;
/// mirr(&cash_flows, finance_rate, reinvest_rate);
/// ```
pub fn mirr<T: FloatLike>(cash_flows: &[T], finance_rate: T, reinvest_rate: T) -> T {
    // Num of compounding perids does not include the final period
    let n = cash_flows.len() - 1;

    let mut npv_neg = T::zero();
    let mut fv_pos = T::zero();
    for (i, &cf) in cash_flows.iter().enumerate() {
        if cf < T::zero() {
            // Calculate the present value of negative cash flows
            npv_neg += pv(finance_rate, T::from_usize(i), T::zero(), Some(cf), None);
        } else {
            // Calculate the future value of positive cash flows
            fv_pos += fv(reinvest_rate, T::from_usize(n - i), T::zero(), Some(cf), None);
        }
    }
    npv_neg = npv_neg.abs(); // Ensure npv_neg is positive for the calculation
    cagr(
        // Calculate the CAGR using the future value of positive cash flows and the present value of negative cash flows
        npv_neg,
        fv_pos,
        T::from_usize(n),
    )
}

/// XMIRR - Modified Internal Rate of Return for Irregular Cash Flows
///
/// The XMIRR function calculates the modified internal rate of return for a schedule of cash flows that is not necessarily periodic.
///
/// # Arguments
/// * `flow_table` - A slice of tuples representing the cash flows and dates for each period `(cash_flow, date)`
/// where `date` represents the number of days from an arbitrary epoch. The first cash flow is assumed to be the initial investment date
/// at time 0, the order of subsequent cash flows does not matter.
/// * `finance_rate` - The cost of capital (interest rate) for financing
/// * `reinvest_rate` - The reinvestment rate for positive cash flows
///
/// Most time libraries will provide a method for the number of days from an epoch. For example, in the `chrono` library
/// you can use the `num_days_from_ce` method to get the number of days from the Common Era (CE) epoch, simply convert
/// your date types to an integer representing the number of days from any epoch. Alternatively, you can calculate the
/// time delta in days from an arbitrary epoch, such as the initial investment date.
///
/// Cash flows are discounted assuming a 365-day year.
///
/// # Returns
/// * The modified internal rate of return (MIRR)
///
/// # Example
/// * Cash flows of $-100, $-20, $20, $20, $20, finance rate of 0.1, reinvestment rate of 0.05
/// ```
/// use rust_finprim::rate::xmirr;
///
/// let flow_table = vec![
///   (-100.0, 0),
///   (-20.0, 359),
///   (20.0, 400),
///   (20.0, 1000),
///   (20.0, 2000),
/// ];
/// let finance_rate = 0.1;
/// let reinvest_rate = 0.05;
/// xmirr(&flow_table, finance_rate, reinvest_rate);
pub fn xmirr<T: FloatLike>(flow_table: &[(T, i32)], finance_rate: T, reinvest_rate: T) -> T {
    let init_date = flow_table.first().unwrap().1;

    let n = T::from_i32(flow_table.last().unwrap().1);
    let mut npv_neg = T::zero();
    let mut fv_pos = T::zero();
    // Calculate the NPV of negative cash flows and the FV of positive cash Flows
    for &(cf, date) in flow_table {
        // For negative cash flows, calculate the present value
        // For positive cash flows, calculate the future value
        if cf < T::zero() {
            npv_neg += pv(
                finance_rate,
                T::from_i32(date - init_date) / T::from_u16(365),
                T::zero(),
                Some(cf),
                None,
            );
        } else {
            fv_pos += fv(
                reinvest_rate,
                (n - T::from_i32(date)) / T::from_u16(365),
                T::zero(),
                Some(cf),
                None,
            );
        }
    }
    npv_neg.abs(); // Ensure npv_neg is positive for the calculation
    cagr(
        // Calculate the CAGR using the future value of positive cash flows and the present value of negative cash flows
        npv_neg,
        fv_pos,
        n / T::from_u16(365), // Convert to years by dividing by 365
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(feature = "std"))]
    extern crate std;
    #[cfg(not(feature = "std"))]
    use std::{assert, vec};

    #[test]
    fn test_mirr() {
        let cash_flows = vec![-100.0, -20.0, 20.0, 20.0, 20.0];
        let finance_rate = 0.1;
        let reinvest_rate = 0.05;
        let result = mirr(&cash_flows, finance_rate, reinvest_rate);
        let expected: f64 = -0.14536;
        assert!(
            (result - expected).abs() < 1e-5,
            "Failed on case: {}. Expected: {}, Result: {}",
            "Cash flows of -100, -20, 20, 20, 20, finance rate of 0.1, reinvestment rate of 0.05",
            expected,
            result
        );
    }

    #[test]
    fn test_xmirr() {
        let finance_rate = 0.1;
        let reinvest_rate = 0.05;

        // Simtle 1 year case
        let flow_table = vec![(-100.0, 0), (-20.0, 365), (20.0, 730), (20.0, 1095), (20.0, 1460)];
        let result = xmirr(&flow_table, finance_rate, reinvest_rate);
        let expected: f64 = -0.14536;
        assert!(
            (result - expected).abs() < 1e-5,
            "Failed on case: {}. Expected: {}, Result: {}",
            "Cash flows of -100, -20, 20, 20, 20, finance rate of 0.1, reinvestment rate of 0.05",
            expected,
            result
        );

        // More complex case
        let flow_table = vec![(-100.0, 0), (-20.0, 359), (20.0, 400), (20.0, 1000), (20.0, 2000)];
        let result = xmirr(&flow_table, finance_rate, reinvest_rate);
        let expected: f64 = -0.09689;
        assert!(
            (result - expected).abs() < 1e-5,
            "Failed on case: {}. Expected: {}, Result: {}",
            "Cash flows of -100, -20, 20, 20, 20, at 0, 359, 400, 1000, 2000 days,
            finance rate of 0.1, reinvestment rate of 0.05",
            expected,
            result
        );
    }
}
