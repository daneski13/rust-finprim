#![doc = include_str!(concat!("../README.md"))]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod amort_dep_tax;
pub mod derivatives;
pub mod rate;
pub mod tvm;

pub use rust_decimal::Decimal;
pub use rust_decimal_macros::*;

const ONE: Decimal = Decimal::ONE;
const ZERO: Decimal = Decimal::ZERO;
