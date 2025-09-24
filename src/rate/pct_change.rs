use crate::FinPrimError;
use crate::FloatLike;

/// Percentage Change
///
/// The percentage change is a measure of the relative change in value between two points in time.
///
/// # Arguments
/// * `beginning_value` - The initial value or starting point
/// * `ending_value` - The final value or ending point
///
/// # Returns
/// * The percentage change as a `Result` containing a `FloatLike` or `DivideByZero` error.
///
/// # Formula
/// $$\\% \Delta = \frac{\mathrm{Ending\ Value} - \mathrm{Beginning\ Value}}{|\mathrm{Beginning\ Value}|}$$
///
/// # Example
/// * Beginning value of $1000, ending value of $1500
///
/// ```
/// use rust_finprim::rate::pct_change;
///
/// let beginning_value = 1000.0;
/// let ending_value = 1500.0;
///
/// let result = pct_change(beginning_value, ending_value);
/// ```
pub fn pct_change<T: FloatLike>(beginning_value: T, ending_value: T) -> Result<T, FinPrimError<T>> {
    if beginning_value.is_zero() {
        // Avoid division by zero
        return Err(FinPrimError::DivideByZero);
    }

    // Calculate the percentage change
    Ok((ending_value - beginning_value) / beginning_value.abs()) // Use abs to ensure the division is correct for negative values
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
///
/// let value = 1000.0;
/// let pct_change = 0.5; // 50%
///
/// let result = apply_pct_change(value, pct_change);
/// ```
pub fn apply_pct_change<T: FloatLike>(value: T, pct_change: T) -> T {
    pct_change * value.abs() + value
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        beginning_value: f64,
        ending_value: f64,
        pct_change: f64,
    }

    impl TestCase {
        fn new(beginning_value: f64, ending_value: f64, pct_change: f64) -> Self {
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
            TestCase::new(1000.0, 1500.0, 0.5),   // 50% change
            TestCase::new(1000.0, 500.0, -0.5),   // -50% change
            TestCase::new(1000.0, 1000.0, 0.0),   // 0% change
            TestCase::new(1000.0, -1500.0, -2.5), // -250% change
            TestCase::new(-1000.0, 1500.0, 2.5),  // 250% change
        ];
        for case in &cases {
            let pct_change_result = pct_change(case.beginning_value, case.ending_value);
            assert_eq!(pct_change_result, Ok(case.pct_change));

            let apply_pct_change_result = apply_pct_change(case.beginning_value, case.pct_change);
            assert_eq!(apply_pct_change_result, case.ending_value);
        }

        // Test with zero beginning value
        let result_zero = pct_change(0.0, 1000.0).unwrap_or(0.0);
        assert_eq!(result_zero, 0.0);
    }
}
