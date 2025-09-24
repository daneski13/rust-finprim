use crate::error::RootFindingError;
use crate::FinPrimError;
use crate::FloatLike;

pub fn newton_raphson<T: FloatLike, F, D>(
    guess: T,
    f: F,
    f_prime: D,
    tolerance: T,
    max_iter: u16,
) -> Result<T, FinPrimError<T>>
where
    F: Fn(T) -> T,
    D: Fn(T) -> T,
{
    let mut x = guess;
    let mut fx = f(x);
    for _ in 0..max_iter {
        if fx.abs() < tolerance {
            return Ok(x);
        }
        let f_prime_x = f_prime(x);
        if f_prime_x.is_zero() {
            return Err(FinPrimError::RootFindingError(RootFindingError::DivideByZero {
                last_x: x,
                last_fx: fx,
            }));
        }
        x -= fx / f_prime_x;
        fx = f(x);
    }
    Err(FinPrimError::RootFindingError(RootFindingError::FailedToConverge {
        last_x: x,
        last_fx: fx,
    }))
}
