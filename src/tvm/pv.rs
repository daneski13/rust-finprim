use crate::FloatLike;

/// PV - Present Value
///
/// A general present value calculation, similar to the Excel `PV` function. Commonly used
/// for bond pricing and annuity calculations.
/// The `due` parameter expresses whether the annuity type is an ordinary annuity (false and the default) or an annuity due (true),
/// Excel provides this parameter as `type` with 0 for ordinary annuity and 1 for annuity due.
///
/// The present value (PV) is the current value of a future sum of money or cash flow given a
/// specified rate of return.
///
/// # Arguments
/// * `rate` - The interest rate per period
/// * `nper` - The number of compounding periods
/// * `pmt` - The payment amount per period (negative for cash outflows)
/// * `fv` (optional) - The future value
/// * `due` (optional) - The timing of the payment (false = end of period, true = beginning of period), default is false
/// (ordinary annuity)
///
/// At least one of `pmt` or `fv` should be non-zero.
///
/// # Returns
/// The present value (PV)
///
/// # Example
/// 10 Year bond with 3% YTM, $1000 future value, and 5% coupon rate (paid annually)
/// * 5% interest rate
/// * 10 compounding periods
/// * $50 payment per period (5% of $1000)
/// ```
/// use rust_finprim::tvm::pv;
///
/// let rate = 0.05; let nper = 10.0; let pmt = -50.0; let fv = 1000.0;
/// pv(rate, nper, pmt, Some(fv), None);
/// ```
pub fn pv<T: FloatLike>(rate: T, nper: T, pmt: T, fv: Option<T>, due: Option<bool>) -> T {
    let fv: T = fv.unwrap_or(T::zero());
    let due = due.unwrap_or(false);

    let pv = if rate == T::zero() {
        // Simplified formula when rate is zero
        fv + (pmt * nper)
    } else {
        let nth_power = (T::one() + rate).powf(nper);
        let fv_discounted = fv / nth_power;
        let factor = (T::one() - (T::one() / nth_power)) / rate;

        if due {
            pmt * factor * (T::one() + rate) + fv_discounted
        } else {
            pmt * factor + fv_discounted
        }
    };

    -pv
}

/// NPV - Net Present Value
///
/// The net present value (NPV) is the difference between the present value of cash inflows and the present value of cash outflows over a period of time.
/// NPV is used in capital budgeting to analyze the profitability of an investment or project.
/// The NPV is calculated by discounting all cash flows to the present value using a specified discount rate.
/// If the NPV is positive, the investment is considered profitable. If the NPV is negative, the investment is considered unprofitable.
/// Similar to the Excel `NPV` function, with the main difference is that this implementation
/// assumes the first cash flow is at time value 0 (initial investment).
///
/// # Arguments
/// * `rate` - The discount rate per period
/// * `cash_flows` - A slice of values representing the cash flows of the investment,
/// note that the first cash flow is assumed to be at time value 0 (initial investment)
///
/// # Returns
/// * The net present value (NPV)
///
/// # Example
/// * 5% discount rate
/// * Cash flows of $-100, $50, $40, $30, $20
/// ```
/// use rust_finprim::tvm::npv;
///
/// let rate = 0.05;
/// let cash_flows = vec![-100.0, 50.0, 40.0, 30.0, 20.0];
/// npv(rate, &cash_flows);
/// ```
/// # Formula
/// The NPV is calculated by discounting all cash flows to the present value using a specified discount rate.
/// The formula is:
/// $$NPV = \sum_{t=0}^{n} \frac{CF_t}{(1+r)^t}$$
/// Where:
/// * \\(CF_t\\) = cash flow at time \\(t\\)
/// * \\(r\\) = discount rate
pub fn npv<T: FloatLike>(rate: T, cash_flows: &[T]) -> T {
    // Accumulator for the power of (1 + rate)
    // (1 + rate)^(t) so first iteration is (1 + rate)^0 = 1
    let mut powf_acc = T::one();
    let mut npv = T::zero();
    for &cf in cash_flows.iter() {
        npv += cf / (powf_acc);
        powf_acc *= T::one() + rate;
    }
    npv
}

/// NPV Differing Rates - Net Present Value with differing discount rates
///
/// The net present value (NPV) is the difference between the present value of cash inflows and the present value of cash outflows over a period of time.
/// NPV is used in capital budgeting to analyze the profitability of an investment or project.
/// The NPV is calculated by discounting all cash flows to the present value using a specified discount rate.
/// If the NPV is positive, the investment is considered profitable. If the NPV is negative, the investment is considered unprofitable.
/// This function allows for differing discount rates for each cash flow.
///
/// # Arguments
/// * `flow_table` - A slice of tuples representing the cash flows and discount rates for each period `(cash_flow, discount_rate)`,
/// note that the first cash flow is assumed to be at time value 0 (initial investment)
///
/// # Returns
/// * The net present value (NPV)
///
/// # Example
/// * Cash flows of $-100, $50, $40, $30, $20
/// * Discount rates of 5%, 6%, 7%, 8%, 9%
/// ```
/// use rust_finprim::tvm::npv_differing_rates;
///
/// let flow_table = vec![
///     (-100.0, 0.05),
///     (50.0, 0.06),
///     (40.0, 0.07),
///     (30.0, 0.08),
///     (20.0, 0.09),
/// ];
/// npv_differing_rates(&flow_table);
/// ```
///
/// # Formula
/// The NPV is calculated by discounting all cash flows to the present value using a specified discount rate.
/// $$NPV = \sum_{t=0}^{n} \frac{CF_t}{(1+r_t)^t}$$
/// Where:
/// * \\(CF_t\\) = cash flow at time \\(t\\)
/// * \\(r_t\\) = discount rate at time \\(t\\)
pub fn npv_differing_rates<T: FloatLike>(flow_table: &[(T, T)]) -> T {
    flow_table
        .iter()
        .enumerate()
        .map(|(t, &(cf, rate))| cf / (T::one() + rate).powf(T::from_usize(t)))
        .sum()
}

/// XNPV - Net Present Value for irregular cash flows
///
/// The XNPV function calculates the net present value of a series of cash flows that are not necessarily periodic.
///
/// # Arguments
/// * `rate` - The discount rate
/// * `flow_table` - A slice of tuples representing the cash flows and dates for each period `(cash_flow, date)`
/// where `date` represents the number of days from an arbitrary epoch. The first cash flow
/// is assumed to be the initial investment date, the order of subsequent cash flows does
/// not matter.
///
/// Most time libraries will provide a method yielding the number of days from an epoch. For example, in the `chrono` library
/// you can use the `num_days_from_ce` method to get the number of days from the Common Era (CE) epoch, simply convert
/// your date types to an integer representing the number of days from any epoch. Alternatively, you can calculate the
/// time delta in days from an arbitrary epoch, such as the initial investment date.
///
/// Cash flows are discounted assuming a 365-day year.
///
/// # Returns
/// * The net present value (NPV)
///
/// # Example
/// * 5% discount rate
/// * Cash flows of $-100, $50, $40, $30, $20
/// * Dates of 0, 365, 420, 1360, 1460
///
/// ```
/// use rust_finprim::tvm::xnpv;
///
/// let rate = 0.05;
/// let flows_table = vec![
///    (-100.0, 0),
///    (50.0, 365),
///    (40.0, 420),
///    (30.0, 1360),
///    (20.0, 1460),
/// ];
/// xnpv(rate, &flows_table);
pub fn xnpv<T: FloatLike>(rate: T, flow_table: &[(T, i32)]) -> T {
    // First date should be 0 (initial investment) and the rest should be difference from the initial date
    let init_date = flow_table.first().unwrap().1;

    let one_plus_r = T::one() + rate;
    let yr_length: T = T::from_u16(365);
    flow_table
        .iter()
        .map(|&(cf, date)| {
            let years = T::from_i32(date - init_date) / yr_length;
            cf / (one_plus_r).powf(years)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(feature = "std"))]
    extern crate std;
    #[cfg(not(feature = "std"))]
    use std::{assert, vec, vec::Vec};

    #[test]
    fn test_xnpv() {
        let rate = 0.05;
        let flows_table = vec![(-100.0, 0), (50.0, 365), (40.0, 730), (30.0, 1095), (20.0, 1460)];

        let result = xnpv(rate, &flows_table);
        let expected: f64 = 26.26940;
        assert!(
            (result - expected).abs() < 1e-5,
            "Failed on case: {}. Expected: {}, Result: {}",
            "5% discount rate, cash flows of -100, 50, 40, 30, 20",
            expected,
            result
        );
    }

    #[test]
    fn test_pv() {
        struct TestCase {
            rate: f64,
            nper: f64,
            pmt: f64,
            fv: Option<f64>,
            due: Option<bool>,
            expected: f64,
            description: &'static str,
        }
        impl TestCase {
            fn new(
                rate: f64,
                nper: f64,
                pmt: f64,
                fv: Option<f64>,
                due: Option<bool>,
                expected: f64,
                description: &'static str,
            ) -> TestCase {
                TestCase {
                    rate,
                    nper,
                    pmt,
                    fv,
                    due,
                    expected,
                    description,
                }
            }
        }

        let cases = [
            TestCase::new(
                0.05,
                10.0,
                100.0,
                None,
                None,
                -772.17349,
                "Standard case with 5% rate, 10 periods, and $100 pmt",
            ),
            TestCase::new(
                0.05,
                10.0,
                100.0,
                None,
                Some(true),
                -810.78217,
                "Payment at the beg of period should result in higher present value",
            ),
            TestCase::new(0.0, 10.0, -100.0, None, None, 1000.0, "Zero interest rate no growth"),
            TestCase::new(
                0.05,
                10.0,
                100.0,
                Some(1000.0),
                None,
                -1386.08675,
                "Bond with 5% rate, 10 periods, 10% coupon, and $1000 future value",
            ),
            TestCase::new(
                0.05,
                10.0,
                0.0,
                Some(2000.0),
                None,
                -1227.82651,
                "No cash flows, just a future pay out",
            ),
        ];

        for case in &cases {
            let calculated_pv = pv(case.rate, case.nper, case.pmt, case.fv, case.due);
            assert!(
                (calculated_pv - case.expected).abs() < 1e-5,
                "Failed on case: {}. Expected {}, got {}",
                case.description,
                case.expected,
                calculated_pv
            );
        }
    }

    #[test]
    fn test_npv() {
        struct TestCase {
            rate: f64,
            cash_flows: Vec<f64>,
            expected: f64,
            description: &'static str,
        }
        impl TestCase {
            fn new(rate: f64, cash_flows: Vec<f64>, expected: f64, description: &'static str) -> TestCase {
                TestCase {
                    rate,
                    cash_flows,
                    expected,
                    description,
                }
            }
        }

        let cases = [
            TestCase::new(
                0.05,
                vec![-100.0, 50.0, 40.0, 30.0, 20.0],
                26.26940,
                "Standard case with 5% rate and cash flows of -100, 50, 40, 30, 20",
            ),
            TestCase::new(
                0.05,
                vec![100.0, 50.0, 40.0, 30.0, 20.0],
                226.26940,
                "All positive cash flows",
            ),
            TestCase::new(
                0.05,
                vec![-100.0, 50.0, 40.0, 30.0, 20.0, 1000.0],
                809.79557,
                "Additional future cash flow should increase NPV",
            ),
        ];

        for case in &cases {
            let calculated_npv = npv(case.rate, &case.cash_flows);
            assert!(
                (calculated_npv - case.expected).abs() < 1e-5,
                "Failed on case: {}. Expected {}, got {}",
                case.description,
                case.expected,
                calculated_npv
            );
        }
    }

    #[test]
    fn test_npv_differing_rates() {
        struct TestCase {
            flow_table: Vec<(f64, f64)>,
            expected: f64,
            description: &'static str,
        }
        impl TestCase {
            fn new(rates: Vec<f64>, cash_flows: Vec<f64>, expected: f64, description: &'static str) -> TestCase {
                let flow_table = cash_flows.iter().zip(rates.iter()).map(|(&cf, &r)| (cf, r)).collect();
                TestCase {
                    flow_table,
                    expected,
                    description,
                }
            }
        }

        let cases = [
            TestCase::new(
                vec![0.05, 0.06, 0.07, 0.08, 0.09],
                vec![-100.0, 50.0, 40.0, 30.0, 20.0],
                20.09083,
                "Increasing rate and cash flows of -100, 50, 40, 30, 20",
            ),
            TestCase::new(
                vec![0.05, 0.06, 0.07, 0.08, 0.09],
                vec![100.0, 50.0, 40.0, 30.0, 20.0],
                220.09083,
                "All positive cash flows",
            ),
            TestCase::new(
                vec![0.05, 0.06, 0.07, 0.08, 0.09, 0.1],
                vec![-100.0, 50.0, 40.0, 30.0, 20.0, 1000.0],
                641.01215,
                "Additional future cash flow should increase NPV",
            ),
        ];

        for case in &cases {
            let calculated_npv = npv_differing_rates(&case.flow_table);
            assert!(
                (calculated_npv - case.expected).abs() < 1e-5,
                "Failed on case: {}. Expected {}, got {}",
                case.description,
                case.expected,
                calculated_npv
            );
        }
    }
}
