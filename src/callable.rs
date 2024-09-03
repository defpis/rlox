use chrono::Utc;

use crate::{
    interpreter::{Interpreter, InterpreterError},
    object::Object,
};
use std::fmt::{self, Debug};

pub trait Callable: Debug + fmt::Display {
    fn arity(&self) -> usize;

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, InterpreterError>;
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

    fn call(&self, _: &mut Interpreter, _: Vec<Object>) -> Result<Object, InterpreterError> {
        Ok(Object::Number(Utc::now().timestamp() as f64))
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "<native clock>")
    }
}
