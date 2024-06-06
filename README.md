# Financial Calculation Primitives

This is a minimal library providing various primitive functions to perform financial calculations.

Built on top of the [`rust_decimal`](https://github.com/paupino/rust-decimal) crate, providing high precision while maintaining respectable performance. The `Decimal` type provides a fixed-point decimal number of up to 28 significant digits. Offering much more precision than calculations using an `f64` and implementations in spreadsheet programs.

Some functions and their args mimic those found in Excel and Google Sheets.

## Functionality

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

### Interest Rate Calculations

`rust_finprim::rate` module.

- **APR (Annual Percentage Rate)** and **EAR (Effective Annual Rate)** conversions
- **IRR (Internal Rate of Return)**
  - Common Uses: Investment Analysis, Capital Budgeting, Bond Yields (YTM, YTC), etc.
- **Internal Rate of Return for irregular cash flows (XIRR)**
- **MIRR (Modified Internal Rate of Return)**
- **Modified Internal Rate of Return for irregular cash flows (XMIRR)**

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

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rust_finprim = "0.1.0"
```

Or

```sh
cargo add rust_finprim
```

You can also enable the `serde` feature to serialize and deserialize any of the provided structs:

```toml
[dependencies]
rust_finprim = { version = "0.1.0", features = ["serde"] }
```

Or

```sh
cargo add rust_finprim --features serde
```
