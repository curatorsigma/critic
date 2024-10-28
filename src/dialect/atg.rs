//! Load all ATG dialects based on features.

#[cfg(feature = "atg_example")]
mod example;
#[cfg(feature = "atg_example")]
pub use example::ExampleAtgDialect;
