use crate::{expr::Expr, token::Token};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
    Block(BlockStmt),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expression(v) => v.fmt(f),
            Stmt::Print(v) => v.fmt(f),
            Stmt::Var(v) => v.fmt(f),
            Stmt::Block(v) => v.fmt(f),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionStmt {
    pub expression: Expr,
}

impl ExpressionStmt {
    pub fn new(expression: Expr) -> ExpressionStmt {
        ExpressionStmt { expression }
    }
}

impl fmt::Display for ExpressionStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{};", self.expression)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrintStmt {
    pub expression: Expr,
}

impl PrintStmt {
    pub fn new(expression: Expr) -> PrintStmt {
        PrintStmt { expression }
    }
}

impl fmt::Display for PrintStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "print {};", self.expression)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

impl VarStmt {
    pub fn new(name: Token, initializer: Option<Expr>) -> VarStmt {
        VarStmt { name, initializer }
    }
}

impl fmt::Display for VarStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("var {}", self.name.lexeme);
        if let Some(ref initializer) = self.initializer {
            s += &format!(" = {};", initializer);
        } else {
            s += ";";
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

impl BlockStmt {
    pub fn new(statements: Vec<Stmt>) -> BlockStmt {
        BlockStmt { statements }
    }
}

impl fmt::Display for BlockStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        s += "{";
        let len = self.statements.len();
        for idx in 0..len {
            let statement = self.statements.get(idx).unwrap();
            s += &statement.to_string();
            if idx < len - 1 {
                s += ";";
            }
        }
        s += "}";
        write!(f, "{}", s)
    }
}
