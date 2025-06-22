// SPDX-License-Identifier: MIT

// NOTE: This is MIT-licensed for maximum compatibility with other codebases.
// Usage and validation primarily occurs in the RAWR utility and user crates.

#![forbid(unsafe_code)]

// Re-export the proc and derive macros
extern crate rawr_proc_macro;
pub use rawr_proc_macro::{Rawr, rawr};

/// Empty macro for use inside functions.
#[macro_export]
macro_rules! rawr_fn( ($i:ident = $l:literal $(, $i_rest:ident = $l_rest:literal)* ) => {} );
