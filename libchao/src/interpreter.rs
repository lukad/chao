use crate::{
    Env, Expr, builtin,
    functions::{Callable, EvalMode},
};

#[derive(Debug, Clone, thiserror::Error)]
pub enum EvalError {
    #[error("missing varargs")]
    MissingVarArgs,
    #[error("arity mismatch")]
    ArityMismatch,
    #[error("lambda name must be a symbol")]
    LambdaNameMustBeSymbol,
    #[error("variable name must be a symbol")]
    VariableNameMustBeSymbol,
    #[error("can only intern strings")]
    CanOnlyInterStrings,
    #[error("can only apply functions")]
    CanOnlyApplyFunctions,
    #[error("type error")]
    TypeError,
    #[error("def name must be a symbol")]
    DefNameMustBeSymbol,
    #[error("missing argument")]
    ArgumentError,
    #[error("def param must be a symbol")]
    DefParamMustBeSymbol,
    #[error("unbound variable")]
    UnboundVariable,
    #[error("unquote outside quasiquote")]
    UnquoteOutsideQuasiquote,
}

pub type EvalResult<T> = Result<T, EvalError>;

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub(crate) env: Env,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Env::new();
        builtin::load(&mut env);
        Self { env }
    }

    pub(crate) fn with_env<T>(
        &mut self,
        env: Env,
        f: impl FnOnce(&mut Self) -> EvalResult<T>,
    ) -> EvalResult<T> {
        let previous = std::mem::replace(&mut self.env, env);
        let result = f(self);
        self.env = previous;
        result
    }

    pub fn eval(&mut self, expr: &Expr) -> EvalResult<Expr> {
        match expr {
            Expr::Nil
            | Expr::Bool(_)
            | Expr::Int(_)
            | Expr::Float(_)
            | Expr::Str(_)
            | Expr::Callable(_) => Ok(expr.clone()),
            Expr::Symbol(symbol) => Ok(self.env.get(symbol).unwrap_or(Expr::Nil)),
            Expr::Quote(expr) => Ok(*expr.clone()),
            Expr::QuasiQuote(expr) => self.eval_quasiquote(expr, 1),
            Expr::Unquote(_expr) => Err(EvalError::UnquoteOutsideQuasiquote),
            Expr::List(list) => self.eval_list(list),
        }
    }

    fn eval_quasiquote(&mut self, expr: &Expr, depth: usize) -> EvalResult<Expr> {
        match expr {
            Expr::Unquote(inner) if depth == 1 => self.eval(inner),
            Expr::Unquote(inner) => Ok(Expr::Unquote(Box::new(
                self.eval_quasiquote(inner, depth - 1)?,
            ))),
            Expr::QuasiQuote(inner) => Ok(Expr::QuasiQuote(Box::new(
                self.eval_quasiquote(inner, depth + 1)?,
            ))),
            Expr::List(items) => items
                .iter()
                .map(|item| self.eval_quasiquote(item, depth))
                .collect::<EvalResult<Vec<_>>>()
                .map(Expr::List),
            other => Ok(other.clone()),
        }
    }

    fn eval_list(&mut self, list: &[Expr]) -> EvalResult<Expr> {
        let [head, tail @ ..] = list else {
            return Ok(Expr::Nil);
        };

        let callable = self.eval(head)?;

        match callable {
            Expr::Callable(Callable::Builtin(builtin)) => {
                let args = match builtin.mode {
                    EvalMode::Eager => self.eval_args(tail)?,
                    EvalMode::Raw => tail.to_vec(),
                };

                builtin.arity.check(&args)?;
                (builtin.f)(self, &args)
            }
            Expr::Callable(Callable::Lambda(lambda)) => {
                let args = self.eval_args(tail)?;
                let bindings = lambda.params.bind(&args)?;

                self.with_env(lambda.env.child_with(bindings), |interpreter| {
                    interpreter.eval(&lambda.body)
                })
            }
            Expr::Callable(Callable::Macro(macro_)) => {
                let bindings = macro_.params.bind(tail)?;
                let expansion = self.with_env(macro_.env.child_with(bindings), |interpreter| {
                    interpreter.eval(&macro_.body)
                })?;
                self.eval(&expansion)
            }
            _ => Err(EvalError::CanOnlyApplyFunctions),
        }
    }

    fn eval_args(&mut self, args: &[Expr]) -> EvalResult<Vec<Expr>> {
        args.iter().map(|arg| self.eval(arg)).collect()
    }
}
