use crate::{
    expr::{Expr, HashExpr},
    interpreter::Visitor,
    stmt::{FunctionStmt, Stmt},
    token::Token,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type ResolveError = String;

pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
    locals: Rc<RefCell<HashMap<HashExpr, usize>>>,
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {
            scopes: Vec::new(),
            locals: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn resolve(&mut self, statements: &Vec<Rc<Stmt>>) -> Rc<RefCell<HashMap<HashExpr, usize>>> {
        self.begin_scope();
        for statement in statements {
            match self.visit_stmt(statement) {
                Ok(_) => (),
                Err(err) => panic!("{}", err),
            }
        }
        self.end_scope();
        self.locals.clone()
    }

    fn resolve_local(&mut self, hash_expr: &HashExpr, name: &Token) {
        let len: usize = self.scopes.len();
        for i in (0..len).rev() {
            let scope = self.scopes.get(i).unwrap();
            if scope.contains_key(&name.lexeme) {
                self.locals
                    .borrow_mut()
                    .insert(hash_expr.clone(), len - 1 - i);
                return;
            }
        }
    }

    fn resolve_fun(&mut self, fun_expr: &FunctionStmt) -> Result<(), ResolveError> {
        self.begin_scope();
        for param in &fun_expr.params {
            self.declare(&param);
            self.define(&param);
        }
        for statement in &fun_expr.body {
            self.visit_stmt(&statement)?;
        }
        self.end_scope();
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn peek(&mut self) -> Option<&mut HashMap<String, bool>> {
        self.scopes.last_mut()
    }

    fn declare(&mut self, name: &Token) {
        if let Some(scope) = self.peek() {
            scope.insert(name.lexeme.clone(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.peek() {
            scope.insert(name.lexeme.clone(), true);
        }
    }
}

impl Visitor<(), ResolveError> for Resolver {
    fn visit_expr(&mut self, hash_expr: &HashExpr) -> Result<(), ResolveError> {
        match &hash_expr.expr {
            Expr::Unary(expr) => {
                self.visit_expr(&expr.right)?;
                Ok(())
            }
            Expr::Logical(expr) => {
                self.visit_expr(&expr.left)?;
                self.visit_expr(&expr.right)?;
                Ok(())
            }
            Expr::Literal(_) => Ok(()),
            Expr::Grouping(expr) => self.visit_expr(&expr.expression),
            Expr::Call(expr) => {
                self.visit_expr(&expr.callee)?;
                for argument in &expr.arguments {
                    self.visit_expr(argument)?
                }
                Ok(())
            }
            Expr::Binary(expr) => {
                self.visit_expr(&expr.left)?;
                self.visit_expr(&expr.right)?;
                Ok(())
            }
            Expr::Assign(expr) => {
                self.visit_expr(&expr.value)?;
                self.resolve_local(hash_expr, &expr.name);
                Ok(())
            }
            Expr::Variable(expr) => {
                if let Some(false) = self.peek().and_then(|x| x.get(&expr.name.lexeme)) {
                    return Err(format!(
                        "[line {}] <{:?}> : Can't read local variable in its own initializer.",
                        expr.name.line, expr.name
                    ));
                }
                self.resolve_local(hash_expr, &expr.name);
                Ok(())
            }
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), ResolveError> {
        match stmt {
            Stmt::While(stmt) => {
                self.visit_expr(&stmt.condition)?;
                self.visit_stmt(&stmt.body)?;
                Ok(())
            }
            Stmt::Return(stmt) => {
                if let Some(ref expr) = stmt.value {
                    self.visit_expr(&expr)?
                }
                Ok(())
            }
            Stmt::Expression(stmt) => self.visit_expr(&stmt.expression),
            Stmt::Print(stmt) => self.visit_expr(&stmt.expression),
            Stmt::If(stmt) => {
                self.visit_expr(&stmt.condition)?;
                self.visit_stmt(&stmt.then_branch)?;
                if let Some(ref else_branch) = stmt.else_branch {
                    self.visit_stmt(&else_branch)?;
                }
                Ok(())
            }
            Stmt::Function(stmt) => {
                self.declare(&stmt.name);
                self.define(&stmt.name);
                self.resolve_fun(stmt)?;
                Ok(())
            }
            Stmt::Var(stmt) => {
                self.declare(&stmt.name);
                if let Some(ref initializer) = stmt.initializer {
                    self.visit_expr(initializer)?
                }
                self.define(&stmt.name);
                Ok(())
            }
            Stmt::Block(stmt) => {
                self.begin_scope();
                for statement in &stmt.statements {
                    self.visit_stmt(statement)?
                }
                self.end_scope();
                Ok(())
            }
        }
    }
}
