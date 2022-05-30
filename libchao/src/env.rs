use std::collections::HashMap;

use builtin;
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
        builtin::load(&mut e);
        e
    }

    pub fn eval(&mut self, value: &Expr) -> Expr {
        match value {
            Nil => Nil,
            Int(x) => Int(*x),
            Float(x) => Float(*x),
            Bool(x) => Bool(*x),
            Str(x) => Str(x.clone()),
            Symbol(x) => self.get(x.clone()).unwrap_or(Nil),
            Quote(x) => *x.clone(),
            Fun(f, args) => Fun(f.clone(), args.clone()),
            Special(f, args) => Fun(f.clone(), args.clone()),
            List(list) => self.eval_list(list),
            Error(_) => value.clone(),
        }
    }

    pub fn insert_parent(&mut self, key: String, value: Expr) {
        self.stack[0].insert(key, value);
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
            [expr, rest @ ..] => match self.eval(expr) {
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
                other => Error(format!("Can't apply {:?} ({:?})", expr, other)),
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
                    Error("Wrong argument count".to_string());
                }
                for (expr, name) in exprs.iter().zip(names) {
                    self.insert(name, expr.clone());
                }
            }
        }
    }
}
