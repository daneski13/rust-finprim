//! This module provides functions related to loan/mortgage amortization, depreciation, and tax calculations.

// Amortization
mod amort;
pub use amort::{amort_schedule, AmortizationPeriod};

// Depreciation
mod dep;
pub use dep::{db, macrs, sln, syd, DepreciationPeriod};

// Tax
mod tax;
pub use tax::{progressive_tax, progressive_tax_unchecked};

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::*;

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
