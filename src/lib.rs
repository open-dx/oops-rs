#![no_std]

extern crate alloc;

//--
extern crate oops_core;
pub use oops_core::*;

extern crate oops_macros;
pub use oops_macros::Error;

//--
#[macro_use]
pub mod nvmd;

#[macro_use]
pub mod welp;
