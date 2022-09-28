use crate::{
    token::{Token, TokenType},
    value::Value,
};
use krab::error_line;
use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.chars().count()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;

        c
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&mut self, look_ahead: usize) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current + look_ahead).unwrap()
        }
    }

    fn add_token(&mut self, token_type: TokenType, value: Option<Value>) {
        let lexeme = &self.source[self.start..self.current];
        let token = Token::new(token_type, lexeme, value, self.line);
        self.tokens.push(token);
    }

    fn string(&mut self) {
        while self.peek(0) != '"' && !self.is_at_end() {
            if self.peek(0) == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            error_line(self.line, "Unterminated string");
            return;
        }

        // We found the closing ".
        self.advance();

        let string = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, Some(Value::String(string)));
    }

    fn number(&mut self) {
        // Find the end of the number or a decimal
        while self.peek(0).is_digit(10) {
            self.advance();
        }

        if self.peek(0) == '.' && self.peek(1).is_digit(10) {
            self.advance();

            // Find the rest of the number
            while self.peek(0).is_digit(10) {
                self.advance();
            }
        }

        let lexeme = &self.source[self.start..self.current];
        let value = lexeme.parse().expect("Must be a valid double");

        self.add_token(TokenType::Number, Some(Value::Number(value)));
    }

    fn identifier(&mut self) {
        loop {
            let c = self.peek(0);
            if c == '_' || c.is_alphanumeric() {
                self.advance();
            } else {
                break;
            }
        }

        let lexeme = &self.source[self.start..self.current];
        let token_type = KEYWORDS.get(lexeme).unwrap_or(&TokenType::Identifier);

        self.add_token(*token_type, None)
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrance, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                let token_type = if self.is_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type, None)
            }
            '=' => {
                let token_type = if self.is_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, None);
            }
            '<' => {
                let token_type = if self.is_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type, None);
            }
            '>' => {
                let token_type = if self.is_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type, None);
            }
            '/' => {
                if self.is_match('/') {
                    while self.peek(0) != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None)
                }
            }
            // Ignore whitespace
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            c if c.is_digit(10) => self.number(),
            c if c == '_' || c.is_alphabetic() => self.identifier(),
            _ => error_line(self.line, "Unexpected character"),
        }
    }

    pub fn scan_tokens(&mut self) -> &[Token] {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "", None, self.line));

        &self.tokens
    }
}
