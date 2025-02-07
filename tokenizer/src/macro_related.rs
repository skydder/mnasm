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


use util::emit_error;

use crate::{Stream, TokenKind, Tokenizer2};

#[derive(Debug, Clone)]
pub struct Macro<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
    pub stream: Stream<'a>,
}

pub fn read_macro_def<'a>(tokenizer: &Tokenizer2<'a>) -> Macro<'a> {
    tokenizer.skip_token();
    tokenizer.skip_space_silently();
    let name = tokenizer.peek_token().get_identifier().unwrap(); // todo
    tokenizer.skip_token();
    tokenizer.skip_space_silently();
    tokenizer.consume_token_silently(TokenKind::OpenParenthesis);
    tokenizer.skip_space_silently();
    let mut args = Vec::new();
    read_macro_def_args(tokenizer, &mut args);
    tokenizer.consume_token_silently(TokenKind::CloseParenthesis);
    tokenizer.skip_space_silently();

    let stream = read_macro_body(tokenizer);
    Macro {
        name: name,
        args: args,
        stream: stream,
    }
}

fn read_macro_def_args<'a>(tokenizer: &Tokenizer2<'a>, args: &mut Vec<&'a str>) {
    if tokenizer.peek_token().is(TokenKind::CloseParenthesis) {
        return;
    }
    let arg = tokenizer.next_token_silently().get_identifier().unwrap();
    tokenizer.skip_space_silently();
    tokenizer.consume_token_silently(TokenKind::Comma);
    args.push(arg);
    tokenizer.skip_space_silently();
    read_macro_def_args(tokenizer, args);
}

fn read_macro_body<'a>(tokenizer: &Tokenizer2<'a>) -> Stream<'a> {
    match tokenizer.peek_token_sme().kind {
        TokenKind::OpenBrace => {
            let m_begin = tokenizer.location();
            tokenizer.next_token_silently();
            let mut counter: Vec<&str> = vec!["{"];
            while !counter.is_empty() {
                match tokenizer.peek_token_sme().kind {
                    TokenKind::CloseBrace => {
                        counter.pop();
                    },
                    TokenKind::OpenBrace => {
                        counter.push("{");
                    },
                    _ => ()
                };
                tokenizer.skip_token();
            }
            let m_end = tokenizer.peek_token().location;
            Stream::new(m_begin, m_end)
        }
        _ => {
            let m_begin = tokenizer.location();
            while !tokenizer.peek_token_sme().is(TokenKind::NewLine) {
                tokenizer.skip_token();
            }
            let m_end = tokenizer.peek_token().location;
            tokenizer.skip_token();
            Stream::new(m_begin, m_end)
        }
    }
}

// macro marker: @<label> ("(" (<stream>"@,")*")")?
pub fn read_macro_call<'a>(tokenizer: &Tokenizer2<'a>) -> (&'a str, Vec<Stream<'a>>) {
    tokenizer.skip_token();
    let name = tokenizer.peek_token().get_identifier().unwrap(); // todo
    tokenizer.skip_token();
    tokenizer.skip_space_silently();
    let mut args = Vec::new();
    match tokenizer.peek_token_sme().kind {
        TokenKind::OpenBrace | TokenKind::OpenParenthesis => read_macro_call_args(tokenizer, &mut args),
        _ => emit_error!(tokenizer.location(), "unexpected token"),
    };
    let new = (name, args.clone());
    // eprintln!("{:#?}", new);
    new
}

fn read_macro_call_args<'a>(tokenizer: &Tokenizer2<'a>, args: &mut Vec<Stream<'a>>) {
    match tokenizer.peek_token_sme().kind {
        TokenKind::OpenParenthesis => {
            read_macro_call_args_p(tokenizer, args);
        }
        TokenKind::OpenBrace => {
            read_macro_call_args_b(tokenizer, args);
        }
        _ => {
            return;
        }
    }
}

fn read_macro_call_args_p<'a>(tokenizer: &Tokenizer2<'a>, args: &mut Vec<Stream<'a>>) {
    tokenizer.skip_token();
    let m_begin = tokenizer.location();
    if !tokenizer.peek_token_sme().is(TokenKind::CloseParenthesis) {
        let mut counter: Vec<&str> = vec!["("];
        while !counter.is_empty() {
            tokenizer.skip_token();
            match tokenizer.peek_token_sme().kind {
                TokenKind::CloseParenthesis => {
                    counter.pop();
                },
                TokenKind::OpenParenthesis => {
                    counter.push("(");
                },
                _ => ()
            };
        }
    }
    let m_end = tokenizer.peek_token().location;
    tokenizer.skip_token();
    args.push(Stream::new(m_begin, m_end));
    tokenizer.skip_space_silently();
    read_macro_call_args(tokenizer, args);
}

fn read_macro_call_args_b<'a>(tokenizer: &Tokenizer2<'a>, args: &mut Vec<Stream<'a>>) {
    let current_token = tokenizer.peek_token();
   
    let m_begin = current_token.location;
    tokenizer.next_token_silently();
    let mut counter: Vec<&str> = vec!["{"];
    while !counter.is_empty() {
        match tokenizer.peek_token_sme().kind {
            TokenKind::CloseBrace => {
                counter.pop();
            },
            TokenKind::OpenBrace => {
                counter.push("{");
            },
            _ => ()
        };
        tokenizer.skip_token();
    }
    let m_end = tokenizer.peek_token().location;
    args.push(Stream::new(m_begin, m_end));
    tokenizer.skip_space_silently();
    read_macro_call_args(tokenizer, args);
}