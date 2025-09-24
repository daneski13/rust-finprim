use crate::amort_dep_tax::AmortizationPeriod;
use crate::FloatLike;
use crate::RoundingMode;

#[cfg(feature = "std")]
/// Amortization Schedule
///
/// Calculates the amortization schedule for a loan or mortgage.
///
/// The amortization schedule includes a series of payments that are applied to both
/// principal and interest. Each payment reduces the principal balance and pays interest
/// charges based on the remaining balance and the interest rate.
///
///
/// # Feature
/// This function requires the `std` feature to be enabled as it uses `std::Vec`. `amort_schedule_into`
/// can be used in `no_std` environments as any allocation is done by the caller.
///
/// # Arguments
/// * `rate` - The interest rate per period
/// * `nper` - The total number of payment periods
/// * `principal` - The present value or principal amount of the loan (should be positive as cash inflow for a mortgage/loan)
/// * `pmt` - The payment amount per period (should be negative as cash outflow, can be calculated using `pmt` function)
/// * `round` (optional) - A tuple specifying the number of decimal places and a rounding
/// strategy for the amounts `(dp, RoundingMode)`, default is no rounding of calculations. The final principal
/// payment is adjusted to zero out the remaining balance if rounding is enabled.
///
/// # Returns
/// * A vector of `AmortizationPeriod` instances representing each period in the amortization schedule.
///
/// # Examples
/// * 5% rate, 30 year term (360 months), $1,000,000 loan, $4,000 monthly payment
/// ```
/// use rust_finprim::amort_dep_tax::amort_schedule;
/// use rust_finprim::tvm::pmt;
///
/// let rate = 0.05 / 12.0; // Monthly rate
/// let nper = 30 * 12;
/// let principal = 1_000_000.0;
/// let pmt = pmt(rate, nper.into(), principal, None, None);
///
/// let schedule = amort_schedule(rate, nper, principal, pmt, None);
/// ```
pub fn amort_schedule<T: FloatLike>(
    rate: T,
    nper: u32,
    principal: T,
    pmt: T,
    round: Option<(u32, RoundingMode, T)>,
) -> Vec<AmortizationPeriod<T>> {
    let mut periods = vec![AmortizationPeriod::default(); nper as usize];
    amort_schedule_into(periods.as_mut_slice(), rate, principal, pmt, round);
    periods
}

/// Amortization Schedule Into
///
/// Calculates the amortization schedule for a loan or mortgage, mutating a slice of `AmortizationPeriod`.
///
/// The amortization schedule includes a series of payments that are applied to both
/// principal and interest. Each payment reduces the principal balance and pays interest
/// charges based on the remaining balance and the interest rate.
///
///
/// # Arguments
/// * `slice` - A mutable slice of `AmortizationPeriod` instances to be filled with the amortization schedule.
///
/// **Warning**: The length of the slice should be as long as the number of periods (e.g.
/// 30 * 12 for 12 months over 30 years) or there will be unexpected behavior.
/// * `rate` - The interest rate per period
/// * `principal` - The present value or principal amount of the loan (should be positive as cash inflow for a mortgage/loan)
/// * `pmt` - The payment amount per period (should be negative as cash outflow, can be calculated using `pmt` function)
/// * `round` (optional) - A tuple specifying the number of decimal places and a rounding
/// strategy for the amounts `(dp, RoundingMode)`, default is no rounding of calculations. The final principal
/// payment is adjusted to zero out the remaining balance if rounding is enabled.
///
/// # Examples
/// * 5% rate, 30 year term (360 months), $1,000,000 loan, $4,000 monthly payment
/// ```
/// use rust_finprim::amort_dep_tax::{AmortizationPeriod, amort_schedule_into};
/// use rust_finprim::tvm::pmt;
///
/// let nper = 30 * 12;
/// let rate = 0.05 / 12.0;
/// let principal = 1_000_000.0;
/// let pmt = pmt(rate, nper.into(), principal, None, None);
///
/// let mut schedule = vec![AmortizationPeriod::default(); nper as usize];
/// amort_schedule_into(schedule.as_mut_slice(), rate, principal, pmt, None);
/// ```
///
/// If you wanted to add an "initial period" to the schedule efficiently, you could
/// do something like this:
/// ```
/// use rust_finprim::amort_dep_tax::{AmortizationPeriod, amort_schedule_into};
/// use rust_finprim::tvm::pmt;
///
/// let nper = 30 * 12;
/// let rate = 0.05 / 12.0;
/// let principal = 1_000_000.0;
/// let pmt = pmt(rate, nper.into(), principal, None, None);
///
/// let mut schedule = vec![AmortizationPeriod::default(); nper as usize + 1];
/// schedule[0] = AmortizationPeriod::new(0, 0.0, 0.0, principal);
/// amort_schedule_into(&mut schedule[1..], rate, principal, pmt, None);
pub fn amort_schedule_into<T: FloatLike>(
    slice: &mut [AmortizationPeriod<T>],
    rate: T,
    principal: T,
    pmt: T,
    round: Option<(u32, RoundingMode, T)>,
) {
    let pmt = if let Some((dp, rounding, epsilon)) = round {
        -pmt.round_with_mode(dp, rounding, epsilon)
    } else {
        -pmt
    };

    let mut remaining_balance = principal;
    for (period, item) in slice.iter_mut().enumerate() {
        let mut interest_payment = remaining_balance * rate;
        let mut principal_payment = pmt - interest_payment;

        if let Some((dp, rounding, epsilon)) = round {
            principal_payment = principal_payment.round_with_mode(dp, rounding, epsilon);
            interest_payment = interest_payment.round_with_mode(dp, rounding, epsilon);
        }

        remaining_balance -= principal_payment;

        *item = AmortizationPeriod::new(
            period as u32 + 1,
            principal_payment,
            interest_payment,
            remaining_balance,
        );
    }

    // Zero out the final balance when rounding is enabled
    // by subtracting the remaining balance from the final payment
    // (adding the remaining balance to the principal payment)
    if round.is_some() {
        let final_payment = slice.last_mut().unwrap();
        final_payment.principal_payment += final_payment.remaining_balance;
        final_payment.remaining_balance = T::zero();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(feature = "std"))]
    extern crate std;
    #[cfg(not(feature = "std"))]
    use std::{assert_eq, println};

    #[test]
    fn test_amort_schedule_into() {
        let rate = 0.05 / 12.0;
        const NPER: f64 = 30f64 * 12f64;
        let principal = 250_000.0;
        let pmt = crate::tvm::pmt(rate, NPER, principal, None, None);
        println!("PMT: {}", pmt);
        let mut schedule = [AmortizationPeriod::default(); NPER as usize];
        amort_schedule_into(&mut schedule, rate, principal, pmt, None);
        schedule.iter().for_each(|period| {
            println!("{:?}", period);
        });
        // Check the final balance is close to zero
        assert!(schedule.last().unwrap().remaining_balance.abs() < 1e-8);

        let mut schedule_round = [AmortizationPeriod::default(); NPER as usize];

        amort_schedule_into(
            &mut schedule_round,
            rate,
            principal,
            pmt,
            Some((2, RoundingMode::HalfToEven, 1e-8)),
        );
        schedule_round.iter().for_each(|period| {
            println!("{:?}", period);
        });
        // Check the final balance is Zero
        let sec_last_elem = schedule_round.get(358).unwrap();
        let last_elem = schedule_round.last().unwrap();
        assert_eq!(sec_last_elem.remaining_balance - last_elem.principal_payment, 0.0);
        assert_eq!(last_elem.remaining_balance, 0.0);
    }
}
