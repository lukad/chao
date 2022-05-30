use env::Env;
use expr::{Arguments, Expr, Expr::*, Function};

pub fn load(env: &mut Env) {
    env.insert(
        "+".to_string(),
        Fun(Function::Builtin(add), Arguments::Variadic),
    );
    env.insert(
        "-".to_string(),
        Fun(Function::Builtin(sub), Arguments::Variadic),
    );
    env.insert(
        "*".to_string(),
        Fun(Function::Builtin(mul), Arguments::Variadic),
    );
    env.insert(
        "/".to_string(),
        Fun(Function::Builtin(div), Arguments::Variadic),
    );
    env.insert(
        "=".to_string(),
        Fun(Function::Builtin(eq), Arguments::Variadic),
    );
    env.insert(
        ">".to_string(),
        Fun(
            Function::Builtin(gt),
            Arguments::Fixed(vec!["a".to_string(), "b".to_string()]),
        ),
    );
    env.insert(
        "<".to_string(),
        Fun(
            Function::Builtin(lt),
            Arguments::Fixed(vec!["a".to_string(), "b".to_string()]),
        ),
    );
    env.insert(
        "if".to_string(),
        Special(
            Function::Builtin(iff),
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
            Function::Builtin(lambda),
            Arguments::Fixed(vec!["args".to_string(), "body".to_string()]),
        ),
    );
    env.insert(
        "set".to_string(),
        Fun(
            Function::Builtin(set),
            Arguments::Fixed(vec!["name".to_string(), "value".to_string()]),
        ),
    );
    env.insert(
        "intern".to_string(),
        Fun(
            Function::Builtin(intern),
            Arguments::Fixed(vec!["string".to_string()]),
        ),
    );
}

fn add(env: &mut Env) -> Expr {
    if let Some(List(args)) = env.get("varargs".to_string()) {
        args.iter().fold(Int(0), |acc, x| acc + x.clone())
    } else {
        Error("could not fetch arguments".to_string())
    }
}

fn sub(env: &mut Env) -> Expr {
    if let Some(List(args)) = env.get("varargs".to_string()) {
        match &args[..] {
            [x] => Int(0) - x.clone(),
            [head, tail @ ..] => tail.iter().fold(head.clone(), |acc, x| acc - x.clone()),
            [] => Error("sub requires at least one argument".to_string()),
        }
    } else {
        Error("could not fetch arguments".to_string())
    }
}

fn mul(env: &mut Env) -> Expr {
    if let Some(List(args)) = env.get("varargs".to_string()) {
        args.iter().fold(Int(1), |acc, x| acc * x.clone())
    } else {
        Error("could not fetch arguments".to_string())
    }
}

fn div(env: &mut Env) -> Expr {
    if let Some(List(args)) = env.get("varargs".to_string()) {
        match &args[..] {
            [head] => Int(1) / head.clone(),
            [head, tail @ ..] => tail.iter().fold(head.clone(), |acc, x| acc / x.clone()),
            [] => Error("div requires at least one argument".to_string()),
        }
    } else {
        Error("could not fetch arguments".to_string())
    }
}

fn iff(env: &mut Env) -> Expr {
    match (
        env.get("cond".to_string()).and_then(|e| Some(env.eval(&e))),
        env.get("expr1".to_string()),
        env.get("expr2".to_string()),
    ) {
        (Some(Error(e)), _, _) => Error(e),
        (Some(cond), Some(expr1), Some(expr2)) => match cond {
            Bool(true) => env.eval(&expr1),
            Bool(false) => env.eval(&expr2),
            _ => Error("not a bool".to_string()),
        },
        _ => Error("not enough arguments supplied to if".to_string()),
    }
}

fn eq(env: &mut Env) -> Expr {
    if let Some(List(args)) = env.get("varargs".to_string()) {
        match &args[..] {
            [Nil] => Bool(false),
            [_head] => Bool(true),
            [head, tail @ ..] => Bool(tail.iter().all(|ref x| *x == head)),
            [] => Error("eq requires a at least one argument".to_string()),
        }
    } else {
        Error("eq requires a at least one argument".to_string())
    }
}

fn lt(env: &mut Env) -> Expr {
    match (env.get("a".to_string()), env.get("b".to_string())) {
        (Some(a), Some(b)) => Bool(a < b),
        _ => Error("< requires two arguments".to_string()),
    }
}

fn gt(env: &mut Env) -> Expr {
    match (env.get("a".to_string()), env.get("b".to_string())) {
        (Some(a), Some(b)) => Bool(a > b),
        _ => Error("> requires two arguments".to_string()),
    }
}

fn lambda(env: &mut Env) -> Expr {
    let body = env.get("body".to_string()).unwrap();
    match env.get("args".to_string()).unwrap() {
        List(args) => {
            let mut arguments = vec![];
            for arg in args.iter() {
                match arg {
                    Symbol(s) => arguments.push(s.clone()),
                    _ => return Error("lambda arguments must be symbols".to_string()),
                }
            }
            Fun(
                Function::Dynamic(Box::new(body)),
                Arguments::Fixed(arguments),
            )
        }
        Nil => Fun(Function::Dynamic(Box::new(body)), Arguments::Fixed(vec![])),
        _ => Error("First lambda argument must be a list of argument names".to_string()),
    }
}

fn set(env: &mut Env) -> Expr {
    match (
        env.get("name".to_string()).unwrap(),
        env.get("value".to_string()).unwrap(),
    ) {
        (Symbol(s), expr) => {
            env.insert_parent(s, expr.clone());
            expr
        }
        (other, _) => Error(format!("Variable name is not a symbol: {:?}", other)),
    }
}

fn intern(env: &mut Env) -> Expr {
    match env.get("string".to_string()).unwrap() {
        Str(s) => Symbol(s),
        other => Error(format!("Can't intern {:?}", other)),
    }
}
