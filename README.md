# Financial Calculations

This is a minimal library providing various functions to perform financial calculations. It is a work and progress and is mostly for practice and learning purposes at the moment.

Built on top of the [`rust_decimal`](https://github.com/paupino/rust-decimal) crate, providing high precision while maintaining respectable performance. The `Decimal` type provides a fixed-point decimal number of up to 28 significant digits. Offering much more precision than calculations using an `f64` and implementations in spreadsheet programs.

Some functions and their args mimic those found in Excel and Google Sheets.

## Current Features

- Time Value of Money (TVM) Calculations
  - Present Value
  - Future Value
  - Net Present Value (NPV)
  - Net Present Value with differing discount rates
  - Net Present Value for irregular cash flows (XNPV)
  - Payment (PMT)
- Interest Rate Calculations
  - APR (Annual Percentage Rate) and EAR (Effective Annual Rate) conversions
  - IRR (Internal Rate of Return)
  - Internal Rate of Return for irregular cash flows (XIRR)
  - MIRR (Modified Internal Rate of Return)
  - Modified Internal Rate of Return for irregular cash flows (XMIRR)
- Amortization, Depreciation, and Tax Calculations
  - Amortization Schedule
  - Depreciation Schedules for various methods
    - Straight line
    - Declining balance (e.g. double declining balance)
    - Sum of years digits
    - MACRS (Modified Accelerated Cost Recovery System) for US tax purposes
  - Progressive Income Tax
- Derivatives of common financial functions for sensitivity analysis and
  optimization problems
