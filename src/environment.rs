use crate::{interpreter::InterpretError, object::Object};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

pub trait Stateful {
    fn get(&self, key: &str) -> Result<Object, InterpretError>;
    fn set(&mut self, key: &str, value: Object) -> Result<(), InterpretError>;
}

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

    pub fn define(&mut self, key: String, value: Object) {
        self.values.insert(key, value);
    }

    pub fn get_at(&self, distance: usize, key: &str) -> Result<Object, InterpretError> {
        if distance > 0 {
            if let Some(ref enclosing) = self.enclosing {
                enclosing.borrow().get_at(distance - 1, key)
            } else {
                panic!("Unreachable error!")
            }
        } else {
            self.get(key)
        }
    }

    pub fn set_at(
        &mut self,
        distance: usize,
        key: &str,
        value: Object,
    ) -> Result<(), InterpretError> {
        if distance > 0 {
            if let Some(ref enclosing) = self.enclosing {
                enclosing.borrow_mut().set_at(distance - 1, key, value)
            } else {
                panic!("Unreachable error!")
            }
        } else {
            self.set(key, value)
        }
    }
}

impl Stateful for Environment {
    fn get(&self, key: &str) -> Result<Object, InterpretError> {
        if let Some(value) = self.values.get(key).cloned() {
            return Ok(value);
        }

        if let Some(parent) = self.enclosing.as_ref() {
            return parent.borrow().get(key);
        }

        Err(InterpretError::Error(format!(
            "Undefined variable `{}`.",
            key,
        )))
    }
    fn set(&mut self, key: &str, value: Object) -> Result<(), InterpretError> {
        if self.values.contains_key(key) {
            self.values.insert(key.to_string(), value);
            return Ok(());
        }

        if let Some(enclosing) = self.enclosing.as_ref() {
            enclosing.borrow_mut().set(key, value)?;
            return Ok(());
        }

        Err(InterpretError::Error(format!(
            "Undefined variable `{}`.",
            key,
        )))
    }
}
