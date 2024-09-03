use crate::{expr::Expr, token::Token};
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
    Block(BlockStmt),
    If(IfStmt),
    While(WhileStmt),
    Function(FunctionStmt),
    Return(ReturnStmt),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionStmt {
    pub expression: Rc<Expr>,
}

impl ExpressionStmt {
    pub fn new(expression: Rc<Expr>) -> ExpressionStmt {
        ExpressionStmt { expression }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrintStmt {
    pub expression: Rc<Expr>,
}

impl PrintStmt {
    pub fn new(expression: Rc<Expr>) -> PrintStmt {
        PrintStmt { expression }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarStmt {
    pub name: Rc<Token>,
    pub initializer: Option<Rc<Expr>>,
}

impl VarStmt {
    pub fn new(name: Rc<Token>, initializer: Option<Rc<Expr>>) -> VarStmt {
        VarStmt { name, initializer }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStmt {
    pub statements: Vec<Rc<Stmt>>,
}

impl BlockStmt {
    pub fn new(statements: Vec<Rc<Stmt>>) -> BlockStmt {
        BlockStmt { statements }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStmt {
    pub condition: Rc<Expr>,
    pub then_branch: Rc<Stmt>,
    pub else_branch: Option<Rc<Stmt>>,
}

impl IfStmt {
    pub fn new(
        condition: Rc<Expr>,
        then_branch: Rc<Stmt>,
        else_branch: Option<Rc<Stmt>>,
    ) -> IfStmt {
        IfStmt {
            condition,
            then_branch,
            else_branch,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStmt {
    pub condition: Rc<Expr>,
    pub body: Rc<Stmt>,
}

impl WhileStmt {
    pub fn new(condition: Rc<Expr>, body: Rc<Stmt>) -> WhileStmt {
        WhileStmt { condition, body }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionStmt {
    pub name: Rc<Token>,
    pub params: Vec<Rc<Token>>,
    pub body: Vec<Rc<Stmt>>,
}

impl FunctionStmt {
    pub fn new(name: Rc<Token>, params: Vec<Rc<Token>>, body: Vec<Rc<Stmt>>) -> FunctionStmt {
        FunctionStmt { name, params, body }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStmt {
    pub keyword: Rc<Token>,
    pub value: Option<Rc<Expr>>,
}

impl ReturnStmt {
    pub fn new(keyword: Rc<Token>, value: Option<Rc<Expr>>) -> ReturnStmt {
        ReturnStmt { keyword, value }
    }
}
