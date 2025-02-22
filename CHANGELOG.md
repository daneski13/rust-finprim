# Changelog

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
