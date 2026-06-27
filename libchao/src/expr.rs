use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use colored::*;
use itertools::Itertools;

use crate::functions::Callable;
use crate::interpreter::{EvalError, EvalResult};

#[derive(Clone, PartialEq)]
pub enum Expr {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Symbol(String),
    Quote(Box<Expr>),
    Callable(Callable),
    List(Vec<Expr>),
}

impl Expr {
    pub fn is_truthy(&self) -> bool {
        !matches!(self, Nil | Bool(false))
    }

    pub fn is_falsy(&self) -> bool {
        matches!(self, Nil | Bool(false))
    }
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Nil, Nil) => Some(Ordering::Equal),
            (Bool(a), Bool(b)) => PartialOrd::partial_cmp(a, b),
            (Int(a), Int(b)) => PartialOrd::partial_cmp(a, b),
            (Int(a), Float(b)) => PartialOrd::partial_cmp(&(*a as f64), b),
            (Float(a), Float(b)) => PartialOrd::partial_cmp(a, b),
            (Float(a), Int(b)) => PartialOrd::partial_cmp(a, &(*b as f64)),
            (Str(a), Str(b)) => PartialOrd::partial_cmp(a, b),
            (Symbol(a), Symbol(b)) => PartialOrd::partial_cmp(a, b),
            (Quote(a), Quote(b)) => PartialOrd::partial_cmp(a, b),
            _ => None,
        }
    }
}

use Expr::*;

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Nil => write!(f, "nil"),
            Bool(x) => write!(f, "{:?}", x),
            Int(x) => write!(f, "{:?}", x),
            Float(x) => write!(f, "{:?}", x),
            Str(x) => write!(f, "{:?}", x),
            Symbol(x) => write!(f, "{}", x),
            Quote(x) => write!(f, "'{:?}", x),
            Callable(_) => write!(f, "<callable>"),
            List(xs) => write!(f, "({})", xs.iter().map(|x| format!("{:?}", x)).join(" ")),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Nil => write!(f, "{}", format!("{:?}", self).cyan()),
            Bool(_) => write!(f, "{}", format!("{:?}", self).green()),
            Int(_) => write!(f, "{}", format!("{:?}", self).blue()),
            Float(_) => write!(f, "{}", format!("{:?}", self).blue()),
            Str(_) => write!(f, "{}", format!("{:?}", self).yellow()),
            Symbol(_) => write!(f, "{}", format!("{:?}", self).bright_white()),
            Quote(x) => write!(f, "'{}", x),
            Callable(_) => write!(f, "{}", format!("{:?}", self).magenta()),
            List(xs) => write!(f, "({})", xs.iter().map(|x| format!("{}", x)).join(" ")),
        }
    }
}

impl Add for Expr {
    type Output = EvalResult<Self>;

    fn add(self, other: Self) -> Self::Output {
        let sum = match (self, other) {
            (Int(a), Int(b)) => Int(a + b),
            (Int(a), Float(b)) => Float(a as f64 + b),
            (Float(a), Int(b)) => Float(a + b as f64),
            (Float(a), Float(b)) => Float(a + b),
            (Str(a), Str(b)) => Str(format!("{}{}", a, b)),
            (_, _) => {
                return Err(EvalError::TypeError);
            }
        };
        Ok(sum)
    }
}

impl Sub for Expr {
    type Output = EvalResult<Self>;

    fn sub(self, other: Self) -> Self::Output {
        let diff = match (self, other) {
            (Int(a), Int(b)) => Int(a - b),
            (Int(a), Float(b)) => Float(a as f64 - b),
            (Float(a), Int(b)) => Float(a - b as f64),
            (Float(a), Float(b)) => Float(a - b),
            (_, _) => {
                return Err(EvalError::TypeError);
            }
        };
        Ok(diff)
    }
}

impl Mul for Expr {
    type Output = EvalResult<Self>;

    fn mul(self, other: Self) -> Self::Output {
        let product = match (self, other) {
            (Int(a), Int(b)) => Int(a * b),
            (Int(a), Float(b)) => Float(a as f64 * b),
            (Float(a), Int(b)) => Float(a * b as f64),
            (Float(a), Float(b)) => Float(a * b),
            (_, _) => {
                return Err(EvalError::TypeError);
            }
        };
        Ok(product)
    }
}

impl Div for Expr {
    type Output = EvalResult<Self>;

    fn div(self, other: Self) -> Self::Output {
        let quotient = match (self, other) {
            (Int(a), Int(b)) => Int(a / b),
            (Int(a), Float(b)) => Float(a as f64 / b),
            (Float(a), Int(b)) => Float(a / b as f64),
            (Float(a), Float(b)) => Float(a / b),
            (_, _) => {
                return Err(EvalError::TypeError);
            }
        };
        Ok(quotient)
    }
}
