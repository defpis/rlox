#[cfg(test)]
mod expr_test {
    use rlox::{expr::*, object::*, token::*};

    #[test]
    fn test_to_string() {
        let expression = Expr::Binary(BinaryExpr::new(
            Expr::Unary(UnaryExpr::new(
                Token::new(TokenType::Minus, "-".to_string(), Object::Nil, 1),
                Expr::Literal(LiteralExpr::new(Object::Number(123.))),
            )),
            Token::new(TokenType::Star, "*".to_string(), Object::Nil, 1),
            Expr::Grouping(GroupingExpr::new(Expr::Literal(LiteralExpr::new(
                Object::Number(45.67),
            )))),
        ));

        assert_eq!(expression.to_string(), "(* (- 123) (45.67))".to_string());
    }
}
