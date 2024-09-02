use crate::{
    callable::Clock, environment::Environment, expr::Expr, object::Object, stmt::Stmt,
    token::TokenType,
};
use std::{
    cell::RefCell,
    panic,
    rc::{Rc, Weak},
};

pub fn interpret(statements: Vec<Rc<Stmt>>) {
    let interpreter = Interpreter::new();
    for statement in statements {
        match interpreter.borrow_mut().execute(&statement) {
            Ok(_) => (),
            Err(err) => panic!("{}", err.msg),
        }
    }
}

pub struct InterpreterError {
    pub msg: String,
    pub returning: Option<Rc<Object>>,
}

trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> Result<T, InterpreterError>;
    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), InterpreterError>;
}

pub struct Interpreter {
    this: Weak<RefCell<Interpreter>>,
    globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    fn new() -> Rc<RefCell<Interpreter>> {
        let globals = Rc::new(RefCell::new(Environment::new(None)));

        globals.borrow_mut().define(
            String::from("clock"),
            Object::Callable(Rc::new(Clock::new())),
        );

        let environment = globals.clone();

        let instance = Rc::new(RefCell::new(Interpreter {
            this: Weak::new(),
            globals,
            environment,
        }));

        instance.borrow_mut().this = Rc::downgrade(&instance);

        instance
    }

    fn shared_from_this(&self) -> Rc<RefCell<Interpreter>> {
        self.this.upgrade().unwrap()
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Object, InterpreterError> {
        self.visit_expr(expr)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), InterpreterError> {
        self.visit_stmt(stmt)?;
        Ok(())
    }

    fn execute_block(
        &mut self,
        statements: &Vec<Rc<Stmt>>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), InterpreterError> {
        let previous = self.environment.clone();
        self.environment = environment;
        for statement in statements {
            match self.execute(&statement) {
                Ok(_) => (),
                Err(err) => {
                    self.environment = previous;
                    return Err(err);
                }
            }
        }
        self.environment = previous;
        Ok(())
    }
}

impl Visitor<Object> for Interpreter {
    fn visit_expr(&mut self, expr: &Expr) -> Result<Object, InterpreterError> {
        match expr {
            Expr::Literal(expr) => Ok(expr.value.clone()),
            Expr::Grouping(expr) => self.evaluate(&expr.expression),
            Expr::Unary(expr) => {
                let right = self.evaluate(&expr.right)?;

                match expr.op.token_type {
                    TokenType::Minus => match right {
                        Object::Number(n) => Ok(Object::Number(-n)),
                        _ => Err(InterpreterError {
                            msg: format!(
                                "[line {}] <{:?}> : Operator must be a number.",
                                expr.op.line, expr.op
                            ),
                            returning: None,
                        }),
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
                        _ => Err(InterpreterError {
                            msg: format!(
                                "[line {}] <{:?}> : Operators must be two numbers.",
                                expr.op.line, expr.op
                            ),
                            returning: None,
                        }),
                    },
                    TokenType::Plus => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l + r)),
                        (Object::String(l), Object::String(r)) => {
                            Ok(Object::String(format!("{}{}", l, r)))
                        }
                        _ => Err(InterpreterError {
                            msg: format!(
                                "[line {}] <{:?}> : Operators must be two numbers or strings.",
                                expr.op.line, expr.op
                            ),
                            returning: None,
                        }),
                    },
                    TokenType::Slash => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => match r {
                            0.0 => Err(InterpreterError {
                                msg: format!(
                                    "[line {}] <{:?}> : Division by zero.",
                                    expr.op.line, expr.op
                                ),
                                returning: None,
                            }),
                            _ => Ok(Object::Number(l / r)),
                        },
                        _ => Err(InterpreterError {
                            msg: format!(
                                "[line {}] <{:?}> : Operators must be two numbers.",
                                expr.op.line, expr.op
                            ),
                            returning: None,
                        }),
                    },
                    TokenType::Star => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l * r)),
                        _ => Err(InterpreterError {
                            msg: format!(
                                "[line {}] <{:?}> : Operators must be two numbers.",
                                expr.op.line, expr.op
                            ),
                            returning: None,
                        }),
                    },
                    TokenType::Greater => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l > r)),
                        _ => Err(InterpreterError {
                            msg: format!(
                                "[line {}] <{:?}> : Operators must be two numbers.",
                                expr.op.line, expr.op
                            ),
                            returning: None,
                        }),
                    },
                    TokenType::GreaterEqual => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l >= r)),
                        _ => Err(InterpreterError {
                            msg: format!(
                                "[line {}] <{:?}> : Operators must be two numbers.",
                                expr.op.line, expr.op
                            ),
                            returning: None,
                        }),
                    },
                    TokenType::Less => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l < r)),
                        _ => Err(InterpreterError {
                            msg: format!(
                                "[line {}] <{:?}> : Operators must be two numbers.",
                                expr.op.line, expr.op
                            ),
                            returning: None,
                        }),
                    },
                    TokenType::LessEqual => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l <= r)),
                        _ => Err(InterpreterError {
                            msg: format!(
                                "[line {}] <{:?}> : Operators must be two numbers.",
                                expr.op.line, expr.op
                            ),
                            returning: None,
                        }),
                    },
                    TokenType::BangEqual => Ok(Object::Boolean(left != right)),
                    TokenType::EqualEqual => Ok(Object::Boolean(left == right)),
                    _ => panic!("Unreachable error!"),
                }
            }
            Expr::Variable(expr) => Ok(self.environment.borrow().get(expr.name.clone())?),
            Expr::Assign(expr) => {
                let value = self.evaluate(&expr.value)?;
                self.environment
                    .borrow_mut()
                    .assign(expr.name.clone(), value.clone())?;
                Ok(value)
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
                            return Err(InterpreterError {
                                msg: format!(
                                    "[line {}] <{:?}> : Expected {} arguments but got {}.",
                                    expr.paren.line,
                                    expr.paren,
                                    function.arity(),
                                    arguments.len()
                                ),
                                returning: None,
                            });
                        }
                        function.call(self.shared_from_this(), arguments)
                    }
                    _ => Err(InterpreterError {
                        msg: format!(
                            "[line {}] <{:?}> : Can only call functions and classes.",
                            expr.paren.line, expr.paren
                        ),
                        returning: None,
                    }),
                }
            }
        }
    }
    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), InterpreterError> {
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
                    Rc::new(RefCell::new(Environment::new(Some(
                        self.environment.clone(),
                    )))),
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
        }
    }
}
