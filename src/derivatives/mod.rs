//! This module contains various derivatives of financial calculations.
//! Useful for sensitivity analysis and optimization problems.

mod wacc;
pub use wacc::{wacc_prime2_de, wacc_prime_de};

mod pv;
pub use pv::{npv_prime2_r, npv_prime_r, pv_prime2_r, pv_prime_r};
