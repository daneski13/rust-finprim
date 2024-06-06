use crate::{ONE, ZERO};
use rust_decimal::prelude::*;

/// FV - Future Value
///
/// A general future value calculation, similar to the Excel `FV` function.
///
///
/// The future value (FV) is the value of an asset or cash at a specified date in the future based on a certain rate of return.
/// The future value is the amount of money that an investment made today will grow to by a future date.
/// It is calculated by applying a rate of return to the initial investment over a specified period of time.
///
/// # Arguments
/// * `rate` - The interest rate per period
/// * `nper` - The number of compounding periods
/// * `pmt` - The payment amount per period
/// * `pv` (optional) - The present value, default is 0
/// * `due` (optional) - The timing of the payment (false = end of period, true = beginning of period), default is false
/// (ordinary annuity)
///
/// At least one of `pmt` or `pv` should be non-zero.
///
/// # Returns
/// * The future value (FV)
///
/// # Example
/// * 5% interest rate
/// * 10 compounding periods
/// * $100 payment per period
/// ```
/// use rust_finprim::tvm::fv;
/// use rust_decimal_macros::*;
///
/// let rate = dec!(0.05); let nper = dec!(10); let pmt = dec!(-100);
/// fv(rate, nper, pmt, None, None);
/// ```
pub fn fv(rate: Decimal, nper: Decimal, pmt: Decimal, pv: Option<Decimal>, due: Option<bool>) -> Decimal {
    let pv: Decimal = pv.unwrap_or(ZERO);
    let due = due.unwrap_or(false);

    if rate == ZERO {
        // If the rate is zero, the nth_power should be 1 (since (1 + 0)^n = 1)
        // The future value calculation when rate is zero is simplified
        pmt * nper + pv
    } else {
        let nth_power = (ONE + rate).powd(nper);

        if due {
            pmt * ((ONE - nth_power) / rate) * (ONE + rate) + (pv * nth_power)
        } else {
            (pmt * (ONE - nth_power) / rate) + (pv * nth_power)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::*;

    #[test]
    fn test_fv() {
        struct TestCase {
            rate: Decimal,
            nper: Decimal,
            pmt: Decimal,
            pv: Option<Decimal>,
            due: Option<bool>,
            expected: Decimal,
            description: &'static str,
        }
        impl TestCase {
            fn new(
                rate: f64,
                nper: f64,
                pmt: f64,
                pv: Option<f64>,
                due: Option<bool>,
                expected: f64,
                description: &'static str,
            ) -> TestCase {
                TestCase {
                    rate: Decimal::from_f64(rate).unwrap(),
                    nper: Decimal::from_f64(nper).unwrap(),
                    pmt: Decimal::from_f64(pmt).unwrap(),
                    pv: pv.map(Decimal::from_f64).unwrap_or(None),
                    due,
                    expected: Decimal::from_f64(expected).unwrap(),
                    description,
                }
            }
        }

        let cases = [
            TestCase::new(
                0.05,
                10.0,
                -100.0,
                None,
                None,
                1257.78925,
                "Standard case with 5% rate, 10 periods, and $100 pmt",
            ),
            TestCase::new(
                0.05,
                10.0,
                -100.0,
                None,
                Some(true),
                1320.67872,
                "Payment at the beg of period should result in higher future value",
            ),
            TestCase::new(0.0, 10.0, -100.0, None, None, -1000.0, "Zero interest rate no growth"),
            TestCase::new(
                0.05,
                10.0,
                -100.0,
                Some(1000.0),
                None,
                2886.68388,
                "Initial investment should result in higher future value",
            ),
        ];

        for case in &cases {
            let calculated_fv = fv(case.rate, case.nper, case.pmt, case.pv, case.due);
            assert!(
                (calculated_fv - case.expected).abs() < dec!(1e-5),
                "Failed on case: {}. Expected {}, got {}",
                case.description,
                case.expected,
                calculated_fv
            );
        }
    }
}
