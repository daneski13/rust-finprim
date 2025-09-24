use crate::derivatives::{pv_prime2_r, pv_prime_r};
use crate::tvm::{npv, xnpv};
use crate::utils::halley;
use crate::FinPrimError;
use crate::FloatLike;

/// IRR - Internal Rate of Return
///
/// The internal rate of return (IRR) is a metric used in capital budgeting to estimate the profitability of potential investments.
/// The IRR is the interest rate (discount rate) that makes the net present value (NPV) of all cash flows from a particular project equal to zero.
/// IRR calculations rely on the same formula as NPV does, but in this case, the NPV is set to zero and the discount rate is the unknown variable.
/// Similar behavior and usage to the `IRR` function in Excel.
///
/// # Arguments
/// * `cash_flows` - A vector of values representing the cash flows of the investment
/// * `guess` (optional) - A guess for the IRR, defaults to 0.1. Providing a guess can help the function converge faster
/// * `tolerance` (optional) - The tolerance/maximum error bound for the IRR calculation, defaults to 1e-5 i.e. 0.00001
/// * `max_iter` (optional) - The maximum number of iterations to perform, defaults to 20.
///
/// # Returns
/// * Result of the IRR calculation
/// * If the calculation fails, it returns a tuple of the error type with a tuple of the last estimated rate and the NPV at that rate
/// * If the NPV is close to zero, you may consider lowering the tolerance or providing a guess at
/// the last estimated rate. Otherwise, there may be no IRR.
///
/// # Example
/// * Cash flows of $-100, $50, $40, $30, $20
/// ```
/// use rust_finprim::rate::irr;
///
/// let cash_flows = vec![-100.0, 50.0, 40.0, 30.0, 20.0];
/// irr(&cash_flows, None, None, None);
/// ```
///
/// # Formula
/// The IRR is calculated by finding the discount rate that makes the net present value (NPV) of all cash flows equal to zero.
/// The formula is:
/// $$NPV = \sum_{t=0}^{n} \frac{CF_t}{(1+IRR)^t} = 0$$
///
/// Where:
/// * \\(CF_t\\) = cash flow at time \\(t\\)
/// * \\(IRR\\) = internal rate of return
///
/// This function uses the Halley method to find the root of the NPV formula, maxing out
/// at 20 iterations.
pub fn irr<T: FloatLike>(
    cash_flows: &[T],
    guess: Option<T>,
    tolerance: Option<T>,
    max_iter: Option<u16>,
) -> Result<T, FinPrimError<T>> {
    let max_iter = max_iter.unwrap_or(20);
    let tolerance = tolerance.unwrap_or(T::from_f32(1e-5));
    let rate = guess.unwrap_or(T::from_f32(0.1));

    // Halley's Method
    let f = |x: T| npv(x, &cash_flows);
    let f_prime = |x: T| {
        cash_flows
            .iter()
            .enumerate()
            .map(|(i, &cf)| pv_prime_r(x, T::from_usize(i), cf))
            .sum()
    };
    let f_prime2 = |x: T| {
        cash_flows
            .iter()
            .enumerate()
            .map(|(i, &cf)| pv_prime2_r(x, T::from_usize(i), cf))
            .sum()
    };
    halley(rate, f, f_prime, f_prime2, tolerance, max_iter)
}

/// XIRR - Internal Rate of Return for Irregular Cash Flows
///
/// The XIRR function calculates the internal rate of return for a schedule of cash flows that is not necessarily periodic.
///
/// # Arguments
/// * `flow_table` - A slice of tuples representing the cash flows and dates for each period `(cash_flow, date)`
/// where `date` represents the number of days from an arbitrary epoch. The first cash flow
/// is assumed to be the initial investment date, the order of subsequent cash flows does
/// not matter.
/// * `guess` (optional) - A guess for the IRR, defaults to 0.1. Providing a guess can help the function converge faster
/// * `tolerance` (optional) - The tolerance/maximum error bound for the IRR calculation, defaults to 1e-5 i.e. 0.00001
/// * `max_iter` (optional) - The maximum number of iterations to perform, defaults to 20.
///
/// Most time libraries will provide a method for the number of days from an epoch. For example, in the `chrono` library
/// you can use the `num_days_from_ce` method to get the number of days from the Common Era (CE) epoch, simply convert
/// your date types to an integer representing the number of days from any epoch. Alternatively, you can calculate the
/// time delta in days from an arbitrary epoch, such as the initial investment date.
///
/// Cash flows are discounted assuming a 365-day year.
///
/// # Returns
/// * Result of the IRR calculation
/// * If the calculation fails, it returns a tuple of the error type with a tuple of the last estimated rate and the NPV at that rate
/// * If the NPV is close to zero, you may consider lowering the tolerance or providing a guess at
/// the last estimated rate. Otherwise, there may be no IRR.
///
/// # Example
/// * Cash flows of $-100, $50, $40, $30, $20
/// ```
/// use rust_finprim::rate::xirr;
///
/// let flow_table = vec![
///    (-100.0, 0),
///    (50.0, 359),
///    (40.0, 400),
///    (30.0, 1000),
///    (20.0, 2000),
/// ];
/// xirr(&flow_table, None, None, None);
/// ```
///
/// This function uses the Halley method to find the root of the NPV formula, maxing out
/// at 20 iterations.
pub fn xirr<T: FloatLike>(
    flow_table: &[(T, i32)],
    guess: Option<T>,
    tolerance: Option<T>,
    max_iter: Option<u16>,
) -> Result<T, FinPrimError<T>> {
    let max_iter = max_iter.unwrap_or(20);
    let tolerance = tolerance.unwrap_or(T::from_f32(1e-5));
    // First date should be 0 (initial investment) and the rest should be difference from the initial date
    let init_date = flow_table.first().unwrap().1;
    let rate = guess.unwrap_or(T::from_f32(0.1));

    // Halley's Method
    let f = |x: T| xnpv(x, &flow_table);
    let f_prime = |x: T| {
        flow_table
            .iter()
            .map(|&(cf, date)| pv_prime_r(x, T::from_i32(date - init_date) / T::from_u16(365), cf))
            .sum::<T>()
    };
    let f_prime2 = |x: T| {
        flow_table
            .iter()
            .map(|&(cf, date)| pv_prime2_r(x, T::from_i32(date - init_date) / T::from_u16(365), cf))
            .sum::<T>()
    };

    // Halley's method
    halley(rate, f, f_prime, f_prime2, tolerance, max_iter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(feature = "std"))]
    extern crate std;
    #[cfg(not(feature = "std"))]
    use std::{assert, vec};

    #[test]
    fn test_irr() {
        let cash_flows = vec![-100.0, 50.0, 40.0, 30.0, 1000.0];
        let result = irr(&cash_flows, None, Some(1e-5), None);
        let expected = 1.008240536;
        result.unwrap_or_else(|e| panic!("IRR calculation failed: {:?}", e));
        assert!(
            (result.unwrap() - expected).abs() < 1e-5,
            "Failed on case: {}. Expected: {}, Result: {}",
            "Cash flows of -100, 50, 40, 30, 1000",
            expected,
            result.unwrap()
        );
    }

    #[test]
    fn test_xirr() {
        let flow_table = vec![(-100.0, 0), (50.0, 359), (40.0, 400), (30.0, 1000), (20.0, 2000)];
        let xirr = xirr(&flow_table, None, Some(1e-5), None);
        let expected = 0.20084;
        assert!(
            (xirr.unwrap() - expected).abs() < 1e-5,
            "Failed on case: {}. Expected: {}, Result: {}",
            "Cash flows of -100, 50, 40, 30, 20",
            expected,
            xirr.unwrap()
        );
    }
}
