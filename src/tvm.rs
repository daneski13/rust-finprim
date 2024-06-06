//! This module contains functions related to Time Value of Money (TVM) calculations.
//!
//! For example, you can calculate the future value of an investment, the present value of a future cash flow, or the payment amount for a loan.
//!
//! The Time Value of Money (TVM) is a financial concept that states that money in the present is worth more than the same amount in the future due to its potential earning capacity.
//! This concept is the basis for many financial calculations, including the calculation of interest rates, loan payments, and investment returns.

// FV - Future Value
mod fv;
pub use fv::fv;

// PV - Present Value
mod pv;
pub use pv::{npv, npv_differing_rates, pv, xnpv};

// PMT - Payment
mod pmt;
pub use pmt::pmt;
