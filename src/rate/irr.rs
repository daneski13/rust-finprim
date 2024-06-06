use crate::derivatives::pv_prime_r;
use crate::tvm::{npv, xnpv};
use rust_decimal::prelude::*;
use rust_decimal_macros::*;

/// IRR - Internal Rate of Return
///
/// The internal rate of return (IRR) is a metric used in capital budgeting to estimate the profitability of potential investments.
/// The IRR is the interest rate (discount rate) that makes the net present value (NPV) of all cash flows from a particular project equal to zero.
/// IRR calculations rely on the same formula as NPV does, but in this case, the NPV is set to zero and the discount rate is the unknown variable.
/// Similar behavior and usage to the `IRR` function in Excel.
///
/// # Arguments
/// * `cash_flows` - A vector of Decimal values representing the cash flows of the investment
/// * `guess` (optional) - A guess for the IRR, defaults to 0.1. Providing a guess can help the function converge faster
/// * `tolerance` (optional) - The tolerance/maximum error bound for the IRR calculation, defaults to 1e-5 i.e. 0.00001
///
/// # Returns
/// * Result of the IRR calculation
/// * If the calculation fails, it returns a tuple of the last estimated rate and the NPV at that rate
/// * If the NPV is close to zero, you may consider lowering the tolerance or providing a guess at
/// the last estimated rate. Otherwise, there may be no IRR.
///
/// # Example
/// * Cash flows of $-100, $50, $40, $30, $20
/// ```
/// use rust_finprim::rate::irr;
/// use rust_decimal_macros::*;
///
/// let cash_flows = vec![dec!(-100), dec!(50), dec!(40), dec!(30), dec!(20)];
/// irr(&cash_flows, None, None);
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
/// This function uses the Newton-Raphson method to find the root of the NPV formula, maxing out
/// at 20 iterations.
pub fn irr(
    cash_flows: &[Decimal],
    guess: Option<Decimal>,
    tolerance: Option<Decimal>,
) -> Result<Decimal, (Decimal, Decimal)> {
    const MAX_ITER: u8 = 20;
    let tolerance = tolerance.unwrap_or(dec!(1e-5));

    // Newton-Raphson method
    let mut rate = guess.unwrap_or(dec!(0.1));
    for _ in 0..MAX_ITER {
        let npv_value = npv(rate, cash_flows);
        if npv_value.abs() < tolerance {
            return Ok(rate);
        }
        let drate: Decimal = cash_flows
            .iter()
            .enumerate()
            .map(|(i, cf)| pv_prime_r(rate, i.into(), *cf))
            .sum();
        rate -= npv_value / drate;
    }
    Err((rate, npv(rate, cash_flows)))
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
/// * If the calculation fails, it returns a tuple of the last estimated rate and the NPV at that rate
/// * If the NPV is close to zero, you may consider lowering the tolerance or providing a guess at
/// the last estimated rate. Otherwise, there may be no IRR.
///
/// # Example
/// * Cash flows of $-100, $50, $40, $30, $20
/// ```
/// use rust_finprim::rate::xirr;
/// use rust_decimal_macros::*;
///
/// let flow_table = vec![
///    (dec!(-100), 0),
///    (dec!(50), 359),
///    (dec!(40), 400),
///    (dec!(30), 1000),
///    (dec!(20), 2000),
/// ];
/// xirr(&flow_table, None, None);
pub fn xirr(
    flow_table: &[(Decimal, i32)],
    guess: Option<Decimal>,
    tolerance: Option<Decimal>,
) -> Result<Decimal, (Decimal, Decimal)> {
    let tolerance = tolerance.unwrap_or(dec!(1e-5));
    const MAX_ITER: u8 = 20;
    // First date should be 0 (initial investment) and the rest should be difference from the initial date
    let init_date = flow_table.first().unwrap().1;
    let mut flow_table = flow_table.to_vec();
    for (_, date) in flow_table.iter_mut() {
        *date -= init_date;
    }

    let mut rate = guess.unwrap_or(dec!(0.1));
    for _ in 0..MAX_ITER {
        let npv_value = xnpv(rate, &flow_table);
        if npv_value.abs() < tolerance {
            return Ok(rate);
        }
        let drate: Decimal = flow_table
            .iter()
            .map(|(cf, date)| pv_prime_r(rate, Decimal::from_i32(*date).unwrap() / dec!(365), *cf))
            .sum();
        rate -= npv_value / drate;
    }
    Err((rate, xnpv(rate, &flow_table)))
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "std"))]
    extern crate std;
    use super::*;
    #[cfg(not(feature = "std"))]
    use std::prelude::v1::*;
    #[cfg(not(feature = "std"))]
    use std::{assert, vec};

    #[test]
    fn test_irr() {
        let cash_flows = vec![dec!(-100), dec!(50), dec!(40), dec!(30), dec!(1000)];
        let result = irr(&cash_flows, None, Some(dec!(1e-20)));
        if let Err((rate, npv)) = result {
            assert!(
                (npv).abs() < dec!(1e-20),
                "Failed to converge at 1e-20 precision. Last rate: {}, NPV: {}",
                rate,
                npv
            );
        } else {
            assert!(true);
        }
    }

    #[test]
    fn test_xirr() {
        let flow_table = vec![
            (dec!(-100), 0),
            (dec!(50), 359),
            (dec!(40), 400),
            (dec!(30), 1000),
            (dec!(20), 2000),
        ];
        let xirr = xirr(&flow_table, None, Some(dec!(1e-20)));
        if let Err((rate, npv)) = xirr {
            assert!(
                (npv).abs() < dec!(1e-20),
                "Failed to converge at 1e-20 precision. Last rate: {}, NPV: {}",
                rate,
                npv
            );
        } else {
            let expected = dec!(0.20084);
            assert!(
                (xirr.unwrap() - expected).abs() < dec!(1e-5),
                "Failed on case: {}. Expected: {}, Result: {}",
                "Cash flows of -100, 50, 40, 30, 20",
                expected,
                xirr.unwrap()
            );
        }
    }
}
