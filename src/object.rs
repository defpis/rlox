use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Nil => false,
            Object::Boolean(b) => *b,
            Object::Number(n) => *n != 0.0,
            Object::String(s) => (*s).len() > 0,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Nil => write!(f, "nil"),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Number(n) => write!(f, "{}", n),
            Object::String(s) => write!(f, "{}", s),
        }
    }
}
