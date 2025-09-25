# Changelog

## [v0.5.1]

- Fixed `fv` function

## [v0.5.0]

### Breaking

- The `Decimal` type is now an optional dependency, enabled by the `rust_decimal` feature. If you want to use `Decimal`, you need to enable this feature in your `Cargo.toml`
- `rate::irr` and `rate::xirr`
  - Changed the error return type to `Result<FloatLike, FinPrimError<T>>` to handle errors more gracefully
  - Added `max_iterations` parameter to control the maximum number of iterations for root finding, default is 20 iterations
- `pct_change` now returns a `Result` with an error for division by zero instead of an option
- `twr` now returns a `Result` with an error for division by zero
- `no_std` requires enabling the `no_std` feature with `default-features = false` in your `Cargo.toml`. Core float types require the `libm` crate for mathematical operations

- Replaced all of `Decimal`'s `RoundingStrategy` with new `RoundingMode`.
  - `RoundingMode` is now used for all rounding operations in the library
  - Functions that take a rounding argument now require an epsilon value for precision control of the half even/up/down rounding modes

### Added

- `FloatLike` trait to allow for generic implementations of financial calculations across different types
  - All functions now accept any type that implements `FloatLike`.
  - By default, `FloatLike` is implemented for `f32`, `f64`, and `Decimal` from [rust_decimal](https://github.com/paupino/rust-decimal) (via `rust_decimal` feature)
- `utils` module
  - `utils::newton_raphson`, Newton-Raphson root finding algorithm
  - `utils::halley`, Halley's method root finding algorithm
- `npv_prime_r` and `npv_prime2_r`, first and second derivatives of NPV with respect to the discount rate, offers some optimization over summing the respective derivatives of `pv`

### Changed

- Fixed `pv` function to return positive values appropriately

- `npv` optimization, now approx. 7x+ faster

## [v0.4.0]

### Breaking

- Non-`*_into` `amort_dep_tax` module functions are marked `std` and disabled by `no_std` feature. Use the `*_into` functions for no-alloc APIs

### Added

- Derivatives for WACC with respect to the debt/equity ratio
  - `derivatives::wacc_prime_de`, first derivative of WACC with respect to the debt/equity ratio
  - `derivatives::wacc_prime2_de`, second derivative of WACC with respect to the debt/equity ratio
- `rate::pct_change`, percentage change between two values
- `rate::apply_pct_change`, apply a percentage change to a value
- `rate::twr`, time-weighted return of an investment
- No-alloc APIs for `amort_dep_tax` module
  - `amort_dep_tax::amort_schedule_into`, amortization schedule into a slice
  - `amort_dep_tax::db_into`, declining balance depreciation schedule into a slice
  - `amort_dep_tax::syd_into`, sum of years digits depreciation schedule into a slice
  - `amort_dep_tax::macrs_into`, MACRS depreciation schedule into a slice
  - `amort_dep_tax::sln_into`, straight line depreciation schedule into a slice

### Changed

- No functions allocate on the heap with exceptions being non-`*_into` `amort_dep_tax` module functions that return `Vec`
- Various minor optimizations, code cleanups, and documentation improvements

### Removed

- Unnecessary unsafe tag from `amort_dep_tax::progressive_tax_unchecked`
