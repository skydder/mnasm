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
        Token::tokenize(self.current_slice(), self.location.borrow().clone())
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
        self.peek_token()
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
            emit_error!(current_token.location, "expected {:?}, but found {:?}", expecting_token, current_token.kind)
        }
    }

    pub fn expect_newline(&self) {
        let current_token = self.peek_token();
        match current_token.kind {
            TokenKind::NewLine => self.advance_location_by_token(&current_token),
            TokenKind::EOF => (),
            _ => {
                emit_error!(current_token.location, "expected new line")
            }
        }
    }

    pub fn expect_symbol(&self, expecting_token: TokenKind) {
        self.skip_space();
        let current_token = self.peek_token();
        if current_token.is(expecting_token) {
            self.advance_location_by_token(&current_token);
        } else {
            emit_error!(
                self.location(),
                "expected {:#?}, but found {:#?}",
                expecting_token,
                current_token.kind
            )
        }
    }

    pub fn expect_indent(&self) {
        let loc = self.location();
        for _ in 0..4 {
            match self.peek_token().kind {
                TokenKind::Space => {
                    self.next_token();
                }
                TokenKind::NewLine | TokenKind::EOF => (),
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
        code: "< test > {\n     \ntest()\n}".to_string(),
    };
    let loc = Location::new(&source);
    let t = Tokenizer::new(loc);
    // t.expect_indent();;
    loop {
        eprintln!("{:#?}", t.next_token());
    }
}
