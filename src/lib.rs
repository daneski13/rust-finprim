#![doc = include_str!(concat!("../README.md"))]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod amort_dep_tax;
pub mod derivatives;
pub mod rate;
pub mod tvm;
pub mod utils;

mod floatlike;
pub use floatlike::FloatLike;

mod rounding;
pub use rounding::RoundingMode;

mod error;
pub use error::FinPrimError;

#[cfg(feature = "rust_decimal")]
pub use rust_decimal::Decimal;
#[cfg(feature = "rust_decimal")]
pub use rust_decimal_macros::*;
