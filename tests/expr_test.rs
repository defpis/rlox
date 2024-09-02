#[cfg(test)]
mod expr_test {
    use rlox::{expr::*, object::*, token::*};
    use std::rc::Rc;

    #[test]
    fn test_to_string() {
        let expression = Expr::Binary(BinaryExpr::new(
            Rc::new(Expr::Unary(UnaryExpr::new(
                Rc::new(Token::new(
                    TokenType::Minus,
                    "-".to_string(),
                    Object::Nil,
                    1,
                )),
                Rc::new(Expr::Literal(LiteralExpr::new(Object::Number(123.)))),
            ))),
            Rc::new(Token::new(TokenType::Star, "*".to_string(), Object::Nil, 1)),
            Rc::new(Expr::Grouping(GroupingExpr::new(Rc::new(Expr::Literal(
                LiteralExpr::new(Object::Number(45.67)),
            ))))),
        ));

        assert_eq!(expression.to_string(), "(* (- 123) (45.67))".to_string());
    }
}
