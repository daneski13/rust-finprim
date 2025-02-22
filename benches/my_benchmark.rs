use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use rust_finprim::amort_dep_tax::AmortizationPeriod;
use rust_finprim::rate::{mirr, xmirr};
use rust_finprim::tvm::*;

const ZERO: Decimal = Decimal::ZERO;
const ONE: Decimal = Decimal::ONE;

pub fn amort_schedule_basic(
    rate: Decimal,
    nper: u32,
    pv: Decimal,
    pmt: Decimal,
    round: Option<(u32, RoundingStrategy)>,
) -> Vec<AmortizationPeriod> {
    // Allocate the memory upfront for the vector
    let mut periods = Vec::with_capacity(nper as usize);

    let pmt = if let Some((dp, rounding)) = round {
        pmt.round_dp_with_strategy(dp, rounding)
    } else {
        pmt
    };

    let mut remaining_balance = pv;
    for period in 1..=nper {
        let mut interest_payment = remaining_balance * rate;
        let mut principal_payment = pmt - interest_payment;

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

pub fn amort_schedule_iter(
    rate: Decimal,
    nper: u32,
    pv: Decimal,
    pmt: Decimal,
    round: Option<(u32, RoundingStrategy)>,
) -> Vec<AmortizationPeriod> {
    let pmt = if let Some((dp, rounding)) = round {
        pmt.round_dp_with_strategy(dp, rounding)
    } else {
        pmt
    };

    let mut periods = (1..=nper)
        .scan(pv, |remaining_balance, period| {
            let mut interest_payment = *remaining_balance * rate;
            let mut principal_payment = pmt - interest_payment;

            if let Some((dp, rounding)) = round {
                principal_payment = principal_payment.round_dp_with_strategy(dp, rounding);
                interest_payment = interest_payment.round_dp_with_strategy(dp, rounding);
            }

            *remaining_balance -= principal_payment;

            Some(AmortizationPeriod::new(
                period,
                principal_payment,
                interest_payment,
                *remaining_balance,
            ))
        })
        .collect::<Vec<_>>();

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

pub fn progressive_tax_for(agi: Decimal, deductions: Decimal, rate_table: &[(Decimal, Decimal)]) -> Option<Decimal> {
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

pub fn progressive_tax_fold(agi: Decimal, deductions: Decimal, rate_table: &[(Decimal, Decimal)]) -> Option<Decimal> {
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

    let tax = rate_table
        .iter()
        .fold((ZERO, ZERO), |(total_tax, prev_bracket), &(bracket, rate)| {
            // Stop calculation if the previous bracket was greater than the taxable income
            if prev_bracket > taxable_income {
                return (total_tax, prev_bracket);
            }

            let bracket_tax = if taxable_income > bracket {
                (bracket - prev_bracket) * rate
            } else {
                (taxable_income - prev_bracket) * rate
            };
            (total_tax + bracket_tax, bracket)
        })
        .0; // Extract the total_tax part of the tuple as the result

    Some(tax)
}

pub fn mirr_old(cash_flows: &[Decimal], finance_rate: Decimal, reinvest_rate: Decimal) -> Decimal {
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

pub fn xmrr_old(flow_table: &[(Decimal, i32)], finance_rate: Decimal, reinvest_rate: Decimal) -> Decimal {
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

fn criterion_benchmark(c: &mut Criterion) {
    let rate = black_box(Decimal::from_f64(0.05 / 12.0).unwrap());
    let nper: u32 = black_box(30 * 12);
    let loan = black_box(dec!(250_000));
    let pmt = pmt(rate, Decimal::from_u32(nper).unwrap(), loan, None, None);

    let mut group = c.benchmark_group("Amort");
    group.bench_with_input("Amort Basic", &(rate, nper, loan, pmt, None), |b, args| {
        b.iter(|| amort_schedule_basic(args.0, args.1, args.2, args.3, args.4))
    });
    group.bench_with_input("Amort Iter", &(rate, nper, loan, pmt, None), |b, args| {
        b.iter(|| amort_schedule_iter(args.0, args.1, args.2, args.3, args.4))
    });
    group.finish();

    let mut group = c.benchmark_group("Progressive Tax");
    let agi = black_box(dec!(100_000));
    let deductions = black_box(dec!(12_000));
    let rate_table = &[
        (dec!(9_875), dec!(0.10)),
        (dec!(40_125), dec!(0.12)),
        (dec!(85_525), dec!(0.22)),
        (dec!(163_300), dec!(0.24)),
        (dec!(207_350), dec!(0.32)),
        (dec!(518_400), dec!(0.35)),
        (dec!(1_000_000), dec!(0.37)),
    ];
    group.bench_with_input("Progressive Tax For", &(agi, deductions, rate_table), |b, args| {
        b.iter(|| progressive_tax_for(args.0, args.1, args.2))
    });
    group.bench_with_input("Progressive Tax Fold", &(agi, deductions, rate_table), |b, args| {
        b.iter(|| progressive_tax_fold(args.0, args.1, args.2))
    });
    group.finish();

    let mut group = c.benchmark_group("MIRR");
    let cash_flows = black_box(vec![dec!(-100), dec!(50), dec!(40), dec!(30), dec!(20)]);
    let finance_rate = black_box(dec!(0.1));
    let reinvest_rate = black_box(dec!(0.05));
    group.bench_with_input(
        "MIRR Old",
        &(cash_flows.as_slice(), finance_rate, reinvest_rate),
        |b, args| b.iter(|| mirr_old(args.0, args.1, args.2)),
    );
    group.bench_with_input(
        "MIRR",
        &(cash_flows.as_slice(), finance_rate, reinvest_rate),
        |b, args| b.iter(|| mirr(args.0, args.1, args.2)),
    );
    let cash_flows_xirr = black_box(vec![
        (dec!(-100), 0),
        (dec!(50), 359),
        (dec!(40), 400),
        (dec!(30), 1000),
        (dec!(20), 2000),
    ]);
    group.bench_with_input(
        "X MIRR Old",
        &(cash_flows_xirr.as_slice(), finance_rate, reinvest_rate),
        |b, args| b.iter(|| xmrr_old(args.0, args.1, args.2)),
    );
    group.bench_with_input(
        "X MIRR",
        &(cash_flows_xirr.as_slice(), finance_rate, reinvest_rate),
        |b, args| b.iter(|| xmirr(args.0, args.1, args.2)),
    );
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
