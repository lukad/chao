#![feature(slice_patterns)]

#[macro_use]
extern crate combine;
extern crate colored;
extern crate itertools;

mod builtin;
mod env;
mod expr;
mod parser;

pub use env::Env;
pub use expr::Expr;
pub use parser::parse;
