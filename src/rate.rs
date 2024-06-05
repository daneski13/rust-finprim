//! This module contains functions for calculating interest rates.
//!
//! For example, you can calculate the annual percentage rate (APR) from the effective annual rate (EAR), or vice versa.
//! Also incudes IRR (Internal Rate of Return), MIRR (Modified Internal Rate of Return), and more.

use crate::derivatives::pv_prime_r;
use crate::tvm::{fv, npv, pv, xnpv};
use crate::{ONE, ZERO};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

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
/// * `cash_flows` - A slice of Decimal values representing the cash flows of the investment
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
/// use rust_decimal_macros::*;
///
/// let cash_flows = vec![dec!(-100), dec!(50), dec!(40), dec!(30), dec!(20)];
/// let finance_rate = dec!(0.1);
/// let reinvest_rate = dec!(0.05);
/// mirr(&cash_flows, finance_rate, reinvest_rate);
/// ```
pub fn mirr(cash_flows: &[Decimal], finance_rate: Decimal, reinvest_rate: Decimal) -> Decimal {
    // Num of compounding perids does not include the final period
    let n = cash_flows.len() - 1;

    let (npv_neg, fv_pos) = cash_flows
        .iter()
        .enumerate()
        .fold((ZERO, ZERO), |(npv_neg, fv_pos), (i, &cf)| {
            if cf < ZERO {
                (npv_neg + pv(finance_rate, i.into(), ZERO, Some(cf), None), fv_pos)
            } else {
                (
                    npv_neg,
                    fv_pos + fv(reinvest_rate, (n - i).into(), ZERO, Some(cf), None),
                )
            }
        });
    (fv_pos / -npv_neg).powd(ONE / Decimal::from_usize(n).unwrap()) - ONE
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
/// use rust_decimal_macros::*;
///
/// let flow_table = vec![
///   (dec!(-100), 0),
///   (dec!(-20), 359),
///   (dec!(20), 400),
///   (dec!(20), 1000),
///   (dec!(20), 2000),
/// ];
/// let finance_rate = dec!(0.1);
/// let reinvest_rate = dec!(0.05);
/// xmirr(&flow_table, finance_rate, reinvest_rate);
pub fn xmirr(flow_table: &[(Decimal, i32)], finance_rate: Decimal, reinvest_rate: Decimal) -> Decimal {
    let init_date = flow_table.first().unwrap().1;

    let mut flow_table = flow_table.to_vec();
    for (_, date) in flow_table.iter_mut() {
        *date -= init_date;
    }

    let n = Decimal::from_i32(flow_table.last().unwrap().1).unwrap();
    let (npv_neg, fv_pos) = flow_table.iter().fold((ZERO, ZERO), |(npv_neg, fv_pos), &(cf, date)| {
        if cf < ZERO {
            (
                npv_neg
                    + pv(
                        finance_rate,
                        Decimal::from_i32(date).unwrap() / dec!(365),
                        ZERO,
                        Some(cf),
                        None,
                    ),
                fv_pos,
            )
        } else {
            (
                npv_neg,
                fv_pos
                    + fv(
                        reinvest_rate,
                        (n - Decimal::from_i32(date).unwrap()) / dec!(365),
                        ZERO,
                        Some(cf),
                        None,
                    ),
            )
        }
    });
    (fv_pos / -npv_neg).powd(ONE / (n / dec!(365))) - ONE
}

/// CAGR - Compound Annual Growth Rate
///
/// The compound annual growth rate (CAGR) is the rate of return that would be required for an investment
/// to grow from its beginning balance to its ending balance, assuming the profits were reinvested at the
/// end of each period of the investment’s life span.
///
/// # Arguments
/// * `beginning_balance` - The initial investment or balance
/// * `ending_balance` - The final investment or balance
/// * `n` - The number of years
///
/// # Returns
/// * The compound annual growth rate (CAGR)
///
/// # Example
/// * Beginning balance of $1000, ending balance of $2000 after 5 years
/// ```
/// use rust_finprim::rate::cagr;
/// use rust_decimal_macros::*;
///
/// let beginning_balance = dec!(1000);
/// let ending_balance = dec!(2000);
/// let n = dec!(5);
///
/// cagr(beginning_balance, ending_balance, n);
/// ```
pub fn cagr(beginning_balance: Decimal, ending_balance: Decimal, n: Decimal) -> Decimal {
    (ending_balance / beginning_balance).powd(ONE / n) - ONE
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_cagr() {
        let beginning_balance = dec!(1000);
        let ending_balance = dec!(500);
        let n = dec!(5);
        let result = cagr(beginning_balance, ending_balance, n);
        let expected = dec!(-0.12945);
        assert!(
            (result - expected).abs() < dec!(1e-5),
            "Failed on case: {}. Expected: {}, Result: {}",
            "Beginning balance of $1000, ending balance of $500 after 5 years",
            expected,
            result
        );
    }

    #[test]
    fn test_mirr() {
        let cash_flows = vec![dec!(-100), dec!(-20), dec!(20), dec!(20), dec!(20)];
        let finance_rate = dec!(0.1);
        let reinvest_rate = dec!(0.05);
        let result = mirr(&cash_flows, finance_rate, reinvest_rate);
        let expected = dec!(-0.14536);
        assert!(
            (result - expected).abs() < dec!(1e-5),
            "Failed on case: {}. Expected: {}, Result: {}",
            "Cash flows of -100, -20, 20, 20, 20, finance rate of 0.1, reinvestment rate of 0.05",
            expected,
            result
        );
    }

    #[test]
    fn test_xmirr() {
        let finance_rate = dec!(0.1);
        let reinvest_rate = dec!(0.05);

        // Simtle 1 year case
        let flow_table = vec![
            (dec!(-100), 0),
            (dec!(-20), 365),
            (dec!(20), 730),
            (dec!(20), 1095),
            (dec!(20), 1460),
        ];
        let result = xmirr(&flow_table, finance_rate, reinvest_rate);
        let expected = dec!(-0.14536);
        assert!(
            (result - expected).abs() < dec!(1e-5),
            "Failed on case: {}. Expected: {}, Result: {}",
            "Cash flows of -100, -20, 20, 20, 20, finance rate of 0.1, reinvestment rate of 0.05",
            expected,
            result
        );

        // More complex case
        let flow_table = vec![
            (dec!(-100), 0),
            (dec!(-20), 359),
            (dec!(20), 400),
            (dec!(20), 1000),
            (dec!(20), 2000),
        ];
        let result = xmirr(&flow_table, finance_rate, reinvest_rate);
        let expected = dec!(-0.09689);
        assert!(
            (result - expected).abs() < dec!(1e-5),
            "Failed on case: {}. Expected: {}, Result: {}",
            "Cash flows of -100, -20, 20, 20, 20, at 0, 359, 400, 1000, 2000 days,
            finance rate of 0.1, reinvestment rate of 0.05",
            expected,
            result
        );
    }

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
