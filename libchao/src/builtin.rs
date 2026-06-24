use std::rc::Rc;

use crate::{
    Interpreter,
    env::Env,
    expr::{
        Arguments,
        Expr::{self, *},
        Function,
    },
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

fn add(interpreter: &mut Interpreter) -> Expr {
    if let Some(List(args)) = interpreter.env.get("varargs") {
        args.iter().fold(Int(0), |acc, x| acc + x.clone())
    } else {
        Error("could not fetch arguments".to_string())
    }
}

fn sub(interpreter: &mut Interpreter) -> Expr {
    if let Some(List(args)) = interpreter.env.get("varargs") {
        match &args[..] {
            [x] => Int(0) - x.clone(),
            [head, tail @ ..] => tail.iter().fold(head.clone(), |acc, x| acc - x.clone()),
            [] => Error("sub requires at least one argument".to_string()),
        }
    } else {
        Error("could not fetch arguments".to_string())
    }
}

fn mul(interpreter: &mut Interpreter) -> Expr {
    if let Some(List(args)) = interpreter.env.get("varargs") {
        args.iter().fold(Int(1), |acc, x| acc * x.clone())
    } else {
        Error("could not fetch arguments".to_string())
    }
}

fn div(interpreter: &mut Interpreter) -> Expr {
    if let Some(List(args)) = interpreter.env.get("varargs") {
        match &args[..] {
            [head] => Int(1) / head.clone(),
            [head, tail @ ..] => tail.iter().fold(head.clone(), |acc, x| acc / x.clone()),
            [] => Error("div requires at least one argument".to_string()),
        }
    } else {
        Error("could not fetch arguments".to_string())
    }
}

fn iff(interpreter: &mut Interpreter) -> Expr {
    match (
        interpreter.env.get("cond").map(|e| interpreter.eval(&e)),
        interpreter.env.get("expr1"),
        interpreter.env.get("expr2"),
    ) {
        (Some(Error(e)), _, _) => Error(e),
        (Some(cond), Some(expr1), Some(expr2)) => match cond {
            Bool(true) => interpreter.eval(&expr1),
            Bool(false) => interpreter.eval(&expr2),
            _ => Error("not a bool".to_string()),
        },
        _ => Error("not enough arguments supplied to if".to_string()),
    }
}

fn eq(interpreter: &mut Interpreter) -> Expr {
    if let Some(List(args)) = interpreter.env.get("varargs") {
        match &args[..] {
            [Nil] => Bool(false),
            [_head] => Bool(true),
            [head, tail @ ..] => Bool(tail.iter().all(|x| x == head)),
            [] => Error("eq requires a at least one argument".to_string()),
        }
    } else {
        Error("eq requires a at least one argument".to_string())
    }
}

fn lt(interpreter: &mut Interpreter) -> Expr {
    match (interpreter.env.get("a"), interpreter.env.get("b")) {
        (Some(a), Some(b)) => Bool(a < b),
        _ => Error("< requires two arguments".to_string()),
    }
}

fn gt(interpreter: &mut Interpreter) -> Expr {
    match (interpreter.env.get("a"), interpreter.env.get("b")) {
        (Some(a), Some(b)) => Bool(a > b),
        _ => Error("> requires two arguments".to_string()),
    }
}

fn lambda(interpreter: &mut Interpreter) -> Expr {
    let body = interpreter.env.get("body").unwrap();
    match interpreter.env.get("args").unwrap() {
        List(args) => {
            let mut arguments = vec![];
            for arg in args.iter() {
                match arg {
                    Symbol(s) => arguments.push(s.clone()),
                    _ => return Error("lambda arguments must be symbols".to_string()),
                }
            }

            let captured_env = interpreter
                .env
                .enclosing()
                .unwrap_or_else(|| interpreter.env.clone());

            Fun(
                Function::Dynamic(Box::new(body), captured_env),
                Arguments::Fixed(arguments),
            )
        }
        Nil => {
            let captured_env = interpreter
                .env
                .enclosing()
                .unwrap_or_else(|| interpreter.env.clone());
            Fun(
                Function::Dynamic(Box::new(body), captured_env),
                Arguments::Fixed(vec![]),
            )
        }
        _ => Error("First lambda argument must be a list of argument names".to_string()),
    }
}

fn set(interpreter: &mut Interpreter) -> Expr {
    match (
        interpreter.env.get("name").unwrap(),
        interpreter.env.get("value").unwrap(),
    ) {
        (Symbol(s), expr) => {
            interpreter.env.insert_in_enclosing(s, expr.clone());
            expr
        }
        (other, _) => Error(format!("Variable name is not a symbol: {:?}", other)),
    }
}

fn intern(interpreter: &mut Interpreter) -> Expr {
    match interpreter.env.get("string").unwrap() {
        Str(s) => Symbol(s),
        other => Error(format!("Can't intern {:?}", other)),
    }
}
