use crate::FloatLike;

/// PV'(r) - Derivative of the present value of a cash flow with respect to the rate.
///
/// # Arguments
/// * `rate` - The discount rate per period
/// * `n` - The nth period
/// * `cash_flow` - The cash flow at period n
///
/// # Returns
/// * The derivative of the present value (PV) with respect to the rate
///
/// Via the sum and difference property, this can be used for finding the derivative of
/// an NPV calculation with respect to the rate.
pub fn pv_prime_r<T: FloatLike>(rate: T, n: T, cash_flow: T) -> T {
    -cash_flow * n / (rate + T::one()).powf(n + T::one())
}

/// NPV'(r) - Derivative of the net present value with respect to the rate.
/// # Arguments
/// * `rate` - The discount rate per period
/// * `cash_flows` - A slice of cash flows, where each cash flow is at a specific period
///
/// # Returns
/// * The derivative of the net present value (NPV) with respect to the rate
///
/// This function calculates the derivative of NPV.
///
/// Optimizations over summing `pv_prime_r` for each cash flow individually.
pub fn npv_prime_r<T: FloatLike>(rate: T, cash_flows: &[T]) -> T {
    // Accumulator for the power of (1 + rate)
    // (1 + rate)^(1 + t) so first iteration is (1 + rate)^1 = (1 + rate)
    let mut powf_acc = T::one() + rate;
    let mut npv_prime = T::zero();
    for (t, &cf) in cash_flows.iter().enumerate() {
        npv_prime += -cf * T::from_usize(t) / powf_acc;
        powf_acc *= T::one() + rate;
    }
    npv_prime
}

/// PV''(r) - Second derivative of the present value of a cash flow with respect to the rate.
///
/// # Arguments
/// * `rate` - The discount rate per period
/// * `n` - The nth period
/// * `cash_flow` - The cash flow at period n
///
/// # Returns
/// * The second derivative of the present value (PV) with respect to the rate
///
/// Via the sum and difference property, this can be used for finding the 2nd derivative of
/// an NPV calculation with respect to the rate.
pub fn pv_prime2_r<T: FloatLike>(rate: T, n: T, cash_flow: T) -> T {
    cash_flow * n * (n + T::one()) / (rate + T::one()).powf(n + T::two())
}

/// NPV''(r) - Second derivative of the net present value with respect to the rate.
///
/// # Arguments
/// * `rate` - The discount rate per period
/// * `cash_flows` - A slice of cash flows, where each cash flow is at a specific period
///
/// # Returns
/// * The second derivative of the net present value (NPV) with respect to the rate
///
/// This function calculates the second derivative of NPV.
/// Optimizations over summing `pv_prime2_r` for each cash flow individually.
pub fn npv_prime2_r<T: FloatLike>(rate: T, cash_flows: &[T]) -> T {
    // Accumulator for the power of (1 + rate)
    // (1 + rate)^(2 + t) so first iteration is (1 + rate)^2 = (1 + rate) * (1 + rate)
    let mut powf_acc = (T::one() + rate) * (T::one() + rate);
    let mut npv_prime2 = T::zero();
    for (t, &cf) in cash_flows.iter().enumerate() {
        let t = T::from_usize(t);
        npv_prime2 += cf * t * (t + T::one()) / powf_acc;
        powf_acc *= T::one() + rate;
    }
    npv_prime2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(feature = "std"))]
    extern crate std;
    #[cfg(not(feature = "std"))]
    use std::{assert, vec};

    #[test]
    fn test_pv_prime() {
        let rate = 0.05;
        let n = 5.0;
        let cash_flow = 1000.0;

        let result = pv_prime_r(rate, n, cash_flow);
        let expected: f64 = -3731.07698;
        assert!(
            (result - expected).abs() < 1e-5,
            "Failed on case: {}. Expected: {}, Result: {}",
            "Rate of 5%, 5th period, cash flow of $1000",
            expected,
            result
        );
    }

    #[test]
    fn test_pv_double_prime() {
        let rate = 0.05;
        let n = 5.0;
        let cash_flow = 1000.0;

        let result = pv_prime2_r(rate, n, cash_flow);
        let expected: f64 = 21320.43990;
        assert!(
            (result - expected).abs() < 1e-5,
            "Failed on case: {}. Expected: {}, Result: {}",
            "Rate of 5%, 5th period, cash flow of $1000",
            expected,
            result
        );
    }

    #[test]
    fn test_npv_prime() {
        let rate = 0.05;
        let cash_flows = vec![1000.0, 2000.0, 3000.0];
        let expected = cash_flows
            .iter()
            .enumerate()
            .map(|(t, &cf)| pv_prime_r(rate, t as f64, cf))
            .sum::<f64>();
        let result = npv_prime_r(rate, &cash_flows);
        assert!(
            (result - expected).abs() < 1e-5,
            "Failed on case: {}. Expected: {}, Result: {}",
            "Rate of 5%, cash flows of $1000, $2000, $3000",
            expected,
            result
        );
    }

    #[test]
    fn test_npv_double_prime() {
        let rate = 0.05;
        let cash_flows = vec![1000.0, 2000.0, 3000.0];
        let expected = cash_flows
            .iter()
            .enumerate()
            .map(|(t, &cf)| pv_prime2_r(rate, t as f64, cf))
            .sum::<f64>();

        let result = npv_prime2_r(rate, &cash_flows);
        assert!(
            (result - expected).abs() < 1e-5,
            "Failed on case: {}. Expected: {}, Result: {}",
            "Rate of 5%, cash flows of $1000, $2000, $3000",
            expected,
            result
        );
    }
}
