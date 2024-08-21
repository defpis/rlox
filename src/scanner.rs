use crate::token::{Token, TokenType};
use std::{collections::HashMap, sync::LazyLock};

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

fn is_digit(char: char) -> bool {
    char >= '0' && char <= '9'
}

fn is_alpha(char: char) -> bool {
    (char >= 'a' && char <= 'z') || (char >= 'A' && char <= 'Z') || char == '_'
}

fn is_alpha_numeric(char: char) -> bool {
    is_alpha(char) || is_digit(char)
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
            self.start = self.current;
            self.scan_token()
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::from(""), self.line));

        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    // Protected by is_at_end()
    fn advance(&mut self) -> char {
        if let Some(char) = self.peek() {
            self.current += 1;
            return char;
        }
        panic!("Unreachable Error!");
    }

    fn find(&mut self, char: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if let Some(c) = self.peek() {
            if c != char {
                return false;
            }
        }

        self.current += 1;
        true
    }

    fn peek_at(&self, idx: usize) -> Option<char> {
        if idx >= self.chars.len() {
            return None;
        }

        Some(*self.chars.get(idx).unwrap())
    }

    fn peek(&self) -> Option<char> {
        self.peek_at(self.current)
    }

    fn add_token(&mut self, token_type: TokenType) {
        let slice = &self.chars[self.start..self.current];
        let lexeme = String::from_iter(slice);

        self.tokens.push(Token::new(token_type, lexeme, self.line));
    }

    fn string(&mut self) {
        while let Some(char) = self.peek() {
            if char == '"' {
                break;
            }

            if char == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            panic!("[line {}]: Unterminated string.", self.line);
        }

        self.advance();

        self.add_token(TokenType::String);
    }

    fn number(&mut self) {
        while let Some(char) = self.peek() {
            if !is_digit(char) {
                break;
            }
            self.advance();
        }

        if let Some('.') = self.peek() {
            if let Some(char) = self.peek_at(self.current + 1) {
                if is_digit(char) {
                    self.advance();

                    while let Some(char) = self.peek() {
                        if !is_digit(char) {
                            break;
                        }
                        self.advance();
                    }
                }
            }
        }

        self.add_token(TokenType::Number);
    }

    fn identifier(&mut self) {
        while let Some(char) = self.peek() {
            if !is_alpha_numeric(char) {
                break;
            }
            self.advance();
        }

        let slice = &self.chars[self.start..self.current];
        let lexeme = String::from_iter(slice);
        let token_type = if let Some(token_type) = KEYWORDS.get(lexeme.as_str()) {
            token_type.clone()
        } else {
            TokenType::Identifier
        };

        self.add_token(token_type);
    }

    fn scan_token(&mut self) {
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
                    while let Some(char) = self.peek() {
                        if char == '\n' {
                            break;
                        }
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
                if is_digit(char) {
                    self.number();
                } else if is_alpha(char) {
                    self.identifier();
                } else {
                    panic!("[line {}]: Unknown Character.", self.line);
                }
            }
        }
    }
}
