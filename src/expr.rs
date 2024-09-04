use crate::{object::Object, token::Token};
use std::hash::{Hash, Hasher};
use std::{fmt, rc::Rc, sync::Mutex};

static EXPR_ID: Mutex<usize> = Mutex::new(0);

#[derive(Debug, PartialEq, Clone)]
pub struct HashExpr {
    pub id: usize,
    pub expr: Expr,
}

impl HashExpr {
    pub fn new(expr: Expr) -> HashExpr {
        let mut id = EXPR_ID.lock().unwrap();
        *id += 1;
        HashExpr { id: *id, expr }
    }
}

impl Eq for HashExpr {}

impl Hash for HashExpr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl fmt::Display for HashExpr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.expr)
    }
}

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
    pub left: Rc<HashExpr>,
    pub op: Rc<Token>,
    pub right: Rc<HashExpr>,
}

impl BinaryExpr {
    pub fn new(left: Rc<HashExpr>, op: Rc<Token>, right: Rc<HashExpr>) -> BinaryExpr {
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
    pub expression: Rc<HashExpr>,
}

impl GroupingExpr {
    pub fn new(expression: Rc<HashExpr>) -> GroupingExpr {
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
    pub right: Rc<HashExpr>,
}

impl UnaryExpr {
    pub fn new(op: Rc<Token>, right: Rc<HashExpr>) -> UnaryExpr {
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
    pub value: Rc<HashExpr>,
}

impl AssignExpr {
    pub fn new(name: Rc<Token>, value: Rc<HashExpr>) -> AssignExpr {
        AssignExpr { name, value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LogicalExpr {
    pub left: Rc<HashExpr>,
    pub op: Rc<Token>,
    pub right: Rc<HashExpr>,
}

impl LogicalExpr {
    pub fn new(left: Rc<HashExpr>, op: Rc<Token>, right: Rc<HashExpr>) -> LogicalExpr {
        LogicalExpr { left, op, right }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpr {
    pub callee: Rc<HashExpr>,
    pub paren: Rc<Token>,
    pub arguments: Vec<Rc<HashExpr>>,
}

impl CallExpr {
    pub fn new(callee: Rc<HashExpr>, paren: Rc<Token>, arguments: Vec<Rc<HashExpr>>) -> CallExpr {
        CallExpr {
            callee,
            paren,
            arguments,
        }
    }
}
