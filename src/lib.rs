//! # Financial Calculations
//!
//! This library provides various functions to perform financial calculations.
//!
//! Built on top of the [`rust_decimal`](https://github.com/paupino/rust-decimal) crate,
//! providing high precision while maintaining respectable performance. The `Decimal` type
//! provides a fixed-point decimal number of up to 28 significant digits. Offering much more
//! precision than calculations using an `f64` and implementations in spreadsheet programs.
//!
//! Some functions and their args mimic those found in Excel and Google Sheets.
//!
//! ## Features
//! * Time Value of Money (TVM) Calculations
//!     * Present Value
//!     * Future Value
//!     * Net Present Value (NPV)
//!     * Net Present Value with differing discount rates
//!     * Net Present Value for irregular cash flows (XNPV)
//!     * Payment (PMT)
//! * Interest Rate calculations
//!     * APR (Annual Percentage Rate) and EAR (Effective Annual Rate) conversions
//!     * IRR (Internal Rate of Return)
//!     * Internal Rate of Return for irregular cash flows (XIRR)
//!     * MIRR (Modified Internal Rate of Return)
//!     * Modified Internal Rate of Return for irregular cash flows (XMIRR)
//! * Tax and Amortization
//!    * Amortization Schedule
//!    * Progressive Income Tax
//! * Derivatives of common financial functions for sensitivity analysis and optimization problems

pub mod amort_tax;
pub mod derivatives;
pub mod rate;
pub mod tvm;

pub use rust_decimal::Decimal;
pub use rust_decimal_macros::*;

const ONE: Decimal = Decimal::ONE;
const ZERO: Decimal = Decimal::ZERO;
