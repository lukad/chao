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
        self.insert("+".to_string(), Fun(Function(add), Arguments::Variadic));
        self.insert(
            "if".to_string(),
            Special(
                Function(iff),
                Arguments::Fixed(vec![
                    "cond".to_string(),
                    "expr1".to_string(),
                    "expr2".to_string(),
                ]),
            ),
        );
    }

    pub fn eval(&mut self, value: &Expr) -> Expr {
        match value {
            Int(x) => Int(*x),
            Bool(x) => Bool(*x),
            Str(x) => Str(x.clone()),
            Symbol(x) => Symbol(x.clone()),
            Fun(f, args) => Fun(*f, args.clone()),
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
            [Symbol(s), rest..] => match self.get(s.clone()) {
                Some(Fun(func, arg_names)) => {
                    let mut evaluated_args: Vec<Expr> = vec![];
                    self.enter();
                    for expr in rest.iter() {
                        evaluated_args.push(self.eval(expr));
                    }
                    self.bind_args(evaluated_args, arg_names);
                    let result = func.0(self);
                    self.exit();
                    result
                }
                Some(Special(func, arg_names)) => {
                    self.enter();
                    self.bind_args(rest.to_vec(), arg_names);
                    let result = func.0(self);
                    self.exit();
                    result
                }
                other => panic!("Can't apply {:?}: {:?}", s, other),
            },
            _ => panic!("Can't apply {:?}", list),
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
        let sum = args.iter().fold(0, |acc, x| match x {
            Int(n) => acc + n,
            _ => panic!("Can't add {:?}", x),
        });
        Int(sum)
    } else {
        panic!("meh :(");
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
