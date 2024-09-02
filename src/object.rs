use crate::callable::Callable;
use std::{fmt, rc::Rc};

#[derive(Debug, Clone)]
pub enum Object {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Callable(Rc<dyn Callable>),
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Nil => false,
            Object::Boolean(b) => *b,
            Object::Number(n) => *n != 0.0,
            Object::String(s) => s.len() > 0,
            _ => true,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Nil => write!(fmt, "nil"),
            Object::Boolean(b) => write!(fmt, "{}", b),
            Object::Number(n) => write!(fmt, "{}", n),
            Object::String(s) => write!(fmt, "{}", s),
            Object::Callable(c) => write!(fmt, "{}", c),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Nil, Object::Nil) => true,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::String(a), Object::String(b)) => a == b,
            (Object::Callable(a), Object::Callable(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
