use crate::{
    interpreter::{InterpretError, Interpreter},
    object::Object,
};
use chrono::Utc;
use std::fmt::{self, Debug};

pub trait Callable: Debug + fmt::Display {
    fn arity(&self) -> usize;

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, InterpretError>;
}

#[derive(Debug)]
pub struct Clock {}

impl Clock {
    pub fn new() -> Clock {
        Clock {}
    }
}

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _: &mut Interpreter, _: Vec<Object>) -> Result<Object, InterpretError> {
        Ok(Object::Number(Utc::now().timestamp() as f64))
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "<builtin-fun clock>")
    }
}
