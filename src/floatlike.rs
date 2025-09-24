use crate::RoundingMode;

#[cfg(not(feature = "std"))]
use libm;

#[cfg(feature = "rust_decimal")]
use rust_decimal::prelude::FromPrimitive;
#[cfg(feature = "rust_decimal")]
use rust_decimal::{Decimal, MathematicalOps};

/// The FloatLike trait is designed to abstract over any fractional numeric type. By default,
/// it supports f32, f64, and `rust_decimal`'s Decimal. This allows for the generic implementation of
/// the mathematical operations supported by this library.
pub trait FloatLike:
    Copy
    + core::fmt::Display
    + core::fmt::Debug
    + PartialOrd
    + PartialEq
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::Mul<Output = Self>
    + core::ops::Div<Output = Self>
    + core::ops::Neg<Output = Self>
    + core::iter::Sum<Self>
    + core::iter::Product<Self>
    + core::ops::SubAssign
    + core::ops::AddAssign
    + core::ops::MulAssign
{
    const MAX: Self;
    fn zero() -> Self;
    fn one() -> Self;
    fn powf(self, n: Self) -> Self;
    fn two() -> Self {
        Self::one() + Self::one()
    }
    fn abs(&self) -> Self;
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn from_u16(n: u16) -> Self;
    fn from_usize(n: usize) -> Self;
    fn from_i32(n: i32) -> Self;
    fn from_f32(n: f32) -> Self;

    fn round_with_mode(&self, dp: u32, mode: RoundingMode, epsilon: Self) -> Self;
}

// This macro generates implementations of the FloatLike trait for f32 and f64.
#[crabtime::function]
fn gen_floats(ftype: Vec<String>) {
    for t in ftype.iter() {
        let append = if *t == "f32" { "f" } else { "" };
        crabtime::output! {
            impl FloatLike for {{t}} {
                const MAX: Self = {{t}}::MAX;
                fn zero() -> Self {
                    0.0
                }
                fn one() -> Self {
                    1.0
                }
                fn powf(self, n: Self) -> Self {
                    #[cfg(feature = "std")]
                    { {{t}}::powf(self, n) }
                    #[cfg(not(feature = "std"))]
                    { libm::pow{{append}}(self, n) }
                }
                fn abs(&self) -> Self {
                    #[cfg(feature = "std")]
                    { {{t}}::abs(*self) }
                    #[cfg(not(feature = "std"))]
                    { libm::fabs{{append}}(*self) }
                }
                fn min(self, other: Self) -> Self {
                    {{t}}::min(self, other)
                }
                fn max(self, other: Self) -> Self {
                    {{t}}::max(self, other)
                }
                fn from_u16(n: u16) -> Self {
                    n as {{t}}
                }
                fn from_usize(n: usize) -> Self {
                    n as {{t}}
                }
                fn from_i32(n: i32) -> Self {
                    n as {{t}}
                }
                fn from_f32(n: f32) -> Self {
                    n as {{t}}
                }
                fn round_with_mode(&self, dp: u32, mode: RoundingMode, epsilon: Self) -> Self {
                    let factor = FloatLike::powf(10.0{{t}}, dp as {{t}});
                    let shifted = *self * factor;

                    #[cfg(feature = "std")]
                    let floor = shifted.floor();
                    #[cfg(not(feature = "std"))]
                    let floor = libm::floor{{append}}(shifted);

                    // Shifted rounded
                    #[cfg(feature = "std")]
                    let shifted_rd = shifted.round();
                    #[cfg(not(feature = "std"))]
                    let shifted_rd = libm::round{{append}}(shifted);

                    let diff_floor = shifted - floor;

                    #[cfg(feature = "std")]
                    let ceil = shifted.ceil();
                    #[cfg(not(feature = "std"))]
                    let ceil = libm::ceil{{append}}(shifted);

                    let rounded = match mode {
                        RoundingMode::HalfToEven =>
                        // Case 1: we aren't at the half point, return normal "rounding"
                        {
                            if (diff_floor.abs() - 0.5).abs() > epsilon {
                                shifted_rd
                            } else {
                                // Case 2: we are at the half point, round to the nearest even number
                                if floor as i64 % 2 == 0 {
                                    floor
                                } else {
                                    ceil
                                }
                            }
                        }
                        RoundingMode::HalfTowardZero => {
                            // Case 1: we aren't at the half point, return normal "rounding"
                            if (diff_floor.abs() - 0.5).abs() > epsilon {
                                shifted_rd
                            } else {
                                // Case 2: we are at the half point, round toward zero
                                if shifted > 0.0 {
                                    floor
                                } else {
                                    ceil
                                }
                            }
                        }
                        RoundingMode::HalfAwayFromZero => {
                            // Case 1: we aren't at the half point, return normal "rounding"
                            if (diff_floor.abs() - 0.5).abs() > epsilon {
                                shifted_rd
                            } else {
                                // Case 2: we are at the half point, round away from zero
                                if shifted > 0.0 {
                                    ceil
                                } else {
                                    floor
                                }
                            }
                        }
                        RoundingMode::TowardZero => {
                            // Round down
                            if shifted > 0.0 {
                                floor
                            } else {
                                ceil
                            }
                        }
                        RoundingMode::AwayFromZero => {
                            // Round up
                            if shifted > 0.0 {
                                ceil
                            } else {
                                floor
                            }
                        }
                        // Round down for positive numbers, round up for negative numbers
                        RoundingMode::ToNegativeInfinity => floor,
                        // Round up for positive numbers, round down for negative numbers
                        RoundingMode::ToInfinity => ceil
                    };
                    rounded / factor
                }
            }
        }
    }
}
gen_floats!(["f32", "f64"]);

#[cfg(feature = "rust_decimal")]
impl FloatLike for Decimal {
    const MAX: Self = Decimal::MAX;
    fn zero() -> Self {
        Decimal::ZERO
    }

    fn one() -> Self {
        Decimal::ONE
    }
    fn powf(self, n: Self) -> Self {
        self.powd(n)
    }
    fn abs(&self) -> Self {
        Decimal::abs(self)
    }
    fn round_with_mode(&self, dp: u32, mode: RoundingMode, epsilon: Self) -> Self {
        let rounding_mode = match mode {
            RoundingMode::HalfToEven => rust_decimal::RoundingStrategy::MidpointNearestEven,
            RoundingMode::HalfAwayFromZero => rust_decimal::RoundingStrategy::MidpointAwayFromZero,
            RoundingMode::HalfTowardZero => rust_decimal::RoundingStrategy::MidpointTowardZero,
            RoundingMode::TowardZero => rust_decimal::RoundingStrategy::ToZero,
            RoundingMode::AwayFromZero => rust_decimal::RoundingStrategy::AwayFromZero,
            RoundingMode::ToNegativeInfinity => rust_decimal::RoundingStrategy::ToNegativeInfinity,
            RoundingMode::ToInfinity => rust_decimal::RoundingStrategy::ToPositiveInfinity,
        };
        self.round_dp_with_strategy(dp, rounding_mode)
    }
    fn min(self, other: Self) -> Self {
        Decimal::min(self, other)
    }
    fn max(self, other: Self) -> Self {
        Decimal::max(self, other)
    }
    fn from_usize(n: usize) -> Self {
        Decimal::from(n)
    }
    fn from_u16(n: u16) -> Self {
        Decimal::from(n)
    }
    fn from_i32(n: i32) -> Self {
        Decimal::from(n)
    }
    // This may panic if the f32 cannot be represented as a Decimal
    fn from_f32(n: f32) -> Self {
        <Decimal as FromPrimitive>::from_f32(n).expect("Failed to convert f32 to Decimal")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(feature = "std"))]
    extern crate std;
    #[cfg(not(feature = "std"))]
    use std::assert_eq;

    #[test]
    fn test_floatlike_rounding() {
        let value: f32 = 2.5;
        assert_eq!(value.round_with_mode(0, RoundingMode::HalfToEven, 1e-5), 2.0);
        assert_eq!(value.round_with_mode(0, RoundingMode::HalfAwayFromZero, 1e-5), 3.0);
        assert_eq!(value.round_with_mode(0, RoundingMode::HalfTowardZero, 1e-5), 2.0);
        assert_eq!(value.round_with_mode(0, RoundingMode::TowardZero, 1e-5), 2.0);
        assert_eq!(value.round_with_mode(0, RoundingMode::AwayFromZero, 1e-5), 3.0);
        assert_eq!(value.round_with_mode(0, RoundingMode::ToNegativeInfinity, 1e-5), 2.0);
        assert_eq!(value.round_with_mode(0, RoundingMode::ToInfinity, 1e-5), 3.0);

        let value: f32 = -2.5;
        assert_eq!(value.round_with_mode(0, RoundingMode::HalfToEven, 1e-5), -2.0);
        assert_eq!(value.round_with_mode(0, RoundingMode::HalfAwayFromZero, 1e-5), -3.0);
        assert_eq!(value.round_with_mode(0, RoundingMode::HalfTowardZero, 1e-5), -2.0);
        assert_eq!(value.round_with_mode(0, RoundingMode::TowardZero, 1e-5), -2.0);
        assert_eq!(value.round_with_mode(0, RoundingMode::AwayFromZero, 1e-5), -3.0);
        assert_eq!(value.round_with_mode(0, RoundingMode::ToNegativeInfinity, 1e-5), -3.0);
        assert_eq!(value.round_with_mode(0, RoundingMode::ToInfinity, 1e-5), -2.0);

        let value: f32 = 3.005;
        assert_eq!(value.round_with_mode(2, RoundingMode::HalfToEven, 1e-5), 3.00);
        assert_eq!(value.round_with_mode(2, RoundingMode::HalfAwayFromZero, 1e-5), 3.01);
        assert_eq!(value.round_with_mode(2, RoundingMode::HalfTowardZero, 1e-5), 3.00);
        assert_eq!(value.round_with_mode(2, RoundingMode::TowardZero, 1e-5), 3.00);
        assert_eq!(value.round_with_mode(2, RoundingMode::AwayFromZero, 1e-5), 3.01);
        assert_eq!(value.round_with_mode(2, RoundingMode::ToNegativeInfinity, 1e-5), 3.00);
        assert_eq!(value.round_with_mode(2, RoundingMode::ToInfinity, 1e-5), 3.01);

        let value: f64 = -3.005;
        assert_eq!(value.round_with_mode(2, RoundingMode::HalfToEven, 1e-5), -3.00);
        assert_eq!(value.round_with_mode(2, RoundingMode::HalfAwayFromZero, 1e-5), -3.01);
        assert_eq!(value.round_with_mode(2, RoundingMode::HalfTowardZero, 1e-5), -3.00);
        assert_eq!(value.round_with_mode(2, RoundingMode::TowardZero, 1e-5), -3.00);
        assert_eq!(value.round_with_mode(2, RoundingMode::AwayFromZero, 1e-5), -3.01);
        assert_eq!(value.round_with_mode(2, RoundingMode::ToNegativeInfinity, 1e-5), -3.01);
        assert_eq!(value.round_with_mode(2, RoundingMode::ToInfinity, 1e-5), -3.00);

        let value: f64 = 0.333;
        assert_eq!(value.round_with_mode(2, RoundingMode::HalfToEven, 1e-5), 0.33);
        assert_eq!(value.round_with_mode(2, RoundingMode::HalfAwayFromZero, 1e-5), 0.33);
        assert_eq!(value.round_with_mode(2, RoundingMode::HalfTowardZero, 1e-5), 0.33);
        assert_eq!(value.round_with_mode(2, RoundingMode::TowardZero, 1e-5), 0.33);
        assert_eq!(value.round_with_mode(2, RoundingMode::AwayFromZero, 1e-5), 0.34);
        assert_eq!(value.round_with_mode(2, RoundingMode::ToNegativeInfinity, 1e-5), 0.33);
        assert_eq!(value.round_with_mode(2, RoundingMode::ToInfinity, 1e-5), 0.34);

        let value: f64 = -0.333;
        assert_eq!(value.round_with_mode(2, RoundingMode::HalfToEven, 1e-5), -0.33);
        assert_eq!(value.round_with_mode(2, RoundingMode::HalfAwayFromZero, 1e-5), -0.33);
        assert_eq!(value.round_with_mode(2, RoundingMode::HalfTowardZero, 1e-5), -0.33);
        assert_eq!(value.round_with_mode(2, RoundingMode::TowardZero, 1e-5), -0.33);
        assert_eq!(value.round_with_mode(2, RoundingMode::AwayFromZero, 1e-5), -0.34);
        assert_eq!(value.round_with_mode(2, RoundingMode::ToNegativeInfinity, 1e-5), -0.34);
        assert_eq!(value.round_with_mode(2, RoundingMode::ToInfinity, 1e-5), -0.33);

        let value: f64 = 0.555;
        assert_eq!(value.round_with_mode(2, RoundingMode::HalfToEven, 1e-5), 0.56);
        assert_eq!(value.round_with_mode(2, RoundingMode::HalfAwayFromZero, 1e-5), 0.56);
        assert_eq!(value.round_with_mode(2, RoundingMode::HalfTowardZero, 1e-5), 0.55);
        assert_eq!(value.round_with_mode(2, RoundingMode::TowardZero, 1e-5), 0.55);
        assert_eq!(value.round_with_mode(2, RoundingMode::AwayFromZero, 1e-5), 0.56);
        assert_eq!(value.round_with_mode(2, RoundingMode::ToNegativeInfinity, 1e-5), 0.55);
        assert_eq!(value.round_with_mode(2, RoundingMode::ToInfinity, 1e-5), 0.56);

        let value: f64 = 3.00;
        assert_eq!(value.round_with_mode(2, RoundingMode::HalfToEven, 1e-5), 3.00);
        assert_eq!(value.round_with_mode(2, RoundingMode::HalfAwayFromZero, 1e-5), 3.00);
        assert_eq!(value.round_with_mode(2, RoundingMode::HalfTowardZero, 1e-5), 3.00);
        assert_eq!(value.round_with_mode(2, RoundingMode::TowardZero, 1e-5), 3.00);
        assert_eq!(value.round_with_mode(2, RoundingMode::AwayFromZero, 1e-5), 3.00);
        assert_eq!(value.round_with_mode(2, RoundingMode::ToNegativeInfinity, 1e-5), 3.00);
        assert_eq!(value.round_with_mode(2, RoundingMode::ToInfinity, 1e-5), 3.00);
    }
}
