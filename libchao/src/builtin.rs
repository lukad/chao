use crate::{
    Interpreter,
    env::Env,
    expr::Expr::{self, *},
    functions::{Arity, Builtin, Callable, EvalMode, Lambda, LambdaParams},
    interpreter::{EvalError, EvalResult},
};

fn insert_builtin(
    env: &mut Env,
    name: &str,
    mode: EvalMode,
    arity: Arity,
    f: fn(&mut Interpreter, &[Expr]) -> EvalResult<Expr>,
) {
    env.insert(
        name.to_string(),
        Expr::Callable(Callable::Builtin(Builtin { arity, mode, f })),
    );
}

pub fn load(env: &mut Env) {
    insert_builtin(env, "+", EvalMode::Eager, Arity::Any, add);
    insert_builtin(env, "-", EvalMode::Eager, Arity::AtLeast(1), sub);
    insert_builtin(env, "*", EvalMode::Eager, Arity::Any, mul);
    insert_builtin(env, "/", EvalMode::Eager, Arity::AtLeast(1), div);
    insert_builtin(env, "=", EvalMode::Eager, Arity::AtLeast(1), eq);
    insert_builtin(env, ">", EvalMode::Eager, Arity::Exact(2), gt);
    insert_builtin(env, "<", EvalMode::Eager, Arity::Exact(2), lt);
    insert_builtin(env, "if", EvalMode::Raw, Arity::Exact(3), iff);
    insert_builtin(env, "intern", EvalMode::Eager, Arity::Exact(1), intern);
    insert_builtin(env, "lambda", EvalMode::Raw, Arity::Exact(2), lambda);
    insert_builtin(env, "set", EvalMode::Raw, Arity::Exact(2), set);
    insert_builtin(env, "def", EvalMode::Raw, Arity::AtLeast(2), def);
}

fn add(_: &mut Interpreter, args: &[Expr]) -> EvalResult<Expr> {
    args.iter().try_fold(Int(0), |acc, x| acc + x.clone())
}

fn sub(_: &mut Interpreter, args: &[Expr]) -> EvalResult<Expr> {
    match args {
        [x] => Int(0) - x.clone(),
        [head, tail @ ..] => tail.iter().try_fold(head.clone(), |acc, x| acc - x.clone()),
        [] => Err(EvalError::ArityMismatch),
    }
}

fn mul(_: &mut Interpreter, args: &[Expr]) -> EvalResult<Expr> {
    args.iter().try_fold(Int(1), |acc, x| acc * x.clone())
}

fn div(_: &mut Interpreter, args: &[Expr]) -> EvalResult<Expr> {
    match args {
        [head] => Int(1) / head.clone(),
        [head, tail @ ..] => tail.iter().try_fold(head.clone(), |acc, x| acc / x.clone()),
        [] => Err(EvalError::ArityMismatch),
    }
}

fn iff(interpreter: &mut Interpreter, args: &[Expr]) -> EvalResult<Expr> {
    let [cond, then_branch, else_branch] = args else {
        return Err(EvalError::ArityMismatch);
    };
    if interpreter.eval(cond)?.is_truthy() {
        interpreter.eval(then_branch)
    } else {
        interpreter.eval(else_branch)
    }
}

fn eq(_: &mut Interpreter, args: &[Expr]) -> EvalResult<Expr> {
    match args {
        [head] if head.is_falsy() => Ok(Bool(false)),
        [_head] => Ok(Bool(true)),
        [head, tail @ ..] => Ok(Bool(tail.iter().all(|x| x == head))),
        [] => Err(EvalError::ArityMismatch),
    }
}

fn lt(_: &mut Interpreter, args: &[Expr]) -> EvalResult<Expr> {
    match args {
        [a, b] => Ok(Bool(a < b)),
        _ => Err(EvalError::ArityMismatch),
    }
}

fn gt(_: &mut Interpreter, args: &[Expr]) -> EvalResult<Expr> {
    match args {
        [a, b] => Ok(Bool(a > b)),
        _ => Err(EvalError::ArityMismatch),
    }
}

fn lambda(interpreter: &mut Interpreter, args: &[Expr]) -> EvalResult<Expr> {
    match args {
        [List(params), body] => {
            let params = parse_lambda_params(params)?;

            Ok(Callable(Callable::Lambda(Lambda {
                params,
                body: Box::new(body.clone()),
                env: interpreter.env.clone(),
            })))
        }
        [Nil, body] => Ok(Callable(Callable::Lambda(Lambda {
            params: LambdaParams::Fixed(vec![]),
            body: Box::new(body.clone()),
            env: interpreter.env.clone(),
        }))),
        _ => Err(EvalError::LambdaNameMustBeSymbol),
    }
}

fn set(interpreter: &mut Interpreter, args: &[Expr]) -> EvalResult<Expr> {
    match args {
        [Symbol(name), value_expr] => {
            let value = interpreter.eval(value_expr)?;

            if interpreter.env.assign(name, value.clone()) {
                Ok(value)
            } else {
                Err(EvalError::UnboundVariable)
            }
        }
        _ => Err(EvalError::VariableNameMustBeSymbol),
    }
}

fn intern(_: &mut Interpreter, args: &[Expr]) -> EvalResult<Expr> {
    match args {
        [Str(s)] => Ok(Symbol(s.clone())),
        [_] => Err(EvalError::CanOnlyInterStrings),
        _ => Err(EvalError::ArityMismatch),
    }
}

fn def(interpreter: &mut Interpreter, args: &[Expr]) -> EvalResult<Expr> {
    match args {
        [Symbol(name), value] => {
            let value = interpreter.eval(value)?;
            interpreter.env.insert(name.clone(), value.clone());
            Ok(value)
        }
        [Symbol(name), List(params), body] => {
            let params = parse_lambda_params(params)?;

            let value = Callable(Callable::Lambda(Lambda {
                params,
                body: Box::new(body.clone()),
                env: interpreter.env.clone(),
            }));

            interpreter.env.insert(name.clone(), value.clone());
            Ok(value)
        }
        [Symbol(name), Nil, body] => {
            let value = Callable(Callable::Lambda(Lambda {
                params: LambdaParams::Fixed(vec![]),
                body: Box::new(body.clone()),
                env: interpreter.env.clone(),
            }));

            interpreter.env.insert(name.clone(), value.clone());
            Ok(value)
        }
        _ => Err(EvalError::ArgumentError),
    }
}

fn parse_lambda_params(params: &[Expr]) -> EvalResult<LambdaParams> {
    let mut arg_names = vec![];
    for param in params {
        match param {
            Symbol(name) => arg_names.push(name.clone()),
            _ => return Err(EvalError::DefParamMustBeSymbol),
        }
    }

    Ok(LambdaParams::Fixed(arg_names))
}
