use std::cell::RefCell;

use util::{AsmError, AsmResult, Location, Token, TokenKind, Tokenizer as TokenizerTrait};

#[derive(Debug, Clone)]
pub struct Tokenizer<'code> {
    location: RefCell<Location<'code>>,
    eos: RefCell<Location<'code>>,
}

impl<'code> Tokenizer<'code> {
    pub fn new(location: Location<'code>, eos: Location<'code>) -> Self {
        Self {
            location: RefCell::new(location),
            eos: RefCell::new(eos),
        }
    }

    pub fn advance_location_by_token(&self, token: &Token) {
        if token.is(&TokenKind::NewLine) {
            self.location
                .replace_with(|loc| loc.advance_line(1).advance_nth(token.len));
        } else {
            self.location
                .replace_with(|loc| loc.advance_column(token.len).advance_nth(token.len));
        }
    }

    pub fn swap(&self, location: Location<'code>, eos: Location<'code>) -> Location<'code> {
        self.eos.replace(eos);
        self.location.replace(location)
    }
}

impl<'code> TokenizerTrait<'code> for Tokenizer<'code> {
    fn location(&self) -> Location<'code> {
        self.location.borrow().clone()
    }

    fn peek_token(&self) -> Token<'code> {
        if self.location >= self.eos {
            return Token::new(TokenKind::EOS, 0, self.eos.borrow().clone());
        }
        let tok = Token::tokenize(self.location());
        tok
    }

    fn next_token(&self) -> Token<'code> {
        let token = self.peek_token();
        self.advance_location_by_token(&token);
        token
    }

    fn consume_token(&self, expecting_token: TokenKind) -> AsmResult<'code, ()> {
        let current_token = self.peek_token();
        if current_token.is(&expecting_token) {
            self.advance_location_by_token(&current_token);
            Ok(())
        } else {
            Err(AsmError::ParseError(
                current_token.location,
                format!(
                    "expected {:?}, but found {:?}",
                    expecting_token, current_token.kind
                ),
                String::new(),
            ))
        }
    }

    fn skip_space(&self) {
        while self.peek_token().is(&TokenKind::Space) {
            self.next_token();
        }
    }
}
