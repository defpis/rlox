use crate::{interpreter::InterpretError, object::Object, token::Token};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Environment {
    this: Weak<RefCell<Environment>>,
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Rc<RefCell<Environment>> {
        let instance = Rc::new(RefCell::new(Environment {
            this: Weak::new(),
            enclosing,
            values: HashMap::new(),
        }));

        instance.borrow_mut().this = Rc::downgrade(&instance);

        instance
    }

    // fn shared_from_this(&self) -> Rc<RefCell<Environment>> {
    //     self.this.upgrade().unwrap()
    // }

    pub fn define(&mut self, key: String, value: Object) {
        self.values.insert(key, value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, InterpretError> {
        if let Some(value) = self.values.get(&name.lexeme).cloned() {
            return Ok(value);
        }

        if let Some(parent) = self.enclosing.as_ref() {
            return parent.borrow().get(name);
        }

        Err(InterpretError::Error(format!(
            "[line {}] <{:?}> : Undefined variable `{}`.",
            name.line, name, name.lexeme,
        )))
    }

    pub fn get_at(&self, distance: usize, name: &Token) -> Result<Object, InterpretError> {
        if distance > 0 {
            if let Some(ref enclosing) = self.enclosing {
                enclosing.borrow().get_at(distance - 1, name)
            } else {
                panic!("Unreachable error!")
            }
        } else {
            self.get(name)
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), InterpretError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }

        if let Some(enclosing) = self.enclosing.as_ref() {
            enclosing.borrow_mut().assign(name, value)?;
            return Ok(());
        }

        Err(InterpretError::Error(format!(
            "[line {}] <{:?}> : Undefined variable `{}`.",
            name.line, name, name.lexeme,
        )))
    }

    pub fn assign_at(
        &mut self,
        distance: usize,
        name: &Token,
        value: Object,
    ) -> Result<(), InterpretError> {
        if distance > 0 {
            if let Some(ref enclosing) = self.enclosing {
                enclosing.borrow_mut().assign_at(distance - 1, name, value)
            } else {
                panic!("Unreachable error!")
            }
        } else {
            self.assign(name, value)
        }
    }
}
