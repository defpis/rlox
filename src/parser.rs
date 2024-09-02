use std::rc::Rc;

use crate::{
    expr::{
        AssignExpr, BinaryExpr, CallExpr, Expr, GroupingExpr, LiteralExpr, LogicalExpr, UnaryExpr,
        VariableExpr,
    },
    object::Object,
    stmt::{BlockStmt, ExpressionStmt, IfStmt, PrintStmt, Stmt, VarStmt, WhileStmt},
    token::{Token, TokenType},
};

pub fn parse(tokens: Vec<Rc<Token>>) -> Vec<Rc<Stmt>> {
    let mut parser = Parser::new(tokens);
    let mut statements: Vec<Rc<Stmt>> = Vec::new();
    while !parser.is_at_end() {
        match parser.declaration() {
            Ok(stmt) => statements.push(stmt),
            Err(err) => panic!("{}", err.msg),
        }
    }
    statements
}

struct ParserError {
    msg: String,
}

struct Parser {
    tokens: Vec<Rc<Token>>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Rc<Token>>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn is_at_end(&self) -> bool {
        self.peek().map_or(true, |x| x.token_type == TokenType::Eof)
    }

    fn peek_at(&self, idx: usize) -> Option<Rc<Token>> {
        self.tokens.get(idx).cloned()
    }

    fn peek(&self) -> Option<Rc<Token>> {
        self.peek_at(self.current)
    }

    fn previous(&self) -> Rc<Token> {
        self.peek_at(self.current - 1).unwrap()
    }

    fn advance(&mut self) -> Rc<Token> {
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

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<Rc<Token>, ParserError> {
        let token = self.peek().unwrap();

        match self.check(token_type) {
            true => Ok(self.advance()),
            false => Err(ParserError {
                msg: format!("[line {}] <{:?}> : {}", token.line, token, message),
            }),
        }
    }

    fn declaration(&mut self) -> Result<Rc<Stmt>, ParserError> {
        if self.find(&[TokenType::Var]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Result<Rc<Stmt>, ParserError> {
        let name = self.consume(&TokenType::Identifier, "Expect variable name.")?;

        let mut initializer: Option<Rc<Expr>> = None;
        if self.find(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            &TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Rc::new(Stmt::Var(VarStmt::new(name, initializer))))
    }

    fn statement(&mut self) -> Result<Rc<Stmt>, ParserError> {
        if self.find(&[TokenType::For]) {
            return self.for_statement();
        }
        if self.find(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.find(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.find(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.find(&[TokenType::LeftBrace]) {
            return Ok(Rc::new(Stmt::Block(BlockStmt::new(self.block()?))));
        }
        self.expression_statement()
    }

    fn block(&mut self) -> Result<Vec<Rc<Stmt>>, ParserError> {
        let mut statements: Vec<Rc<Stmt>> = Vec::new();

        while !self.check(&TokenType::RightBrace) {
            statements.push(self.declaration()?);
        }

        self.consume(&TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn for_statement(&mut self) -> Result<Rc<Stmt>, ParserError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let mut initializer: Option<Rc<Stmt>> = None;
        if self.find(&[TokenType::Semicolon]) {
            // nothing to do...
        } else if self.find(&[TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let mut condition: Option<Rc<Expr>> = None;
        if !self.check(&TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }

        self.consume(&TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let mut increment: Option<Rc<Expr>> = None;
        if !self.check(&TokenType::RightParen) {
            increment = Some(self.expression()?);
        }

        self.consume(&TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Rc::new(Stmt::Block(BlockStmt::new(vec![
                body,
                Rc::new(Stmt::Expression(ExpressionStmt::new(increment))),
            ])));
        }

        if let Some(condition) = condition {
            body = Rc::new(Stmt::While(WhileStmt::new(condition, body)))
        } else {
            body = Rc::new(Stmt::While(WhileStmt::new(
                Rc::new(Expr::Literal(LiteralExpr::new(Object::Boolean(true)))),
                body,
            )))
        }

        if let Some(initializer) = initializer {
            body = Rc::new(Stmt::Block(BlockStmt::new(vec![initializer, body])));
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Rc<Stmt>, ParserError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expect ')' after while condition.")?;
        let body = self.statement()?;
        Ok(Rc::new(Stmt::While(WhileStmt::new(condition, body))))
    }

    fn if_statement(&mut self) -> Result<Rc<Stmt>, ParserError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expect ')' after if condition.")?;
        let then_branch = self.statement()?;
        let mut else_branch: Option<Rc<Stmt>> = None;
        if self.find(&[TokenType::Else]) {
            else_branch = Some(self.statement()?);
        }
        Ok(Rc::new(Stmt::If(IfStmt::new(
            condition,
            then_branch,
            else_branch,
        ))))
    }

    fn print_statement(&mut self) -> Result<Rc<Stmt>, ParserError> {
        let value = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Rc::new(Stmt::Print(PrintStmt::new(value))))
    }

    fn expression_statement(&mut self) -> Result<Rc<Stmt>, ParserError> {
        let expr = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Rc::new(Stmt::Expression(ExpressionStmt::new(expr))))
    }

    fn expression(&mut self) -> Result<Rc<Expr>, ParserError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Rc<Expr>, ParserError> {
        let expr = self.or()?;

        if self.find(&[TokenType::Equal]) {
            let equal = self.previous();
            let value = self.assignment()?;

            return match expr.as_ref() {
                Expr::Variable(expr) => Ok(Rc::new(Expr::Assign(AssignExpr::new(
                    expr.name.clone(),
                    value,
                )))),
                _ => Err(ParserError {
                    msg: format!(
                        "[line {}] <{:?}> : Invalid assignment target.",
                        equal.line, equal
                    ),
                }),
            };
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Rc<Expr>, ParserError> {
        let mut expr = self.and()?;

        while self.find(&[TokenType::Or]) {
            let op = self.previous();
            let right = self.and()?;
            expr = Rc::new(Expr::Logical(LogicalExpr::new(expr, op, right)));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Rc<Expr>, ParserError> {
        let mut expr = self.equality()?;

        while self.find(&[TokenType::And]) {
            let op = self.previous();
            let right = self.equality()?;
            expr = Rc::new(Expr::Logical(LogicalExpr::new(expr, op, right)));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Rc<Expr>, ParserError> {
        let mut expr = self.comparison()?;

        while self.find(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison()?;
            expr = Rc::new(Expr::Binary(BinaryExpr::new(expr, op, right)));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Rc<Expr>, ParserError> {
        let mut expr = self.term()?;

        while self.find(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous();
            let right = self.term()?;
            expr = Rc::new(Expr::Binary(BinaryExpr::new(expr, op, right)));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Rc<Expr>, ParserError> {
        let mut expr = self.factor()?;

        while self.find(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.previous();
            let right = self.factor()?;
            expr = Rc::new(Expr::Binary(BinaryExpr::new(expr, op, right)));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Rc<Expr>, ParserError> {
        let mut expr = self.unary()?;

        while self.find(&[TokenType::Slash, TokenType::Star]) {
            let op = self.previous();
            let right = self.unary()?;
            expr = Rc::new(Expr::Binary(BinaryExpr::new(expr, op, right)));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Rc<Expr>, ParserError> {
        if self.find(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary()?;
            return Ok(Rc::new(Expr::Unary(UnaryExpr::new(op, right))));
        }

        self.call()
    }

    fn call(&mut self) -> Result<Rc<Expr>, ParserError> {
        let mut expr = self.primary()?;

        loop {
            if self.find(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Rc<Expr>) -> Result<Rc<Expr>, ParserError> {
        let mut arguments: Vec<Rc<Expr>> = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    let token = self.previous();
                    return Err(ParserError {
                        msg: format!(
                            "[line {}] <{:?}> : Can't have more than 255 arguments.",
                            token.line, token
                        ),
                    });
                }
                arguments.push(self.expression()?);
                if !self.find(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(&TokenType::RightParen, "Expect ')' after arguments.")?;

        Ok(Rc::new(Expr::Call(CallExpr::new(callee, paren, arguments))))
    }

    fn primary(&mut self) -> Result<Rc<Expr>, ParserError> {
        let token = self.advance();

        match token.as_ref().token_type {
            TokenType::False => Ok(Rc::new(Expr::Literal(LiteralExpr::new(Object::Boolean(
                false,
            ))))),
            TokenType::True => Ok(Rc::new(Expr::Literal(LiteralExpr::new(Object::Boolean(
                true,
            ))))),
            TokenType::Nil => Ok(Rc::new(Expr::Literal(LiteralExpr::new(Object::Nil)))),
            TokenType::Number | TokenType::String => Ok(Rc::new(Expr::Literal(LiteralExpr::new(
                token.literal.clone(),
            )))),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(&TokenType::RightParen, "Expect ')' after expression.")?;
                Ok(Rc::new(Expr::Grouping(GroupingExpr::new(expr))))
            }
            TokenType::Identifier => Ok(Rc::new(Expr::Variable(VariableExpr::new(token)))),
            _ => Err(ParserError {
                msg: format!("[line {}] <{:?}> : Unexpected token.", token.line, token),
            }),
        }
    }
}
