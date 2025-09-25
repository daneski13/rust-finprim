# Financial Calculation Primitives (FinPrim)

This is a minimal library providing various primitive functions to perform financial calculations.

Most functions with Excel and Google Sheets counterparts mimic their behavior and arguments.

- [Functionality](#functionality)
  - [Type Agnostic](#type-agnostic)
  - [Time Value of Money (TVM) Calculations](#time-value-of-money-tvm-calculations)
  - [Rate Calculations](#rate-calculations)
  - [Amortization, Depreciation, and Tax Calculations](#amortization-depreciation-and-tax-calculations)
  - [Derivatives](#derivatives)
- [Features](#features)
- [Installation](#installation)

## Functionality

### Type Agnostic

This library is designed to be type-agnostic, allowing you to use any type that implements the `FloatLike` trait.

Default implementations:
- `f32`
- `f64`
- `Decimal` from [rust_decimal](https://github.com/paupino/rust-decimal) (via `rust_decimal` feature)

You can implement the `FloatLike` trait for your own types or wrap any decimal-like / floating-point type providing crate for use with this library. Allowing you to choose a type that best fits your precision and performance requirements.

### Time Value of Money (TVM) Calculations

`rust_finprim::tvm` module.

- **Present Value**
  - Common Uses: Bond Pricing, Discounted Cash Flow, Annuities, etc.
- **Future Value**
  - Common Uses: Compound Interest/Growth, Annuities etc.
- **Net Present Value (NPV)**
  - Common Uses: Capital Budgeting, Investment Analysis, etc.
- **Net Present Value with differing discount rates**
- **Net Present Value for irregular cash flows (XNPV)**
- **Payment (PMT)**
  - Common Uses: Bonds, Loan/Mortgage Payments, Annuities, etc.

### Rate Calculations

`rust_finprim::rate` module.

- **APR (Annual Percentage Rate)** and **EAR (Effective Annual Rate)** conversions
- **IRR (Internal Rate of Return)**
  - Common Uses: Investment Analysis, Capital Budgeting, Bond Yields (YTM, YTC), etc.
- **Internal Rate of Return for irregular cash flows (XIRR)**
- **MIRR (Modified Internal Rate of Return)**
- **Modified Internal Rate of Return for irregular cash flows (XMIRR)**
- **Time Weighted Return**
  - Common Uses: Performance Measurement, Portfolio Analysis, Due Diligence, etc.
- **Percentage Change**

### Amortization, Depreciation, and Tax Calculations

`rust_finprim::amort_dep_tax` module.

- **Amortization Schedule**
  - Common Uses: Loan/Mortgage Amortization
- Depreciation Schedules for various methods
  - **Straight line**
  - **Declining balance (e.g. double declining balance)**
  - **Sum of years digits**
  - **MACRS (Modified Accelerated Cost Recovery System) for US tax purposes**
- **Progressive Income Tax**

### Derivatives

`rust_finprim::derivatives` module.

- 1st and 2nd derivative of present value with respect to interest rate
  - Useful for calculating duration, convexity and various optimization problems

- 1st and 2nd derivative of WACC (Weighted Average Cost of Capital) with respect to the debt/equity ratio

## Features

- `std` - Enabled by default. You can use the library in a `no_std` environment with `default-features = false` and enabling `no_std` feature (`no_std` depends on the `libm` crate for the core float types).
- `serde` - Enables serialization and deserialization of the provided structs using `serde`.
- `rust_decimal` - Enables support for the `Decimal` type from the [rust_decimal](https://github.com/paupino/rust-decimal) crate.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rust_finprim = "0.5.1"
```

Or

```sh
cargo add rust_finprim
```

Enable everything:

```sh
cargo add rust_finprim --features "serde rust_decimal"
```

Disable `std` and enable `no_std`:

```sh
cargo add rust_finprim --no-default-features --features "no_std"
```
