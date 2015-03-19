#![crate_name = "cpy_zeromq"]
#![unstable]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(box_syntax)]
#![feature(int_uint)]
#[macro_use] extern crate log;

pub use ctx::Context;


mod ctx;
mod inproc;
mod msg;
mod req;
mod rep;
mod result;
mod socket;
mod socket_base;
mod stream_engine;
mod tcp_connecter;
mod tcp_listener;
mod options;
mod v2_encoder;
mod v2_decoder;
mod v2_protocol;
mod consts;
