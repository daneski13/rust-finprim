use crate::FloatLike;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Amorization Period
///
/// Represents a single period in an amortization schedule.
///
/// An amortization period includes information about the payment period, the portion
/// of the payment allocated to principal, the portion allocated to interest, and the
/// remaining balance of the loan or mortgage.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AmortizationPeriod<T> {
    /// The period number of the amortization schedule.
    pub period: u32,

    /// The amount of the payment allocated to reduce the principal balance.
    pub principal_payment: T,

    /// The amount of the payment allocated to pay interest charges.
    pub interest_payment: T,

    /// The remaining balance of the loan or mortgage after the payment.
    pub remaining_balance: T,
}

impl<T: FloatLike> AmortizationPeriod<T> {
    /// Creates a new `AmortizationPeriod` instance.
    ///
    /// # Arguments
    /// * `period`: The period number of the amortization schedule.
    /// * `principal_payment`: The amount allocated to reduce the principal balance.
    /// * `interest_payment`: The amount allocated to pay interest charges.
    /// * `remaining_balance`: The remaining balance of the loan or mortgage after the payment.
    ///
    /// # Returns
    ///
    /// A new `AmortizationPeriod` instance initialized with the provided values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_finprim::amort_dep_tax::AmortizationPeriod;
    ///
    /// let period = AmortizationPeriod::new(1, 100.0, 50.0, 850.0);
    /// ```
    pub fn new(period: u32, principal_payment: T, interest_payment: T, remaining_balance: T) -> Self {
        Self {
            period,
            principal_payment,
            interest_payment,
            remaining_balance,
        }
    }

    /// Default implementation for `AmortizationPeriod`.
    pub fn default() -> Self {
        Self {
            period: 0,
            principal_payment: T::zero(),
            interest_payment: T::zero(),
            remaining_balance: T::zero(),
        }
    }
}

/// Depreciation Period
///
/// Represents a single period in an asset's depreciation schedule.
///
/// An asset depreciation period includes information about the period number,
/// the depreciation expense for the period, and the remaining book value of the asset.
/// The book value is the original cost of the asset minus the accumulated depreciation.
///
/// # Examples
/// ```
/// use rust_finprim::amort_dep_tax::DepreciationPeriod;
///
/// let period = DepreciationPeriod::new(1, 100.0, 900.0);
/// ```
/// The above example creates a new `DepreciationPeriod` instance with a period number of 1,
/// a depreciation expense of $100, and a remaining book value of $900.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DepreciationPeriod<T> {
    /// The period number of the depreciation schedule.
    pub period: u32,

    /// The depreciation expense for the period.
    pub depreciation_expense: T,

    /// The remaining book value of the asset.
    pub remaining_book_value: T,
}

impl<T: FloatLike> DepreciationPeriod<T> {
    /// Creates a new `DepreciationPeriod` instance.
    ///
    /// # Arguments
    /// * `period`: The period number of the depreciation schedule.
    /// * `depreciation_expense`: The depreciation expense for the period.
    /// * `remaining_book_value`: The remaining book value of the asset.
    ///
    /// # Returns
    ///
    /// A new `DepreciationPeriod` instance initialized with the provided values.
    pub fn new(period: u32, depreciation_expense: T, remaining_book_value: T) -> Self {
        Self {
            period,
            depreciation_expense,
            remaining_book_value,
        }
    }

    /// Default implementation for `DepreciationPeriod`.
    pub fn default() -> Self {
        Self {
            period: 0,
            depreciation_expense: T::zero(),
            remaining_book_value: T::zero(),
        }
    }
}
