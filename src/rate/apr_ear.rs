use crate::ONE;
use rust_decimal::prelude::*;

/// APR - Annual (Nominal) Percentage Rate
///
/// Calculated from the effective interest rate and the number of compounding periods per year.
/// Similar behavior and usage to the `NOMINAL` function in Excel.
///
/// The APR is the annualized interest rate that you are charged on your loan.
/// It is expressed as a percentage that represents the actual yearly cost of funds over the term of a loan.
/// This includes any fees or additional costs associated with the transaction but does not take compounding into account.
///
/// # Arguments
/// * `ear` - The effective interest rate (EAR)
/// * `npery` - The number of compounding periods per year
///
/// # Returns
/// * The annual percentage rate (nominal interest rate)
///
/// # Example
/// * EAR of 5% with 12 compounding periods per year
/// ```
/// use rust_finprim::rate::apr;
/// use rust_decimal_macros::*;
/// let ear = dec!(0.05); let npery = dec!(12);
/// apr(ear, npery);
/// ```
///
/// # Formula
/// $$APR=n(\sqrt\[n\]{1+EAR}-1)$$
///
/// Where:
/// * \\(n\\) = number of compounding periods per year
/// * \\(EAR\\) = effective annual Rate
pub fn apr(ear: Decimal, npery: Decimal) -> Decimal {
    let nth_root = (ONE + ear).powd(ONE / npery);
    npery * (nth_root - ONE)
}

/// EAR - Effective Annual Rate
///
/// The effective annual rate (EAR) is the interest rate on a loan or financial product restated
/// from the nominal interest rate with compounding taken into account.
/// Similar behavior and usage to the `EFFECT` function in Excel.
///
/// The EAR is the rate actually paid or earned on an investment, loan or other financial product
/// due to the result of compounding over a given time period.
///
/// # Arguments
/// * `apr` - The annual percentage rate (APR, nominal interest rate)
/// * `npery` - The number of compounding periods per year
///
/// # Returns
/// * The effective annual rate (EAR)
///
/// # Example
/// * APR of 5% with 12 compounding periods per year
/// ```
/// use rust_finprim::rate::ear;
/// use rust_decimal_macros::*;
///
/// let apr = dec!(0.05); let npery = dec!(12);
/// ear(apr, npery);
/// ```
///
/// # Formula
/// $$EAR=(1+\frac{APR}{n})^n-1$$
///
/// Where:
/// * \\(APR\\) = annual percentage rate (nominal interest rate)
/// * \\(n\\) = number of compounding periods per year
pub fn ear(apr: Decimal, npery: Decimal) -> Decimal {
    let nth_root = ONE + apr / npery;
    nth_root.powd(npery) - ONE
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "std"))]
    extern crate std;
    use super::*;
    use rust_decimal_macros::dec;
    #[cfg(not(feature = "std"))]
    use std::assert;
    #[cfg(not(feature = "std"))]
    use std::prelude::v1::*;

    #[test]
    fn test_apr() {
        struct TestCase {
            n: Decimal,
            ear: Decimal,
            expected: Decimal,
            description: &'static str,
        }
        impl TestCase {
            fn new(n: f64, ear: f64, expected: f64, description: &'static str) -> TestCase {
                TestCase {
                    n: Decimal::from_f64(n).unwrap(),
                    ear: Decimal::from_f64(ear).unwrap(),
                    expected: Decimal::from_f64(expected).unwrap(),
                    description,
                }
            }
        }

        let test_cases = [
            TestCase::new(
                12.0,
                0.05,
                0.04889,
                "Standard case with EAR of 0.05 and monthly compounding",
            ),
            TestCase::new(12.0, 0.0, 0.0, "Zero EAR should result in zero APR"),
            TestCase::new(12.0, 0.2, 0.18371, "High EAR of 0.2 with monthly compounding"),
        ];

        for case in &test_cases {
            let calculated_apr = apr(case.ear, case.n);
            assert!(
                (calculated_apr - case.expected).abs() < dec!(1e-5),
                "Failed on case: {}. Expected {}, got {}",
                case.description,
                case.expected,
                calculated_apr
            );
        }
    }

    #[test]
    fn test_ear() {
        struct TestCase {
            n: Decimal,
            apr: Decimal,
            expected: Decimal,
            description: &'static str,
        }
        impl TestCase {
            fn new(n: f64, apr: f64, expected: f64, description: &'static str) -> TestCase {
                TestCase {
                    n: Decimal::from_f64(n).unwrap(),
                    apr: Decimal::from_f64(apr).unwrap(),
                    expected: Decimal::from_f64(expected).unwrap(),
                    description,
                }
            }
        }

        let test_cases = [
            TestCase::new(
                12.0,
                0.05,
                0.05116,
                "Standard case with APR of 0.05 and monthly compounding",
            ),
            TestCase::new(12.0, 0.0, 0.0, "Zero APR should result in zero EAR"),
            TestCase::new(12.0, 0.2, 0.21939, "High APR of 0.2 with monthly compounding"),
        ];

        for case in &test_cases {
            let calculated_ear = ear(case.apr, case.n);
            assert!(
                (calculated_ear - case.expected).abs() < dec!(1e-5),
                "Failed on case: {}. Expected {}, got {}",
                case.description,
                case.expected,
                calculated_ear
            );
        }
    }
}
