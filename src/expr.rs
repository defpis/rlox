use crate::{object::Object, token::Token};
use std::{fmt, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
    Assign(AssignExpr),
    Logical(LogicalExpr),
    Call(CallExpr),
}

impl fmt::Display for Expr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary(v) => v.fmt(fmt),
            Expr::Grouping(v) => v.fmt(fmt),
            Expr::Literal(v) => v.fmt(fmt),
            Expr::Unary(v) => v.fmt(fmt),
            _ => Ok(()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr {
    pub left: Rc<Expr>,
    pub op: Rc<Token>,
    pub right: Rc<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Rc<Expr>, op: Rc<Token>, right: Rc<Expr>) -> BinaryExpr {
        BinaryExpr { left, op, right }
    }
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "({} {} {})", self.op.lexeme, self.left, self.right)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GroupingExpr {
    pub expression: Rc<Expr>,
}

impl GroupingExpr {
    pub fn new(expression: Rc<Expr>) -> GroupingExpr {
        GroupingExpr { expression }
    }
}

impl fmt::Display for GroupingExpr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "({})", self.expression)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LiteralExpr {
    pub value: Object,
}

impl LiteralExpr {
    pub fn new(value: Object) -> LiteralExpr {
        LiteralExpr { value }
    }
}

impl fmt::Display for LiteralExpr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpr {
    pub op: Rc<Token>,
    pub right: Rc<Expr>,
}

impl UnaryExpr {
    pub fn new(op: Rc<Token>, right: Rc<Expr>) -> UnaryExpr {
        UnaryExpr { op, right }
    }
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "({} {})", self.op.lexeme, self.right)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableExpr {
    pub name: Rc<Token>,
}

impl VariableExpr {
    pub fn new(name: Rc<Token>) -> VariableExpr {
        VariableExpr { name }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignExpr {
    pub name: Rc<Token>,
    pub value: Rc<Expr>,
}

impl AssignExpr {
    pub fn new(name: Rc<Token>, value: Rc<Expr>) -> AssignExpr {
        AssignExpr { name, value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LogicalExpr {
    pub left: Rc<Expr>,
    pub op: Rc<Token>,
    pub right: Rc<Expr>,
}

impl LogicalExpr {
    pub fn new(left: Rc<Expr>, op: Rc<Token>, right: Rc<Expr>) -> LogicalExpr {
        LogicalExpr { left, op, right }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpr {
    pub callee: Rc<Expr>,
    pub paren: Rc<Token>,
    pub arguments: Vec<Rc<Expr>>,
}

impl CallExpr {
    pub fn new(callee: Rc<Expr>, paren: Rc<Token>, arguments: Vec<Rc<Expr>>) -> CallExpr {
        CallExpr {
            callee,
            paren,
            arguments,
        }
    }
}
