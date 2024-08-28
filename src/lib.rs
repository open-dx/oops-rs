#![no_std]

//--
extern crate oops_core;

extern crate oops_macros;
pub use oops_macros::Error;

//--
#[macro_use]
pub mod nvmd;

#[macro_use]
pub mod welp;
