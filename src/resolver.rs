use crate::{
    expr::{Expr, HashExpr},
    interpreter::Visitor,
    stmt::{FunctionStmt, Stmt},
    token::Token,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type ResolveError = String;

#[derive(Clone, Copy, PartialEq)]
enum FunctionType {
    None,
    Function,
    Method,
    Initializer,
}

#[derive(Clone, Copy, PartialEq)]
enum ClassType {
    None,
    Class,
    Subclass,
}

pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
    locals: Rc<RefCell<HashMap<HashExpr, usize>>>,
    current_function: FunctionType,
    current_class: ClassType,
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {
            scopes: Vec::new(),
            locals: Rc::new(RefCell::new(HashMap::new())),
            current_function: FunctionType::None,
            current_class: ClassType::None,
        }
    }

    pub fn resolve(&mut self, statements: &Vec<Rc<Stmt>>) -> Rc<RefCell<HashMap<HashExpr, usize>>> {
        self.begin_scope();
        for statement in statements {
            if let Err(err) = self.visit_stmt(statement) {
                panic!("{}", err);
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

    fn resolve_fun(
        &mut self,
        fun_expr: &FunctionStmt,
        fun_type: FunctionType,
    ) -> Result<(), ResolveError> {
        let enclosing_function = self.current_function;
        self.current_function = fun_type;

        self.begin_scope();
        for param in &fun_expr.params {
            self.declare(&param)?;
            self.define(&param)?;
        }
        for statement in &fun_expr.body {
            self.visit_stmt(&statement)?;
        }
        self.end_scope();

        self.current_function = enclosing_function;
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

    fn declare(&mut self, name: &Token) -> Result<(), ResolveError> {
        if let Some(scope) = self.peek() {
            if scope.contains_key(&name.lexeme) {
                return Err(format!(
                    "[line {}] <{:?}> : Already a variable with this name in this scope.",
                    name.line, name
                ));
            }

            scope.insert(name.lexeme.clone(), false);
        }

        Ok(())
    }

    fn define(&mut self, name: &Token) -> Result<(), ResolveError> {
        if let Some(scope) = self.peek() {
            scope.insert(name.lexeme.clone(), true);
        }

        Ok(())
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
            Expr::Get(expr) => {
                self.visit_expr(&expr.object)?;
                Ok(())
            }
            Expr::Set(expr) => {
                self.visit_expr(&expr.value)?;
                self.visit_expr(&expr.object)?;
                Ok(())
            }
            Expr::This(expr) => {
                if self.current_class == ClassType::None {
                    return Err(format!(
                        "[line {}] <{:?}> : Can't use 'this' outside of a class.",
                        expr.keyword.line, expr.keyword
                    ));
                }

                self.resolve_local(hash_expr, &expr.keyword);
                Ok(())
            }
            Expr::Super(expr) => {
                if self.current_class == ClassType::None {
                    return Err(format!(
                        "[line {}] <{:?}> : Can't use 'super' outside of a class.",
                        expr.keyword.line, expr.keyword
                    ));
                } else if self.current_class != ClassType::Subclass {
                    return Err(format!(
                        "[line {}] <{:?}> : Can't use 'super' in a class with no superclass.",
                        expr.keyword.line, expr.keyword
                    ));
                }

                self.resolve_local(hash_expr, &expr.keyword);
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
                if self.current_function == FunctionType::None {
                    return Err(format!(
                        "[line {}] <{:?}> : Can't return from top-level code.",
                        stmt.keyword.line, stmt.keyword
                    ));
                }

                if let Some(ref hash_expr) = stmt.value {
                    match &hash_expr.expr {
                        Expr::This(_) => (), // Skip
                        _ => {
                            if self.current_function == FunctionType::Initializer {
                                return Err(format!(
                                    "[line {}] <{:?}> : Can't return a value from an initializer.",
                                    stmt.keyword.line, stmt.keyword
                                ));
                            }
                        }
                    }

                    self.visit_expr(&hash_expr)?
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
                self.declare(&stmt.name)?;
                self.define(&stmt.name)?;
                self.resolve_fun(stmt, FunctionType::Function)?;
                Ok(())
            }
            Stmt::Var(stmt) => {
                self.declare(&stmt.name)?;
                if let Some(ref initializer) = stmt.initializer {
                    self.visit_expr(initializer)?
                }
                self.define(&stmt.name)?;
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
            Stmt::Class(stmt) => {
                let enclosing_class = self.current_class;
                self.current_class = ClassType::Class;

                self.declare(&stmt.name)?;

                if let Some(ref hash_expr) = stmt.superclass {
                    if let Expr::Variable(ref expr) = hash_expr.expr {
                        if expr.name.lexeme == stmt.name.lexeme {
                            return Err(format!(
                                "[line {}] <{:?}> : A class can't inherit from itself.",
                                stmt.name.line, stmt.name
                            ));
                        }
                    }

                    self.current_class = ClassType::Subclass;
                    self.visit_expr(hash_expr)?;

                    self.begin_scope();
                    if let Some(scope) = self.peek() {
                        scope.insert("super".to_string(), true);
                    }
                }

                self.begin_scope();
                if let Some(scope) = self.peek() {
                    scope.insert("this".to_string(), true);
                }

                for method in &stmt.methods {
                    let mut declaration = FunctionType::Method;

                    if method.name.lexeme == "init" {
                        declaration = FunctionType::Initializer;
                    }

                    self.resolve_fun(&method, declaration)?
                }

                self.end_scope();

                if let Some(_) = stmt.superclass {
                    self.end_scope();
                }

                self.define(&stmt.name)?;

                self.current_class = enclosing_class;
                Ok(())
            }
        }
    }
}
