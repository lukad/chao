use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

use colored::*;
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
    Quote(Box<Expr>),
    Fun(Function, Arguments),
    Special(Function, Arguments),
    List(Vec<Expr>),
    Error(String),
}

impl Eq for Expr {}

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
            (Fun(a, _), Fun(b, _)) => PartialOrd::partial_cmp(a, b),
            (Special(a, _), Special(b, _)) => PartialOrd::partial_cmp(a, b),
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
            Fun(_, args) => write!(f, "<function {:?}>", args),
            Special(_, _) => write!(f, "<special>"),
            List(xs) => write!(f, "({})", xs.iter().map(|x| format!("{:?}", x)).join(" ")),
            Error(err) => write!(f, "ERROR: {}", err),
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
            Fun(_, _) => write!(f, "{}", format!("{:?}", self).magenta()),
            Special(_, _) => write!(f, "{}", format!("{:?}", self).magenta()),
            List(xs) => write!(f, "({})", xs.iter().map(|x| format!("{}", x)).join(" ")),
            Error(_) => write!(f, "{}", format!("{:?}", self).red()),
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
            (a, b) => Error(format!("Can't add {:?} and {:?}", a, b)),
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
            (a, b) => Error(format!("Can't subtract {:?} from {:?}", b, a)),
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
            (a, b) => Error(format!("Can't multiply {:?} with {:?}", a, b)),
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
            (a, b) => Error(format!("Can't divide {:?} by {:?}", a, b)),
        }
    }
}

#[derive(Clone)]
pub enum Function {
    Builtin(fn(&mut Env) -> Expr),
    Dynamic(Box<Expr>),
}

use self::Function::*;

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Dynamic(a), Dynamic(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Function {}

impl PartialOrd for Function {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        None
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq)]
pub enum Arguments {
    Variadic,
    Fixed(Vec<String>),
}

impl fmt::Debug for Arguments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Arguments::Variadic => write!(f, "..."),
            Arguments::Fixed(args) => {
                let mut result = String::new();
                for (i, arg) in args.iter().enumerate() {
                    result.push_str(arg.as_str());
                    if i + 1 < args.len() {
                        result.push_str(", ");
                    }
                }
                write!(f, "({})", result)
            }
        }
    }
}
