use crate::error::RootFindingError;
use crate::FinPrimError;
use crate::FloatLike;

/// Returns the numerator and denominator of the Halley's step (allows for some checking)
#[inline(always)]
pub(crate) fn step_halley<T: FloatLike>(f_x: T, f_prime_x: T, f_prime2_x: T) -> (T, T) {
    let _2 = T::two();
    let numerator = _2 * f_x * f_prime_x;
    let denominator = _2 * f_prime_x * f_prime_x - f_x * f_prime2_x;
    (numerator, denominator)
}

pub fn halley<T: FloatLike, F, D, D2>(
    guess: T,
    f: F,
    f_prime: D,
    f_prime2: D2,
    tolerance: T,
    max_iter: u16,
) -> Result<T, FinPrimError<T>>
where
    F: Fn(T) -> T,
    D: Fn(T) -> T,
    D2: Fn(T) -> T,
{
    let mut x = guess;
    let mut fx = f(x);
    for _ in 0..max_iter {
        if fx.abs() < tolerance {
            return Ok(x);
        }
        let f_prime_x = f_prime(x);
        let f_prime2 = f_prime2(x);

        let (numerator, denominator) = step_halley(fx, f_prime_x, f_prime2);
        if denominator.is_zero() {
            return Err(FinPrimError::RootFindingError(RootFindingError::DivideByZero {
                last_x: x,
                last_fx: fx,
            }));
        }
        x -= numerator / denominator;
        fx = f(x);
    }
    Err(FinPrimError::RootFindingError(RootFindingError::FailedToConverge {
        last_x: x,
        last_fx: fx,
    }))
}
