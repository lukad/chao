use std::collections::HashMap;

use expr::{
    Arguments,
    Expr::{self, *},
    Function,
};

type Data = HashMap<String, Expr>;

#[derive(Debug)]
pub struct Env {
    stack: Vec<Data>,
}

impl Env {
    pub fn new() -> Self {
        let mut e = Env {
            stack: vec![HashMap::new()],
        };
        e.initialize();
        e
    }

    fn initialize(&mut self) {
        self.insert(
            "+".to_string(),
            Fun(Function::Builtin(add), Arguments::Variadic),
        );
        self.insert(
            "-".to_string(),
            Fun(Function::Builtin(sub), Arguments::Variadic),
        );
        self.insert(
            "*".to_string(),
            Fun(Function::Builtin(mul), Arguments::Variadic),
        );
        self.insert(
            "/".to_string(),
            Fun(Function::Builtin(div), Arguments::Variadic),
        );
        self.insert(
            "=".to_string(),
            Fun(Function::Builtin(eq), Arguments::Variadic),
        );
        self.insert(
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
        self.insert(
            "lambda".to_string(),
            Special(
                Function::Builtin(lambda),
                Arguments::Fixed(vec!["args".to_string(), "body".to_string()]),
            ),
        );
    }

    pub fn eval(&mut self, value: &Expr) -> Expr {
        match value {
            Int(x) => Int(*x),
            Bool(x) => Bool(*x),
            Str(x) => Str(x.clone()),
            Symbol(x) => self.get(x.clone()).unwrap(),
            Fun(f, args) => Fun(f.clone(), args.clone()),
            List(list) => self.eval_list(list),
            _ => Nil,
        }
    }

    pub fn insert(&mut self, key: String, value: Expr) {
        let index = self.stack.len() - 1;
        self.stack[index].insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<Expr> {
        for data in self.stack.iter().rev() {
            if data.contains_key(&key) {
                return data.get(&key).cloned();
            }
        }
        None
    }

    fn enter(&mut self) {
        self.stack.push(HashMap::new())
    }

    fn exit(&mut self) {
        self.stack
            .pop()
            .expect("Attempted to pop empty environment stack");
    }

    fn eval_list(&mut self, list: &Vec<Expr>) -> Expr {
        match &list[..] {
            [expr, rest..] => match self.eval(expr) {
                Fun(fun, arg_names) => {
                    let mut evaluated_args: Vec<Expr> = vec![];
                    self.enter();
                    for expr in rest.iter() {
                        evaluated_args.push(self.eval(expr));
                    }
                    self.bind_args(evaluated_args, arg_names.clone());
                    let result = self.eval_fun(fun.clone());
                    self.exit();
                    result
                }
                Special(fun, arg_names) => {
                    self.enter();
                    self.bind_args(rest.to_vec(), arg_names.clone());
                    let result = self.eval_fun(fun.clone());
                    self.exit();
                    result
                }
                other => panic!("Can't apply {:?}", other),
            },
            [] => Nil,
        }
    }

    fn eval_fun(&mut self, fun: Function) -> Expr {
        match fun {
            Function::Builtin(builtin) => builtin(self),
            Function::Dynamic(expr) => self.eval(&expr),
        }
    }

    fn bind_args(&mut self, exprs: Vec<Expr>, args: Arguments) {
        match args {
            Arguments::Variadic => self.insert("varargs".to_string(), List(exprs)),
            Arguments::Fixed(names) => {
                if exprs.len() != names.len() {
                    panic!("Wrong argument count");
                }
                for (expr, name) in exprs.iter().zip(names) {
                    self.insert(name, expr.clone());
                }
            }
        }
    }
}

fn add(env: &mut Env) -> Expr {
    if let Some(List(args)) = env.get("varargs".to_string()) {
        args.iter().fold(Int(0), |acc, x| acc + x.clone())
    } else {
        panic!("could not fetch arguments");
    }
}

fn sub(env: &mut Env) -> Expr {
    if let Some(List(args)) = env.get("varargs".to_string()) {
        match &args[..] {
            [x] => Int(0) - x.clone(),
            [head, tail..] => tail.iter().fold(head.clone(), |acc, x| acc - x.clone()),
            [] => panic!("sub requires at least one argument"),
        }
    } else {
        panic!("could not fetch arguments");
    }
}

fn mul(env: &mut Env) -> Expr {
    if let Some(List(args)) = env.get("varargs".to_string()) {
        args.iter().fold(Int(1), |acc, x| acc * x.clone())
    } else {
        panic!("could not fetch arguments");
    }
}

fn div(env: &mut Env) -> Expr {
    if let Some(List(args)) = env.get("varargs".to_string()) {
        match &args[..] {
            [head] => Int(1) / head.clone(),
            [head, tail..] => tail.iter().fold(head.clone(), |acc, x| acc / x.clone()),
            [] => panic!("div requires at least one argument"),
        }
    } else {
        panic!("could not fetch arguments");
    }
}

fn iff(env: &mut Env) -> Expr {
    match (
        env.get("cond".to_string()).and_then(|e| Some(env.eval(&e))),
        env.get("expr1".to_string()),
        env.get("expr2".to_string()),
    ) {
        (Some(cond), Some(expr1), Some(expr2)) => match cond {
            Bool(true) => env.eval(&expr1),
            Bool(false) => env.eval(&expr2),
            _ => panic!("not a bool"),
        },
        _ => panic!("ass"),
    }
}

fn eq(env: &mut Env) -> Expr {
    if let Some(List(args)) = env.get("varargs".to_string()) {
        match &args[..] {
            [_head] => Bool(true),
            [head, tail..] => Bool(tail.iter().all(|ref x| *x == head)),
            [] => panic!("eq requires a at least one argument"),
        }
    } else {
        panic!("eq requires a at least one argument")
    }
}

fn lambda(env: &mut Env) -> Expr {
    let body = env.get("body".to_string()).unwrap();
    match env.get("args".to_string()).unwrap() {
        List(args) => {
            let arguments = Arguments::Fixed(
                args.iter()
                    .map(|arg| match arg {
                        Symbol(s) => s.clone(),
                        _ => panic!("lambda arguments must be symbols"),
                    })
                    .collect::<Vec<_>>(),
            );
            Fun(Function::Dynamic(Box::new(body)), arguments)
        }
        _ => panic!("This should not happen"),
    }
}
