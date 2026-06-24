use crate::{
    Env, Expr, builtin,
    expr::{Arguments, Function},
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

    fn with_child_env<T>(&mut self, f: impl FnOnce(&mut Self) -> EvalResult<T>) -> EvalResult<T> {
        self.with_env(self.env.child(), f)
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
            | Expr::Fun(_, _)
            | Expr::Special(_, _) => Ok(expr.clone()),
            Expr::Symbol(symbol) => Ok(self.env.get(symbol).unwrap_or_else(|| Expr::Nil)),
            Expr::Quote(expr) => Ok(*expr.clone()),
            Expr::List(list) => self.eval_list(list),
        }
    }

    pub fn eval_list(&mut self, list: &[Expr]) -> EvalResult<Expr> {
        match list {
            [head, tail @ ..] => match self.eval(head)? {
                Expr::Fun(Function::Builtin(builtin), arg_names) => {
                    let args = tail
                        .iter()
                        .map(|e| self.eval(e))
                        .collect::<Result<Vec<_>, _>>()?;
                    self.with_child_env(|interpreter| {
                        interpreter.bind_args(&args, &arg_names)?;
                        builtin(interpreter)
                    })
                }
                Expr::Fun(Function::Dynamic(body, captured_env), arg_names) => {
                    let args = tail
                        .iter()
                        .map(|e| self.eval(e))
                        .collect::<Result<Vec<_>, _>>()?;
                    self.with_env(captured_env.child(), |interpreter| {
                        interpreter.bind_args(&args, &arg_names)?;
                        interpreter.eval(&body)
                    })
                }
                Expr::Special(Function::Builtin(builtin), arg_names) => {
                    self.with_child_env(|interpreter| {
                        interpreter.bind_args(tail, &arg_names)?;
                        builtin(interpreter)
                    })
                }
                Expr::Special(Function::Dynamic(body, captured_env), arg_names) => {
                    self.with_env(captured_env.child(), |interpreter| {
                        interpreter.bind_args(tail, &arg_names)?;
                        interpreter.eval(&body)
                    })
                }
                _ => Err(EvalError::CanOnlyApplyFunctions),
            },
            [] => Ok(Expr::Nil),
        }
    }

    fn bind_args(&mut self, args: &[Expr], params: &Arguments) -> EvalResult<()> {
        match params {
            Arguments::Variadic => self
                .env
                .insert("varargs".to_string(), Expr::List(args.to_vec())),
            Arguments::Fixed(names) => {
                if args.len() != names.len() {
                    panic!("Wrong argument count");
                    // TODO: return Err
                }
                for (name, arg) in names.iter().zip(args.iter()) {
                    self.env.insert(name.clone(), arg.clone());
                }
            }
        }
        Ok(())
    }
}
