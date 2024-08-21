use crate::{object::Object, token::Token};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary(v) => v.fmt(f),
            Expr::Grouping(v) => v.fmt(f),
            Expr::Literal(v) => v.fmt(f),
            Expr::Unary(v) => v.fmt(f),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Expr, operator: Token, right: Expr) -> BinaryExpr {
        BinaryExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.operator.lexeme, self.left, self.right)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

impl GroupingExpr {
    pub fn new(expression: Expr) -> GroupingExpr {
        GroupingExpr {
            expression: Box::new(expression),
        }
    }
}

impl fmt::Display for GroupingExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LiteralExpr {
    pub value: Object,
}

impl LiteralExpr {
    pub fn new(literal: Object) -> LiteralExpr {
        LiteralExpr { value: literal }
    }
}

impl fmt::Display for LiteralExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(operator: Token, right: Expr) -> UnaryExpr {
        UnaryExpr {
            operator,
            right: Box::new(right),
        }
    }
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.operator.lexeme, self.right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_display() {
        let expression = Expr::Binary(BinaryExpr::new(
            Expr::Unary(UnaryExpr::new(
                Token::new(TokenType::Minus, "-".to_string(), 1),
                Expr::Literal(LiteralExpr::new(Object::Number(123.))),
            )),
            Token::new(TokenType::Star, "*".to_string(), 1),
            Expr::Grouping(GroupingExpr::new(Expr::Literal(LiteralExpr::new(
                Object::Number(45.67),
            )))),
        ));
        assert_eq!(
            expression.to_string(),
            "(* (- 123) (group 45.67))".to_string()
        );
    }
}
