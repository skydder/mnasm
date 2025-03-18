use std::
    cell::RefCell
;

use util::{emit_error, Location, Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Tokenizer<'a> {
    location: RefCell<Location<'a>>,
    eos: RefCell<Location<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(location: Location<'a>, eos: Location<'a>) -> Self {
        Self {
            location: RefCell::new(location),
            eos: RefCell::new(eos),
        }
    }

    pub fn location(&self) -> Location<'a> {
        *self.location.borrow()
    }

    pub fn peek_token(&self) -> Token<'a> {
        if self.location >= self.eos {
            return Token::new(TokenKind::EOS, 0, *self.eos.borrow());
        }
        let tok = Token::tokenize(*self.location.borrow());
        tok
    }

    pub fn advance_location_by_token(&self, token: &Token) {
        if token.is(TokenKind::NewLine) {
            self.location
                .replace_with(|loc| loc.advance_line(1).advance_nth(token.len));
        } else {
            self.location
                .replace_with(|loc| loc.advance_column(token.len).advance_nth(token.len));
        }
    }

    pub fn next_token(&self) -> Token<'a> {
        let token = self.peek_token();
        self.advance_location_by_token(&token);
        token
    }

    pub fn consume_token(&self, expecting_token: TokenKind) {
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

    pub fn swap(&self, location: Location<'a>, eos: Location<'a>) -> Location<'a> {
        self.eos.replace(eos);
        self.location.replace(location)
    }
}