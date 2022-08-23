#![allow(clippy::unit_arg)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate smart_default;
#[macro_use]
extern crate param_macro;

pub mod client;
pub mod error;
pub mod macros;
pub mod models;
pub mod services;
