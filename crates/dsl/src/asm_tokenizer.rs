use std::cell::Cell;

use util::{emit_error, Location, Source, Source2, Token, TokenKind, Tokenizer};

#[derive(Debug, Clone)]
pub(crate) struct TKNZR4ASM<'a> {
    location: Cell<Location<'a>>,
}

impl<'a> TKNZR4ASM<'a> {
    pub(crate) fn new(input: String, original_sources: Source2<'a>) -> Self {
        Self {
            location: Cell::new(Location::new_source(
                original_sources,
                Source::new(input, "macro"),
            )),
        }
    }

    fn advance_location_by_token(&self, token: &Token) {
        if token.is(TokenKind::NewLine) {
            let new_loc = self.location().advance_line(1).advance_nth(token.len);
            self.location.replace(new_loc);
        } else {
            let new_loc = self
                .location()
                .advance_line(token.len)
                .advance_nth(token.len);
            self.location.replace(new_loc);
        }
    }
}

#[allow(unused_variables)]
impl<'a> Tokenizer<'a> for TKNZR4ASM<'a> {
    fn location(&self) -> util::Location<'a> {
        self.location.get()
    }

    fn peek_token(&self, macro_expand: bool) -> util::Token<'a> {
        assert!(!macro_expand);
        if self.location().is_eos() {
            Token::new(TokenKind::EOS, 0, self.location())
        } else {
            Token::tokenize(self.location())
        }
    }

    fn next_token(&self) -> util::Token<'a> {
        let token = self.peek_token(false);
        self.advance_location_by_token(&token);
        token
    }

    fn skip_space(&self, macro_expand: bool) {
        while self.peek_token(macro_expand).is(TokenKind::Space) {
            self.skip_token();
        }
    }

    fn skip_token(&self) {
        let _ = self.next_token();
    }

    fn consume_token(&self, consumeing_token: util::TokenKind<'a>) {
        let current_token = self.peek_token(false);
        if current_token.is(consumeing_token) {
            self.advance_location_by_token(&current_token);
        } else {
            emit_error!(
                current_token.location,
                "expected {:?}, but found {:?}",
                consumeing_token,
                current_token.kind
            )
        }
    }

    fn consume_newline(&self) {
        let current_token = self.peek_token(false);
        match current_token.kind {
            TokenKind::NewLine => {
                self.skip_token();
            }
            TokenKind::Semicolon => {
                self.skip_token();
            }
            TokenKind::EOS => (),
            _ => {
                emit_error!(
                    current_token.location,
                    "expected new line: {:#?}",
                    current_token
                )
            }
        }
    }

    fn consume_indent(&self) {
        for _ in 0..4 {
            match self.peek_token(false).kind {
                TokenKind::Space => {
                    self.skip_token();
                }
                TokenKind::NewLine | TokenKind::EOS => (),
                _ => (),
            }
        }
    }

    fn add_to_code(&self, tokenkind: util::TokenKind<'a>) {}

    fn code(&self) -> String {
        String::new()
    }
}
