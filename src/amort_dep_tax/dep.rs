use crate::amort_dep_tax::DepreciationPeriod;
use crate::FloatLike;
use crate::RoundingMode;

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
///
/// let cost = 10_000.0;
/// let salvage = 1_000.0;
/// let life = 5;
/// let schedule = sln(cost, salvage, life);
/// ```
pub fn sln<T: FloatLike>(cost: T, salvage: T, life: u32) -> Vec<DepreciationPeriod<T>> {
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
///
/// let life = 5;
/// let cost = 10_000.0;
/// let salvage = 1_000.0;
///
/// let mut schedule = vec![DepreciationPeriod::default(); life as usize];
/// sln_into(&mut schedule, cost, salvage);
/// ```
pub fn sln_into<T: FloatLike>(slice: &mut [DepreciationPeriod<T>], cost: T, salvage: T) {
    let life = slice.len();
    let depreciation_expense = (cost - salvage) / T::from_usize(life);

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
/// # Feature
/// This function requires the `std` feature to be enabled as it uses the `std::Vec`. `sln_into`
/// can be used in a `no_std` environment as any allocation is done by the caller.
///
/// # Arguments
/// * `cost` - The initial cost of the assert
/// * `salvage` - The estimated salvage value of the asset at the end of its useful life
/// * `life` - The number of periods over which the asset will be depreciated
/// * `factor` (optional) - The factor by which the straight-line depreciation rate is multiplied (default is 2 for double-declining balance)
/// * `round` (optional) - A tuple specifying the number of decimal places and a rounding strategy for the amounts `(dp, RoundingMode)`,
/// default is no rounding of calculations. The final depreciation expense is adjusted to ensure the remaining book value is equal to the salvage value.
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
///
/// let cost = 10_000.0;
/// let salvage = 1_000.0;
/// let life = 5;
/// let schedule = db(cost, salvage, life, None, None);
/// ```
pub fn db<T: FloatLike>(
    cost: T,
    salvage: T,
    life: u32,
    factor: Option<T>,
    round: Option<(u32, RoundingMode, T)>,
) -> Vec<DepreciationPeriod<T>> {
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
/// * `round` (optional) - A tuple specifying the number of decimal places and a rounding strategy for the amounts `(dp, RoundingMode)`,
/// default is no rounding of calculations. The final depreciation expense is adjusted to ensure the remaining book value is equal to the salvage value.
///
/// If rounding is enabled, the final period will be adjusted to "zero" out the remaining book
/// value to the salvage value.
///
/// # Examples
/// * $10,000 asset, $1,000 salvage value, 5 year life
/// ```
/// use rust_finprim::amort_dep_tax::{DepreciationPeriod, db_into};
///
/// let life = 5;
/// let cost = 10_000.0;
/// let salvage = 1_000.0;
///
/// let mut schedule = vec![DepreciationPeriod::default(); life as usize];
/// db_into(&mut schedule, cost, salvage, None, None);
/// ```
pub fn db_into<T: FloatLike>(
    slice: &mut [DepreciationPeriod<T>],
    cost: T,
    salvage: T,
    factor: Option<T>,
    round: Option<(u32, RoundingMode, T)>,
) {
    let factor = factor.unwrap_or(T::two());
    let life = slice.len();

    let mut remain_bv = cost;
    let mut accum_dep = T::zero();
    for (period, item) in slice.iter_mut().enumerate() {
        let mut dep_exp = factor * (cost - accum_dep) / T::from_usize(life);
        if let Some((dp, rounding, epsilon)) = round {
            dep_exp = dep_exp.round_with_mode(dp, rounding, epsilon);
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
/// Calculates the depreciation schedule for an asset using the sum of the years' digits method.
/// The sum of the years' digits method is an accelerated depreciation method that allocates
/// more depreciation expense to the early years of an asset's life.
///
/// # Feature
/// This function requires the `std` feature to be enabled as it uses the `std::Vec`. `syd_into`
/// can be used in a `no_std` environment as any allocation is done by the caller.
///
/// # Arguments
/// * `cost` - The initial cost of the asset
/// * `salvage` - The estimated salvage value of the asset at the end of its useful life
/// * `life` - The number of periods over which the asset will be depreciated
/// * `round` (optional) - A tuple specifying the number of decimal places and a rounding strategy for the amounts `(dp, RoundingMode)`,
/// default is no rounding of calculations. The final depreciation expense is adjusted to ensure the remaining book value is equal to the salvage value.
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
///
/// let cost = 10_000.0;
/// let salvage = 1_000.0;
/// let life = 5;
/// let schedule = syd(cost, salvage, life, None);
/// ```
pub fn syd<T: FloatLike>(
    cost: T,
    salvage: T,
    life: u32,
    round: Option<(u32, RoundingMode, T)>,
) -> Vec<DepreciationPeriod<T>> {
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
/// * `round` (optional) - A tuple specifying the number of decimal places and a rounding strategy for the amounts `(dp, RoundingMode)`,
/// default is no rounding of calculations. The final depreciation expense is adjusted to ensure the remaining book value is equal to the salvage value.
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
///
/// let life = 5;
/// let cost = 10_000.0;
/// let salvage = 1_000.0;
///
/// let mut schedule = vec![DepreciationPeriod::default(); life as usize];
/// syd_into(&mut schedule, cost, salvage, None);
/// ```
pub fn syd_into<T: FloatLike>(
    slice: &mut [DepreciationPeriod<T>],
    cost: T,
    salvage: T,
    round: Option<(u32, RoundingMode, T)>,
) {
    let life = slice.len();
    let mut remain_bv = cost;
    let mut accum_dep = T::zero();
    let sum_of_years = T::from_usize(life * (life + 1)) / T::two();
    for (period, item) in slice.iter_mut().enumerate() {
        let mut dep_exp = (cost - salvage) * T::from_usize(life - (period)) / sum_of_years;
        if let Some((dp, rounding, epsilon)) = round {
            dep_exp = dep_exp.round_with_mode(dp, rounding, epsilon)
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
/// # Feature
/// This function requires the `std` feature to be enabled as it uses the `std::Vec`. `sln_into`
/// can be used in a `no_std` environment as any allocation is done by the caller.
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
///
/// let cost = 10_000.0;
/// let rates = vec![
///    0.20,
///    0.32,
///    0.1920,
///    0.1152,
///    0.1152,
///    0.0576
/// ];
/// let schedule = macrs(cost, &rates);
/// ```
pub fn macrs<T: FloatLike>(cost: T, rates: &[T]) -> Vec<DepreciationPeriod<T>> {
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
///
/// let cost = 10_000.0;
/// let rates = vec![
///    0.20,
///    0.32,
///    0.1920,
///    0.1152,
///    0.1152,
///    0.0576
/// ];
/// let life = rates.len() as u32;
/// let mut schedule = vec![DepreciationPeriod::default(); life as usize];
/// macrs_into(&mut schedule, cost, &rates);
/// ```
pub fn macrs_into<T: FloatLike>(slice: &mut [DepreciationPeriod<T>], cost: T, rates: &[T]) {
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

// since the underlying logic is the same. Just the allocation is different.
#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;

    #[cfg(not(feature = "std"))]
    extern crate std;
    #[cfg(not(feature = "std"))]
    use std::{assert_eq, println, vec};

    #[test]
    fn test_macrs() {
        let cost = 10_000.0;
        let rates = vec![0.20, 0.32, 0.1920, 0.1152, 0.1152, 0.0576];
        const LIFE: usize = 6;
        let mut schedule: [DepreciationPeriod<f64>; LIFE] = [DepreciationPeriod::default(); LIFE];
        macrs_into(&mut schedule, cost, &rates);
        schedule.iter().for_each(|period| println!("{:?}", period));
        assert_eq!(schedule.len(), rates.len());
        assert_eq!(schedule[0].depreciation_expense, 2000.0);
        assert_eq!(schedule[0].remaining_book_value, 8000.0);
        assert_eq!(schedule[5].depreciation_expense, 576.0);
        assert_eq!(schedule[5].remaining_book_value, 0.0);
    }

    #[test]
    fn test_syd() {
        struct TestCase {
            cost: f64,
            salvage: f64,
            life: u32,
            round: Option<(u32, RoundingMode, f64)>,
            expected: f64,
        }

        impl TestCase {
            fn new(cost: f64, salvage: f64, life: u32, round: Option<(u32, RoundingMode)>, expected: f64) -> Self {
                Self {
                    cost,
                    salvage,
                    life,
                    round: round.map(|(dp, mode)| (dp, mode, 1e-5)),
                    expected,
                }
            }
        }

        let cases = [
            TestCase::new(10_000.00, 1_000.00, 5, None, 600.00),
            TestCase::new(9_000.00, 1_000.00, 5, Some((2, RoundingMode::HalfToEven)), 533.33),
            TestCase::new(9_000.00, 1_500.00, 10, Some((2, RoundingMode::HalfToEven)), 136.36),
        ];
        for case in &cases {
            let schedule = syd(case.cost, case.salvage, case.life, case.round);
            schedule.iter().for_each(|period| println!("{:?}", period));
            assert_eq!(schedule.len(), case.life as usize);
            assert!((schedule.last().unwrap().depreciation_expense - case.expected).abs() < 1e-5);
        }
    }

    #[test]
    fn test_db() {
        struct TestCase {
            cost: f64,
            salvage: f64,
            life: u32,
            factor: Option<f64>,
            round: Option<(u32, RoundingMode, f64)>,
            expected: f64,
        }
        impl TestCase {
            fn new(
                cost: f64,
                salvage: f64,
                life: u32,
                factor: Option<f64>,
                round: Option<(u32, RoundingMode)>,
                expected: f64,
            ) -> Self {
                Self {
                    cost,
                    salvage,
                    life,
                    factor,
                    round: round.map(|(dp, mode)| (dp, mode, 1e-5)),
                    expected,
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
                Some((2, RoundingMode::HalfToEven)),
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
        let cost = 10_000.0;
        let salvage = 1_000.0;
        let life = 5;
        let schedule = sln(cost, salvage, life);
        schedule.iter().for_each(|period| println!("{:?}", period));
        assert_eq!(schedule.len(), 5);
        assert_eq!(schedule[0].depreciation_expense, 1800.0);
        assert_eq!(schedule[0].remaining_book_value, 8200.0);
    }
}
