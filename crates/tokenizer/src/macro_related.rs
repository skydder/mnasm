// requirement
// - macro def reader => macro data
// - macro data holder
// - macro expander => (macro data + args(stream)) => stream

// macro data
// - name
// - args(name)
// - stream

// peek get messy!!
// macro marker: @<label> ("(" (<stream>"@,")*")")?

use util::{emit_error, Stream, TokenKind, Tokenizer};

use crate::Tokenizer2;

#[derive(Debug, Clone)]
pub struct Macro<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
    pub stream: Stream<'a>,
}

impl<'a> Macro<'a> {
    pub fn new(name: &'a str, stream: Stream<'a>, args: Vec<&'a str>) -> Self {
        Self { name, args, stream }
    }
}

pub fn read_macro_def<'a>(tokenizer: &Tokenizer2<'a>) -> Macro<'a> {
    tokenizer.consume_token(TokenKind::Identifier("macro"));
    tokenizer.skip_space();
    let name = tokenizer.peek_token().get_identifier().unwrap(); // todo
    tokenizer.skip_token();
    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::OpenParenthesis);
    tokenizer.skip_space();
    let mut args = Vec::new();
    if !tokenizer.peek_token().is(TokenKind::CloseParenthesis) {
        read_macro_def_args2(tokenizer, &mut args);
    }
    tokenizer.consume_token(TokenKind::CloseParenthesis);

    tokenizer.skip_space();

    let stream = read_macro_body(tokenizer);
    Macro { name, args, stream }
}

fn read_macro_def_args2<'a>(tokenizer: &Tokenizer2<'a>, args: &mut Vec<&'a str>) {
    if tokenizer.peek_token().is(TokenKind::CloseParenthesis) {
        return;
    }
    let arg = tokenizer
        .next_token()
        .get_identifier()
        .unwrap_or_else(|| emit_error!(tokenizer.location(), "what"));
    args.push(arg);
    tokenizer.skip_space();
    match tokenizer.peek_token().kind {
        TokenKind::Comma => {
            tokenizer.consume_token(TokenKind::Comma);
            tokenizer.skip_space();
            read_macro_def_args2(tokenizer, args);
        }
        TokenKind::CloseParenthesis => (),
        _ => {
            emit_error!(tokenizer.location(), "unexpected token")
        }
    }
}

fn read_macro_body<'a>(tokenizer: &Tokenizer2<'a>) -> Stream<'a> {
    match tokenizer.peek_token().kind {
        TokenKind::OpenBrace => {
            let m_begin = tokenizer.location();
            tokenizer.next_token();
            let mut counter = 1;
            while counter > 0 {
                match tokenizer.peek_token().kind {
                    TokenKind::CloseBrace => {
                        counter -= 1;
                    }
                    TokenKind::OpenBrace => {
                        counter += 1;
                    }
                    _ => (),
                };
                tokenizer.skip_token();
            }
            let m_end = tokenizer.peek_token().location;
            Stream::new(m_begin, m_end)
        }
        _ => {
            let m_begin = tokenizer.location();
            while !tokenizer.peek_token().is(TokenKind::NewLine) {
                tokenizer.skip_token();
            }
            let m_end = tokenizer.peek_token().location;
            tokenizer.skip_token();
            Stream::new(m_begin, m_end)
        }
    }
}

pub fn read_macro_def_label<'a>(tokenizer: &Tokenizer2<'a>) -> Macro<'a> {
    tokenizer.consume_token(TokenKind::Identifier("let"));
    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::OpenParenthesis);
    let name = tokenizer.peek_token().get_identifier().unwrap(); // todo
    tokenizer.skip_token();
    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::Comma);
    tokenizer.skip_space();
    let args = Vec::new();
    let m_begin = tokenizer.location();
    if !tokenizer.peek_token().is(TokenKind::CloseParenthesis) {
        let mut counter = 1;
        while counter > 0 {
            tokenizer.skip_token();
            match tokenizer.peek_token().kind {
                TokenKind::CloseParenthesis => {
                    counter -= 1;
                }
                TokenKind::OpenParenthesis => {
                    counter += 1;
                }
                _ => (),
            };
        }
    }
    let m_end = tokenizer.location();
    tokenizer.consume_token(TokenKind::CloseParenthesis);

    tokenizer.skip_space();

    let stream = Stream::new(m_begin, m_end);
    eprintln!("{}->{:?}", name, stream);
    Macro { name, args, stream }
}

// macro marker: @<label> ("(" (<stream>"@,")*")")?
pub fn read_macro_call<'a>(tokenizer: &Tokenizer2<'a>) -> (&'a str, Vec<Stream<'a>>) {
    let name = tokenizer.peek_token().get_identifier().unwrap(); // todo
    eprintln!("hello, {}", name);
    tokenizer.skip_token();
    tokenizer.skip_space();
    let mut args = Vec::new();
    match tokenizer.peek_token().kind {
        TokenKind::OpenBrace | TokenKind::OpenParenthesis => {
            read_macro_call_args(tokenizer, &mut args)
        }
        _ => (),
    };
    (name, args.clone())
}

fn read_macro_call_args<'a>(tokenizer: &Tokenizer2<'a>, args: &mut Vec<Stream<'a>>) {
    match tokenizer.peek_token().kind {
        TokenKind::OpenParenthesis => {
            read_macro_call_args_p(tokenizer, args);
        }
        TokenKind::OpenBrace => {
            read_macro_call_args_b(tokenizer, args);
        }
        _ => (),
    }
}

fn read_macro_call_args_p<'a>(tokenizer: &Tokenizer2<'a>, args: &mut Vec<Stream<'a>>) {
    tokenizer.skip_token();
    let m_begin = tokenizer.location();
    if !tokenizer.peek_token().is(TokenKind::CloseParenthesis) {
        let mut counter = 1;
        while counter > 0 {
            tokenizer.skip_token();
            match tokenizer.peek_token().kind {
                TokenKind::CloseParenthesis => {
                    counter -= 1;
                }
                TokenKind::OpenParenthesis => {
                    counter += 1;
                }
                _ => (),
            };
        }
    }
    let m_end = tokenizer.peek_token().location;
    tokenizer.skip_token();
    args.push(Stream::new(m_begin, m_end));
    tokenizer.skip_space();
    read_macro_call_args(tokenizer, args);
}

fn read_macro_call_args_b<'a>(tokenizer: &Tokenizer2<'a>, args: &mut Vec<Stream<'a>>) {
    let current_token = tokenizer.peek_token();

    let m_begin = current_token.location;
    tokenizer.next_token();
    let mut counter = 1;
    while counter > 0 {
        match tokenizer.peek_token().kind {
            TokenKind::CloseBrace => {
                counter -= 1;
            }
            TokenKind::OpenBrace => {
                counter += 1;
            }
            _ => (),
        };
        tokenizer.skip_token();
    }
    let m_end = tokenizer.peek_token().location;
    args.push(Stream::new(m_begin, m_end));
    tokenizer.skip_space();
    read_macro_call_args(tokenizer, args);
}

pub fn read_macro_call_dsl<'a>(tokenizer: &Tokenizer2<'a>) -> Stream<'a> {
    tokenizer.consume_token(TokenKind::OpenSquareBracket);
    let m_begin = tokenizer.location();
    if !tokenizer.peek_token().is(TokenKind::CloseSquareBracket) {
        let mut counter = 1;
        while counter > 0 {
            tokenizer.skip_token();
            match tokenizer.peek_token().kind {
                TokenKind::CloseSquareBracket => {
                    counter -= 1;
                }
                TokenKind::OpenSquareBracket => {
                    counter += 1;
                }
                _ => (),
            };
        }
    }
    let m_end = tokenizer.peek_token().location;
    tokenizer.skip_token();
    let new = Stream::new(m_begin, m_end);
    // eprintln!("{:#?}", new);
    new
}

pub fn read_dsl_code<'a>(tokenizer: &Tokenizer2<'a>) -> Stream<'a> {
    tokenizer.consume_token(TokenKind::OpenParenthesis);
    let m_begin = tokenizer.location();
    if !tokenizer.peek_token().is(TokenKind::CloseSquareBracket) {
        let mut counter = 1;
        while counter > 0 {
            tokenizer.skip_token();
            match tokenizer.peek_token().kind {
                TokenKind::CloseParenthesis => {
                    counter -= 1;
                }
                TokenKind::OpenParenthesis => {
                    counter += 1;
                }
                _ => (),
            };
        }
    }
    let m_end = tokenizer.peek_token().location;
    tokenizer.skip_token();
    let new = Stream::new(m_begin, m_end);
    // eprintln!("{:#?}", new);
    new
}
