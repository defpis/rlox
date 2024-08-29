use crate::{
    expr::{AssignExpr, BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, VariableExpr},
    object::Object,
    stmt::{BlockStmt, ExpressionStmt, PrintStmt, Stmt, VarStmt},
    token::{Token, TokenType},
};

pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
    let mut parser = Parser::new(tokens);
    let mut statements: Vec<Stmt> = Vec::new();
    while !parser.is_at_end() {
        statements.push(parser.declaration())
    }
    statements
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

    fn declaration(&mut self) -> Stmt {
        if self.find(&[TokenType::Var]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Stmt {
        let name = self
            .consume(&TokenType::Identifier, "Expect variable name.")
            .clone();

        let mut initializer: Option<Expr> = None;
        if self.find(&[TokenType::Equal]) {
            initializer = Some(self.expression());
        }

        self.consume(
            &TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        );

        Stmt::Var(VarStmt::new(name, initializer))
    }

    fn statement(&mut self) -> Stmt {
        if self.find(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.find(&[TokenType::LeftBrace]) {
            return Stmt::Block(BlockStmt::new(self.block()));
        }
        self.expression_statement()
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(&TokenType::RightBrace) {
            statements.push(self.declaration());
        }

        self.consume(&TokenType::RightBrace, "Expect '}' after block.");
        statements
    }

    fn print_statement(&mut self) -> Stmt {
        let value = self.expression();
        self.consume(&TokenType::Semicolon, "Expect ';' after value.");
        Stmt::Print(PrintStmt::new(value))
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(&TokenType::Semicolon, "Expect ';' after expression.");
        Stmt::Expression(ExpressionStmt::new(expr))
    }

    fn expression(&mut self) -> Expr {
        self.assignment()
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.equality();

        if self.find(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment();

            match expr {
                Expr::Variable(expr) => {
                    let name = expr.name;
                    return Expr::Assign(AssignExpr::new(name, value));
                }
                _ => panic!("[line {}] : Invalid assignment target.", equals.line),
            }
        }

        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.find(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary(BinaryExpr::new(expr, op, right));
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
            let op = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary(BinaryExpr::new(expr, op, right));
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.find(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary(BinaryExpr::new(expr, op, right));
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.find(&[TokenType::Slash, TokenType::Star]) {
            let op = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary(BinaryExpr::new(expr, op, right));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.find(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous().clone();
            let right = self.unary();
            return Expr::Unary(UnaryExpr::new(op, right));
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
                Expr::Literal(LiteralExpr::new(token.literal.clone()))
            }
            TokenType::LeftParen => {
                let expr = self.expression();
                self.consume(&TokenType::RightParen, "Expect ')' after expression.");
                Expr::Grouping(GroupingExpr::new(expr))
            }
            TokenType::Identifier => Expr::Variable(VariableExpr::new(token.clone())),
            _ => panic!("[line {}] <{:?}> : Unexpected token.", token.line, token),
        }
    }
}
