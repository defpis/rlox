use crate::{expr::HashExpr, token::Token};
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
    Class(ClassStmt),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionStmt {
    pub expression: Rc<HashExpr>,
}

impl ExpressionStmt {
    pub fn new(expression: Rc<HashExpr>) -> ExpressionStmt {
        ExpressionStmt { expression }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrintStmt {
    pub expression: Rc<HashExpr>,
}

impl PrintStmt {
    pub fn new(expression: Rc<HashExpr>) -> PrintStmt {
        PrintStmt { expression }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarStmt {
    pub name: Rc<Token>,
    pub initializer: Option<Rc<HashExpr>>,
}

impl VarStmt {
    pub fn new(name: Rc<Token>, initializer: Option<Rc<HashExpr>>) -> VarStmt {
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
    pub condition: Rc<HashExpr>,
    pub then_branch: Rc<Stmt>,
    pub else_branch: Option<Rc<Stmt>>,
}

impl IfStmt {
    pub fn new(
        condition: Rc<HashExpr>,
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
    pub condition: Rc<HashExpr>,
    pub body: Rc<Stmt>,
}

impl WhileStmt {
    pub fn new(condition: Rc<HashExpr>, body: Rc<Stmt>) -> WhileStmt {
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
    pub value: Option<Rc<HashExpr>>,
}

impl ReturnStmt {
    pub fn new(keyword: Rc<Token>, value: Option<Rc<HashExpr>>) -> ReturnStmt {
        ReturnStmt { keyword, value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassStmt {
    pub name: Rc<Token>,
    pub superclass: Option<HashExpr>,
    pub methods: Vec<FunctionStmt>,
}

impl ClassStmt {
    pub fn new(
        name: Rc<Token>,
        superclass: Option<HashExpr>,
        methods: Vec<FunctionStmt>,
    ) -> ClassStmt {
        ClassStmt {
            name,
            superclass,
            methods,
        }
    }
}
