use std::cell::{Cell, RefCell};

use util::{emit_error, Location};

use crate::{Token, TokenKind};

#[derive(Debug, Clone)]
struct InnerTokenizer<'a> {
    location: RefCell<Location<'a>>,
}

impl<'a> InnerTokenizer<'a> {
    fn new(location: Location<'a>) -> Self {
        Self { location: RefCell::new(location) }
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
            TokenKind::EOS => (),
            _ => {
                emit_error!(current_token.location, "expected new line")
            }
        }
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

    pub fn swap(&self, location: Location<'a>) -> Location<'a> {
        self.location.replace(location)
    }
}

#[derive(Debug, Clone)]
pub struct Tokenizer2<'a> {
    tokenizer: RefCell<InnerTokenizer<'a>>,
    eos: Cell<Location<'a>>,
    stack: RefCell<Vec<Location<'a>>>, // where the location to return after macro-expansion is saved 
    macro_args: RefCell<Vec<&'a str>>, // ad hoc Ident
    code: RefCell<Vec<TokenKind<'a>>>, // code itself doesn't need location
    args_stack: RefCell<Vec<Vec<&'a str>>>,
}

impl<'a> Tokenizer2<'a> {
    pub fn new_tokenizer(location: Location<'a>) -> Self {
        Self {
            tokenizer: RefCell::new(InnerTokenizer::new(location)),
            eos: Cell::new(location.end()),
            code: RefCell::new(Vec::new()),
            stack: RefCell::new(Vec::new()),
            macro_args: RefCell::new(Vec::new()),
            args_stack: RefCell::new(Vec::new())
        }
    }

    pub fn enter_macro(&self, stream: (Location<'a>, Location<'a>), args: Vec<&'a str>) {
        // todo
        self.stack.borrow_mut().push(self.eos.get());
        let ret = self.location();
        self.stack.borrow_mut().push(ret);
        self.args_stack.borrow_mut().push(self.macro_args.replace(args));
        self.eos.set(stream.1);
        self.tokenizer.borrow_mut().swap(stream.0);
        self.code.borrow_mut().push(TokenKind::OpenParenthesis);
    }

    pub fn leave_macro(&self) {
        // todo
        let ret = self.stack.borrow_mut().pop().unwrap();   // todo
        let eos = self.stack.borrow_mut().pop().unwrap();
        self.macro_args.replace(self.args_stack.borrow_mut().pop().unwrap());
        self.eos.set(eos);
        self.tokenizer.borrow_mut().swap(ret);
        self.code.borrow_mut().push(TokenKind::CloseParenthesis);
    }

    pub fn code(&self) -> String {
        self.code
            .borrow()
            .iter()
            .map(|c| format!("{}", c))
            .collect()
    }
}

impl<'a> Tokenizer2<'a> {
    pub fn location(&self) -> util::Location<'a> {
        self.tokenizer.borrow().location()
    }

    pub fn peek_token(&self) -> Token<'a> {
        let current = self.tokenizer.borrow().peek_token();
        if current.location >= self.eos.get() {
            return Token::new(TokenKind::EOS, 0, self.eos.get());
        } else {
            current
        }
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
        self.code.borrow_mut().push(TokenKind::Space);
        self.tokenizer.borrow().skip_space()
    }

    pub fn consume_token(&self, consumeing_token: TokenKind<'a>) {
        self.code.borrow_mut().push(consumeing_token);
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
}
