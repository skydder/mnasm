use std::cell::RefCell;
use crate::{Token, TokenGenerator, TokenKind};
use util::{emit_error, Location};

#[derive(Debug, Clone, Copy)]
pub struct Tokenizer<'a> {
    location: &'a RefCell<Location<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(location:  &'a RefCell<Location<'a>>,) -> Self {
        Self {
            location: location,
        }
    }

    fn _location(&self) -> Location<'a> {
        self.location.borrow().clone()
    }

    fn current_slice(&self) -> &'a str {
        self.location.borrow().current_slice()
    }

    fn _peek_token(&self) -> Token<'a> {
        let tok = Token::tokenize(self.current_slice(), self.location.borrow().clone());
        tok
    }

    fn advance_location_by_token(&self, token: &Token) {
        if token.is(TokenKind::NewLine) {
            self.location
                .replace_with(|loc| loc.advance_line(1).advance_nth(token.len));
        } else {
            self.location
                .replace_with(|loc| loc.advance_column(token.len).advance_nth(token.len));
        }
    }

    fn _next_token(&self) -> Token<'a> {
        let token = self.peek_token();
        self.advance_location_by_token(&token);
        token
    }
    fn _skip_space(&self) {
        while self.peek_token().is(TokenKind::Space) {
            self.next_token();
        }
    }

    fn _consume_token(&self, expecting_token: TokenKind) {
        let current_token = self.peek_token();
        if current_token.is(expecting_token) {
            self.advance_location_by_token(&current_token);
        } else {
            emit_error!(
                current_token.location,
                "expected {:?}, but found {:?}",
                expecting_token,
                current_token.kind
            )
        }
    }

    fn _consume_newline(&self) {
        let current_token = self.peek_token();
        match current_token.kind {
            TokenKind::NewLine => self.advance_location_by_token(&current_token),
            TokenKind::EOS => (),
            _ => {
                emit_error!(current_token.location, "expected new line")
            }
        }
    }

    fn _consume_indent(&self) {
        let loc = self.location();
        for _ in 0..4 {
            match self.peek_token().kind {
                TokenKind::Space => {
                    self.next_token();
                }
                TokenKind::NewLine | TokenKind::EOS => (),
                _ => {
                    emit_error!(loc, "Indent error, the number of spase must be 4");
                }
            }
        }
    }

    pub fn swap(&self, location:  Location<'a>) -> Location<'a> {
        self.location.replace(location)
    }
}

impl<'a> TokenGenerator<'a> for Tokenizer<'a> {
    fn location(&self) -> Location<'a> {
        self._location()
    }

    fn peek_token(&self) -> Token<'a> {
        self._peek_token()
    }

    fn next_token(&self) -> Token<'a> {
        self._next_token()
    }

    fn skip_space(&self) {
        self._skip_space();
    }

    fn consume_token(&self, consumeing_token: TokenKind) {
        self._consume_token(consumeing_token);
    }

    fn consume_newline(&self) {
        self._consume_newline();
    }

    fn consume_indent(&self) {
        self._consume_indent();
    }
    
    fn kind(&self) -> crate::GenKind {
        crate::GenKind::Tokenizer
    }
}

