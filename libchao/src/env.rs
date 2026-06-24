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
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}
