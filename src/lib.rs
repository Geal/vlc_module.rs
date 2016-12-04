#![allow(non_camel_case_types,dead_code,non_upper_case_globals)]

extern crate libc;
extern crate core;

#[macro_use]
pub mod macros;

#[macro_use]
pub mod vlc;

#[macro_use]
pub mod ffi;

mod types;
mod traits;
