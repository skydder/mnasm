use util::Location;

use tokenizer::{self, Token, TokenGenerator, TokenKind, Tokenizer};

use crate::{Analyze, Codegen, Label, Object};

use super::Stmt;

mod let_macro;

#[derive(Debug)]
pub struct Macro<'a> {
    stream: (Location<'a>, Location<'a>),
    args: Vec<Label<'a>>,
    location: Location<'a>,
}

impl<'a> Macro<'a> {
    pub fn new(
        location: Location<'a>,
        args: Vec<Label<'a>>,
        stream: (Location<'a>, Location<'a>),
    ) -> Self {
        Self {
            // stream: Box::new(stream.clone()),
            stream: stream,
            args: args,
            location: location,
        }
    }

    pub fn ingredients_of_tokenizer(&self) -> (Location<'a>, Location<'a>) {
        self.stream
    }

    pub fn location(&self) -> Location<'a> {
        self.location
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MacroTokenizer2<'a> {
    pub tokenizer: Tokenizer<'a>,
    end: Location<'a>,
    pub ret: Location<'a>,
}

impl<'a> MacroTokenizer2<'a> {
    pub fn new(stream: (Tokenizer<'a>, Location<'a>), ret: Location<'a>) -> Self {
        Self {
            tokenizer: stream.0,
            end: stream.1,
            ret: ret,
        }
    }
}

impl<'a> TokenGenerator<'a> for MacroTokenizer2<'a> {
    fn location(&self) -> Location<'a> {
        self.tokenizer.clone().location()
    }

    fn peek_token(&self) -> Token<'a> {
        let current = self.tokenizer.peek_token();
        if current.location >= self.end {
            return Token::new(TokenKind::EOS, 0, self.end);
        } else {
            current
        }
    }

    fn next_token(&self) -> Token<'a> {
        let current = self.peek_token();
        if current.kind != TokenKind::EOS {
            self.tokenizer.next_token();
        }
        current
    }

    fn skip_space(&self) {
        self.tokenizer.skip_space();
    }

    fn consume_token(&self, consumeing_token: TokenKind) {
        self.tokenizer.consume_token(consumeing_token);
    }

    fn consume_newline(&self) {
        self.tokenizer.consume_newline();
    }

    fn consume_indent(&self) {
        self.tokenizer.consume_indent();
    }

    fn kind(&self) -> tokenizer::GenKind {
        tokenizer::GenKind::MacroTokenizer
    }
}

impl<'a> Object for Macro<'a> {}
impl<'a> Analyze for Macro<'a> {
    fn analyze(&self) {
        // eprintln!("analyzed");
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
