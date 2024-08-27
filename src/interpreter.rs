use crate::{expr::Expr, object::Object, token::TokenType};

pub fn interpret(expr: Expr) -> Object {
    let mut interpreter = Interpreter::new();
    interpreter.evaluate(&expr)
}

trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T;
}

struct Interpreter {}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {}
    }

    fn evaluate(&mut self, expr: &Expr) -> Object {
        self.visit_expr(expr)
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
        }
    }
}
