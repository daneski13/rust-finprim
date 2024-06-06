use crate::tvm::{fv, pv};
use crate::{ONE, ZERO};
use rust_decimal::prelude::*;
use rust_decimal_macros::*;

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
}
