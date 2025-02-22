use crate::{ONE, ZERO};
use rust_decimal::prelude::*;

/// PMT - Payment
///
/// General payment calculation, similar to the Excel `PMT` function. Commonly used for loan and mortgage calculations.
/// The `due` parameter expresses whether the annuity type is an ordinary annuity (false and the default) or an annuity due (true),
/// Excel provides this parameter as `type` with 0 for ordinary annuity and 1 for annuity due.
///
/// The payment (PMT) is the amount of money that is paid or received at each period in an annuity.
///
/// # Arguments
/// * `rate` - The interest rate per period
/// * `nper` - The number of compounding periods
/// * `pv` - The present value of a series of cash flows or principal amount
/// * `fv` (optional) - The future value
/// * `due` (optional) - The timing of the payment (false = end of period, true = beginning of period), default is false
/// (ordinary annuity)
///
/// At least one of `pv` or `fv` should be non-zero.
///
/// # Returns
/// * The payment amount (PMT)
///
/// # Example
/// * 5% interest rate
/// * 10 compounding periods
/// * $1000 present value
/// * $100 future value
/// ```
/// use rust_decimal_macros::*;
/// use rust_finprim::tvm::pmt;
///
/// let rate = dec!(0.05); let nper = dec!(10); let pv = dec!(1000); let fv = dec!(100);
/// pmt(rate, nper, pv, Some(fv), None);
/// ```
///
/// # Formula
/// The payment amount (PMT) is calculated using the formula for the present value of an annuity.
/// The formula is:
/// $$PMT = \frac{r(PV (r+1)^n - FV)}{(r+1)^n -1}$$
///
/// Where:
/// * \\(r\\) = interest rate per period
/// * \\(PV\\) = present value of a series of cash flows or principal amount
/// * \\(FV\\) = future value
/// * \\(n\\) = number of compounding periods
pub fn pmt(rate: Decimal, nper: Decimal, pv: Decimal, fv: Option<Decimal>, due: Option<bool>) -> Decimal {
    let fv: Decimal = fv.unwrap_or(ZERO);
    let due = due.unwrap_or(false);

    if rate == ZERO {
        // If the rate is zero, the nth_power should be 1 (since (1 + 0)^n = 1)
        // The payment calculation when rate is zero is simplified
        return -(pv + fv) / nper;
    }

    let nth_power = (ONE + rate).powd(nper);
    let numerator = rate * (-pv * nth_power - fv);
    let denominator = if due {
        (ONE - nth_power) * (ONE + rate)
    } else {
        ONE - nth_power
    };

    -numerator / denominator
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(feature = "std"))]
    extern crate std;
    use rust_decimal_macros::*;
    #[cfg(not(feature = "std"))]
    use std::assert;
    #[cfg(not(feature = "std"))]
    use std::prelude::v1::*;

    #[test]
    fn test_pmt() {
        struct TestCase {
            rate: Decimal,
            nper: Decimal,
            pv: Decimal,
            fv: Option<Decimal>,
            due: Option<bool>,
            expected: Decimal,
            description: &'static str,
        }

        impl TestCase {
            fn new(
                rate: f64,
                nper: f64,
                pv: f64,
                fv: Option<f64>,
                due: Option<bool>,
                expected: f64,
                description: &'static str,
            ) -> TestCase {
                TestCase {
                    rate: Decimal::from_f64(rate).unwrap(),
                    nper: Decimal::from_f64(nper).unwrap(),
                    pv: Decimal::from_f64(pv).unwrap(),
                    fv: fv.map(Decimal::from_f64).unwrap_or(None),
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
                -1000.0,
                Some(1000.0),
                None,
                50.0,
                "5% coupon bond with 10 periods and $1000 present value",
            ),
            TestCase::new(
                0.05,
                10.0,
                1000.0,
                None,
                None,
                -129.50457,
                "Paying off a $1000 loan with a 5% interest rate",
            ),
            TestCase::new(
                0.0,
                10.0,
                1000.0,
                Some(100.0),
                None,
                -110.0,
                "Zero interest rate no growth",
            ),
            TestCase::new(
                0.05,
                10.0,
                1000.0,
                Some(100.0),
                Some(true),
                -130.90955,
                "Payment at the beg of period should result in lower payment",
            ),
            TestCase::new(0.05, 10.0, 0.0, Some(1000.0), None, -79.50457, "No PV, just a FV"),
            TestCase::new(
                0.05,
                10.0,
                -1100.0,
                Some(1000.0),
                None,
                62.95046,
                "10yr bond trading at a premium, 5% YTM, what's my coupon payment?",
            ),
        ];

        for case in &cases {
            let calculated_pmt = pmt(case.rate, case.nper, case.pv, case.fv, case.due);
            assert!(
                (calculated_pmt - case.expected).abs() < dec!(1e-5),
                "Failed on case: {}. Expected {}, got {}",
                case.description,
                case.expected,
                calculated_pmt
            );
        }
    }
}
