#![deny(clippy::mem_forget)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate tracing;

pub mod cli;
mod err;
mod logging;
mod mem;
