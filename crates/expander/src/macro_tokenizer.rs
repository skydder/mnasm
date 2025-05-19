// Vec<TokenKind>

use std::{cell::RefCell, rc::Rc};

use util::{AsmError, Location, Token, TokenKind, Tokenizer};

#[derive(Clone, Debug)]
pub struct MacroTokenizer<'code> {
    stream: Rc<Vec<TokenKind>>,
    positon: RefCell<usize>,
    location: Location<'code>,
}

impl<'code> MacroTokenizer<'code> {
    pub fn new(location: Location<'code>, stream: Rc<Vec<TokenKind>>) -> Self {
        Self {
            stream,
            location,
            positon: RefCell::new(0),
        }
    }
}

impl<'code> Tokenizer<'code> for MacroTokenizer<'code> {
    fn location(&self) -> util::Location<'code> {
        self.location.clone()
    }

    fn peek_token(&self) -> util::Token<'code> {
        self.stream
            .get(*self.positon.borrow())
            .unwrap_or(&TokenKind::EOS)
            .to_be_token(self.location.clone())
    }

    fn next_token(&self) -> util::Token<'code> {
        let token = self.peek_token();
        *self.positon.borrow_mut() += 1;
        token
    }

    fn consume_token(&self, consumeing_token: TokenKind) -> util::AsmResult<'code, ()> {
        let current_token: Token<'_> = self.peek_token();
        if current_token.is(&consumeing_token) {
            *self.positon.borrow_mut() += 1;
            Ok(())
        } else {
            Err(AsmError::ParseError(
                current_token.location,
                format!(
                    "expected {:?}, but found {:?}",
                    consumeing_token, current_token.kind
                ),
                String::new(),
            ))
        }
    }
}
