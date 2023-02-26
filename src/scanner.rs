use std::{
    collections::HashMap,
    str::{Chars, FromStr},
};

use multipeek::{multipeek, MultiPeek};

use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    pub source: MultiPeek<Chars<'a>>,
    pub tokens: Vec<Token>,
    current_lexeme_buf: Vec<char>,
    current: u64,
    line: u64,
    keywords: HashMap<String, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source: multipeek(source.chars()),
            tokens: Vec::new(),
            current_lexeme_buf: Vec::new(),
            current: 0,
            line: 1,
            keywords: HashMap::from([
                ("and".into(), TokenType::And),
                ("class".into(), TokenType::Class),
                ("else".into(), TokenType::Else),
                ("false".into(), TokenType::False),
                ("fun".into(), TokenType::Fun),
                ("for".into(), TokenType::For),
                ("if".into(), TokenType::If),
                ("nil".into(), TokenType::Nil),
                ("or".into(), TokenType::Or),
                ("print".into(), TokenType::Print),
                ("return".into(), TokenType::Return),
                ("super".into(), TokenType::Super),
                ("this".into(), TokenType::This),
                ("true".into(), TokenType::True),
                ("var".into(), TokenType::Var),
                ("while".into(), TokenType::While),
            ]),
        }
    }
}

impl<'a> Scanner<'a> {
    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.scan_token()
        }

        self.tokens.push(Token {
            r#type: TokenType::EOF,
            lexeme: "".to_string(),
            line: self.line,
        })
    }

    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn peek_nth(&mut self, n: usize) -> Option<&char> {
        self.source.peek_nth(n)
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    fn advance(&mut self) -> Option<char> {
        let char = self.source.next()?;
        self.current += 1;
        self.current_lexeme_buf.push(char);
        Some(char)
    }

    fn advance_on_match(&mut self, c: char) -> bool {
        if self.peek() == Some(&c) {
            self.advance();
            return true;
        }
        false
    }

    fn consume_digits(&mut self) {
        loop {
            if let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    self.advance();
                    continue;
                }
            }
            break;
        }
    }

    fn scan_number(&mut self) {
        self.consume_digits();

        if Some(&'.') == self.peek() {
            if let Some(c) = self.peek_nth(1) {
                if c.is_ascii_digit() {
                    // consume the "."
                    self.advance();
                    // consume the remaining digits
                    self.consume_digits();
                }
            }
        }
        let lexeme = String::from_iter(self.current_lexeme_buf.drain(..));
        // parse the number from the lexeme (string)
        match f64::from_str(lexeme.as_str()) {
            Ok(f) => self.tokens.push(Token {
                r#type: TokenType::Number(f),
                lexeme,
                line: self.line,
            }),
            Err(_) => self.tokens.push(Token {
                r#type: TokenType::SyntaxErr("Failed to parse number".to_string()),
                lexeme,
                line: self.line,
            }),
        }
    }

    // keep advancing the scanner until the next
    // char is equals to char c
    fn advance_until(&mut self, c: char) {
        loop {
            if let Some(next) = self.peek() {
                if next != &c {
                    self.advance();
                    continue;
                }
            }
            break;
        }
    }

    // Matches [A-Z][a-z] and "_"
    fn is_alpha(c: &char) -> bool {
        c.is_ascii_alphabetic() || c == &'_'
    }

    // Matches [A-Z][a-z][0-9] and "_"
    fn is_alphanumeric(c: &char) -> bool {
        Self::is_alpha(c) || c.is_ascii_digit()
    }

    fn add_token(&mut self, r#type: TokenType) {
        self.tokens.push(Token {
            r#type,
            lexeme: String::from_iter(self.current_lexeme_buf.drain(..)),
            line: self.line,
        })
    }

    fn scan_identifier_or_keyword(&mut self) {
        loop {
            if let Some(c) = self.peek() {
                if Self::is_alphanumeric(c) {
                    self.advance();
                    continue;
                }
            }
            break;
        }
        let lexeme = String::from_iter(self.current_lexeme_buf.drain(..));
        match self.keywords.get(&lexeme) {
            // if the map contains this lexeme, then
            // it's a reserved keyword
            Some(token_type) => self.tokens.push(Token {
                r#type: token_type.clone(),
                lexeme,
                line: self.line,
            }),
            // otherwise it's just an identifier
            None => self.tokens.push(Token {
                r#type: TokenType::Identifier,
                lexeme,
                line: self.line,
            }),
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance().unwrap();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '/' => {
                if self.advance_on_match('/') {
                    // in this case it's a comment line
                    // so we consume it until we reach a
                    // new line char '\n'
                    self.advance_until('\n');
                    self.current_lexeme_buf.clear();
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.advance_on_match('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.advance_on_match('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '>' => {
                if self.advance_on_match('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '<' => {
                if self.advance_on_match('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '"' => {
                self.advance_until('"');
                if self.is_at_end() {
                    self.add_token(TokenType::SyntaxErr("Unterminated string".to_string()))
                }
                // eat the closing ".
                self.advance();

                let lexeme = String::from_iter(self.current_lexeme_buf.drain(..));
                // trim the surrounding quotes
                let literal = lexeme.trim_matches('"').to_string();
                self.tokens.push(Token {
                    r#type: TokenType::String(literal),
                    lexeme,
                    line: self.line,
                })
            }
            ' ' | '\r' | '\t' => self.current_lexeme_buf.clear(), // clear and move ahead.
            '\n' => {
                self.current_lexeme_buf.clear();
                self.line += 1
            }
            d if d.is_ascii_digit() => self.scan_number(),
            a if Self::is_alpha(&a) => self.scan_identifier_or_keyword(),
            _ => self.add_token(TokenType::SyntaxErr("Invalid character".to_string())),
        }
    }
}
