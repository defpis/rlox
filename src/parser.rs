use crate::{
    expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
    object::Object,
    token::{Token, TokenType},
};

pub fn parse(tokens: Vec<Token>) -> Option<Expr> {
    let mut parser = Parser::new(tokens);
    if parser.is_at_end() {
        return None;
    }
    Some(parser.expression())
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn is_at_end(&self) -> bool {
        self.peek().map_or(true, |x| x.token_type == TokenType::Eof)
    }

    fn peek_at(&self, idx: usize) -> Option<&Token> {
        self.tokens.get(idx)
    }

    fn peek(&self) -> Option<&Token> {
        self.peek_at(self.current)
    }

    fn previous(&self) -> &Token {
        self.peek_at(self.current - 1).unwrap()
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn check(&self, token_type: &TokenType) -> bool {
        self.peek().map_or(false, |x| x.token_type == *token_type)
    }

    fn find(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> &Token {
        let token = self.peek().unwrap();

        match self.check(token_type) {
            true => self.advance(),
            false => panic!("[line {}] <{:?}> : {}", token.line, token, message),
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.find(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.find(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.find(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.find(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.find(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary(UnaryExpr::new(operator, right));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        let token = self.advance();

        match token.token_type {
            TokenType::False => Expr::Literal(LiteralExpr::new(Object::Boolean(false))),
            TokenType::True => Expr::Literal(LiteralExpr::new(Object::Boolean(true))),
            TokenType::Nil => Expr::Literal(LiteralExpr::new(Object::Nil)),
            TokenType::Number | TokenType::String => {
                Expr::Literal(LiteralExpr::new(self.previous().literal.clone()))
            }
            TokenType::LeftParen => {
                let expr = self.expression();
                self.consume(&TokenType::RightParen, "Expect ')' after expression.");
                Expr::Grouping(GroupingExpr::new(expr))
            }
            _ => panic!("[line {}] <{:?}> : Unexpected Token.", token.line, token),
        }
    }
}
