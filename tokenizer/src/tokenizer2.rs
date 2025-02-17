use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, VecDeque},
    fmt::Debug, rc::Rc,
};

use crate::{init_infix_macro, read_macro_call, read_macro_def, Macro, Stream, Token, TokenKind};
use util::{emit_error, Location};

// macro should be expanded inside to outside

#[derive(Debug, Clone)]
struct InnerTokenizer<'a> {
    location: RefCell<Location<'a>>,
    eos: RefCell<Location<'a>>,
}

impl<'a> InnerTokenizer<'a> {
    fn new(location: Location<'a>, eos: Location<'a>) -> Self {
        Self {
            location: RefCell::new(location),
            eos: RefCell::new(eos),
        }
    }

    fn location(&self) -> Location<'a> {
        *self.location.borrow()
    }

    fn peek_token(&self) -> Token<'a> {
        if self.location >= self.eos {
            return Token::new(TokenKind::EOS, 0, *self.eos.borrow());
        }
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

    pub fn swap(&self, location: Location<'a>, eos: Location<'a>) -> Location<'a> {
        self.eos.replace(eos);
        self.location.replace(location)
    }
}

#[derive(Debug, Clone)]
pub struct TokenizerStatus<'a> {
    stream: Stream<'a>,
    // macro_args: Rc<Vec<(&'a str, Stream<'a>)>>,
    args_data: Rc<HashMap<&'a str, Macro<'a>>>,
    prev_args_data: Rc<HashMap<&'a str, Macro<'a>>>,
    is_macro_record: i8
}

impl<'a> TokenizerStatus<'a> {
    fn new(
        current_location: Location<'a>,
        eos: Location<'a>,
        macro_args: Rc<HashMap<&'a str, Macro<'a>>>,
        prev_args_data: Rc<HashMap<&'a str, Macro<'a>>>,
        is_macro_record: i8
    ) -> Self {
        // let mut args_data = HashMap::new();
        // for (name, stream) in macro_args.iter() {
        //     args_data.insert(*name, Macro::new(name, *stream, Vec::new()));
        // }
        Self {
            // macro_args: macro_args,
            stream: Stream::new(current_location, eos),
            args_data: macro_args,
            prev_args_data: prev_args_data,
            is_macro_record: is_macro_record
        }
    }

    fn update(self, begin: Location<'a>) -> Self {
        Self::new(begin, self.stream.end(), self.args_data, self.prev_args_data, self.is_macro_record)
    }

    fn end(&self) -> Location<'a> {
        self.stream.end()
    }
}

#[derive(Clone)]
pub struct Tokenizer2<'a> {
    tokenizer: RefCell<InnerTokenizer<'a>>,

    current_status: RefCell<TokenizerStatus<'a>>,

    status_stack: RefCell<Vec<TokenizerStatus<'a>>>,

    macro_data: RefCell<HashMap<&'a str, Macro<'a>>>,
    // args_data: RefCell<HashMap<&'a str, Macro<'a>>>,
    code: RefCell<Vec<TokenKind<'a>>>, // code itself doesn't need location

    macro_stack: RefCell<Vec<Rc<HashMap<&'a str, Macro<'a>>>>>,
    macro_stack2: RefCell<VecDeque<Rc<HashMap<&'a str, Macro<'a>>>>>,
    macro_depth: Cell<i64>,
    macro_depth2: Cell<i64>,
    current_arg: RefCell<Rc<Vec<(&'a str, Stream<'a>)>>>,
    record: Cell<bool>,
}

impl<'a> Tokenizer2<'a> {
    pub fn new_tokenizer(location: Location<'a>) -> Self {
        let mut macro_data = HashMap::new();
        macro_data.insert("infix", init_infix_macro());

        Self {
            tokenizer: RefCell::new(InnerTokenizer::new(location, location.end())),
            status_stack: RefCell::new(Vec::new()),
            code: RefCell::new(Vec::new()),
            macro_data: RefCell::new(macro_data),
            current_status: RefCell::new(TokenizerStatus::new(
                location,
                location.end(),
                Rc::new(HashMap::new()),
            Rc::new(HashMap::new()),
                -1
            )),
            macro_depth: Cell::new(0),
            record: Cell::new(true),

            macro_stack: RefCell::new(vec![Rc::new(HashMap::new())]),
            macro_depth2: Cell::new(0),
            current_arg: RefCell::new(Rc::new(Vec::new())),
            macro_stack2: RefCell::new(VecDeque::new()),
        }
    }

    pub fn enter_macro(&self, stream: Stream<'a>, args: Rc<HashMap<&'a str, Macro<'a>>>, is_macro_record: i8) {
        let p = self.current_status.borrow();
        let prev = p.args_data.clone();
        drop(p); 
        let status =
            self.current_status
                .replace(TokenizerStatus::new(stream.begin(), stream.end(), args, prev.clone(), is_macro_record));
        self.status_stack
            .borrow_mut()
            .push(status.clone().update(self.location())); // adhoc
        self.tokenizer
            .borrow_mut()
            .swap(stream.begin(), stream.end());
        self.macro_depth.replace(self.macro_depth.get() + 1);
        if self.current_status.borrow().is_macro_record > 0 {
            self.macro_stack.borrow_mut().insert(self.macro_depth2.get() as usize, prev);
            self.macro_depth2.replace(self.macro_depth2.get() + 1);
        } else if self.current_status.borrow().is_macro_record < 0 {
            self.macro_depth2.replace(self.macro_depth2.get() - 1);
        }
    }

    pub fn leave_macro(&self) {
        assert!(self.macro_depth.get() > 0);
        if self.current_status.borrow().is_macro_record > 0 {
            self.macro_stack.borrow_mut().remove(self.macro_depth2.get() as usize);
            self.macro_depth2.replace(self.macro_depth2.get() - 1);
        } else if self.current_status.borrow().is_macro_record < 0 {
            self.macro_depth2.replace(self.macro_depth2.get() + 1);
        }
        let status = self.status_stack.borrow_mut().pop().unwrap();
        self.tokenizer
            .borrow_mut()
            .swap(status.stream.begin(), status.stream.end());
        let _ = self.current_status.replace(status);
        self.macro_depth.replace(self.macro_depth.get() - 1);
    }

    pub fn code(&self) -> String {
        self.code
            .borrow()
            .iter()
            .map(|c| format!("{}", c))
            .collect()
    }

    fn is_eos(&self) -> bool {
        self.tokenizer.borrow().peek_token().location >= self.current_status.borrow().end()
    }

    pub fn turn_on_the_record(&self) {
        self.record.set(true);
    }
    pub fn turn_off_the_record(&self) {
        self.record.set(false);
    }
}

impl<'a> Tokenizer2<'a> {
    pub fn location(&self) -> util::Location<'a> {
        self.tokenizer.borrow().location()
    }

    pub fn peek_token(&self, macro_expand: bool) -> Token<'a> {
        let current = self.tokenizer.borrow().peek_token();
        if !macro_expand {
            return current;
        }
        if current.is(TokenKind::EOS) {
            while self.is_eos() && self.macro_depth.get() > 0 {
                self.leave_macro();
            }
            return self.tokenizer.borrow().peek_token();
        }
        match current.kind {
            TokenKind::BackQuote => {
                self.skip_token();
                let name = self
                    .peek_token(true)
                    .get_identifier()
                    .unwrap_or_else(|| emit_error!(self.location(), "expected identifier"));
                self.skip_token();
                let cs = self.current_status.borrow();
                let args_data = &cs.args_data;
                // eprintln!("we have :{:?}", args_data);
                let macro_data = args_data
                    .get(name)
                    .unwrap_or_else(|| emit_error!(self.location(), "undefined argment:{}", name))
                    .stream;
                drop(cs);
                let stack = self.macro_stack.borrow();
                let args = stack.get((self.macro_depth2.get() - 1) as usize).unwrap().clone();
                eprintln!("we have :{:?} [{}]", stack, self.macro_depth2.get());
                drop(stack);
                self.enter_macro(macro_data, args.clone(), -1);

                self.peek_token(true)
            }
            TokenKind::At => {
                self.turn_off_the_record();
                let data = read_macro_call(self);
                self.turn_on_the_record();
                let macro_data = self.macro_data.borrow();
                let macro_data = macro_data
                    .get(data.0)
                    .unwrap_or_else(|| emit_error!(self.location(), "undefined macro"));
                let args: Rc<Vec<(&'a str, Stream<'a>)>>=
                    Rc::new(macro_data.args.iter().map(|a| *a).zip(data.1).collect());
                let mut args_data = HashMap::new();
                for (name, stream) in args.iter() {
                    args_data.insert(*name, Macro::new(name, *stream, Vec::new()));
                }
                self.enter_macro(macro_data.stream, Rc::new(args_data), 1);
                self.peek_token(true)
            }
            TokenKind::Identifier("macro") => {
                self.turn_off_the_record();
                let m = read_macro_def(self);
                self.turn_on_the_record();
                self.macro_data.borrow_mut().insert(m.name, m);
                self.peek_token(true)
            }
            _ => current,
        }
    }

    pub fn next_token(&self) -> Token<'a> {
        let current = self.peek_token(true);
        if current.kind != TokenKind::EOS {
            self.tokenizer.borrow().next_token();
        }
        if self.record.get() {
            self.code.borrow_mut().push(current.kind);
        }

        current
    }

    pub fn skip_space(&self, macro_expand: bool) {
        while self.peek_token(macro_expand).is(TokenKind::Space) {
            self.skip_token();
        }
    }

    pub fn skip_token(&self) {
        let _ = self.tokenizer.borrow().next_token();
    }

    pub fn consume_token(&self, consumeing_token: TokenKind<'a>) {
        if self.record.get() {
            self.code.borrow_mut().push(consumeing_token);
        }
        self.tokenizer.borrow().consume_token(consumeing_token)
    }

    pub fn consume_newline(&self) {
        let current_token = self.peek_token(true);
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

    pub fn consume_indent(&self) {
        self.code.borrow_mut().push(TokenKind::Space);
        self.code.borrow_mut().push(TokenKind::Space);
        self.code.borrow_mut().push(TokenKind::Space);
        self.code.borrow_mut().push(TokenKind::Space);
        for _ in 0..4 {
            match self.peek_token(true).kind {
                TokenKind::Space => {
                    self.skip_token();
                }
                TokenKind::NewLine | TokenKind::EOS => (),
                _ => (),
            }
        }
    }

    pub fn add_to_code(&self, tokenkind: TokenKind<'a>) {
        self.code.borrow_mut().push(tokenkind);
    }
}

impl<'a> std::fmt::Debug for Tokenizer2<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tokenizer2")
            .field("tokenizer", &self.tokenizer)
            .field("current_status", &self.current_status)
            .finish()
    }
}
