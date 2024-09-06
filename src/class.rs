use crate::{
    environment::Stateful,
    function::{Function, IsFunction},
    instance::{Instance, IsInstance},
    interpreter::{InterpretError, Interpreter},
    object::Object,
    token::Token,
};
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Debug},
    rc::{Rc, Weak},
};

pub trait IsClass: fmt::Debug + fmt::Display + IsFunction + IsInstance {}

#[derive(Debug)]
pub struct Class {
    this: Weak<RefCell<Class>>,
    pub name: Rc<Token>,
    methods: HashMap<String, Function>,
    fields: HashMap<String, Object>,
}

impl Class {
    pub fn new(name: Rc<Token>, methods: HashMap<String, Function>) -> Rc<RefCell<Class>> {
        let instance = Rc::new(RefCell::new(Class {
            this: Weak::new(),
            name,
            methods,
            fields: HashMap::new(),
        }));

        instance.borrow_mut().this = Rc::downgrade(&instance);

        instance
    }

    fn shared_from_this(&self) -> Rc<RefCell<Class>> {
        self.this.upgrade().unwrap()
    }

    pub fn find_method(&self, name: &str) -> Option<&Function> {
        self.methods.get(name)
    }
}

impl IsFunction for Class {
    fn arity(&self) -> usize {
        if let Some(initializer) = self.find_method("init") {
            initializer.arity()
        } else {
            0
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, InterpretError> {
        let instance = Instance::new(self.shared_from_this());

        if let Some(initializer) = self.find_method("init") {
            initializer
                .bind(instance.clone())?
                .borrow()
                .call(interpreter, arguments)?;
        }

        Ok(Object::Instance(instance))
    }
}

impl Stateful for Class {
    fn get(&self, key: &str) -> Result<Object, InterpretError> {
        if let Some(value) = self.fields.get(key).cloned() {
            return Ok(value);
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

impl fmt::Display for Class {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "<class {}>", self.name.lexeme)
    }
}

impl IsInstance for Class {}

impl IsClass for Class {}
