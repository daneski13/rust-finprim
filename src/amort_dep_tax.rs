//! This module provides functions related to loan/mortgage amortization, depreciation, and tax calculations.

use crate::ZERO;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a single period in an amortization schedule.
///
/// An amortization period includes information about the payment period, the portion
/// of the payment allocated to principal, the portion allocated to interest, and the
/// remaining balance of the loan or mortgage.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AmortizationPeriod {
    /// The period number of the amortization schedule.
    pub period: u32,

    /// The amount of the payment allocated to reduce the principal balance.
    pub principal_payment: Decimal,

    /// The amount of the payment allocated to pay interest charges.
    pub interest_payment: Decimal,

    /// The remaining balance of the loan or mortgage after the payment.
    pub remaining_balance: Decimal,
}

impl AmortizationPeriod {
    /// Creates a new `AmortizationPeriod` instance.
    ///
    /// # Arguments
    /// * `period`: The period number of the amortization schedule.
    /// * `principal_payment`: The amount allocated to reduce the principal balance.
    /// * `interest_payment`: The amount allocated to pay interest charges.
    /// * `remaining_balance`: The remaining balance of the loan or mortgage after the payment.
    ///
    /// # Returns
    ///
    /// A new `AmortizationPeriod` instance initialized with the provided values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_fincalc::amort_dep_tax::AmortizationPeriod;
    /// use rust_decimal_macros::*;
    ///
    /// let period = AmortizationPeriod::new(1, dec!(100), dec!(50), dec!(850));
    /// ```
    pub fn new(period: u32, principal_payment: Decimal, interest_payment: Decimal, remaining_balance: Decimal) -> Self {
        Self {
            period,
            principal_payment,
            interest_payment,
            remaining_balance,
        }
    }
}

/// Calculates the amortization schedule for a loan or mortgage.
///
/// The amortization schedule includes a series of payments that are applied to both
/// principal and interest. Each payment reduces the principal balance and pays interest
/// charges based on the remaining balance and the interest rate.
///
///
/// # Arguments
/// * `rate` - The interest rate per period
/// * `nper` - The total number of payment periods
/// * `principal` - The present value or principal amount of the loan (should be positive as cash inflow for a mortgage/loan)
/// * `pmt` - The payment amount per period (should be negative as cash outflow, can be calculated using `pmt` function)
/// * `round` (optional) - A tuple specifying the number of decimal places and a rounding
/// strategy for the amounts `(dp, RoundingStrategy)`, default is no rounding of calculations. The final principal
/// payment is adjusted to zero out the remaining balance if rounding is enabled.
/// `rust_decimal::RoundingStrategy::MidpointNearestEven` ("Bankers Rounding") is likely
/// what you are looking for
///
/// # Returns
/// * A vector of `AmortizationPeriod` instances representing each period in the amortization schedule.
///
/// # Examples
/// * 5% rate, 30 year term (360 months), $1,000,000 loan, $4,000 monthly payment
/// ```
/// use rust_fincalc::amort_dep_tax::amort_schedule;
/// use rust_decimal_macros::dec;
/// use rust_fincalc::tvm::pmt;
///
/// let rate = dec!(0.05) / dec!(12);
/// let nper = 30 * 12;
/// let principal = dec!(1_000_000);
/// let pmt = pmt(rate, nper.into(), principal, None, None);
///
/// let schedule = amort_schedule(rate, nper, principal, pmt, None);
/// ```
pub fn amort_schedule(
    rate: Decimal,
    nper: u32,
    principal: Decimal,
    pmt: Decimal,
    round: Option<(u32, RoundingStrategy)>,
) -> Vec<AmortizationPeriod> {
    // Allocate vector memory upfront
    let mut periods = Vec::with_capacity(nper as usize);

    let pmt = if let Some((dp, rounding)) = round {
        -pmt.round_dp_with_strategy(dp, rounding)
    } else {
        -pmt
    };

    let mut remaining_balance = principal;
    for period in 1..=nper {
        let mut principal_payment = pmt - (remaining_balance * rate);
        let mut interest_payment = pmt - principal_payment;

        if let Some((dp, rounding)) = round {
            principal_payment = principal_payment.round_dp_with_strategy(dp, rounding);
            interest_payment = interest_payment.round_dp_with_strategy(dp, rounding);
        }

        remaining_balance -= principal_payment;

        periods.insert(
            period as usize - 1,
            AmortizationPeriod::new(period, principal_payment, interest_payment, remaining_balance),
        );
    }

    // Zero out the final balance when rounding is enabled
    // by subtracting the remaining balance from the final payment
    // (adding the remaining balance to the principal payment)
    if round.is_some() {
        let final_payment = periods.last_mut().unwrap();
        final_payment.principal_payment += final_payment.remaining_balance;
        final_payment.remaining_balance = ZERO;
    }

    periods
}

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
/// If AGI is less than deductions, zero is returned (no tax owed). If the rate table is not valid,
/// i.e. the brackets are not sorted in ascending order, None is returned.
///
/// # Examples
/// ```
/// use rust_fincalc::amort_dep_tax::progressive_tax;
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
    // If AGI is less than deductions, return zero (no tax owed)
    // This is a common scenario for students or individuals with low income
    if agi <= deductions {
        return Some(ZERO);
    }

    // Validate the rate table by checking that the brackets are sorted
    // in ascending order. If not, None is returned.
    if rate_table.windows(2).any(|w| w[0].0 > w[1].0) {
        return None;
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
    Some(total_tax)
}

/// Represents a single period in an asset's depreciation schedule.
///
/// An asset depreciation period includes information about the period number,
/// the depreciation expense for the period, and the remaining book value of the asset.
/// The book value is the original cost of the asset minus the accumulated depreciation.
///
/// # Examples
/// ```
/// use rust_fincalc::amort_dep_tax::DepreciationPeriod;
/// use rust_decimal_macros::*;
///
/// let period = DepreciationPeriod::new(1, dec!(100), dec!(900));
/// ```
/// The above example creates a new `DepreciationPeriod` instance with a period number of 1,
/// a depreciation expense of $100, and a remaining book value of $900.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DepreciationPeriod {
    /// The period number of the depreciation schedule.
    pub period: u32,

    /// The depreciation expense for the period.
    pub depreciation_expense: Decimal,

    /// The remaining book value of the asset.
    pub remaining_book_value: Decimal,
}

impl DepreciationPeriod {
    /// Creates a new `DepreciationPeriod` instance.
    ///
    /// # Arguments
    /// * `period`: The period number of the depreciation schedule.
    /// * `depreciation_expense`: The depreciation expense for the period.
    /// * `remaining_book_value`: The remaining book value of the asset.
    ///
    /// # Returns
    ///
    /// A new `DepreciationPeriod` instance initialized with the provided values.
    pub fn new(period: u32, depreciation_expense: Decimal, remaining_book_value: Decimal) -> Self {
        Self {
            period,
            depreciation_expense,
            remaining_book_value,
        }
    }
}

/// Straight Line Depreciation - SLN
///
/// Calculates the depreciation schedule for an asset using the straight-line method.
///
/// # Arguments
/// * `cost` - The initial cost of the asset
/// * `salvage` - The estimated salvage value of the asset at the end of its useful life
/// * `life` - The number of periods over which the asset will be depreciated
///
///
/// # Returns
/// * A vector of `DepreciationPeriod` instances representing each period in the depreciation schedule.
///
/// # Examples
/// * $10,000 asset, $1,000 salvage value, 5 year life
/// ```
/// use rust_fincalc::amort_dep_tax::sln;
/// use rust_decimal_macros::*;
///
/// let cost = dec!(10_000);
/// let salvage = dec!(1_000);
/// let life = 5;
/// let schedule = sln(cost, salvage, life);
/// ```
pub fn sln(cost: Decimal, salvage: Decimal, life: u32) -> Vec<DepreciationPeriod> {
    let depreciation_expense = (cost - salvage) / Decimal::from_u32(life).unwrap();

    let mut periods = Vec::with_capacity(life as usize);
    let mut remaining_book_value = cost;
    for period in 1..=life {
        remaining_book_value -= depreciation_expense;
        periods.insert(
            period as usize - 1,
            DepreciationPeriod::new(period, depreciation_expense, remaining_book_value),
        );
    }
    periods
}

/// Declining Balance Depreciation - DB
///
/// Calculates the depreciation schedule for an asset using the double-declining balance method.
///
/// # Arguments
/// * `cost` - The initial cost of the assert
/// * `salvage` - The estimated salvage value of the asset at the end of its useful life
/// * `life` - The number of periods over which the asset will be depreciated
/// * `factor` (optional) - The factor by which the straight-line depreciation rate is multiplied (default is 2 for double-declining balance)
/// * `round` (optional) - A tuple specifying the number of decimal places and a rounding strategy for the amounts `(dp, RoundingStrategy)`,
/// default is no rounding of calculations. The final depreciation expense is adjusted to ensure the remaining book value is equal to the salvage value.
/// `rust_decimal::RoundingStrategy::MidpointNearestEven` ("Bankers Rounding") is likely what you are looking for as the rounding strategy.
///
/// If rounding is enabled, the final period will be adjusted to "zero" out the remaining book
/// value to the salvage value.
///
/// # Returns
/// * A vector of `DepreciationPeriod` instances representing each period in the depreciation schedule.
///
/// # Examples
/// * $10,000 asset, $1,000 salvage value, 5 year life
/// ```
/// use rust_fincalc::amort_dep_tax::db;
/// use rust_decimal_macros::*;
///
/// let cost = dec!(10_000);
/// let salvage = dec!(1_000);
/// let life = 5;
/// let schedule = db(cost, salvage, life, None, None);
/// ```
pub fn db(
    cost: Decimal,
    salvage: Decimal,
    life: u32,
    factor: Option<Decimal>,
    round: Option<(u32, RoundingStrategy)>,
) -> Vec<DepreciationPeriod> {
    let factor = factor.unwrap_or(Decimal::TWO);

    let mut periods = Vec::with_capacity(life as usize);
    let mut remain_bv = cost;
    let mut accum_dep = ZERO;
    for period in 1..=life {
        let mut dep_exp = factor * (cost - accum_dep) / Decimal::from_u32(life).unwrap();
        if let Some((dp, rounding)) = round {
            dep_exp = dep_exp.round_dp_with_strategy(dp, rounding);
        }

        if dep_exp > remain_bv - salvage {
            dep_exp = remain_bv - salvage;
        }
        accum_dep += dep_exp;
        remain_bv -= dep_exp;

        periods.insert(period as usize - 1, DepreciationPeriod::new(period, dep_exp, remain_bv));
    }

    if round.is_some() {
        let last = periods.last_mut().unwrap();
        last.depreciation_expense += last.remaining_book_value - salvage;
        last.remaining_book_value = salvage;
    }
    periods
}

/// Sum of the Years Digits - SYD
///
/// Calculates the depreciation schedule for an asset using the sum of the years' digits method.
/// The sum of the years' digits method is an accelerated depreciation method that allocates
/// more depreciation expense to the early years of an asset's life.
///
/// # Arguments
/// * `cost` - The initial cost of the asset
/// * `salvage` - The estimated salvage value of the asset at the end of its useful life
/// * `life` - The number of periods over which the asset will be depreciated
/// * `round` (optional) - A tuple specifying the number of decimal places and a rounding strategy for the amounts `(dp, RoundingStrategy)`,
/// default is no rounding of calculations. The final depreciation expense is adjusted to ensure the remaining book value is equal to the salvage value.
/// `rust_decimal::RoundingStrategy::MidpointNearestEven` ("Bankers Rounding") is likely what you are looking for as the rounding strategy.
///
/// If rounding is enabled, the final period will be adjusted to "zero" out the remaining book value to the salvage value.
///
/// # Returns
/// * A vector of `DepreciationPeriod` instances representing each period in the depreciation schedule.
///
/// # Examples
/// * $10,000 asset, $1,000 salvage value, 5 year life
/// ```
/// use rust_fincalc::amort_dep_tax::syd;
/// use rust_decimal_macros::*;
///
/// let cost = dec!(10_000);
/// let salvage = dec!(1_000);
/// let life = 5;
/// let schedule = syd(cost, salvage, life, None);
/// ```
pub fn syd(
    cost: Decimal,
    salvage: Decimal,
    life: u32,
    round: Option<(u32, RoundingStrategy)>,
) -> Vec<DepreciationPeriod> {
    let mut periods = Vec::with_capacity(life as usize);
    let mut remain_bv = cost;
    let mut accum_dep = ZERO;
    let sum_of_years = Decimal::from_u32(life * (life + 1)).unwrap() / Decimal::TWO;
    for period in 1..=life {
        let mut dep_exp = (cost - salvage) * Decimal::from_u32(life - period + 1).unwrap() / sum_of_years;
        if let Some((dp, rounding)) = round {
            dep_exp = dep_exp.round_dp_with_strategy(dp, rounding)
        };

        accum_dep += dep_exp;
        remain_bv -= dep_exp;

        periods.insert(period as usize - 1, DepreciationPeriod::new(period, dep_exp, remain_bv));
    }

    if round.is_some() {
        let last = periods.last_mut().unwrap();
        last.depreciation_expense += last.remaining_book_value - salvage;
        last.remaining_book_value = salvage;
    }
    periods
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        let period = AmortizationPeriod::new(1, dec!(100), dec!(50), dec!(850));
        let serialized = serde_json::to_string(&period).unwrap();
        let deserialized: AmortizationPeriod = serde_json::from_str(&serialized).unwrap();
        assert_eq!(period, deserialized);
        let clone = period.clone();
        assert_eq!(period, clone);

        let dep_period = DepreciationPeriod::new(1, dec!(100), dec!(900));
        let serialized = serde_json::to_string(&dep_period).unwrap();
        let deserialized: DepreciationPeriod = serde_json::from_str(&serialized).unwrap();
        assert_eq!(dep_period, deserialized);
        let clone = dep_period.clone();
        assert_eq!(dep_period, clone);
    }

    #[test]
    fn test_syd() {
        struct TestCase {
            cost: Decimal,
            salvage: Decimal,
            life: u32,
            round: Option<(u32, RoundingStrategy)>,
            expected: Decimal,
        }

        impl TestCase {
            fn new(cost: f64, salvage: f64, life: u32, round: Option<(u32, RoundingStrategy)>, expected: f64) -> Self {
                Self {
                    cost: Decimal::from_f64(cost).unwrap(),
                    salvage: Decimal::from_f64(salvage).unwrap(),
                    life,
                    round,
                    expected: Decimal::from_f64(expected).unwrap(),
                }
            }
        }

        let cases = [
            TestCase::new(10_000.00, 1_000.00, 5, None, 600.00),
            TestCase::new(
                9_000.00,
                1_000.00,
                5,
                Some((2, RoundingStrategy::MidpointNearestEven)),
                533.33,
            ),
            TestCase::new(
                9_000.00,
                1_500.00,
                10,
                Some((2, RoundingStrategy::MidpointNearestEven)),
                136.36,
            ),
        ];
        for case in &cases {
            let schedule = syd(case.cost, case.salvage, case.life, case.round);
            schedule.iter().for_each(|period| println!("{:?}", period));
            assert_eq!(schedule.len(), case.life as usize);
            assert_eq!(schedule.last().unwrap().depreciation_expense, case.expected);
        }
    }

    #[test]
    fn test_db() {
        struct TestCase {
            cost: Decimal,
            salvage: Decimal,
            life: u32,
            factor: Option<Decimal>,
            round: Option<(u32, RoundingStrategy)>,
            expected: Decimal,
        }
        impl TestCase {
            fn new(
                cost: f64,
                salvage: f64,
                life: u32,
                factor: Option<f64>,
                round: Option<(u32, RoundingStrategy)>,
                expected: f64,
            ) -> Self {
                Self {
                    cost: Decimal::from_f64(cost).unwrap(),
                    salvage: Decimal::from_f64(salvage).unwrap(),
                    life,
                    factor: factor.map(Decimal::from_f64).unwrap_or(None),
                    round,
                    expected: Decimal::from_f64(expected).unwrap(),
                }
            }
        }

        let cases = [
            TestCase::new(4_000.00, 1_000.00, 5, None, None, 0.00),
            TestCase::new(10_000.00, 1_000.00, 5, None, None, 296.00),
            TestCase::new(10_000.00, 1_000.00, 10, None, None, 268.435456),
            TestCase::new(
                10_000.00,
                1_000.00,
                10,
                None,
                Some((2, RoundingStrategy::MidpointNearestEven)),
                342.18,
            ),
        ];
        for case in &cases {
            let schedule = db(case.cost, case.salvage, case.life, case.factor, case.round);
            schedule.iter().for_each(|period| println!("{:?}", period));
            assert_eq!(schedule.len(), case.life as usize);
            assert_eq!(schedule.last().unwrap().depreciation_expense, case.expected);
        }
    }

    #[test]
    fn test_sln() {
        let cost = dec!(10_000);
        let salvage = dec!(1_000);
        let life = 5;
        let schedule = sln(cost, salvage, life);
        schedule.iter().for_each(|period| println!("{:?}", period));
        assert_eq!(schedule.len(), 5);
        assert_eq!(schedule[0].depreciation_expense, dec!(1800));
        assert_eq!(schedule[0].remaining_book_value, dec!(8200));
    }

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

    #[test]
    fn test_amort_schedule() {
        let rate = dec!(0.05) / dec!(12);
        let nper: u32 = 30 * 12;
        let principal = dec!(250_000);
        let pmt = crate::tvm::pmt(rate, Decimal::from_u32(nper).unwrap(), principal, None, None);
        println!("PMT: {}", pmt);

        let schedule = amort_schedule(rate, nper, principal, pmt, None);
        schedule.iter().for_each(|period| {
            println!("{:?}", period);
        });
        // Check the final balance is close to zero
        assert_eq!(schedule.last().unwrap().remaining_balance.abs() < dec!(1e-20), true);

        let schedule_round = amort_schedule(
            rate,
            nper,
            principal,
            pmt,
            Some((2, RoundingStrategy::MidpointNearestEven)),
        );
        schedule_round.iter().for_each(|period| {
            println!("{:?}", period);
        });
        // Check the final balance is Zero
        let sec_last_elem = schedule_round.get(358).unwrap();
        let last_elem = schedule_round.last().unwrap();
        assert_eq!(sec_last_elem.remaining_balance - last_elem.principal_payment, ZERO);
    }
}
