use core::fmt::{self, Debug, Display};

/// Root finding general errors
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RootFindingError<T> {
    /// Error when the method fails to converge
    FailedToConverge { last_x: T, last_fx: T },
    /// Error when the method attempts to divide by zero
    DivideByZero { last_x: T, last_fx: T },
    /// Error when the bracket provided is invalid
    InvalidBracket,
}

impl<T: Display> Display for RootFindingError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RootFindingError::FailedToConverge { last_x, last_fx } => {
                write!(f, "Failed to converge. Last x: {}, Last f(x): {}", last_x, last_fx)
            }
            RootFindingError::DivideByZero { last_x, last_fx } => {
                write!(
                    f,
                    "Attempted to divide by zero. Last x: {}, Last f(x): {}",
                    last_x, last_fx
                )
            }
            RootFindingError::InvalidBracket => write!(f, "Invalid bracket provided."),
        }
    }
}
#[cfg(feature = "std")]
impl<T: Display + Debug> std::error::Error for RootFindingError<T> {}

/// Custom error type for financial calculations in FinPrim
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FinPrimError<T> {
    /// Divide by zero error
    DivideByZero,
    /// The method failed to find a root
    RootFindingError(RootFindingError<T>),
}

impl<T: Display> Display for FinPrimError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FinPrimError::DivideByZero => write!(f, "Division by zero error."),
            FinPrimError::RootFindingError(e) => write!(f, "Root finding error: {}", e),
        }
    }
}

#[cfg(feature = "std")]
impl<T: Display + Debug> std::error::Error for FinPrimError<T> {}
