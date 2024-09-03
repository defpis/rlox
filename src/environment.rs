use crate::{interpreter::InterpreterError, object::Object, token::Token};
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

    pub fn get(&self, name: Rc<Token>) -> Result<Object, InterpreterError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.clone());
        }

        if let Some(parent) = self.enclosing.as_ref() {
            return parent.borrow().get(name);
        }

        Err(InterpreterError {
            msg: format!(
                "[line {}] <{:?}> : Undefined variable `{}`.",
                name.line, name, name.lexeme,
            ),
            returning: None,
        })
    }

    pub fn assign(&mut self, name: Rc<Token>, value: Object) -> Result<(), InterpreterError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }

        if let Some(enclosing) = self.enclosing.as_ref() {
            enclosing.borrow_mut().assign(name, value)?;
            return Ok(());
        }

        Err(InterpreterError {
            msg: format!(
                "[line {}] <{:?}> : Undefined variable `{}`.",
                name.line, name, name.lexeme,
            ),
            returning: None,
        })
    }
}
