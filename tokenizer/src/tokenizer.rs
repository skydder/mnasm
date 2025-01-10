use std::cell::RefCell;

use crate::{Token, TokenGenerator, TokenKind};
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

    fn _location(&self) -> Location {
        *self.location.borrow()
    }

    fn current_slice(&self) -> &'a str {
        self.location.borrow().current_slice()
    }

    fn _peek_token(&self) -> Token {
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

    fn _next_token(&self) -> Token {
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
}

impl<'a> TokenGenerator for Tokenizer<'a> {
    fn location(&self) -> Location {
        self._location()
    }

    fn peek_token(&self) -> Token {
        self._peek_token()
    }

    fn next_token(&self) -> Token {
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
