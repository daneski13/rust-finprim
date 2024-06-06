//! This module contains functions for calculating interest rates.
//!
//! For example, you can calculate the annual percentage rate (APR) from the effective annual rate (EAR), or vice versa.
//! Also incudes IRR (Internal Rate of Return), MIRR (Modified Internal Rate of Return), and more.

// APR and EAR
mod apr_ear;
pub use apr_ear::{apr, ear};

// IRR and MIRR
mod irr;
pub use irr::{irr, xirr};

// XIRR and XMIRR
mod mirr;
pub use mirr::{mirr, xmirr};

// CAGR
mod cagr;
pub use cagr::cagr;
