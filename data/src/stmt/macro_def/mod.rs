use std::{cell::RefCell, rc::Rc};

use util::{emit_error, Location};

use tokenizer::{self, Token, TokenGenerator, TokenKind};

use crate::{Analyze, Codegen, Object};

use super::Stmt;

mod let_macro;

#[derive(Debug)]
pub struct Macro<'a> {
    stream: Box<Vec<Token<'a>>>,
    location: Location<'a>,
    tokenizer: &'a (dyn TokenGenerator + 'a)
}

impl<'a> Macro<'a> {
    pub fn new(location: Location<'a>, stream: Vec<Token<'a>>, tokenizer: &'a MacroTokenizer<'a>) -> Self {
        Self {
            stream: Box::new(stream.clone()),
            location: location,
            tokenizer: tokenizer,
        }
    }

    // fn iter(&self) -> MacroTokenizer<'a> {
    //     MacroTokenizer::new(self.stream.to_vec())
    // }

    pub fn tokenizer(&self) -> &'a (dyn TokenGenerator + 'a) {
        self.tokenizer
    }
}

// impl<'a> std::iter::IntoIterator for Macro<'a> {
//     type Item = Token<'a>;

//     type IntoIter = MacroTokenizer<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }


#[derive(Debug)]
pub struct MacroTokenizer<'a>(Vec<Token<'a>>, RefCell<usize>);

impl<'a> MacroTokenizer<'a> {
    pub fn new(
        stream: Vec<Token<'a>>
    ) -> Self {
        Self(stream, RefCell::new(0))
    }
}

impl<'a> TokenGenerator for MacroTokenizer<'a> {
    fn location(&self) -> Location {
        self.0[*self.1.borrow()].location
    }

    fn peek_token(&self) -> Token {
        if *self.1.borrow() >= self.0.len() {
            return Token::new(tokenizer::TokenKind::EOS, 0, self.0.last().unwrap().location);
        } 
        self.0[*self.1.borrow()]
    }

    fn next_token(&self) -> Token {
        let token = self.peek_token();
        *self.1.borrow_mut() += 1;
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

// impl<'a> std::iter::Iterator for MacroTokenizer<'a> {
//     type Item = Token<'a>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let item = self.0.get(*self.1.borrow());
//         *self.1.borrow_mut() += 1;
//         item.copied()
//     }
// }


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
