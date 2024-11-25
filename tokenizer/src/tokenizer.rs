use std::cell::RefCell;

use crate::{Token, TokenKind};
use util::Location;

pub struct Tokenizer<'a> {
    location: RefCell<Location<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(location: Location<'a>) -> Self {
        Self { location: RefCell::new(location) }
    }

    fn current_slice(&self) -> &'a str {
        self.location.borrow().current_slice()
    }

    pub fn peek_token(&self) -> Token {
        if let Some(token) = Token::tokenize(self.current_slice(), self.location.borrow().clone()) {
            return token;
        } else {
            todo!()
        }
    }

    fn advance_location_by_token(&self, token: &Token) {
        if token.is(TokenKind::NewLine) {
            self.location.replace_with(|loc| loc.advance_line(1).advance_nth(1));
        } else {
            self.location.replace_with(|loc| loc.advance_column(token.len).advance_nth(token.len));
        }
    }

    pub fn next_token(&self) -> Token {
        let token = self.peek_token();
        self.advance_location_by_token(&token);
        token
    }

    pub fn expect_token(&self, expecting_token: TokenKind) {
        let current_token = self.peek_token();
        if current_token.is(expecting_token) {
            self.advance_location_by_token(&current_token);
        } else {
            todo!();
        }
    }
}

#[test]
fn test_tokenizer() {
    use util::Source;
    let source = Source { file: "test", code: "<test:local:text>{\n100; test}".to_string()};
    let loc = Location::new(&source);
    let t = Tokenizer::new(loc);
    loop {
        eprintln!("{:#?}", t.next_token());
    }
}