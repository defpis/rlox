use crate::{environment::Environment, expr::Expr, object::Object, stmt::Stmt, token::TokenType};
use std::{cell::RefCell, panic, rc::Rc};

pub fn interpret(statements: Vec<Stmt>) {
    let mut interpreter = Interpreter::new();
    for statement in statements {
        interpreter.execute(&statement)
    }
}

trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T;
    fn visit_stmt(&mut self, stmt: &Stmt) -> ();
}

struct Interpreter {
    globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    fn new() -> Interpreter {
        let globals = Rc::new(RefCell::new(Environment::new(None)));
        let environment = globals.clone();

        Interpreter {
            globals,
            environment,
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Object {
        self.visit_expr(expr)
    }

    fn execute(&mut self, stmt: &Stmt) {
        self.visit_stmt(stmt);
    }

    fn execute_block(&mut self, statements: &Vec<Stmt>, environment: Rc<RefCell<Environment>>) {
        let previous = self.environment.clone();
        self.environment = environment;
        for statement in statements {
            self.execute(&statement);
        }
        self.environment = previous;
    }
}

impl Visitor<Object> for Interpreter {
    fn visit_expr(&mut self, expr: &Expr) -> Object {
        match expr {
            Expr::Literal(expr) => expr.value.clone(),
            Expr::Grouping(expr) => self.visit_expr(&expr.expression),
            Expr::Unary(expr) => {
                let right = self.visit_expr(&expr.right);

                match expr.op.token_type {
                    TokenType::Minus => match right {
                        Object::Number(n) => Object::Number(-n),
                        _ => panic!(
                            "[line {}] <{:?}> : Operator must be a number.",
                            expr.op.line, expr.op
                        ),
                    },
                    TokenType::Bang => Object::Boolean(!right.is_truthy()),
                    _ => panic!("Unreachable error!"),
                }
            }
            Expr::Binary(expr) => {
                let left = self.visit_expr(&expr.left);
                let right = self.visit_expr(&expr.right);

                match expr.op.token_type {
                    TokenType::Minus => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Object::Number(l - r),
                        _ => panic!(
                            "[line {}] <{:?}> : Operators must be two numbers.",
                            expr.op.line, expr.op
                        ),
                    },
                    TokenType::Plus => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Object::Number(l + r),
                        (Object::String(l), Object::String(r)) => Object::String(l + &r),
                        _ => panic!(
                            "[line {}] <{:?}> : Operators must be two numbers or strings.",
                            expr.op.line, expr.op
                        ),
                    },
                    TokenType::Slash => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => match r {
                            0.0 => {
                                panic!(
                                    "[line {}] <{:?}> : Division by zero.",
                                    expr.op.line, expr.op
                                )
                            }
                            _ => Object::Number(l / r),
                        },
                        _ => panic!(
                            "[line {}] <{:?}> : Operators must be two numbers.",
                            expr.op.line, expr.op
                        ),
                    },
                    TokenType::Star => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Object::Number(l * r),
                        _ => panic!(
                            "[line {}] <{:?}> : Operators must be two numbers.",
                            expr.op.line, expr.op
                        ),
                    },
                    TokenType::Greater => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Object::Boolean(l > r),
                        _ => panic!(
                            "[line {}] <{:?}> : Operators must be two numbers.",
                            expr.op.line, expr.op
                        ),
                    },
                    TokenType::GreaterEqual => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Object::Boolean(l >= r),
                        _ => panic!(
                            "[line {}] <{:?}> : Operators must be two numbers.",
                            expr.op.line, expr.op
                        ),
                    },
                    TokenType::Less => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Object::Boolean(l < r),
                        _ => panic!(
                            "[line {}] <{:?}> : Operators must be two numbers.",
                            expr.op.line, expr.op
                        ),
                    },
                    TokenType::LessEqual => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Object::Boolean(l <= r),
                        _ => panic!(
                            "[line {}] <{:?}> : Operators must be two numbers.",
                            expr.op.line, expr.op
                        ),
                    },
                    TokenType::BangEqual => Object::Boolean(left != right),
                    TokenType::EqualEqual => Object::Boolean(left == right),
                    _ => panic!("Unreachable error!"),
                }
            }
            Expr::Variable(expr) => self.environment.borrow().get(&expr.name),
            Expr::Assign(expr) => {
                let value = self.evaluate(&expr.value);
                self.environment.borrow_mut().assign(&expr.name, &value);
                return value;
            }
        }
    }
    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expression(stmt) => {
                self.evaluate(&stmt.expression);
            }
            Stmt::Print(stmt) => {
                let value = self.evaluate(&stmt.expression);
                println!("{}", value);
            }
            Stmt::Var(stmt) => {
                let mut value = Object::Nil;
                if let Some(ref initializer) = stmt.initializer {
                    value = self.evaluate(initializer);
                }
                self.environment
                    .borrow_mut()
                    .define(stmt.name.lexeme.clone(), value)
            }
            Stmt::Block(stmt) => {
                self.execute_block(
                    &stmt.statements,
                    Rc::new(RefCell::new(Environment::new(Some(
                        self.environment.clone(),
                    )))),
                );
            }
        }
    }
}
