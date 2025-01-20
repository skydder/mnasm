use std::cell::RefCell;

use tokenizer::{Token, Tokenizer, TokenGenerator};
use data::MacroTokenizer2;
use util::Location;

#[derive(Debug, Clone, Copy)]
enum TokenizerKind<'a> {
    Tokenizer(Tokenizer<'a>),
    MacroTokenizer(MacroTokenizer2<'a>),
}

#[derive(Debug, Clone, Copy)]
pub struct Tokenizer2<'a> {
    tokenizer: TokenizerKind<'a>,
}

impl<'a> Tokenizer2<'a> {
    fn enter_macro(&'a mut self, stream: (Location<'a>, Location<'a>)) {
        let tokenizer = if let TokenizerKind::Tokenizer(t)= self.tokenizer {
            t
        } else {
            todo!()
        };
        let ret = tokenizer.swap(stream.0);
        self.tokenizer = TokenizerKind::MacroTokenizer(MacroTokenizer2::new((tokenizer, stream.1), ret));
    }

    fn leave_macro(&'a mut self) {
        let tokenizer = if let TokenizerKind::MacroTokenizer(t)= self.tokenizer {
            t
        } else {
            todo!()
        };
        let ret = tokenizer.tokenizer.swap(tokenizer.ret);
        self.tokenizer = TokenizerKind::Tokenizer(tokenizer.tokenizer);
    }
}

impl<'a> Tokenizer2<'a> {
    pub fn location(&self) -> util::Location {
        self.tokenizer.location()
    }

    pub fn peek_token(&self) -> Token {
        self.tokenizer.peek_token()
    }

    pub fn next_token(&self) -> Token {
        self.tokenizer.peek_token()
    }

    pub fn skip_space(&self) {
        self.tokenizer.skip_space()
    }

    pub fn consume_token(&self, consumeing_token: tokenizer::TokenKind) {
        self.tokenizer.consume_token(consumeing_token)
    }

    pub fn consume_newline(&self) {
        self.tokenizer.consume_newline()
    }

    pub fn consume_indent(&self) {
        self.tokenizer.consume_indent()
    }

    pub fn kind(&self) -> tokenizer::GenKind {
        self.tokenizer.kind()
    }
}

impl<'a> TokenGenerator for TokenizerKind<'a> {
    fn location(&self) -> util::Location {
        match self {
            TokenizerKind::MacroTokenizer(tok) => tok.location(),
            TokenizerKind::Tokenizer(tok ) => tok.location(),
        }
    }

    fn peek_token(&self) -> Token {
        match self {
            TokenizerKind::MacroTokenizer(tok) => tok.peek_token(),
            TokenizerKind::Tokenizer(tok ) => tok.peek_token(),
        }
    }

    fn next_token(&self) -> Token {
        match self {
            TokenizerKind::MacroTokenizer(tok) => tok.next_token(),
            TokenizerKind::Tokenizer(tok ) => tok.next_token(),
        }
    }

    fn skip_space(&self) {
        match self {
            TokenizerKind::MacroTokenizer(tok) => tok.skip_space(),
            TokenizerKind::Tokenizer(tok ) => tok.skip_space(),
        }
    }

    fn consume_token(&self, consumeing_token: tokenizer::TokenKind) {
        match self {
            TokenizerKind::MacroTokenizer(tok) => tok.consume_token(consumeing_token),
            TokenizerKind::Tokenizer(tok ) => tok.consume_token(consumeing_token),
        }
    }

    fn consume_newline(&self) {
        match self {
            TokenizerKind::MacroTokenizer(tok) => tok.consume_newline(),
            TokenizerKind::Tokenizer(tok ) => tok.consume_newline(),
        }
    }

    fn consume_indent(&self) {
        match self {
            TokenizerKind::MacroTokenizer(tok) => tok.consume_indent(),
            TokenizerKind::Tokenizer(tok ) => tok.consume_indent(),
        }
    }

    fn kind(&self) -> tokenizer::GenKind {
        match self {
            TokenizerKind::MacroTokenizer(tok) => tok.kind(),
            TokenizerKind::Tokenizer(tok ) => tok.kind(),
        }
    }
}