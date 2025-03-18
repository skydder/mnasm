// const PUNCTUATOR: &[&str] = &["<", ">", "{", "}", "(", ")", ":", ";"];

use crate::Location;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenKind<'a> {
    LessThan,
    GreaterThan,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    OpenSquareBracket,
    CloseSquareBracket,
    Colon,
    Semicolon,
    Comma,
    Minus,
    Dot,
    Not,
    At,
    BackQuote,
    // MacroEnd,
    Number(u64),
    String(&'a str),
    Identifier(&'a str),
    // Reserved(&'a str),
    NewLine,
    Space,
    EOS,
    Arcane(&'a str), // for infix macro
                     // Add,
                     // Mov,
                     // And,
                     // Sub,
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
                TokenKind::OpenSquareBracket => format!("["),
                TokenKind::CloseSquareBracket => format!("]"),
                TokenKind::Colon => format!(":"),
                TokenKind::Semicolon => format!(";"),
                TokenKind::Comma => format!(", "),
                TokenKind::Minus => format!("-"),
                TokenKind::Dot => format!("."),
                TokenKind::Not => format!("!"),
                TokenKind::At => format!("@"),
                // TokenKind::MacroEnd => format!("@:"),
                TokenKind::BackQuote => format!("`"),
                TokenKind::Number(i) => format!("{}", i),
                TokenKind::String(s) => format!("{:?}", s),
                TokenKind::Identifier(i) => format!("{}", i),
                TokenKind::NewLine => format!("\n"),
                TokenKind::Space => format!(" "),
                TokenKind::EOS => format!("\n"),
                TokenKind::Arcane(_) => format!(""),
                // TokenKind::Add => format!("+="),
                // TokenKind::And => format!("&="),
                // TokenKind::Mov => format!("="),
                // TokenKind::Sub => format!("-="),
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub len: usize,
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

    pub fn tokenize(location: Location<'a>) -> Token<'a> {
        let raw = location.current_slice();
        match raw.chars().nth(0) {
            None => Token::new(TokenKind::EOS, 0, location),
            Some(' ') => Token::new(TokenKind::Space, 1, location),
            Some('\n') => Token::new(TokenKind::NewLine, 1, location),
            Some('#') => {
                let mut i = 0;
                while raw.chars().nth(i).is_some_and(|c| c != '\n') {
                    i += 1;
                }
                Token::new(TokenKind::NewLine, i + 1, location)
            }
            s if s.is_some_and(|c| c.is_ascii_digit()) => {
                let mut i = 0;
                while raw.chars().nth(i).is_some_and(|c| c.is_ascii_digit()) {
                    i += 1;
                }
                let integer = raw[..i].parse().unwrap();
                Token::new(TokenKind::Number(integer), i, location)
            }
            s if s.is_some_and(|c| c.is_ascii_alphabetic() || c == '_') => {
                let mut i = 0;
                while raw
                    .chars()
                    .nth(i)
                    .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
                {
                    i += 1;
                }
                Token::new(TokenKind::Identifier(&raw[..i]), i, location)
            }
            Some('\"') => {
                let mut i = 1;
                while !raw.chars().nth(i).is_some_and(|c| c == '"') {
                    i += 1;
                }
                Token::new(TokenKind::String(&raw[1..i]), i + 1, location)
            }
            Some('>') => Token::new(TokenKind::GreaterThan, 1, location),
            Some('<') => Token::new(TokenKind::LessThan, 1, location),
            Some('(') => Token::new(TokenKind::OpenParenthesis, 1, location),
            Some(')') => Token::new(TokenKind::CloseParenthesis, 1, location),
            Some('{') => Token::new(TokenKind::OpenBrace, 1, location),
            Some('}') => Token::new(TokenKind::CloseBrace, 1, location),
            Some(':') => Token::new(TokenKind::Colon, 1, location),
            Some(';') => Token::new(TokenKind::Semicolon, 1, location),
            Some(',') => Token::new(TokenKind::Comma, 1, location),
            Some('-') => Token::new(TokenKind::Minus, 1, location),
            Some('.') => Token::new(TokenKind::Dot, 1, location),
            Some('@') => Token::new(TokenKind::At, 1, location),
            Some('`') => Token::new(TokenKind::BackQuote, 1, location),
            Some('!') => Token::new(TokenKind::Not, 1, location),
            Some('[') => Token::new(TokenKind::OpenSquareBracket, 1, location),
            Some(']') => Token::new(TokenKind::CloseSquareBracket, 1, location),
            _ => Token::new(TokenKind::Arcane(&raw[..1]), 1, location),
        }
    }
}
