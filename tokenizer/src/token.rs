// const PUNCTUATOR: &[&str] = &["<", ">", "{", "}", "(", ")", ":", ";"];

use crate::Location;

#[derive(PartialEq, Debug)]
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
    Space,
}

struct TokenBuilder<'a> {
    kind: Option<TokenKind<'a>>,
    len: Option<usize>,
    location: Option<Location<'a>>
} 

impl<'a> TokenBuilder<'a> {
    fn new() -> Self {
        Self { kind: None, len: None, location: None }
    }

    fn kind(self, kind: TokenKind<'a>) -> Self {
        Self {
            kind: Some(kind),
            len:  self.len,
            location: self.location
        }
    } 

    fn location(self, location: Location<'a>) -> Self {
        Self {
            kind: self.kind,
            len:  self.len,
            location: Some(location)
        }
    }

    fn len(self, len: usize) -> Self {
        Self {
            kind: self.kind,
            len:  Some(len),
            location: self.location
        }
    }

    fn build(self) -> Option<Token<'a>> {
        if self.kind.is_some() && self.len.is_some() &&self.location.is_some() {
            Some(Token::new(self.kind.unwrap(), self.len.unwrap(), self.location.unwrap()))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub(crate) len: usize,
    pub location: Location<'a>
}

impl<'a> Token<'a> {
    pub(crate) fn new(kind: TokenKind<'a>, len: usize, location: Location<'a>) -> Self {
        Self { kind, len, location }
    }

    fn check_if_space(s: &'a str) -> Option<TokenBuilder<'a>> {
        let builder = TokenBuilder::new();
        if s.starts_with(' ') {
            Some(builder.kind(TokenKind::Space).len(1))
        } else {
            None
        }
    }

    fn check_if_newline(s: &'a str) -> Option<TokenBuilder<'a>> {
        let builder = TokenBuilder::new();
        if s.starts_with('\n') {
            Some(builder.kind(TokenKind::NewLine).len(1))
        } else {
            None
        }
    }

    fn check_if_punc(s: &'a str) -> Option<TokenBuilder<'a>> {
        let builder = TokenBuilder::new();
        if s.starts_with("<") {
            Some(builder.kind(TokenKind::LessThan).len(1))
        } else if s.starts_with(">") { 
            Some(builder.kind(TokenKind::GreaterThan).len(1))
        } else if s.starts_with("(") {
            Some(builder.kind(TokenKind::OpenParenthesis).len(1))
        } else if s.starts_with(")") {
            Some(builder.kind(TokenKind::CloseParenthesis).len(1))
        } else if s.starts_with("{") {
            Some(builder.kind(TokenKind::OpenBrace).len(1))
        } else if s.starts_with("}") {
            Some(builder.kind(TokenKind::CloseBrace).len(1))
        } else if s.starts_with(":") {
            Some(builder.kind(TokenKind::Colon).len(1))
        } else if s.starts_with(";") {
            Some(builder.kind(TokenKind::Semicolon).len(1))
        } else {
            None
        }
    }

    fn check_if_ident(s: &'a str) -> Option<TokenBuilder<'a>> {
        let builder = TokenBuilder::new();
        if !s.chars().peekable().peek().is_some_and(|c| c.is_ascii_alphabetic()) {
            return None;
        }
        let mut n = 0;
        while s.chars().nth(n).is_some_and(|c| c.is_ascii_alphanumeric() || c == '_') {
            n += 1;
        }
        Some(builder.kind(TokenKind::Identifier(&s[..n])).len(n))
    }

    fn check_if_number(s: &'a str) -> Option<TokenBuilder<'a>> {
        let builder = TokenBuilder::new();
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
        Some(builder.kind(TokenKind::Number(i)).len(n))
    }

    pub(crate) fn tokenize(s: &'a str, location: Location<'a>) -> Option<Token<'a>> {
        let builder = if let Some(b) = Token::check_if_punc(s) {
            b
        } else if let Some(b) = Token::check_if_ident(s) {
            b
        } else if let Some(b) = Token::check_if_number(s) {
            b
        } else if let Some(b) = Token::check_if_space(s) {
            b
        } else if let Some(b) = Token::check_if_newline(s) {
            b
        } else {
            return None;
        };
        builder.location(location).build()
    }

}