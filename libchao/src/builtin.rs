use std::rc::Rc;

use crate::{
    Interpreter,
    env::Env,
    expr::{
        Arguments,
        Expr::{self, *},
        Function,
    },
    interpreter::{EvalError, EvalResult},
};

pub fn load(env: &mut Env) {
    env.insert(
        "+".to_string(),
        Fun(Function::Builtin(Rc::new(add)), Arguments::Variadic),
    );
    env.insert(
        "-".to_string(),
        Fun(Function::Builtin(Rc::new(sub)), Arguments::Variadic),
    );
    env.insert(
        "*".to_string(),
        Fun(Function::Builtin(Rc::new(mul)), Arguments::Variadic),
    );
    env.insert(
        "/".to_string(),
        Fun(Function::Builtin(Rc::new(div)), Arguments::Variadic),
    );
    env.insert(
        "=".to_string(),
        Fun(Function::Builtin(Rc::new(eq)), Arguments::Variadic),
    );
    env.insert(
        ">".to_string(),
        Fun(
            Function::Builtin(Rc::new(gt)),
            Arguments::Fixed(vec!["a".to_string(), "b".to_string()]),
        ),
    );
    env.insert(
        "<".to_string(),
        Fun(
            Function::Builtin(Rc::new(lt)),
            Arguments::Fixed(vec!["a".to_string(), "b".to_string()]),
        ),
    );
    env.insert(
        "if".to_string(),
        Special(
            Function::Builtin(Rc::new(iff)),
            Arguments::Fixed(vec![
                "cond".to_string(),
                "expr1".to_string(),
                "expr2".to_string(),
            ]),
        ),
    );
    env.insert(
        "lambda".to_string(),
        Special(
            Function::Builtin(Rc::new(lambda)),
            Arguments::Fixed(vec!["args".to_string(), "body".to_string()]),
        ),
    );
    env.insert(
        "set".to_string(),
        Fun(
            Function::Builtin(Rc::new(set)),
            Arguments::Fixed(vec!["name".to_string(), "value".to_string()]),
        ),
    );
    env.insert(
        "intern".to_string(),
        Fun(
            Function::Builtin(Rc::new(intern)),
            Arguments::Fixed(vec!["string".to_string()]),
        ),
    );
}

fn add(interpreter: &mut Interpreter) -> EvalResult<Expr> {
    if let Some(List(args)) = interpreter.env.get("varargs") {
        args.iter().try_fold(Int(0), |acc, x| acc + x.clone())
    } else {
        Err(EvalError::ArityMismatch)
    }
}

fn sub(interpreter: &mut Interpreter) -> EvalResult<Expr> {
    if let Some(List(args)) = interpreter.env.get("varargs") {
        match &args[..] {
            [x] => Int(0) - x.clone(),
            [head, tail @ ..] => tail.iter().try_fold(head.clone(), |acc, x| acc - x.clone()),
            [] => Err(EvalError::ArityMismatch),
        }
    } else {
        Err(EvalError::ArityMismatch)
    }
}

fn mul(interpreter: &mut Interpreter) -> EvalResult<Expr> {
    if let Some(List(args)) = interpreter.env.get("varargs") {
        args.iter().try_fold(Int(1), |acc, x| acc * x.clone())
    } else {
        Err(EvalError::ArityMismatch)
    }
}

fn div(interpreter: &mut Interpreter) -> EvalResult<Expr> {
    if let Some(List(args)) = interpreter.env.get("varargs") {
        match &args[..] {
            [head] => Int(1) / head.clone(),
            [head, tail @ ..] => tail.iter().try_fold(head.clone(), |acc, x| acc / x.clone()),
            [] => Err(EvalError::ArityMismatch),
        }
    } else {
        Err(EvalError::ArityMismatch)
    }
}

fn iff(interpreter: &mut Interpreter) -> EvalResult<Expr> {
    let cond_expr = interpreter
        .env
        .get("cond")
        .ok_or(EvalError::ArityMismatch)?;
    let cond = interpreter.eval(&cond_expr)?.is_truthy();
    match (interpreter.env.get("expr1"), interpreter.env.get("expr2")) {
        (Some(expr1), Some(expr2)) => match cond {
            true => interpreter.eval(&expr1),
            false => interpreter.eval(&expr2),
        },
        _ => Err(EvalError::ArityMismatch),
    }
}

fn eq(interpreter: &mut Interpreter) -> EvalResult<Expr> {
    if let Some(List(args)) = interpreter.env.get("varargs") {
        match &args[..] {
            [Nil] => Ok(Bool(false)),
            [_head] => Ok(Bool(true)),
            [head, tail @ ..] => Ok(Bool(tail.iter().all(|x| x == head))),
            [] => Err(EvalError::ArityMismatch),
        }
    } else {
        Err(EvalError::ArityMismatch)
    }
}

fn lt(interpreter: &mut Interpreter) -> EvalResult<Expr> {
    match (interpreter.env.get("a"), interpreter.env.get("b")) {
        (Some(a), Some(b)) => Ok(Bool(a < b)),
        _ => Err(EvalError::ArityMismatch),
    }
}

fn gt(interpreter: &mut Interpreter) -> EvalResult<Expr> {
    match (interpreter.env.get("a"), interpreter.env.get("b")) {
        (Some(a), Some(b)) => Ok(Bool(a > b)),
        _ => Err(EvalError::ArityMismatch),
    }
}

fn lambda(interpreter: &mut Interpreter) -> EvalResult<Expr> {
    let body = interpreter.env.get("body").unwrap();
    match interpreter.env.get("args").unwrap() {
        List(args) => {
            let mut arguments = vec![];
            for arg in args.iter() {
                match arg {
                    Symbol(s) => arguments.push(s.clone()),
                    _ => return Err(EvalError::LambdaNameMustBeSymbol),
                }
            }

            let captured_env = interpreter
                .env
                .enclosing()
                .unwrap_or_else(|| interpreter.env.clone());

            Ok(Fun(
                Function::Dynamic(Box::new(body), captured_env),
                Arguments::Fixed(arguments),
            ))
        }
        Nil => {
            let captured_env = interpreter
                .env
                .enclosing()
                .unwrap_or_else(|| interpreter.env.clone());
            Ok(Fun(
                Function::Dynamic(Box::new(body), captured_env),
                Arguments::Fixed(vec![]),
            ))
        }
        _ => Err(EvalError::LambdaNameMustBeSymbol),
    }
}

fn set(interpreter: &mut Interpreter) -> EvalResult<Expr> {
    match (
        interpreter.env.get("name").unwrap(),
        interpreter.env.get("value").unwrap(),
    ) {
        (Symbol(s), expr) => {
            interpreter.env.insert_in_enclosing(s, expr.clone());
            Ok(expr)
        }
        _ => Err(EvalError::VariableNameMustBeSymbol),
    }
}

fn intern(interpreter: &mut Interpreter) -> EvalResult<Expr> {
    match interpreter.env.get("string").unwrap() {
        Str(s) => Ok(Symbol(s)),
        _ => Err(EvalError::CanOnlyInterStrings),
    }
}
