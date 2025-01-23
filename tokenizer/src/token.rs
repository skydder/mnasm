// const PUNCTUATOR: &[&str] = &["<", ">", "{", "}", "(", ")", ":", ";"];

use util::Location;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenKind<'a> {
    LessThan,
    GreaterThan,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Colon,
    Semicolon,
    Comma,
    Minus,
    Dot,
    At,
    Number(u64),
    String(&'a str),
    Identifier(&'a str),
    // Reserved(&'a str),
    NewLine,
    Space,
    EOS,
}

impl<'a> std::fmt::Display for TokenKind<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                TokenKind::LessThan => format!("<"),
                TokenKind::GreaterThan => format!(">"),
                TokenKind::OpenParenthesis => format!("("),
                TokenKind::CloseParenthesis => format!(")"),
                TokenKind::OpenBrace => format!("{{"),
                TokenKind::CloseBrace => format!("}}"),
                TokenKind::Colon => format!(":"),
                TokenKind::Semicolon => format!(";"),
                TokenKind::Comma => format!(","),
                TokenKind::Minus => format!("-"),
                TokenKind::Dot => format!("."),
                TokenKind::At => format!("@"),
                TokenKind::Number(i) => format!("{}", i),
                TokenKind::String(s) => format!("{}", s),
                TokenKind::Identifier(i) => format!("{}", i),
                TokenKind::NewLine => format!("\n"),
                TokenKind::Space => format!(" "),
                TokenKind::EOS => format!("\n"),
            }
        )
    }
}

struct TokenBuilder<'a> {
    kind: Option<TokenKind<'a>>,
    len: Option<usize>,
    location: Option<Location<'a>>,
}

impl<'a> TokenBuilder<'a> {
    fn new() -> Self {
        Self {
            kind: None,
            len: None,
            location: None,
        }
    }

    fn kind(self, kind: TokenKind<'a>) -> Self {
        Self {
            kind: Some(kind),
            len: self.len,
            location: self.location,
        }
    }

    fn location(self, location: Location<'a>) -> Self {
        Self {
            kind: self.kind,
            len: self.len,
            location: Some(location),
        }
    }

    fn len(self, len: usize) -> Self {
        Self {
            kind: self.kind,
            len: Some(len),
            location: self.location,
        }
    }

    fn build(self) -> Token<'a> {
        if self.kind.is_some() && self.len.is_some() && self.location.is_some() {
            Token::new(
                self.kind.unwrap(),
                self.len.unwrap(),
                self.location.unwrap(),
            )
        } else {
            todo!()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub(crate) len: usize,
    pub location: Location<'a>,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind<'a>, len: usize, location: Location<'a>) -> Self {
        Self {
            kind,
            len,
            location,
        }
    }

    pub fn is_identifier(&self) -> bool {
        match self.kind {
            TokenKind::Identifier(_) => true,
            _ => false,
        }
    }

    pub fn is(&self, token: TokenKind) -> bool {
        if self.kind == token {
            true
        } else {
            false
        }
    }

    pub fn get_identifier(&self) -> Option<&'a str> {
        match self.kind {
            TokenKind::Identifier(ident) => Some(ident),
            _ => None,
        }
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
        } else if s.starts_with(",") {
            Some(builder.kind(TokenKind::Comma).len(1))
        } else if s.starts_with("-") {
            Some(builder.kind(TokenKind::Minus).len(1))
        } else if s.starts_with(".") {
            Some(builder.kind(TokenKind::Dot).len(1))
        } else if s.starts_with("@") {
            Some(builder.kind(TokenKind::At).len(1))
        } else {
            None
        }
    }

    fn check_if_ident(s: &'a str) -> Option<TokenBuilder<'a>> {
        let builder = TokenBuilder::new();
        if !s
            .chars()
            .peekable()
            .peek()
            .is_some_and(|c| c.is_ascii_alphabetic() || *c == '_')
        {
            return None;
        }
        let mut n = 0;
        while s
            .chars()
            .nth(n)
            .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            n += 1;
        }
        Some(builder.kind(TokenKind::Identifier(&s[..n])).len(n))
    }

    fn check_if_number(s: &'a str) -> Option<TokenBuilder<'a>> {
        let builder = TokenBuilder::new();
        if !s
            .chars()
            .peekable()
            .peek()
            .is_some_and(|c| c.is_ascii_digit())
        {
            return None;
        }
        let mut n = 0;
        while s.chars().nth(n).is_some_and(|c| c.is_ascii_digit()) {
            n += 1;
        }
        let i = s[..n].parse::<u64>().unwrap_or_else(|_| {
            eprintln!("failed to convert '<str>' to '<u64>'");
            ::std::process::exit(1);
        });
        Some(builder.kind(TokenKind::Number(i)).len(n))
    }

    fn check_if_comment(s: &'a str) -> Option<TokenBuilder<'a>> {
        if !s.starts_with('#') {
            return None;
        }
        let builder = TokenBuilder::new();
        let mut n = 0;
        while s.chars().nth(n).is_some_and(|c| c != '\n') {
            n += 1;
        }
        Some(builder.kind(TokenKind::NewLine).len(n + 1))
    }

    fn check_if_string(s: &'a str) -> Option<TokenBuilder<'a>> {
        let builder = TokenBuilder::new();
        if !s.chars().peekable().peek().is_some_and(|c| *c == '"') {
            return None;
        }
        let mut n = 1;
        while !s.chars().nth(n).is_some_and(|c| c == '"') {
            n += 1;
        }
        Some(builder.kind(TokenKind::String(&s[1..n])).len(n + 1))
    }

    pub(crate) fn tokenize(s: &'a str, location: Location<'a>) -> Token<'a> {
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
        } else if let Some(b) = Token::check_if_comment(s) {
            b
        } else if let Some(b) = Token::check_if_string(s) {
            b
        } else {
            TokenBuilder::new().kind(TokenKind::EOS).len(1)
        };
        builder.location(location).build()
    }
}
