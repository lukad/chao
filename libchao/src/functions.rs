use crate::{
    Env, Expr, Interpreter,
    interpreter::{EvalError, EvalResult},
};

pub type BuiltinFn = fn(&mut Interpreter, &[Expr]) -> EvalResult<Expr>;

#[derive(Debug, Clone)]
pub enum Callable {
    Builtin(Builtin),
    Lambda(Lambda),
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Callable::Builtin(a), Callable::Builtin(b)) => a == b,
            (Callable::Lambda(a), Callable::Lambda(b)) => a == b,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Builtin {
    pub arity: Arity,
    pub mode: EvalMode,
    pub f: BuiltinFn,
}

impl PartialEq for Builtin {
    fn eq(&self, other: &Self) -> bool {
        self.arity == other.arity
            && self.mode == other.mode
            && std::ptr::fn_addr_eq(self.f, other.f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arity {
    Exact(usize),
    AtLeast(usize),
    Any,
}

impl Arity {
    pub fn check(&self, args: &[Expr]) -> EvalResult<()> {
        match self {
            Arity::Exact(n) if args.len() == *n => Ok(()),
            Arity::AtLeast(n) if args.len() >= *n => Ok(()),
            Arity::Any => Ok(()),
            _ => Err(EvalError::ArityMismatch),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalMode {
    Eager,
    Raw,
}

#[derive(Debug, Clone)]
pub struct Lambda {
    pub params: LambdaParams,
    pub body: Box<Expr>,
    pub env: Env,
}

impl PartialEq for Lambda {
    fn eq(&self, other: &Self) -> bool {
        self.params == other.params && self.body == other.body
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LambdaParams {
    Fixed(Vec<String>),
}

impl LambdaParams {
    pub fn bind(&self, args: &[Expr]) -> EvalResult<Vec<(String, Expr)>> {
        match self {
            LambdaParams::Fixed(names) if names.len() == args.len() => {
                Ok(names.iter().cloned().zip(args.iter().cloned()).collect())
            }
            LambdaParams::Fixed(_) => Err(EvalError::ArityMismatch),
        }
    }
}
