use crate::{
    object::Object,
    token::{Token, TokenType},
};
use std::{collections::HashMap, f64, sync::LazyLock};

pub struct Scanner {
    // Input
    chars: Vec<char>,
    // States
    start: usize,
    current: usize,
    line: usize,
    // Output
    tokens: Vec<Token>,
}

static KEYWORDS: LazyLock<HashMap<&str, TokenType>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    m.insert("and", TokenType::And);
    m.insert("class", TokenType::Class);
    m.insert("else", TokenType::Else);
    m.insert("false", TokenType::False);
    m.insert("for", TokenType::For);
    m.insert("fun", TokenType::Fun);
    m.insert("if", TokenType::If);
    m.insert("nil", TokenType::Nil);
    m.insert("or", TokenType::Or);
    m.insert("print", TokenType::Print);
    m.insert("return", TokenType::Return);
    m.insert("super", TokenType::Super);
    m.insert("this", TokenType::This);
    m.insert("true", TokenType::True);
    m.insert("var", TokenType::Var);
    m.insert("while", TokenType::While);

    m
});

impl Scanner {
    fn is_digit(char: char) -> bool {
        char >= '0' && char <= '9'
    }

    fn is_alpha(char: char) -> bool {
        (char >= 'a' && char <= 'z') || (char >= 'A' && char <= 'Z') || char == '_'
    }

    fn is_alpha_numeric(char: char) -> bool {
        Scanner::is_alpha(char) || Scanner::is_digit(char)
    }

    pub fn new(code: &str) -> Scanner {
        Scanner {
            chars: code.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.scan_token()
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            Object::None,
            self.line,
        ));

        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        !self.peek().is_some()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self.previous();
    }

    fn find(&mut self, char: char) -> bool {
        self.peek().map_or(false, |x| {
            if x == char {
                self.current += 1;
                true
            } else {
                false
            }
        })
    }

    fn peek_at(&self, idx: usize) -> Option<char> {
        if idx >= self.chars.len() {
            return None;
        }

        Some(self.chars[idx])
    }

    fn peek(&self) -> Option<char> {
        self.peek_at(self.current)
    }

    fn previous(&self) -> char {
        self.peek_at(self.current - 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let slice = &self.chars[self.start..self.current];
        let lexeme = String::from_iter(slice);

        let literal = match token_type {
            TokenType::Number => Object::Number(lexeme.parse::<f64>().unwrap()),
            TokenType::String => Object::String(String::from_iter(
                &self.chars[self.start + 1..self.current - 1],
            )),
            _ => Object::None,
        };

        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.line));
    }

    fn string(&mut self) {
        while let Some(char) = self.peek() {
            match char {
                '"' => {
                    self.advance();
                    self.add_token(TokenType::String);
                    return;
                }
                '\n' => self.line += 1,
                _ => {
                    self.advance();
                }
            }
        }

        panic!("[line {}] : Unterminated string.", self.line);
    }

    fn number(&mut self) {
        while self.peek().map_or(false, Scanner::is_digit) {
            self.advance();
        }

        if self.peek() == Some('.')
            && self
                .peek_at(self.current + 1)
                .map_or(false, Scanner::is_digit)
        {
            self.advance();
            while self.peek().map_or(false, Scanner::is_digit) {
                self.advance();
            }
        }

        self.add_token(TokenType::Number);
    }

    fn identifier(&mut self) {
        while self.peek().map_or(false, Scanner::is_alpha_numeric) {
            self.advance();
        }

        let slice = &self.chars[self.start..self.current];
        let lexeme = String::from_iter(slice);

        let token_type = KEYWORDS
            .get(lexeme.as_str())
            .cloned()
            .unwrap_or(TokenType::Identifier);

        self.add_token(token_type);
    }

    fn scan_token(&mut self) {
        self.start = self.current;

        let char = self.advance();

        match char {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),

            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Star),

            '/' => {
                if self.find('/') {
                    while self.peek().map_or(false, |x| x != '\n') {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            '!' => {
                if self.find('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.find('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '>' => {
                if self.find('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '<' => {
                if self.find('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }

            '.' => self.add_token(TokenType::Dot),
            ',' => self.add_token(TokenType::Comma),
            ';' => self.add_token(TokenType::Semicolon),

            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            '"' => self.string(),

            char => {
                if Scanner::is_digit(char) {
                    self.number();
                } else if Scanner::is_alpha(char) {
                    self.identifier();
                } else {
                    panic!("[line {}] : Unknown Character.", self.line);
                }
            }
        }
    }
}
