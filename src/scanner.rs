use crate::{
    object::Object,
    token::{Token, TokenType},
};
use std::{collections::HashMap, f64, rc::Rc, sync::LazyLock};

pub fn scan_tokens(code: &str) -> Vec<Rc<Token>> {
    let chars: Vec<char> = code.chars().collect(); // utf-8

    let mut scanner = Scanner::new(chars);
    let mut tokens: Vec<Rc<Token>> = Vec::new();

    while !scanner.is_at_end() {
        match scanner.scan_token() {
            Ok(token) => {
                if let Some(token) = token {
                    tokens.push(Rc::new(token))
                }
            }
            Err(err) => panic!("{}", err),
        }
    }

    tokens.push(Rc::new(scanner.eof()));

    tokens
}

type ScanError = String;

struct Scanner {
    chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    const KEYWORDS: LazyLock<HashMap<&'static str, TokenType>> = LazyLock::new(|| {
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

    fn is_digit(char: char) -> bool {
        char >= '0' && char <= '9'
    }

    fn is_alpha(char: char) -> bool {
        (char >= 'a' && char <= 'z') || (char >= 'A' && char <= 'Z') || char == '_'
    }

    fn is_alpha_numeric(char: char) -> bool {
        Scanner::is_alpha(char) || Scanner::is_digit(char)
    }

    fn new(chars: Vec<char>) -> Scanner {
        Scanner {
            chars,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn eof(&self) -> Token {
        Token::new(TokenType::Eof, "".to_string(), Object::Nil, self.line)
    }

    fn is_at_end(&self) -> bool {
        !self.peek().is_some()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.previous()
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
        self.chars.get(idx).cloned()
    }

    fn peek(&self) -> Option<char> {
        self.peek_at(self.current)
    }

    fn previous(&self) -> char {
        self.peek_at(self.current - 1).unwrap()
    }

    fn token(&mut self, token_type: TokenType) -> Result<Option<Token>, ScanError> {
        let slice = &self.chars[self.start..self.current];
        let lexeme = String::from_iter(slice);

        let literal = match token_type {
            TokenType::Number => Object::Number(lexeme.parse::<f64>().unwrap()),
            TokenType::String => Object::String(String::from_iter(
                &self.chars[self.start + 1..self.current - 1],
            )),
            _ => Object::Nil,
        };

        Ok(Some(Token::new(token_type, lexeme, literal, self.line)))
    }

    fn string(&mut self) -> Result<Option<Token>, ScanError> {
        while let Some(char) = self.peek() {
            match char {
                '"' => {
                    self.advance();
                    return self.token(TokenType::String);
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => {
                    self.advance();
                }
            }
        }

        Err(format!("[line {}] : Unterminated string.", self.line))
    }

    fn number(&mut self) -> Result<Option<Token>, ScanError> {
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

        if let Some(char) = self.peek() {
            // `123abc` or `123.`
            if Scanner::is_alpha(char) || char == '.' {
                return Err(format!("[line {}] : Invalid number.", self.line));
            }
        }

        self.token(TokenType::Number)
    }

    fn identifier(&mut self) -> Result<Option<Token>, ScanError> {
        while self.peek().map_or(false, Scanner::is_alpha_numeric) {
            self.advance();
        }

        let slice = &self.chars[self.start..self.current];
        let lexeme = String::from_iter(slice);

        let token_type = Scanner::KEYWORDS
            .get(lexeme.as_str())
            .cloned()
            .unwrap_or(TokenType::Identifier);

        self.token(token_type)
    }

    fn scan_token(&mut self) -> Result<Option<Token>, ScanError> {
        self.start = self.current;

        let char = self.advance();

        match char {
            '(' => self.token(TokenType::LeftParen),
            ')' => self.token(TokenType::RightParen),
            '{' => self.token(TokenType::LeftBrace),
            '}' => self.token(TokenType::RightBrace),

            '+' => self.token(TokenType::Plus),
            '-' => self.token(TokenType::Minus),
            '*' => self.token(TokenType::Star),

            '/' => {
                if self.find('/') {
                    while self.peek().map_or(false, |x| x != '\n') {
                        self.advance();
                    }
                    Ok(None)
                } else {
                    self.token(TokenType::Slash)
                }
            }

            '!' => {
                if self.find('=') {
                    self.token(TokenType::BangEqual)
                } else {
                    self.token(TokenType::Bang)
                }
            }
            '=' => {
                if self.find('=') {
                    self.token(TokenType::EqualEqual)
                } else {
                    self.token(TokenType::Equal)
                }
            }
            '>' => {
                if self.find('=') {
                    self.token(TokenType::GreaterEqual)
                } else {
                    self.token(TokenType::Greater)
                }
            }
            '<' => {
                if self.find('=') {
                    self.token(TokenType::LessEqual)
                } else {
                    self.token(TokenType::Less)
                }
            }

            '.' => self.token(TokenType::Dot),
            ',' => self.token(TokenType::Comma),
            ';' => self.token(TokenType::Semicolon),

            ' ' | '\r' | '\t' => Ok(None),
            '\n' => {
                self.line += 1;
                Ok(None)
            }

            '"' => self.string(),

            char => {
                if Scanner::is_digit(char) {
                    self.number()
                } else if Scanner::is_alpha(char) {
                    self.identifier()
                } else {
                    Err(format!("[line {}] : Unknown character.", self.line))
                }
            }
        }
    }
}
