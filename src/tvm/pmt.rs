use crate::FloatLike;

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
/// use rust_finprim::tvm::pmt;
///
/// let rate = 0.05; let nper = 10.0; let pv = 1000.0; let fv = 100.0;
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
pub fn pmt<T: FloatLike>(rate: T, nper: T, pv: T, fv: Option<T>, due: Option<bool>) -> T {
    let fv: T = fv.unwrap_or(T::zero());
    let due = due.unwrap_or(false);

    if rate == T::zero() {
        // If the rate is zero, the nth_power should be 1 (since (1 + 0)^n = 1)
        // The payment calculation when rate is zero is simplified
        return -(pv + fv) / nper;
    }

    let nth_power = (T::one() + rate).powf(nper);
    let numerator = rate * (-pv * nth_power - fv);
    let denominator = if due {
        (T::one() - nth_power) * (T::one() + rate)
    } else {
        T::one() - nth_power
    };

    -numerator / denominator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(feature = "std"))]
    extern crate std;
    #[cfg(not(feature = "std"))]
    use std::assert;

    #[test]
    fn test_pmt() {
        struct TestCase {
            rate: f64,
            nper: f64,
            pv: f64,
            fv: Option<f64>,
            due: Option<bool>,
            expected: f64,
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
                    rate,
                    nper,
                    pv,
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
                (calculated_pmt - case.expected).abs() < 1e-5,
                "Failed on case: {}. Expected {}, got {}",
                case.description,
                case.expected,
                calculated_pmt
            );
        }
    }
}
