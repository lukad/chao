#![feature(slice_patterns)]

#[macro_use]
extern crate combine;
extern crate colored;
extern crate itertools;

mod builtin;
pub mod env;
pub mod expr;
pub mod parser;

pub use env::Env;
pub use expr::Expr;
pub use parser::parse;
