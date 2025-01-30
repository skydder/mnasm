use std::{cell::{Cell, RefCell}, collections::HashMap};

use util::{emit_error, Location};
use crate::{read_macro_call, read_macro_def, Macro, Stream, Token, TokenKind};

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
        self.location.borrow().clone()
    }

    fn current_slice(&self) -> &'a str {
        self.location.borrow().current_slice()
    }

    fn peek_token(&self) -> Token<'a> {
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

    fn consume_newline(&self) {
        let current_token = self.peek_token();
        match current_token.kind {
            TokenKind::NewLine => self.advance_location_by_token(&current_token),
            TokenKind::Semicolon => self.advance_location_by_token(&current_token),
            TokenKind::EOS => (),
            _ => {
                emit_error!(current_token.location, "expected new line")
            }
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

#[derive(Debug,Clone)]
// use stream
struct TokenizerStatus<'a> {
    current_location: Location<'a>,
    eos: Location<'a>,
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
            current_location: current_location,
            eos: eos,
            is_auto_leave: is_auto_leave,
            macro_args: macro_args,
        }
    }
}
// delete auto leave, since macro auto leaves.
#[derive(Debug, Clone)]
pub struct Tokenizer2<'a> {
    tokenizer: RefCell<InnerTokenizer<'a>>,
    eos: Cell<Location<'a>>,
    is_auto_leave: Cell<bool>,
    macro_args: RefCell<Vec<(&'a str, Stream<'a>)>>,

    status_stack: RefCell<Vec<TokenizerStatus<'a>>>, 

    macro_data: RefCell<HashMap<&'a str, Macro<'a>>>,

    code: RefCell<Vec<TokenKind<'a>>>, // code itself doesn't need location
}

impl<'a> Tokenizer2<'a> {
    pub fn new_tokenizer(location: Location<'a>) -> Self {
        Self {
            tokenizer: RefCell::new(InnerTokenizer::new(location)),
            eos: Cell::new(location.end()),
            macro_args: RefCell::new(Vec::new()),
            is_auto_leave: Cell::new(false),
            status_stack: RefCell::new(Vec::new()),
            code: RefCell::new(Vec::new()),
            macro_data: RefCell::new(HashMap::new()),
        }
    }

    pub fn enter_macro(
        &self,
        stream: Stream<'a>,
        args: Vec<(&'a str, Stream<'a>)>,
    ) {
        let current_location = self.location();
        let eos = self.eos.get();
        let is_auto_leave = self.is_auto_leave.get();
        let macro_args = self.macro_args.replace(args);
        let status = TokenizerStatus::new(current_location, eos, is_auto_leave, macro_args);
        
        self.status_stack.borrow_mut().push(status);

        self.eos.set(stream.end);
        self.tokenizer.borrow_mut().swap(stream.begin);
    }

    pub fn leave_macro(&self) {
        let status = self.status_stack.borrow_mut().pop().unwrap();

        self.macro_args.replace(status.macro_args);
        self.is_auto_leave.replace(status.is_auto_leave);
        self.eos.set(status.eos);
        self.tokenizer.borrow_mut().swap(status.current_location);
    }

    pub fn code(&self) -> String {
        self.code
            .borrow()
            .iter()
            .map(|c| format!("{}", c))
            .collect()
    }

    fn match_arg(&self, arg: &'a str) -> Option<Stream<'a>> {
        for a in self.macro_args.borrow().iter() {
            if arg == a.0 {
                return Some(a.1);
            }
        }
        None
    }

    fn set_auto_leave(&self) {
        self.is_auto_leave.set(true);
    }
}

impl<'a> Tokenizer2<'a> {
    pub fn location(&self) -> util::Location<'a> {
        self.tokenizer.borrow().location()
    }

    pub fn peek_token(&self) -> Token<'a> {
        let current = self.tokenizer.borrow().peek_token();
        if current.location >= self.eos.get() {
            if self.is_auto_leave.get() {
                self.leave_macro();
                return self.tokenizer.borrow().peek_token();
            }
            return Token::new(TokenKind::EOS, 0, self.eos.get());
        }
        match current.kind {
            TokenKind::BackQuote => {
                self.next_token_with_out_record();
                let name = self.next_token_with_out_record().get_identifier().unwrap();
                self.enter_macro(self.match_arg(name).unwrap(), Vec::new());
                self.peek_token()
            },
            TokenKind::At => {
                let data = read_macro_call(self);
                let m = self.macro_data.borrow();
                let m = m.get(data.0).unwrap(); //todo
                let args: Vec<(&'a str, Stream<'a>)> = m.args.iter().map(|a| *a).zip(data.1).collect();
                self.enter_macro(m.stream, args);
                self.peek_token()
            },
            TokenKind::Identifier("macro") => {
                let m = read_macro_def(self);
                self.macro_data.borrow_mut().insert(m.name, m);
                self.peek_token()
            },
            _ => current
        }
    }

    pub fn next_token_with_out_record(&self) -> Token<'a> {
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
        eprintln!("{:#?}", current);
        current
    }

    pub fn skip_space(&self) {
        self.code.borrow_mut().push(TokenKind::Space);
        self.tokenizer.borrow().skip_space()
    }

    pub fn skip_space_silently(&self) {
        self.code.borrow_mut().push(TokenKind::Space);
        self.tokenizer.borrow().skip_space()
    }

    pub fn skip_token(&self) {
        self.tokenizer.borrow().next_token();
    }

    pub fn peek_token_silently(&self) -> Token<'a> {
        let current = self.tokenizer.borrow().peek_token();
        if current.location >= self.eos.get() {
            return Token::new(TokenKind::EOS, 0, self.eos.get());
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
        self.code.borrow_mut().push(TokenKind::NewLine);
        self.tokenizer.borrow().consume_newline()
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
