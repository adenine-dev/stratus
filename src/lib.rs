#![warn(missing_docs)]

//! TODO: write crate docs

mod timer;
pub use timer::*;

mod instrumentor;
pub use instrumentor::*;

extern crate stratus_macros;
pub use stratus_macros::*;
