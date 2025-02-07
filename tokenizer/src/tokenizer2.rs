use std::{cell::RefCell, collections::HashMap, fmt::Debug};

use crate::{read_macro_call, read_macro_def, Macro, Stream, Token, TokenKind};
use util::{emit_error, Location};

#[derive(Debug, Clone)]
struct InnerTokenizer<'a> {
    location: RefCell<Location<'a>>,
}

impl<'a> InnerTokenizer<'a> {
    fn new(location: Location<'a>) -> Self {
        Self {
            location: RefCell::new(location),
        }
    }

    fn location(&self) -> Location<'a> {
        *self.location.borrow()
    }

    fn peek_token(&self) -> Token<'a> {
        let tok = Token::tokenize(*self.location.borrow());
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

    fn next_token(&self) -> Token<'a> {
        let token = self.peek_token();
        self.advance_location_by_token(&token);
        token
    }
    fn skip_space(&self) {
        while self.peek_token().is(TokenKind::Space) {
            self.next_token();
        }
    }

    fn consume_token(&self, expecting_token: TokenKind) {
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

    fn consume_indent(&self) {
        // let loc = self.location();
        for _ in 0..4 {
            match self.peek_token().kind {
                TokenKind::Space => {
                    self.next_token();
                }
                TokenKind::NewLine | TokenKind::EOS => (),
                _ => {
                    // emit_error!(loc, "Indent error, the number of spase must be 4");
                }
            }
        }
    }

    pub fn swap(&self, location: Location<'a>) -> Location<'a> {
        self.location.replace(location)
    }
}

#[derive(Debug, Clone)]
struct TokenizerStatus<'a> {
    stream: Stream<'a>,
    is_auto_leave: bool,
    macro_args: Vec<(&'a str, Stream<'a>)>,
}

impl<'a> TokenizerStatus<'a> {
    fn new(
        current_location: Location<'a>,
        eos: Location<'a>,
        is_auto_leave: bool,
        macro_args: Vec<(&'a str, Stream<'a>)>,
    ) -> Self {
        Self {
            is_auto_leave: is_auto_leave,
            macro_args: macro_args,
            stream: Stream::new(current_location, eos),
        }
    }
    fn update(self, begin: Location<'a>) -> Self {
        Self::new(
            begin,
            self.stream.end(),
            self.is_auto_leave,
            self.macro_args,
        )
    }
}
// delete auto leave, since macro auto leaves.
#[derive(Clone)]
pub struct Tokenizer2<'a> {
    tokenizer: RefCell<InnerTokenizer<'a>>,

    current_status: RefCell<TokenizerStatus<'a>>,

    status_stack: RefCell<Vec<TokenizerStatus<'a>>>,

    macro_data: RefCell<HashMap<&'a str, Macro<'a>>>,

    code: RefCell<Vec<TokenKind<'a>>>, // code itself doesn't need location
}

impl<'a> Tokenizer2<'a> {
    pub fn new_tokenizer(location: Location<'a>) -> Self {
        Self {
            tokenizer: RefCell::new(InnerTokenizer::new(location)),
            status_stack: RefCell::new(Vec::new()),
            code: RefCell::new(Vec::new()),
            macro_data: RefCell::new(HashMap::new()),
            current_status: RefCell::new(TokenizerStatus::new(
                location,
                location.end(),
                false,
                Vec::new(),
            )),
        }
    }

    pub fn enter_macro(
        &self,
        stream: Stream<'a>,
        args: Vec<(&'a str, Stream<'a>)>,
        is_auto_leave: bool,
    ) {
        let status = self.current_status.replace(TokenizerStatus::new(
            stream.begin(),
            stream.end(),
            is_auto_leave,
            args,
        ));
        self.status_stack
            .borrow_mut()
            .push(status.update(self.location()));
        self.tokenizer.borrow_mut().swap(stream.begin());
    }

    pub fn leave_macro(&self) {
        let status = self.status_stack.borrow_mut().pop().unwrap();
        self.tokenizer.borrow_mut().swap(status.stream.begin());
        let _ = self.current_status.replace(status);
    }

    pub fn code(&self) -> String {
        self.code
            .borrow()
            .iter()
            .map(|c| format!("{}", c))
            .collect()
    }

    fn match_arg(&self, arg: &'a str) -> Option<Stream<'a>> {
        for a in self.current_status.borrow().macro_args.iter() {
            if arg == a.0 {
                return Some(a.1);
            }
        }
        None
    }

    fn end(&self) -> Location<'a> {
        self.current_status.borrow().stream.end()
    }

    fn is_auto_leave(&self) -> bool {
        self.current_status.borrow().is_auto_leave
    }
}

impl<'a> Tokenizer2<'a> {
    pub fn location(&self) -> util::Location<'a> {
        self.tokenizer.borrow().location()
    }

    pub fn peek_token(&self) -> Token<'a> {
        let current = self.tokenizer.borrow().peek_token();
        if current.location >= self.end() {
            if self.is_auto_leave() {
                self.leave_macro();
                return self.peek_token_sme();
            }
            return Token::new(TokenKind::EOS, 0, self.end());
        }
        match current.kind {
            TokenKind::BackQuote => {
                self.skip_token();
                let name = self.peek_token().get_identifier().unwrap();
                self.skip_token();
                self.enter_macro(self.match_arg(name).unwrap(), Vec::new(), true);
                self.peek_token()
            }
            TokenKind::At => {
                let data = read_macro_call(self);
                let macro_data = self.macro_data.borrow();
                let macro_data = macro_data.get(data.0).unwrap(); //todo
                let args: Vec<(&'a str, Stream<'a>)> =
                    macro_data.args.iter().map(|a| *a).zip(data.1).collect();
                self.enter_macro(macro_data.stream, args, true);
                self.peek_token()
            }
            TokenKind::Identifier("macro") => {
                let m = read_macro_def(self);
                self.macro_data.borrow_mut().insert(m.name, m);
                self.peek_token()
            }
            _ => current,
        }
    }

    pub fn next_token_silently(&self) -> Token<'a> {
        let current = self.peek_token();
        if current.kind != TokenKind::EOS {
            self.tokenizer.borrow().next_token();
        }
        current
    }

    pub fn next_token(&self) -> Token<'a> {
        let current = self.peek_token();
        if current.kind != TokenKind::EOS {
            self.tokenizer.borrow().next_token();
        }
        self.code.borrow_mut().push(current.kind);
        current
    }

    pub fn skip_space(&self) {
        // self.code.borrow_mut().push(TokenKind::Space);
        self.tokenizer.borrow().skip_space()
    }

    pub fn skip_space_silently(&self) {
        // self.code.borrow_mut().push(TokenKind::Space);
        self.tokenizer.borrow().skip_space()
    }

    pub fn skip_token(&self) {
        let _ = self.tokenizer.borrow().next_token();
    }

    pub fn peek_token_sme(&self) -> Token<'a> {
        let current = self.tokenizer.borrow().peek_token();
        if current.location >= self.end() {
            return Token::new(TokenKind::EOS, 0, self.end());
        } else {
            current
        }
    }

    pub fn consume_token(&self, consumeing_token: TokenKind<'a>) {
        self.code.borrow_mut().push(consumeing_token);
        self.tokenizer.borrow().consume_token(consumeing_token)
    }

    pub fn consume_token_silently(&self, consumeing_token: TokenKind<'a>) {
        self.tokenizer.borrow().consume_token(consumeing_token)
    }

    pub fn consume_newline(&self) {
        let current_token = self.peek_token();
        match current_token.kind {
            TokenKind::NewLine => {
                self.next_token_silently();
            }
            TokenKind::Semicolon => {
                self.next_token_silently();
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
        // self.code.borrow_mut().push(TokenKind::NewLine);
        // self.tokenizer.borrow().consume_newline()
    }

    pub fn consume_indent(&self) {
        self.code.borrow_mut().push(TokenKind::Space);
        self.code.borrow_mut().push(TokenKind::Space);
        self.code.borrow_mut().push(TokenKind::Space);
        self.code.borrow_mut().push(TokenKind::Space);
        self.tokenizer.borrow().consume_indent()
    }

    pub fn add_to_code(&self, tokenkind: TokenKind<'a>) {
        self.code.borrow_mut().push(tokenkind);
    }
}

impl<'a> std::fmt::Debug for Tokenizer2<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tokenizer2").field("tokenizer", &self.tokenizer).field("current_status", &self.current_status).finish()
    }
}