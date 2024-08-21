use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    None,
    Bool(bool),
    Number(f64),
    String(String),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::None => write!(f, "nil"),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Number(n) => write!(f, "{}", n),
            Object::String(s) => write!(f, "{}", s),
        }
    }
}
