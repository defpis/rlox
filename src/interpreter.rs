use crate::{
    callable::Clock,
    environment::Environment,
    expr::{Expr, HashExpr},
    function::Function,
    object::Object,
    resolver::Resolver,
    stmt::Stmt,
    token::{Token, TokenType},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub fn interpret(statements: &Vec<Rc<Stmt>>) {
    let mut resolver = Resolver::new();
    let locals = resolver.resolve(statements);

    // println!("{:?}", locals);

    let mut interpreter = Interpreter::new(locals);

    for statement in statements {
        match interpreter.execute(statement) {
            Ok(_) => (),
            Err(err) => match err {
                InterpretError::Return(_) => panic!("Unreachable error!"),
                InterpretError::Error(message) => panic!("{}", message),
            },
        }
    }
}

pub enum InterpretError {
    Error(String),
    Return(Object),
}

pub trait Visitor<T, U> {
    fn visit_expr(&mut self, hash_expr: &HashExpr) -> Result<T, U>;
    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), U>;
}

pub struct Interpreter {
    locals: Rc<RefCell<HashMap<HashExpr, usize>>>,
    globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    fn new(locals: Rc<RefCell<HashMap<HashExpr, usize>>>) -> Interpreter {
        let globals = Environment::new(None);

        globals
            .borrow_mut()
            .define(format!("clock"), Object::Callable(Rc::new(Clock::new())));

        let environment = globals.clone();

        Interpreter {
            locals,
            globals,
            environment,
        }
    }

    fn lookup_variable(
        &self,
        name: &Token,
        hash_expr: &HashExpr,
    ) -> Result<Object, InterpretError> {
        if let Some(distance) = self.locals.borrow().get(hash_expr).cloned() {
            self.environment.borrow().get_at(distance, name)
        } else {
            self.globals.borrow().get(name)
        }
    }

    fn evaluate(&mut self, hash_expr: &HashExpr) -> Result<Object, InterpretError> {
        self.visit_expr(hash_expr)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), InterpretError> {
        self.visit_stmt(stmt)?;
        Ok(())
    }

    pub fn execute_block(
        &mut self,
        statements: &Vec<Rc<Stmt>>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Object, InterpretError> {
        let previous = self.environment.clone();
        self.environment = environment;
        for statement in statements {
            match self.execute(&statement) {
                Ok(_) => (),
                Err(err) => match err {
                    InterpretError::Return(value) => {
                        self.environment = previous;
                        return Ok(value);
                    }
                    InterpretError::Error(message) => {
                        self.environment = previous;
                        return Err(InterpretError::Error(message));
                    }
                },
            }
        }
        self.environment = previous;
        Ok(Object::Nil)
    }
}

impl Visitor<Object, InterpretError> for Interpreter {
    fn visit_expr(&mut self, hash_expr: &HashExpr) -> Result<Object, InterpretError> {
        match &hash_expr.expr {
            Expr::Literal(expr) => Ok(expr.value.clone()),
            Expr::Grouping(expr) => self.evaluate(&expr.expression),
            Expr::Unary(expr) => {
                let right = self.evaluate(&expr.right)?;

                match expr.op.token_type {
                    TokenType::Minus => match right {
                        Object::Number(n) => Ok(Object::Number(-n)),
                        _ => Err(InterpretError::Error(format!(
                            "[line {}] <{:?}> : Operator must be a number.",
                            expr.op.line, expr.op
                        ))),
                    },
                    TokenType::Bang => Ok(Object::Boolean(!right.is_truthy())),
                    _ => panic!("Unreachable error!"),
                }
            }
            Expr::Binary(expr) => {
                let left = self.evaluate(&expr.left)?;
                let right = self.evaluate(&expr.right)?;

                match expr.op.token_type {
                    TokenType::Minus => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l - r)),
                        _ => Err(InterpretError::Error(format!(
                            "[line {}] <{:?}> : Operators must be two numbers.",
                            expr.op.line, expr.op
                        ))),
                    },
                    TokenType::Plus => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l + r)),
                        (Object::String(l), Object::String(r)) => {
                            Ok(Object::String(format!("{}{}", l, r)))
                        }
                        _ => Err(InterpretError::Error(format!(
                            "[line {}] <{:?}> : Operators must be two numbers or strings.",
                            expr.op.line, expr.op
                        ))),
                    },
                    TokenType::Slash => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => match r {
                            0.0 => Err(InterpretError::Error(format!(
                                "[line {}] <{:?}> : Division by zero.",
                                expr.op.line, expr.op
                            ))),
                            _ => Ok(Object::Number(l / r)),
                        },
                        _ => Err(InterpretError::Error(format!(
                            "[line {}] <{:?}> : Operators must be two numbers.",
                            expr.op.line, expr.op
                        ))),
                    },
                    TokenType::Star => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l * r)),
                        _ => Err(InterpretError::Error(format!(
                            "[line {}] <{:?}> : Operators must be two numbers.",
                            expr.op.line, expr.op
                        ))),
                    },
                    TokenType::Greater => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l > r)),
                        _ => Err(InterpretError::Error(format!(
                            "[line {}] <{:?}> : Operators must be two numbers.",
                            expr.op.line, expr.op
                        ))),
                    },
                    TokenType::GreaterEqual => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l >= r)),
                        _ => Err(InterpretError::Error(format!(
                            "[line {}] <{:?}> : Operators must be two numbers.",
                            expr.op.line, expr.op
                        ))),
                    },
                    TokenType::Less => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l < r)),
                        _ => Err(InterpretError::Error(format!(
                            "[line {}] <{:?}> : Operators must be two numbers.",
                            expr.op.line, expr.op
                        ))),
                    },
                    TokenType::LessEqual => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l <= r)),
                        _ => Err(InterpretError::Error(format!(
                            "[line {}] <{:?}> : Operators must be two numbers.",
                            expr.op.line, expr.op
                        ))),
                    },
                    TokenType::BangEqual => Ok(Object::Boolean(left != right)),
                    TokenType::EqualEqual => Ok(Object::Boolean(left == right)),
                    _ => panic!("Unreachable error!"),
                }
            }
            Expr::Variable(expr) => Ok(self.lookup_variable(&expr.name, hash_expr))?,
            Expr::Assign(expr) => {
                let value = self.evaluate(&expr.value)?;
                if let Some(distance) = self.locals.borrow().get(hash_expr).cloned() {
                    self.environment
                        .borrow_mut()
                        .assign_at(distance, &expr.name, value)?
                } else {
                    self.globals.borrow_mut().assign(&expr.name, value)?
                }
                Ok(Object::Nil)
            }
            Expr::Logical(expr) => {
                let left = self.evaluate(&expr.left)?;

                match expr.op.token_type {
                    TokenType::Or => {
                        // 或：第一个为真，就为真
                        if left.is_truthy() {
                            return Ok(left);
                        }
                    }
                    TokenType::And => {
                        // 与：第一个为假，就为假
                        if !left.is_truthy() {
                            return Ok(left);
                        }
                    }
                    _ => panic!("Unreachable error!"),
                }

                self.evaluate(&expr.right)
            }
            Expr::Call(expr) => {
                let callee = self.evaluate(&expr.callee)?;

                let mut arguments: Vec<Object> = Vec::new();
                for argument in &expr.arguments {
                    arguments.push(self.evaluate(argument)?)
                }

                match callee {
                    Object::Callable(function) => {
                        if arguments.len() != function.arity() {
                            return Err(InterpretError::Error(format!(
                                "[line {}] <{:?}> : Expected {} arguments but got {}.",
                                expr.paren.line,
                                expr.paren,
                                function.arity(),
                                arguments.len()
                            )));
                        }
                        function.call(self, arguments)
                    }
                    _ => Err(InterpretError::Error(format!(
                        "[line {}] <{:?}> : Can only call functions and classes.",
                        expr.paren.line, expr.paren
                    ))),
                }
            }
        }
    }
    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), InterpretError> {
        match stmt {
            Stmt::Expression(stmt) => {
                self.evaluate(&stmt.expression)?;
                Ok(())
            }
            Stmt::Print(stmt) => {
                let value = self.evaluate(&stmt.expression)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::Var(stmt) => {
                let mut value = Object::Nil;
                if let Some(ref initializer) = stmt.initializer {
                    value = self.evaluate(initializer)?
                }
                self.environment
                    .borrow_mut()
                    .define(stmt.name.lexeme.clone(), value);
                Ok(())
            }
            Stmt::Block(stmt) => {
                self.execute_block(
                    &stmt.statements,
                    Environment::new(Some(self.environment.clone())),
                )?;
                Ok(())
            }
            Stmt::If(stmt) => {
                if self.evaluate(&stmt.condition)?.is_truthy() {
                    self.execute(&stmt.then_branch)?
                } else if let Some(ref else_branch) = stmt.else_branch {
                    self.execute(else_branch)?
                }
                Ok(())
            }
            Stmt::While(stmt) => {
                while self.evaluate(&stmt.condition)?.is_truthy() {
                    self.execute(&stmt.body)?
                }
                Ok(())
            }
            Stmt::Function(stmt) => {
                let function = Function::new(stmt.clone(), self.environment.clone());
                self.environment.borrow_mut().define(
                    stmt.name.lexeme.clone(),
                    Object::Callable(Rc::new(function)),
                );
                Ok(())
            }
            Stmt::Return(stmt) => {
                let mut value = Object::Nil;
                if let Some(ref expr) = stmt.value {
                    value = self.evaluate(expr)?;
                }
                Err(InterpretError::Return(value))
            }
        }
    }
}
