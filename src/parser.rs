use crate::{
    expr::{
        AssignExpr, BinaryExpr, CallExpr, Expr, GetExpr, GroupingExpr, HashExpr, LiteralExpr,
        LogicalExpr, SetExpr, ThisExpr, UnaryExpr, VariableExpr,
    },
    object::Object,
    stmt::{
        BlockStmt, ClassStmt, ExpressionStmt, FunctionStmt, IfStmt, PrintStmt, ReturnStmt, Stmt,
        VarStmt, WhileStmt,
    },
    token::{Token, TokenType},
};
use std::rc::Rc;

pub fn parse(tokens: Vec<Rc<Token>>) -> Vec<Rc<Stmt>> {
    let mut parser = Parser::new(tokens);
    let mut statements: Vec<Rc<Stmt>> = Vec::new();
    while !parser.is_at_end() {
        match parser.declaration() {
            Ok(stmt) => statements.push(stmt),
            Err(err) => panic!("{}", err),
        }
    }
    statements
}

type ParseError = String;

struct Parser {
    tokens: Vec<Rc<Token>>,
    current: usize,
}

impl Parser {
    const PARAM_MAX_COUNT: usize = 255;

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
        self.previous()
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

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<Rc<Token>, ParseError> {
        let token = self.peek().unwrap();

        match self.check(token_type) {
            true => Ok(self.advance()),
            false => Err(format!("[line {}] <{:?}> : {}", token.line, token, message)),
        }
    }

    fn declaration(&mut self) -> Result<Rc<Stmt>, ParseError> {
        if self.find(&[TokenType::Class]) {
            return self.class_declaration();
        }
        if self.find(&[TokenType::Var]) {
            return self.var_declaration();
        }
        if self.find(&[TokenType::Fun]) {
            return Ok(Rc::new(Stmt::Function(self.function("function")?)));
        }
        self.statement()
    }

    fn function(&mut self, kind: &str) -> Result<FunctionStmt, ParseError> {
        let name = self.consume(
            &TokenType::Identifier,
            format!("Expect {} name.", kind).as_str(),
        )?;

        self.consume(
            &TokenType::LeftParen,
            format!("Expect '(' after {} name.", kind).as_str(),
        )?;

        let mut parameters: Vec<Rc<Token>> = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                if parameters.len() > Parser::PARAM_MAX_COUNT {
                    let token = self.previous();
                    return Err(format!(
                        "[line {}] <{:?}> : Can't have more than 255 arguments.",
                        token.line, token
                    ));
                }
                parameters.push(self.consume(&TokenType::Identifier, "Expect parameter name.")?);
                if !self.find(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(&TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(
            &TokenType::LeftBrace,
            format!("Expect '{{' before {} body.", kind).as_str(),
        )?;

        let body = self.block()?;

        Ok(FunctionStmt::new(name, parameters, body))
    }

    fn class_declaration(&mut self) -> Result<Rc<Stmt>, ParseError> {
        let name = self.consume(&TokenType::Identifier, "Expect class name.")?;
        self.consume(&TokenType::LeftBrace, "Expect '{' before class body.")?;

        let mut methods: Vec<FunctionStmt> = Vec::new();
        while !self.check(&TokenType::RightBrace) {
            methods.push(self.function("method")?)
        }

        self.consume(&TokenType::RightBrace, "Expect '}' before class body.")?;

        Ok(Rc::new(Stmt::Class(ClassStmt::new(name, methods))))
    }

    fn var_declaration(&mut self) -> Result<Rc<Stmt>, ParseError> {
        let name = self.consume(&TokenType::Identifier, "Expect variable name.")?;

        let mut initializer: Option<Rc<HashExpr>> = None;
        if self.find(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            &TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Rc::new(Stmt::Var(VarStmt::new(name, initializer))))
    }

    fn statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
        if self.find(&[TokenType::Return]) {
            return self.return_statement();
        }
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

    fn block(&mut self) -> Result<Vec<Rc<Stmt>>, ParseError> {
        let mut statements: Vec<Rc<Stmt>> = Vec::new();

        while !self.check(&TokenType::RightBrace) {
            statements.push(self.declaration()?);
        }

        self.consume(&TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn for_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let mut initializer: Option<Rc<Stmt>> = None;
        if self.find(&[TokenType::Semicolon]) {
            // nothing to do...
        } else if self.find(&[TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let mut condition: Option<Rc<HashExpr>> = None;
        if !self.check(&TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }

        self.consume(&TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let mut increment: Option<Rc<HashExpr>> = None;
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
                Rc::new(HashExpr::new(Expr::Literal(LiteralExpr::new(
                    Object::Boolean(true),
                )))),
                body,
            )))
        }

        if let Some(initializer) = initializer {
            body = Rc::new(Stmt::Block(BlockStmt::new(vec![initializer, body])));
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expect ')' after while condition.")?;
        let body = self.statement()?;
        Ok(Rc::new(Stmt::While(WhileStmt::new(condition, body))))
    }

    fn return_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
        let keyword = self.previous();
        let mut value: Option<Rc<HashExpr>> = None;
        if !self.check(&TokenType::Semicolon) {
            value = Some(self.expression()?);
        }
        self.consume(&TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Rc::new(Stmt::Return(ReturnStmt::new(keyword, value))))
    }

    fn if_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
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

    fn print_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
        let value = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Rc::new(Stmt::Print(PrintStmt::new(value))))
    }

    fn expression_statement(&mut self) -> Result<Rc<Stmt>, ParseError> {
        let expr = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Rc::new(Stmt::Expression(ExpressionStmt::new(expr))))
    }

    fn expression(&mut self) -> Result<Rc<HashExpr>, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Rc<HashExpr>, ParseError> {
        let expr = self.or()?;

        if self.find(&[TokenType::Equal]) {
            let equal = self.previous();
            let value = self.assignment()?;

            return match &expr.expr {
                Expr::Get(expr) => Ok(Rc::new(HashExpr::new(Expr::Set(SetExpr::new(
                    expr.object.clone(),
                    expr.name.clone(),
                    value,
                ))))),
                Expr::Variable(expr) => Ok(Rc::new(HashExpr::new(Expr::Assign(AssignExpr::new(
                    expr.name.clone(),
                    value,
                ))))),
                _ => Err(format!(
                    "[line {}] <{:?}> : Invalid assignment target.",
                    equal.line, equal
                )),
            };
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Rc<HashExpr>, ParseError> {
        let mut expr = self.and()?;

        while self.find(&[TokenType::Or]) {
            let op = self.previous();
            let right = self.and()?;
            expr = Rc::new(HashExpr::new(Expr::Logical(LogicalExpr::new(
                expr, op, right,
            ))));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Rc<HashExpr>, ParseError> {
        let mut expr = self.equality()?;

        while self.find(&[TokenType::And]) {
            let op = self.previous();
            let right = self.equality()?;
            expr = Rc::new(HashExpr::new(Expr::Logical(LogicalExpr::new(
                expr, op, right,
            ))));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Rc<HashExpr>, ParseError> {
        let mut expr = self.comparison()?;

        while self.find(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison()?;
            expr = Rc::new(HashExpr::new(Expr::Binary(BinaryExpr::new(
                expr, op, right,
            ))));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Rc<HashExpr>, ParseError> {
        let mut expr = self.term()?;

        while self.find(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous();
            let right = self.term()?;
            expr = Rc::new(HashExpr::new(Expr::Binary(BinaryExpr::new(
                expr, op, right,
            ))));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Rc<HashExpr>, ParseError> {
        let mut expr = self.factor()?;

        while self.find(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.previous();
            let right = self.factor()?;
            expr = Rc::new(HashExpr::new(Expr::Binary(BinaryExpr::new(
                expr, op, right,
            ))));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Rc<HashExpr>, ParseError> {
        let mut expr = self.unary()?;

        while self.find(&[TokenType::Slash, TokenType::Star]) {
            let op = self.previous();
            let right = self.unary()?;
            expr = Rc::new(HashExpr::new(Expr::Binary(BinaryExpr::new(
                expr, op, right,
            ))));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Rc<HashExpr>, ParseError> {
        if self.find(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary()?;
            return Ok(Rc::new(HashExpr::new(Expr::Unary(UnaryExpr::new(
                op, right,
            )))));
        }

        self.call()
    }

    fn call(&mut self) -> Result<Rc<HashExpr>, ParseError> {
        let mut expr = self.primary()?;

        loop {
            if self.find(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.find(&[TokenType::Dot]) {
                let name =
                    self.consume(&TokenType::Identifier, "Expect property name after '.'.")?;
                expr = Rc::new(HashExpr::new(Expr::Get(GetExpr::new(expr, name))));
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Rc<HashExpr>) -> Result<Rc<HashExpr>, ParseError> {
        let mut arguments: Vec<Rc<HashExpr>> = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= Parser::PARAM_MAX_COUNT {
                    let token = self.previous();
                    return Err(format!(
                        "[line {}] <{:?}> : Can't have more than 255 arguments.",
                        token.line, token
                    ));
                }
                arguments.push(self.expression()?);
                if !self.find(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(&TokenType::RightParen, "Expect ')' after arguments.")?;

        Ok(Rc::new(HashExpr::new(Expr::Call(CallExpr::new(
            callee, paren, arguments,
        )))))
    }

    fn primary(&mut self) -> Result<Rc<HashExpr>, ParseError> {
        let token = self.advance();

        match token.as_ref().token_type {
            TokenType::This => Ok(Rc::new(HashExpr::new(Expr::This(ThisExpr::new(
                self.previous(),
            ))))),
            TokenType::False => Ok(Rc::new(HashExpr::new(Expr::Literal(LiteralExpr::new(
                Object::Boolean(false),
            ))))),
            TokenType::True => Ok(Rc::new(HashExpr::new(Expr::Literal(LiteralExpr::new(
                Object::Boolean(true),
            ))))),
            TokenType::Nil => Ok(Rc::new(HashExpr::new(Expr::Literal(LiteralExpr::new(
                Object::Nil,
            ))))),
            TokenType::Number | TokenType::String => Ok(Rc::new(HashExpr::new(Expr::Literal(
                LiteralExpr::new(token.literal.clone()),
            )))),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(&TokenType::RightParen, "Expect ')' after expression.")?;
                Ok(Rc::new(HashExpr::new(Expr::Grouping(GroupingExpr::new(
                    expr,
                )))))
            }
            TokenType::Identifier => Ok(Rc::new(HashExpr::new(Expr::Variable(VariableExpr::new(
                token,
            ))))),
            _ => Err(format!(
                "[line {}] <{:?}> : Unexpected token.",
                token.line, token
            )),
        }
    }
}
