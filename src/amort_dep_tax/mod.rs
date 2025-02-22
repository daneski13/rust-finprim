//! This module provides functions related to loan/mortgage amortization, depreciation, and tax calculations.

// Structs
mod structs;
pub use structs::{AmortizationPeriod, DepreciationPeriod};

// Amortization
mod amort;
#[cfg(feature = "std")]
pub use amort::amort_schedule;
pub use amort::amort_schedule_into;

// Depreciation
mod dep;
#[cfg(feature = "std")]
pub use dep::{db, macrs, sln, syd};
pub use dep::{db_into, macrs_into, sln_into, syd_into};

// Tax
mod tax;
pub use tax::{progressive_tax, progressive_tax_unchecked};

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    #[cfg(not(feature = "std"))]
    extern crate std;
    use super::*;
    use rust_decimal_macros::*;
    #[cfg(not(feature = "std"))]
    use std::prelude::v1::*;
    #[cfg(not(feature = "std"))]
    use std::{assert, assert_eq, println, vec};

    #[test]
    fn test_serde() {
        let period = AmortizationPeriod::new(1, dec!(100), dec!(50), dec!(850));
        let serialized = serde_json::to_string(&period).unwrap();
        let deserialized: AmortizationPeriod = serde_json::from_str(&serialized).unwrap();
        assert_eq!(period, deserialized);
        let clone = period.clone();
        assert_eq!(period, clone);

        let dep_period = DepreciationPeriod::new(1, dec!(100), dec!(900));
        let serialized = serde_json::to_string(&dep_period).unwrap();
        let deserialized: DepreciationPeriod = serde_json::from_str(&serialized).unwrap();
        assert_eq!(dep_period, deserialized);
        let clone = dep_period.clone();
        assert_eq!(dep_period, clone);
    }
}
