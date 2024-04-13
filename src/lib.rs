#![no_std]

// TODO: Remove this as soon as we can.
#![feature(error_in_core)] // </3

//--
extern crate oops_core;

extern crate oops_macros;
pub use oops_macros::Error;

//--
#[macro_use]
pub mod nvmd;

#[macro_use]
pub mod welp;
