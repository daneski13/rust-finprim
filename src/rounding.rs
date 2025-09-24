/// Rounding modes to be used by FloatLike types.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RoundingMode {
    /// "Bankers Rounding"
    /// Rounds .5 to the nearest even number, typical rounding behavior otherwise.
    /// e.g. 2.5 becomes 2, 3.5 becomes 4.
    HalfToEven,
    /// Rounds .5 away from zero (typical rounding behavior).
    /// e.g. 2.5 becomes 3, -2.5 becomes -3.
    HalfAwayFromZero,
    /// Rounds .5 towards zero.
    /// e.g. 2.5 becomes 2, -2.5 becomes -2.
    HalfTowardZero,
    /// Rounds towards zero (typical round down).
    /// e.g. 2.7 becomes 2, -2.7 becomes -2.
    TowardZero,
    /// Rounds towards negative/positive infinity (typical round up).
    /// e.g. 2.3 becomes 3, -2.3 becomes -3.
    AwayFromZero,
    /// Rounds towards negative infinity (round positive numbers down and negative numbers "up").
    /// e.g. 2.7 becomes 2, -2.3 becomes -3.
    ToNegativeInfinity,
    /// Rounds towards positive infinity (round positive numbers up and negative numbers "down").
    /// e.g. 2.3 becomes 3, -2.7 becomes -2.
    ToInfinity, // And beyond
}
