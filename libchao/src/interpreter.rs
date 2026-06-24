use crate::{
    Env, Expr, builtin,
    expr::{Arguments, Function},
};

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

    fn with_child_env<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> T {
        self.with_env(self.env.child(), f)
    }

    fn with_env<T>(&mut self, env: Env, f: impl FnOnce(&mut Self) -> T) -> T {
        let previous = std::mem::replace(&mut self.env, env);
        let result = f(self);
        self.env = previous;
        result
    }

    pub fn eval(&mut self, expr: &Expr) -> Expr {
        match expr {
            Expr::Nil
            | Expr::Bool(_)
            | Expr::Int(_)
            | Expr::Float(_)
            | Expr::Str(_)
            | Expr::Fun(_, _)
            | Expr::Special(_, _)
            | Expr::Error(_) => expr.clone(),
            Expr::Symbol(symbol) => self.env.get(symbol).unwrap_or_else(|| Expr::Nil),
            Expr::Quote(expr) => *expr.clone(),
            Expr::List(list) => self.eval_list(list),
        }
    }

    pub fn eval_list(&mut self, list: &[Expr]) -> Expr {
        match list {
            [head, tail @ ..] => match self.eval(head) {
                Expr::Fun(Function::Builtin(builtin), arg_names) => {
                    let args = tail.iter().map(|e| self.eval(e)).collect::<Vec<_>>();
                    self.with_child_env(|interpreter| {
                        interpreter.bind_args(&args, &arg_names);
                        builtin(interpreter)
                    })
                }
                Expr::Fun(Function::Dynamic(body, captured_env), arg_names) => {
                    let args = tail.iter().map(|e| self.eval(e)).collect::<Vec<_>>();
                    self.with_env(captured_env.child(), |interpreter| {
                        interpreter.bind_args(&args, &arg_names);
                        interpreter.eval(&body)
                    })
                }
                Expr::Special(Function::Builtin(builtin), arg_names) => {
                    self.with_child_env(|interpreter| {
                        interpreter.bind_args(tail, &arg_names);
                        builtin(interpreter)
                    })
                }
                Expr::Special(Function::Dynamic(body, captured_env), arg_names) => {
                    self.with_env(captured_env.child(), |interpreter| {
                        interpreter.bind_args(tail, &arg_names);
                        interpreter.eval(&body)
                    })
                }
                other => Expr::Error(format!("Can't apply {:?} ({:?})", head, other)),
            },
            [] => Expr::Nil,
        }
    }

    fn bind_args(&mut self, args: &[Expr], params: &Arguments) {
        match params {
            Arguments::Variadic => self
                .env
                .insert("varargs".to_string(), Expr::List(args.to_vec())),
            Arguments::Fixed(names) => {
                if args.len() != names.len() {
                    panic!("Wrong argument count");
                }
                for (name, arg) in names.iter().zip(args.iter()) {
                    self.env.insert(name.clone(), arg.clone());
                }
            }
        }
    }
}
