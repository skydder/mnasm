use std::{cell::RefCell,  rc::Rc};

use util::{emit_error, Location};

use tokenizer::{self, Token, TokenGenerator, TokenKind};

use crate::{Analyze, Codegen, Object};

use super::Stmt;

mod let_macro;

#[derive(Debug)]
pub struct Macro<'a> {
    stream: Rc<Vec<Token<'a>>>,
    location: Location<'a>,
    // arg: Vec<()>
}

impl<'a> Macro<'a> {
    pub fn new(location: Location<'a>, stream: Rc<Vec<Token<'a>>>) -> Self {
        Self {
            stream: stream,
            location: location,
        }
    }

    pub fn tokenizer(&self) -> Box<dyn TokenGenerator + 'a> {
        Box::new(MacroTokenizer::new(self.stream.clone(), self.location))
    }
}

struct MacroTokenizer<'a> {
    stream: Rc<Vec<Token<'a>>>,
    nth: RefCell<usize>,
    location: Location<'a>
}

impl<'a> MacroTokenizer<'a> {
    fn new(
        stream: Rc<Vec<Token<'a>>>, // RefCell<std::slice::Iter<'a, Token<'a>>>,
        location: Location<'a>
    ) -> Self {
        Self { stream: stream, nth: RefCell::new(0), location: location }
    }
}

impl<'a> TokenGenerator for MacroTokenizer<'a> {
    fn location(&self) -> Location {
        self.location
    }

    fn peek_token(&self) -> Token {
        if *self.nth.borrow() >= self.stream.len() {
            return Token::new(tokenizer::TokenKind::EOS, 0, self.location);
        } 
        self.stream[*self.nth.borrow()]
    }

    fn next_token(&self) -> Token {
        let token = self.peek_token();
        *self.nth.borrow_mut() += 1;
        return token;
    }

    fn skip_space(&self) {
        while self.peek_token().is(TokenKind::Space) {
            self.next_token();
        }
    }

    fn consume_token(&self, expecting_token: tokenizer::TokenKind) {
        let current_token = self.peek_token();
        if current_token.is(expecting_token) {
            self.next_token();
        } else {
            emit_error!(
                current_token.location,
                "expected {:?}, but found {:?}",
                expecting_token,
                current_token.kind
            )
        }
    }

    fn consume_newline(&self) {
        let current_token = self.peek_token();
        match current_token.kind {
            TokenKind::NewLine => {
                self.next_token();
            },
            TokenKind::EOS => (),
            _ => {
                emit_error!(current_token.location, "expected new line")
            }
        };
    }

    fn consume_indent(&self) {
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



impl<'a> Object for Macro<'a> {}
impl<'a> Analyze for Macro<'a> {
    fn analyze(&self) {
        eprintln!("analyzed");
    }
}
impl<'a> Codegen for Macro<'a> {
    fn codegen(&self) -> String {
        String::new()
    }
}

impl<'a> Stmt<'a> for Macro<'a> {
    fn kind(&self) -> super::StmtKind {
        super::StmtKind::Macro
    }
}
