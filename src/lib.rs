// #![warn(missing_docs)]
#![feature(proc_macro)]

mod timer;
pub use timer::*;

#[macro_use] extern crate stratus_macros;
pub use stratus_macros::*;
