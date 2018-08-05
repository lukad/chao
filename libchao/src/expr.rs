use env::Env;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f32),
    Str(String),
    Symbol(String),
    Fun(Function, Arguments),
    Special(Function, Arguments),
    List(Vec<Expr>),
}

pub struct Function(pub fn(&mut Env) -> Expr);

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<fn>")
    }
}

impl Copy for Function {}

impl Clone for Function {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Debug, Clone)]
pub enum Arguments {
    Variadic,
    Fixed(Vec<String>),
}
