use crate::{class::IsClass, function::IsFunction, instance::IsInstance};
use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Debug, Clone)]
pub enum Object {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Function(Rc<RefCell<dyn IsFunction>>),
    Instance(Rc<RefCell<dyn IsInstance>>),
    Class(Rc<RefCell<dyn IsClass>>),
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Nil => false,
            Object::Boolean(false) => false,
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
            Object::Function(f) => write!(fmt, "{}", f.borrow()),
            Object::Instance(i) => write!(fmt, "{}", i.borrow()),
            Object::Class(c) => write!(fmt, "{}", c.borrow()),
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
            (Object::Function(a), Object::Function(b)) => Rc::ptr_eq(a, b),
            (Object::Instance(a), Object::Instance(b)) => Rc::ptr_eq(a, b),
            (Object::Class(a), Object::Class(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
