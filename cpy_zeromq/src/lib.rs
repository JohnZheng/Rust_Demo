#![crate_name = "cpy_zeromq"]
#![unstable]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature[box_syntax]]
#[macro_use] extern crate log;

pub use ctx::Context;


mod ctx;
mod inproc;
