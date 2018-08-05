use std::ops::{Add, Div, Mul, Sub};

use itertools::Itertools;

use env::Env;
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Expr {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Symbol(String),
    Fun(Function, Arguments),
    Special(Function, Arguments),
    List(Vec<Expr>),
}

use Expr::*;

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Nil => write!(f, "nil"),
            Bool(x) => write!(f, "{:?}", x),
            Int(x) => write!(f, "{:?}", x),
            Float(x) => write!(f, "{:?}", x),
            Str(x) => write!(f, "\"{}\"", x),
            Symbol(x) => write!(f, "{}", x),
            Fun(_, _) => write!(f, "<function>"),
            Special(_, _) => write!(f, "<special>"),
            List(xs) => write!(f, "({})", xs.iter().map(|x| format!("{:?}", x)).join(" ")),
        }
    }
}

impl Add for Expr {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Int(a), Int(b)) => Int(a + b),
            (Int(a), Float(b)) => Float(a as f64 + b),
            (Float(a), Int(b)) => Float(a + b as f64),
            (Float(a), Float(b)) => Float(a + b),
            (Str(a), Str(b)) => Str(format!("{}{}", a, b)),
            (a, b) => panic!("Can't add {:?} and {:?}", a, b),
        }
    }
}

impl Sub for Expr {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Int(a), Int(b)) => Int(a - b),
            (Int(a), Float(b)) => Float(a as f64 - b),
            (Float(a), Int(b)) => Float(a - b as f64),
            (Float(a), Float(b)) => Float(a - b),
            (a, b) => panic!("Can't subtract {:?} from {:?}", b, a),
        }
    }
}

impl Mul for Expr {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Int(a), Int(b)) => Int(a * b),
            (Int(a), Float(b)) => Float(a as f64 * b),
            (Float(a), Int(b)) => Float(a * b as f64),
            (Float(a), Float(b)) => Float(a * b),
            (a, b) => panic!("Can't multiply {:?} with {:?}", a, b),
        }
    }
}

impl Div for Expr {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Int(a), Int(b)) => Int(a / b),
            (Int(a), Float(b)) => Float(a as f64 / b),
            (Float(a), Int(b)) => Float(a / b as f64),
            (Float(a), Float(b)) => Float(a / b),
            (a, b) => panic!("Can't divide {:?} a {:?}", a, b),
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
