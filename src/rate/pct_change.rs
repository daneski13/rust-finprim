use rust_decimal::prelude::*;

/// Percentage Change
///
/// The percentage change is a measure of the relative change in value between two points in time.
///
/// # Arguments
/// * `beginning_value` - The initial value or starting point
/// * `ending_value` - The final value or ending point
///
/// # Returns
/// * The percentage change as an `Option` containing a `Decimal` or `None` if there is a division by zero.
///
/// # Formula
/// $$\\% \Delta = \frac{\mathrm{Ending\ Value} - \mathrm{Beginning\ Value}}{|\mathrm{Beginning\ Value}|}$$
///
/// # Example
/// * Beginning value of $1000, ending value of $1500
///
/// ```
/// use rust_finprim::rate::pct_change;
/// use rust_decimal_macros::*;
///
/// let beginning_value = dec!(1000);
/// let ending_value = dec!(1500);
///
/// let result = pct_change(beginning_value, ending_value);
/// ```
pub fn pct_change(beginning_value: Decimal, ending_value: Decimal) -> Option<Decimal> {
    if beginning_value.is_zero() {
        // Avoid division by zero
        return None;
    }

    // Calculate the percentage change
    Some((ending_value - beginning_value) / beginning_value.abs()) // Use abs to ensure the division is correct for negative values
}

/// Apply Percentage Change
///
/// This function applies the percentage change to a given value and returns the new value.
///
/// # Arguments
/// * `value` - The initial value or starting point
/// * `pct_change` - The percentage change to apply
///
/// # Returns
/// * The new value after applying the percentage change.
///
/// # Formula
/// $$\mathrm{New\ Value} = |\mathrm{Value}| \times \\% \Delta + \mathrm{Value}$$
///
/// Fluctuations between pos and neg values are handled properly by using the absolute value as
/// derived by the proper percentage change formula.
///
/// The more common formula for applying the percentage change is:
///
/// $$\mathrm{New\ Value} = \mathrm{Value} \times (1 + \\% \Delta)$$
///
/// However, this does not handle the cases where the value is negative and the percentage change is positive, its a
/// simplification for when it can be assumed that the value is always positive.
///
/// For example, if EBITDA is -$1000 and EBITDA increased to -$500, the percentage change should/would be a pos. 50%
/// but the latter formula would return -$1500 while the former would properly return -$500.
///
/// # Example
/// * Value of $1000, percentage change of 50%
/// ```
/// use rust_finprim::rate::apply_pct_change;
/// use rust_decimal_macros::*;
///
/// let value = dec!(1000);
/// let pct_change = dec!(0.5); // 50%
///
/// let result = apply_pct_change(value, pct_change);
/// ```
pub fn apply_pct_change(value: Decimal, pct_change: Decimal) -> Decimal {
    pct_change * value.abs() + value
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    struct TestCase {
        beginning_value: Decimal,
        ending_value: Decimal,
        pct_change: Decimal,
    }

    impl TestCase {
        fn new(beginning_value: Decimal, ending_value: Decimal, pct_change: Decimal) -> Self {
            TestCase {
                beginning_value,
                ending_value,
                pct_change,
            }
        }
    }

    #[test]
    fn test_pct_change_and_apply_pct_change() {
        let cases = [
            TestCase::new(dec!(1000), dec!(1500), dec!(0.5)),   // 50% change
            TestCase::new(dec!(1000), dec!(500), dec!(-0.5)),   // -50% change
            TestCase::new(dec!(1000), dec!(1000), dec!(0)),     // 0% change
            TestCase::new(dec!(1000), dec!(-1500), dec!(-2.5)), // -250% change
            TestCase::new(dec!(-1000), dec!(1500), dec!(2.5)),  // 250% change
        ];
        for case in &cases {
            let pct_change_result = pct_change(case.beginning_value, case.ending_value);
            assert_eq!(pct_change_result, Some(case.pct_change));

            let apply_pct_change_result = apply_pct_change(case.beginning_value, case.pct_change);
            assert_eq!(apply_pct_change_result, case.ending_value);
        }

        // Test with zero beginning value
        let result_zero = pct_change(dec!(0), dec!(1000)).unwrap_or(dec!(0));
        assert_eq!(result_zero, dec!(0));
    }
}
