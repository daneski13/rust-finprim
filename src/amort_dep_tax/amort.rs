use crate::amort_dep_tax::AmortizationPeriod;
use crate::ZERO;
use rust_decimal::prelude::*;

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

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
/// use rust_finprim::amort_dep_tax::amort_schedule;
/// use rust_decimal_macros::dec;
/// use rust_finprim::tvm::pmt;
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

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    #[cfg(not(feature = "std"))]
    extern crate std;
    #[cfg(not(feature = "std"))]
    use std::prelude::v1::*;
    #[cfg(not(feature = "std"))]
    use std::{assert_eq, println};

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
