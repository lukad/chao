use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::expr::Expr;

#[derive(Debug, Clone)]
pub struct Env {
    inner: Rc<RefCell<EnvInner>>,
}

#[derive(Debug, Clone, Default)]
struct EnvInner {
    values: HashMap<String, Expr>,
    enclosing: Option<Env>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            inner: Default::default(),
        }
    }

    pub fn enclosing(&self) -> Option<Env> {
        self.inner.borrow().enclosing.clone()
    }

    pub fn insert(&mut self, key: String, value: Expr) {
        let mut inner = self.inner.borrow_mut();
        inner.values.insert(key, value);
    }

    pub fn insert_in_enclosing(&self, key: String, value: Expr) {
        let enclosing = self.inner.borrow().enclosing.clone();

        match enclosing {
            Some(mut env) => {
                env.insert(key, value);
            }
            None => panic!("No enclosing environment"),
        }
    }

    pub fn assign(&self, key: &str, value: Expr) -> bool {
        {
            let mut inner = self.inner.borrow_mut();
            if inner.values.contains_key(key) {
                inner.values.insert(key.to_string(), value);
                return true;
            }
        }

        match self.inner.borrow().enclosing.clone() {
            Some(env) => env.assign(key, value),
            None => false,
        }
    }

    pub fn assign_in_enclosing(&self, key: &str, value: Expr) -> bool {
        match self.inner.borrow().enclosing.clone() {
            Some(env) => env.assign(key, value),
            None => false,
        }
    }

    pub fn get(&self, key: &str) -> Option<Expr> {
        if let Some(expr) = self.inner.borrow().values.get(key) {
            return Some(expr.clone());
        }

        self.inner
            .borrow()
            .enclosing
            .as_ref()
            .and_then(|e| e.get(key))
    }

    pub fn child(&self) -> Self {
        Self {
            inner: Rc::new(RefCell::new(EnvInner {
                values: HashMap::new(),
                enclosing: Some(self.clone()),
            })),
        }
    }

    pub fn child_with(&self, bindings: Vec<(String, Expr)>) -> Self {
        let mut child = self.child();

        for (name, value) in bindings {
            child.insert(name, value);
        }

        child
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}
