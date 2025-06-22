extern crate rawr_proc_macro;
pub use rawr_proc_macro::Rawr;
pub use rawr_proc_macro::rawr;

/// Empty macro for use inside functions.
#[macro_export]
macro_rules! rawr_fn( ($i:ident = $l:literal $(, $i_rest:ident = $l_rest:literal)* ) => {} );
