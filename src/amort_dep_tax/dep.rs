use crate::amort_dep_tax::DepreciationPeriod;
use crate::ZERO;
use rust_decimal::prelude::*;

#[cfg(feature = "std")]
/// Straight Line Depreciation (SLN)
///
/// Calculates the depreciation schedule for an asset using the straight-line method.
///
/// # Feature
/// This function requires the `std` feature to be enabled as it uses the `std::Vec`. `sln_into`
/// can be used in a `no_std` environment as any allocation is done by the caller.
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
/// use rust_finprim::amort_dep_tax::sln;
/// use rust_decimal_macros::*;
///
/// let cost = dec!(10_000);
/// let salvage = dec!(1_000);
/// let life = 5;
/// let schedule = sln(cost, salvage, life);
/// ```
pub fn sln(cost: Decimal, salvage: Decimal, life: u32) -> Vec<DepreciationPeriod> {
    // let depreciation_expense = (cost - salvage) / Decimal::from_u32(life).unwrap();
    //
    // let mut periods = Vec::with_capacity(life as usize);
    // let mut remaining_book_value = cost;
    // for period in 1..=life {
    //     remaining_book_value -= depreciation_expense;
    //     periods.insert(
    //         period as usize - 1,
    //         DepreciationPeriod::new(period, depreciation_expense, remaining_book_value),
    //     );
    // }
    let mut periods = vec![DepreciationPeriod::default(); life as usize];
    sln_into(periods.as_mut_slice(), cost, salvage);
    periods
}

/// Straight Line Depreciation (SLN) Into
///
/// Calculates the depreciation schedule for an asset using the straight-line method, mutating a
/// "slice" of `DepreciationPeriod`.
///
/// # Arguments
/// * `slice` - A mutable slice of `DepreciationPeriod` instances to be filled with the depreciation schedule.
///
/// **Warning**: The length of the slice should be as long as the life as the asset or there will
/// be unexpected behavior.
/// * `cost` - The initial cost of the asset
/// * `salvage` - The estimated salvage value of the asset at the end of its useful life
///
/// # Examples
/// * $10,000 asset, $1,000 salvage value, 5 year life
/// ```
/// use rust_finprim::amort_dep_tax::{DepreciationPeriod, sln_into};
/// use rust_decimal_macros::*;
///
/// let life = 5;
/// let cost = dec!(10_000);
/// let salvage = dec!(1_000);
///
/// let mut schedule = vec![DepreciationPeriod::default(); life as usize];
/// sln_into(&mut schedule, cost, salvage);
/// ```
pub fn sln_into(slice: &mut [DepreciationPeriod], cost: Decimal, salvage: Decimal) {
    let life = slice.len() as u32;
    let depreciation_expense = (cost - salvage) / Decimal::from_u32(life).unwrap();

    let mut remaining_book_value = cost;
    for (period, item) in slice.iter_mut().enumerate() {
        remaining_book_value -= depreciation_expense;
        item.period = period as u32 + 1;
        item.depreciation_expense = depreciation_expense;
        item.remaining_book_value = remaining_book_value;
    }
}

#[cfg(feature = "std")]
/// Declining Balance Depreciation (DB)
///
/// Calculates the depreciation schedule for an asset using the declining balance method given a
/// declining balance factor (e.g. double-declining balance).
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
/// use rust_finprim::amort_dep_tax::db;
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
    let mut periods = vec![DepreciationPeriod::default(); life as usize];
    db_into(periods.as_mut_slice(), cost, salvage, factor, round);
    periods
}

/// Declining Balance Depreciation (DB) Into
///
/// Calculates the depreciation schedule for an asset using the declining balance method given a
/// declining balance factor (e.g. double-declining balance), mutating a "slice" of DepreciationPeriod.
///
/// # Arguments
/// * `slice` - A mutable slice of `DepreciationPeriod` instances to be filled with the depreciation schedule.
///
/// **Warning**: The length of the slice should be as long as the life as the asset or there will
/// be unexpected behavior.
/// * `cost` - The initial cost of the assert
/// * `salvage` - The estimated salvage value of the asset at the end of its useful life
/// * `factor` (optional) - The factor by which the straight-line depreciation rate is multiplied (default is 2 for double-declining balance)
/// * `round` (optional) - A tuple specifying the number of decimal places and a rounding strategy for the amounts `(dp, RoundingStrategy)`,
/// default is no rounding of calculations. The final depreciation expense is adjusted to ensure the remaining book value is equal to the salvage value.
/// `rust_decimal::RoundingStrategy::MidpointNearestEven` ("Bankers Rounding") is likely what you are looking for as the rounding strategy.
///
/// If rounding is enabled, the final period will be adjusted to "zero" out the remaining book
/// value to the salvage value.
///
/// # Examples
/// * $10,000 asset, $1,000 salvage value, 5 year life
/// ```
/// use rust_finprim::amort_dep_tax::{DepreciationPeriod, db_into};
/// use rust_decimal_macros::*;
///
/// let life = 5;
/// let cost = dec!(10_000);
/// let salvage = dec!(1_000);
///
/// let mut schedule = vec![DepreciationPeriod::default(); life as usize];
/// db_into(&mut schedule, cost, salvage, None, None);
/// ```
pub fn db_into(
    slice: &mut [DepreciationPeriod],
    cost: Decimal,
    salvage: Decimal,
    factor: Option<Decimal>,
    round: Option<(u32, RoundingStrategy)>,
) {
    let factor = factor.unwrap_or(Decimal::TWO);
    let life = slice.len() as u32;

    let mut remain_bv = cost;
    let mut accum_dep = ZERO;
    for (period, item) in slice.iter_mut().enumerate() {
        let mut dep_exp = factor * (cost - accum_dep) / Decimal::from_u32(life).unwrap();
        if let Some((dp, rounding)) = round {
            dep_exp = dep_exp.round_dp_with_strategy(dp, rounding);
        }

        if dep_exp > remain_bv - salvage {
            dep_exp = remain_bv - salvage;
        }
        accum_dep += dep_exp;
        remain_bv -= dep_exp;

        item.period = period as u32 + 1;
        item.depreciation_expense = dep_exp;
        item.remaining_book_value = remain_bv;
    }

    if round.is_some() {
        let last = slice.last_mut().unwrap();
        last.depreciation_expense += last.remaining_book_value - salvage;
        last.remaining_book_value = salvage;
    }
}

#[cfg(feature = "std")]
/// Sum of the Years Digits (SYD)
///
/// # Feature
/// This function requires the `std` feature to be enabled as it uses the `std::Vec`. `syd_into`
/// can be used in a `no_std` environment as any allocation is done by the caller.
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
/// use rust_finprim::amort_dep_tax::syd;
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
    let mut periods = vec![DepreciationPeriod::default(); life as usize];
    syd_into(periods.as_mut_slice(), cost, salvage, round);
    periods
}

/// Sum of the Years Digits (SYD) Into
///
/// Calculates the depreciation schedule for an asset using the sum of the years' digits method.
/// The sum of the years' digits method is an accelerated depreciation method that allocates
/// more depreciation expense to the early years of an asset's life. Mutates a slice of
/// `DepreciationPeriod`.
///
/// # Arguments
/// * `slice` - A mutable slice of `DepreciationPeriod` instances to be filled with the depreciation schedule.
///
/// **Warning**: The length of the slice should be as long as the life as the asset or there will
/// be unexpected behavior.
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
/// use rust_finprim::amort_dep_tax::{DepreciationPeriod, syd_into};
/// use rust_decimal_macros::*;
///
/// let life = 5;
/// let cost = dec!(10_000);
/// let salvage = dec!(1_000);
///
/// let mut schedule = vec![DepreciationPeriod::default(); life as usize];
/// syd_into(&mut schedule, cost, salvage, None);
/// ```
pub fn syd_into(
    slice: &mut [DepreciationPeriod],
    cost: Decimal,
    salvage: Decimal,
    round: Option<(u32, RoundingStrategy)>,
) {
    let life = slice.len() as u32;
    let mut remain_bv = cost;
    let mut accum_dep = ZERO;
    let sum_of_years = Decimal::from_u32(life * (life + 1)).unwrap() / Decimal::TWO;
    for (period, item) in slice.iter_mut().enumerate() {
        let mut dep_exp = (cost - salvage) * Decimal::from_u32(life - (period as u32)).unwrap() / sum_of_years;
        if let Some((dp, rounding)) = round {
            dep_exp = dep_exp.round_dp_with_strategy(dp, rounding)
        };

        accum_dep += dep_exp;
        remain_bv -= dep_exp;

        item.period = period as u32 + 1;
        item.depreciation_expense = dep_exp;
        item.remaining_book_value = remain_bv;
    }

    if round.is_some() {
        let last = slice.last_mut().unwrap();
        last.depreciation_expense += last.remaining_book_value - salvage;
        last.remaining_book_value = salvage;
    }
}

#[cfg(feature = "std")]
/// MACRS Deprectiation
///
/// Calculates the depreciation schedule for an asset using the Modified Accelerated Cost Recovery
/// System (MACRS method). MACRS is a depreciation method allowed by the IRS for tax purposes.
///
/// # Arguments
/// * `cost` - The initial cost of the asset
/// * `rates` - A slice representing the MACRS depreciation rates for all periods of the asset's
/// life, starting with the first year (period 1) and ending with the last year (period 2). Rates
/// for each period can be found in IRS Publication 946 or other tax resources. The rates should
/// be in decimal form (e.g., 0.20 for 20%).
///
/// # Returns
/// * A vector of `DepreciationPeriod` instances representing each period in the depreciation schedule.
/// The length of the vector will be equal to the number of rates provided.
///
/// # Examples
/// * $10,000 asset, MACRS rates for 5 year life
/// ```
/// use rust_finprim::amort_dep_tax::macrs;
/// use rust_decimal_macros::*;
/// use rust_decimal::Decimal;
///
/// let cost = dec!(10_000);
/// let rates = vec![
///    dec!(0.20),
///    dec!(0.32),
///    dec!(0.1920),
///    dec!(0.1152),
///    dec!(0.1152),
///    dec!(0.0576)
/// ];
/// let schedule = macrs(cost, &rates);
/// ```
pub fn macrs(cost: Decimal, rates: &[Decimal]) -> Vec<DepreciationPeriod> {
    let mut periods = vec![DepreciationPeriod::default(); rates.len()];
    macrs_into(periods.as_mut_slice(), cost, rates);
    periods
}

/// MACRS Deprectiation Into
///
/// Calculates the depreciation schedule for an asset using the Modified Accelerated Cost Recovery
/// System (MACRS method). MACRS is a depreciation method allowed by the IRS for tax purposes.
/// Mutates a slice of `DepreciationPeriod`.
///
/// # Arguments
/// * `slice` - A mutable slice of `DepreciationPeriod` instances to be filled with the depreciation schedule.
///
/// **Warning**: The length of the slice should be as long as the life as the asset, in this case,
/// that is as long as the number of rates provided. If the length of the slice is not equal to
/// the number of rates, this will panic.
/// * `cost` - The initial cost of the asset
/// * `rates` - A slice representing the MACRS depreciation rates for all periods of the asset's
/// life, starting with the first year (period 1) and ending with the last year (period 2). Rates
/// for each period can be found in IRS Publication 946 or other tax resources. The rates should
/// be in decimal form (e.g., 0.20 for 20%).
///
/// # Returns
/// * A vector of `DepreciationPeriod` instances representing each period in the depreciation schedule.
/// The length of the vector will be equal to the number of rates provided.
///
/// # Examples
/// * $10,000 asset, MACRS rates for 5 year life
/// ```
/// use rust_finprim::amort_dep_tax::{DepreciationPeriod, macrs_into};
/// use rust_decimal_macros::*;
/// use rust_decimal::Decimal;
///
/// let cost = dec!(10_000);
/// let rates = vec![
///    dec!(0.20),
///    dec!(0.32),
///    dec!(0.1920),
///    dec!(0.1152),
///    dec!(0.1152),
///    dec!(0.0576)
/// ];
/// let life = rates.len() as u32;
/// let mut schedule = vec![DepreciationPeriod::default(); life as usize];
/// macrs_into(&mut schedule, cost, &rates);
/// ```
pub fn macrs_into(slice: &mut [DepreciationPeriod], cost: Decimal, rates: &[Decimal]) {
    if slice.len() != rates.len() {
        panic!("Length of slice must be equal to the number of rates");
    }
    let mut remain_bv = cost;
    for (period, &rate) in rates.iter().enumerate() {
        let dep_exp = cost * rate;
        remain_bv -= dep_exp;
        let item = &mut slice[period];
        item.period = period as u32 + 1;
        item.depreciation_expense = dep_exp;
        item.remaining_book_value = remain_bv;
    }
}

// TODO: Add tests for no_std environments, but if they pass in std, they should pass in no_std
// since the underlying logic is the same. Just the allocation is different.
#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    #[cfg(not(feature = "std"))]
    extern crate std;
    #[cfg(not(feature = "std"))]
    use std::prelude::v1::*;
    #[cfg(not(feature = "std"))]
    use std::{assert_eq, println, vec};

    #[test]
    fn test_macrs() {
        let cost = dec!(10_000);
        let rates = vec![
            dec!(0.20),
            dec!(0.32),
            dec!(0.1920),
            dec!(0.1152),
            dec!(0.1152),
            dec!(0.0576),
        ];
        const LIFE: u32 = 6;
        let mut schedule: [DepreciationPeriod; LIFE as usize] = [DepreciationPeriod::default(); LIFE as usize];
        macrs_into(&mut schedule, cost, &rates);
        schedule.iter().for_each(|period| println!("{:?}", period));
        assert_eq!(schedule.len(), rates.len());
        assert_eq!(schedule[0].depreciation_expense, dec!(2000));
        assert_eq!(schedule[0].remaining_book_value, dec!(8000));
        assert_eq!(schedule[5].depreciation_expense, dec!(576));
        assert_eq!(schedule[5].remaining_book_value, dec!(0));
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
}
