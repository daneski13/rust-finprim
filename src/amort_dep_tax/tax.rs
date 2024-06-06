use crate::ZERO;
use rust_decimal::prelude::*;

/// Progressive Income Tax
///
/// # Arguments
/// * `agi` - Adjusted Gross Income (AGI) for the tax year, your total income minus any above-the-line deductions
/// * `deductions` - Any below-the-line deductions for the tax year (i.e. standard or itemized deductions)
/// * `rate_table` - A slice of tuples representing the upper income of each bracket and its rate for the tax year `(bracket, rate)`,
/// the last tuple should represent a number to infinity and the highest rate. In practice, the
/// final bracket would the maximum number representable by the Decimal type (`Decimal::MAX`).
///
/// # Returns
/// * An option containing the total tax owed for the tax year based on the progressive rate table.
/// If AGI is less than deductions, zero is returned (no tax owed).
///
/// If the rate table is not valid, i.e. the brackets are not sorted in ascending order or the last bracket
/// is not set to infinity (Decimal::MAX), None is returned. See `progressive_tax_unchecked` for an unchecked
/// (unsafe) version of this function that skips the rate table validation.
///
/// # Examples
/// ```
/// use rust_finprim::amort_dep_tax::progressive_tax;
/// use rust_decimal_macros::*;
/// use rust_decimal::Decimal;
///
/// let rate_table = vec![
///     (dec!(9_875), dec!(0.10)),
///     (dec!(40_125), dec!(0.12)),
///     (dec!(85_525), dec!(0.22)),
///     (dec!(163_300), dec!(0.24)),
///     (dec!(207_350), dec!(0.32)),
///     (dec!(518_400), dec!(0.35)),
///     (Decimal::MAX, dec!(0.37))
/// ];
///
/// let agi = dec!(100_000);
/// let deductions = dec!(12_000);
/// let tax = progressive_tax(agi, deductions, &rate_table);
/// ```
pub fn progressive_tax(agi: Decimal, deductions: Decimal, rate_table: &[(Decimal, Decimal)]) -> Option<Decimal> {
    // Validate the rate table by checking that the brackets are sorted
    // in ascending order. If not, None is returned.
    if rate_table.windows(2).any(|w| w[0].0 > w[1].0) {
        return None;
    }

    // Validate the last bracket is set to infinity (Decimal::MAX)
    if rate_table.last().unwrap().0 != Decimal::MAX {
        return None;
    }

    Some(unsafe { progressive_tax_unchecked(agi, deductions, rate_table) })
}

/// Progressive Income Tax - Unchecked
///
/// This is an unchecked (unsafe) version of the `progressive_tax` function that skips the rate table validation, may provide
/// a performance boost in scenarios where the rate table is known to be valid.
///
/// # Arguments
/// * `agi` - Adjusted Gross Income (AGI) for the tax year, your total income minus any above-the-line deductions
/// * `deductions` - Any below-the-line deductions for the tax year (i.e. standard or itemized deductions)
/// * `rate_table` - A slice of tuples representing the upper income of each bracket and its rate for the tax year `(bracket, rate)`,
/// the last tuple should represent a number to infinity and the highest rate. In practice, the
/// final bracket would the maximum number representable by the Decimal type (`Decimal::MAX`).
///
/// # Returns
/// * The total tax owed for the tax year based on the progressive rate table.
/// If AGI is less than deductions, zero is returned (no tax owed).
///
/// # Examples
/// ```
/// use rust_finprim::amort_dep_tax::progressive_tax;
/// use rust_decimal_macros::*;
/// use rust_decimal::Decimal;
///
/// let rate_table = vec![
///     (dec!(9_875), dec!(0.10)),
///     (dec!(40_125), dec!(0.12)),
///     (dec!(85_525), dec!(0.22)),
///     (dec!(163_300), dec!(0.24)),
///     (dec!(207_350), dec!(0.32)),
///     (dec!(518_400), dec!(0.35)),
///     (Decimal::MAX, dec!(0.37))
/// ];
///
/// let agi = dec!(100_000);
/// let deductions = dec!(12_000);
/// let tax = progressive_tax(agi, deductions, &rate_table);
/// ```
pub unsafe fn progressive_tax_unchecked(
    agi: Decimal,
    deductions: Decimal,
    rate_table: &[(Decimal, Decimal)],
) -> Decimal {
    // If AGI is less than deductions, return zero (no tax owed)
    // This is a common scenario for students or individuals with low income
    if agi <= deductions {
        return ZERO;
    }

    // Taxable income is AGI minus deductions
    let taxable_income = agi - deductions;

    // Calculate the tax owed based on the progressive rate table
    // by iterating over each bracket and applying the rate to the
    // portion of income within that bracket.
    let mut total_tax = ZERO;
    for (i, &(bracket, rate)) in rate_table.iter().enumerate() {
        let prev_bracket = if i == 0 { ZERO } else { rate_table[i - 1].0 };
        // Break early if the previous bracket was greater than the taxable income
        if prev_bracket > taxable_income {
            break;
        }
        total_tax += if taxable_income > bracket {
            (bracket - prev_bracket) * rate
        } else {
            (taxable_income - prev_bracket) * rate
        };
    }
    total_tax
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_progressive_tax() {
        let agi = dec!(60_489.25);
        // Standard single filer deduction for 2024
        let deductions = dec!(14_600);
        //  2024 Federal Income Tax Brackets
        let rate_table = vec![
            (dec!(11_600), dec!(0.10)),
            (dec!(47_150), dec!(0.12)),
            (dec!(100_525), dec!(0.22)),
            (dec!(191_950), dec!(0.24)),
            (dec!(243_725), dec!(0.32)),
            (dec!(609_350), dec!(0.35)),
            (Decimal::MAX, dec!(0.37)),
        ];

        let tax = progressive_tax(agi, deductions, &rate_table);
        assert_eq!(tax, Some(dec!(5_274.71)));

        // Failing rate table (out of order brackets)
        let rate_table_bad = vec![
            (dec!(47_150), dec!(0.12)),
            (dec!(11_600), dec!(0.10)),
            (dec!(100_525), dec!(0.22)),
            (dec!(191_950), dec!(0.24)),
            (dec!(243_725), dec!(0.32)),
            (dec!(609_350), dec!(0.35)),
            (Decimal::MAX, dec!(0.37)),
        ];
        assert_eq!(progressive_tax(agi, deductions, &rate_table_bad), None);

        // AGI less than deductions
        assert_eq!(progressive_tax(dec!(10_000), deductions, &rate_table), Some(ZERO));
    }
}
