use crate::{
    class::{Class, IsClass},
    environment::Stateful,
    function::IsFunction,
    interpreter::InterpretError,
    object::Object,
};
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Debug},
    rc::{Rc, Weak},
};

pub trait IsInstance: fmt::Debug + fmt::Display + Stateful {}

#[derive(Debug)]
pub struct Instance {
    this: Weak<RefCell<Instance>>,
    class: Rc<RefCell<Class>>,
    fields: HashMap<String, Object>,
}

impl Instance {
    pub fn new(class: Rc<RefCell<Class>>) -> Rc<RefCell<Instance>> {
        let instance = Rc::new(RefCell::new(Instance {
            this: Weak::new(),
            class,
            fields: HashMap::new(),
        }));

        instance.borrow_mut().this = Rc::downgrade(&instance);

        instance
    }

    fn shared_from_this(&self) -> Rc<RefCell<Instance>> {
        self.this.upgrade().unwrap()
    }
}

impl Stateful for Instance {
    fn get(&self, key: &str) -> Result<Object, InterpretError> {
        if let Some(value) = self.fields.get(key).cloned() {
            return Ok(value);
        }

        if let Some(method) = self.class.borrow().find_method(key) {
            let function = method.bind(self.shared_from_this())?;
            return Ok(Object::Function(function));
        }

        Err(InterpretError::Error(format!(
            "Undefined property `{}`.",
            key,
        )))
    }

    fn set(&mut self, key: &str, value: Object) -> Result<(), InterpretError> {
        self.fields.insert(key.to_string(), value);
        Ok(())
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "<instance of {}>", self.class.borrow().name.lexeme)
    }
}

impl IsInstance for Instance {}
