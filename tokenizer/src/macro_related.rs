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

    let m_begin = tokenizer.peek_token().location;
    let mut m_end = tokenizer.peek_token_sme().location;
    while !tokenizer.peek_token_sme().is(TokenKind::MacroEnd) {
        tokenizer.skip_token();
        m_end = tokenizer.peek_token_sme().location;
    }

    tokenizer.consume_token_silently(TokenKind::MacroEnd);

    Macro {
        name: name,
        args: args,
        stream: Stream::new(m_begin, m_end),
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

// macro marker: @<label> ("(" (<stream>"@,")*")")?
pub fn read_macro_call<'a>(tokenizer: &Tokenizer2<'a>) -> (&'a str, Vec<Stream<'a>>) {
    tokenizer.skip_token();
    let name = tokenizer.peek_token().get_identifier().unwrap(); // todo
    tokenizer.skip_token();
    tokenizer.consume_token_silently(TokenKind::OpenParenthesis);
    tokenizer.skip_space_silently();
    let mut args = Vec::new();
    read_macro_call_args(tokenizer, &mut args);
    tokenizer.consume_token_silently(TokenKind::CloseParenthesis);
    (name, args.clone())
}

fn read_macro_call_args<'a>(tokenizer: &Tokenizer2<'a>, args: &mut Vec<Stream<'a>>) {
    let current_token = tokenizer.peek_token();
    if current_token.is(TokenKind::CloseParenthesis) {
        return;
    }
    let m_begin = current_token.location;
    while !tokenizer.peek_token().is(TokenKind::MacroEnd) {
        tokenizer.skip_token();
    }
    let m_end = tokenizer.peek_token().location;
    tokenizer.consume_token_silently(TokenKind::MacroEnd);
    args.push(Stream::new(m_begin, m_end));
    tokenizer.skip_space_silently();
    read_macro_call_args(tokenizer, args);
}
