use std::{cell::RefCell, io::Read};

use util::open_safely;

pub struct Source<'a> {
    pub code: String,
    pub file: &'a str,
}

impl<'a> Source<'a> {
    pub fn new(file: &'a str) -> Self {
        let mut code = String::new();
        open_safely(file).read_to_string(&mut code).unwrap_or_else(|_| {
            eprintln!("failed to load '{}' into 'String'", file);
            ::std::process::exit(1);
        });
        Self { code: code, file: file }
    }

    pub fn nth(&self, n: usize) -> &str {
        &self.code[n..]
    }
}

const PUNCTUATOR: &[&str] = &["<", ">", "{", "}", "(", ")", ":", ";"];

fn is_punc(c: &str) -> bool {
    for punc in PUNCTUATOR {
        if c.starts_with(punc) {
            return true;
        }
    }
    false
}


pub enum TokenKind<'a> {
    LessThan,
    GreaterThan,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Colon,
    Semicolon,
    Number(i64),
    String(&'a str),
    Identifier(&'a str),
    NewLine,
    Indent,
}

pub struct Token<'a> {
    kind: TokenKind<'a>,
    len: usize
}

impl<'a> Token<'a> {
    fn new(kind: TokenKind<'a>, len: usize) -> Self {
        Self { kind, len }
    }
    fn check_if_punc(s: &'a str) -> Option<Token> {
        if s.starts_with("<") {
            return Some( Self::new(TokenKind::LessThan, 1));
        } else if s.starts_with(">") {
            return Some(Self::new(TokenKind::GreaterThan, 1));
        } else if s.starts_with("(") {
            return Some(Self::new(TokenKind::OpenParenthesis, 1));
        } else if s.starts_with(")") {
            return Some(Self::new(TokenKind::CloseParenthesis, 1));
        } else if s.starts_with("{") {
            return Some(Self::new(TokenKind::OpenBrace, 1));
        } else if s.starts_with("}") {
            return Some(Self::new(TokenKind::CloseBrace, 1));
        } else if s.starts_with(":") {
            return Some(Self::new(TokenKind::Colon, 1));
        } else if s.starts_with(";") {
            return Some(Self::new(TokenKind::Semicolon, 1));
        } else {
            None
        }
    }

    fn check_if_ident(s: &'a str) -> Option<Token> {
        if !s.chars().peekable().peek().is_some_and(|c| c.is_alphabetic()) {
            return None;
        }
        let mut n = 0;
        while s.chars().nth(n).is_some_and(|c| c.is_alphanumeric() || c == '_') {
            n += 1;
        }
        Some(Token::new(TokenKind::Identifier(&s[..n]), n))
    }

    fn check_if_number(s: &'a str) -> Option<Token> {
        if !s.chars().peekable().peek().is_some_and(|c| c.is_ascii_digit()) {
            return None;
        }
        let mut n = 0;
        while s.chars().nth(n).is_some_and(|c| c.is_ascii_digit()) {
            n += 1;
        }
        let i = s[..n].parse::<i64>().unwrap_or_else(|_| {
            eprintln!("failed to convert '<str>' to '<i64>'");
            ::std::process::exit(1);
        });
        Some(Token::new(TokenKind::Number(i), n))
    }

}

pub struct Location<'a> {
    file: &'a str,
    line: usize,
    column: usize
}

impl<'a> Location<'a> {
    pub fn new(file: &'a str) -> Self {
        Self { file: file, line: 1, column: 1 }
    }

    pub fn add(self, dl: usize, dc: usize) -> Self {
        Self { file: self.file, line: self.line + dl, column: self.column + dc }
    }
}

pub struct Tokenizer<'a> {
    source: Source<'a>,
    location: RefCell<Location<'a>>,
    nth: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(file: &'a str) -> Self {
        Self { source: Source::new(file), location: RefCell::new(Location::new(file)), nth: 0 }
    }

    pub fn peek(&self) -> Token {
        if let Some(token) = Token::check_if_punc(self.source.nth(self.nth)) {
            return token;
        } else if let Some(token) = Token::check_if_ident(self.source.nth(self.nth)) {
            return token;
        }
        todo!()
    }

}