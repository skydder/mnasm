use std::cell::RefCell;

use crate::{Token, TokenKind};
use util::{emit_error, Location};

pub struct Tokenizer<'a> {
    location: RefCell<Location<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(location: Location<'a>) -> Self {
        Self {
            location: RefCell::new(location),
        }
    }

    pub fn location(&self) -> Location {
        *self.location.borrow()
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
            self.location
                .replace_with(|loc| loc.advance_line(1).advance_nth(1));
        } else {
            self.location
                .replace_with(|loc| loc.advance_column(token.len).advance_nth(token.len));
        }
    }

    pub fn next_token(&self) -> Token {
        let token = self.peek_token();
        self.advance_location_by_token(&token);
        token
    }
    pub fn skip_space(&self) {
        while self.peek_token().is(TokenKind::Space) {
            self.next_token();
        }
    }

    pub fn peek_symbol(&self) -> Token {
        self.skip_space();
        if let Some(token) = Token::tokenize(self.current_slice(), self.location.borrow().clone()) {
            return token;
        } else {
            todo!()
        }
    }

    pub fn next_symbol(&self) -> Token {
        self.skip_space();
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

    pub fn expect_symbol(&self, expecting_token: TokenKind) {
        self.skip_space();
        let current_token = self.peek_token();
        if current_token.is(expecting_token) {
            self.advance_location_by_token(&current_token);
        } else {
            todo!();
        }
    }
    
    pub fn expect_indent(&self) {
        let loc = self.location();
        for _ in 0..4 {
            match self.next_token().kind {
                TokenKind::Space => (),
                _ => {
                    emit_error!(loc, "Indent error, the number of spase must be 4");
                }
            }
        }
    }
}

#[test]
fn test_tokenizer() {
    use util::Source;
    let source = Source {
        file: "test",
        code: "< test : local : text > {\n100; test}".to_string(),
    };
    let loc = Location::new(&source);
    let t = Tokenizer::new(loc);
    loop {
        eprintln!("{:#?}", t.next_symbol());
    }
}
