use crate::{object::Object, token::Token};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Environment {
        Environment {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, key: String, value: Object) {
        self.values.insert(key, value);
    }

    pub fn get(&self, name: &Token) -> Object {
        if let Some(value) = self.values.get(&name.lexeme) {
            return value.clone();
        }

        if let Some(parent) = self.enclosing.as_ref() {
            return parent.borrow().get(name);
        }

        panic!(
            "[line {}] <{:?}> : Undefined variable `{}`.",
            name.line, name, name.lexeme
        );
    }

    pub fn assign(&mut self, name: &Token, value: &Object) {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            return;
        }

        if let Some(enclosing) = self.enclosing.as_ref() {
            enclosing.borrow_mut().assign(name, value);
            return;
        }

        panic!(
            "[line {}] <{:?}> : Undefined variable `{}`.",
            name.line, name, name.lexeme
        );
    }
}
