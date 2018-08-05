use itertools::Itertools;

use env::Env;
use std::fmt;

#[derive(Clone, PartialEq)]
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

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Nil => write!(f, "nil"),
            Expr::Bool(x) => write!(f, "{:?}", x),
            Expr::Int(x) => write!(f, "{:?}", x),
            Expr::Float(x) => write!(f, "{:?}", x),
            Expr::Str(x) => write!(f, "\"{}\"", x),
            Expr::Symbol(x) => write!(f, "{}", x),
            Expr::Fun(_, _) => write!(f, "<function>"),
            Expr::Special(_, _) => write!(f, "<special>"),
            Expr::List(xs) => write!(f, "({})", xs.iter().map(|x| format!("{:?}", x)).join(" ")),
        }
    }
}

pub struct Function(pub fn(&mut Env) -> Expr);

impl PartialEq for Function {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Copy for Function {}

impl Clone for Function {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Arguments {
    Variadic,
    Fixed(Vec<String>),
}
