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

use util::{emit_error, Location};

use crate::{Stream, TokenKind, Tokenizer2};

#[derive(Debug, Clone)]
pub struct Macro<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
    pub stream: Stream<'a>,
}

impl<'a> Macro<'a> {
    pub fn new(name: &'a str, stream: Stream<'a>, args: Vec<&'a str>) -> Self {
        Self {
            name: name,
            args: args,
            stream: stream,
        }
    }
}

pub fn read_macro_def<'a>(tokenizer: &Tokenizer2<'a>) -> Macro<'a> {
    tokenizer.consume_token(TokenKind::Identifier("macro"));
    tokenizer.skip_space(false);
    let name = tokenizer.peek_token(false).get_identifier().unwrap(); // todo
    tokenizer.skip_token();
    tokenizer.skip_space(false);
    tokenizer.consume_token(TokenKind::OpenParenthesis);
    tokenizer.skip_space(false);
    let mut args = Vec::new();
    read_macro_def_args(tokenizer, &mut args);
    tokenizer.consume_token(TokenKind::CloseParenthesis);
    tokenizer.skip_space(false);

    let stream = read_macro_body(tokenizer);
    let new = Macro {
        name: name,
        args: args,
        stream: stream,
    };
    new
}

fn read_macro_def_args<'a>(tokenizer: &Tokenizer2<'a>, args: &mut Vec<&'a str>) {
    if tokenizer.peek_token(false).is(TokenKind::CloseParenthesis) {
        return;
    }
    let arg = tokenizer.next_token().get_identifier().unwrap();
    tokenizer.skip_space(false);
    tokenizer.consume_token(TokenKind::Comma);
    args.push(arg);
    tokenizer.skip_space(false);
    read_macro_def_args(tokenizer, args);
}

fn read_macro_body<'a>(tokenizer: &Tokenizer2<'a>) -> Stream<'a> {
    match tokenizer.peek_token(false).kind {
        TokenKind::OpenBrace => {
            let m_begin = tokenizer.location();
            tokenizer.next_token();
            let mut counter = 1;
            while counter > 0 {
                match tokenizer.peek_token(false).kind {
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
            let m_end = tokenizer.peek_token(false).location;
            Stream::new(m_begin, m_end)
        }
        _ => {
            let m_begin = tokenizer.location();
            while !tokenizer.peek_token(false).is(TokenKind::NewLine) {
                tokenizer.skip_token();
            }
            let m_end = tokenizer.peek_token(false).location;
            tokenizer.skip_token();
            Stream::new(m_begin, m_end)
        }
    }
}

// macro marker: @<label> ("(" (<stream>"@,")*")")?
pub fn read_macro_call<'a>(tokenizer: &Tokenizer2<'a>) -> (&'a str, Vec<Stream<'a>>) {
    tokenizer.consume_token(TokenKind::At);
    if tokenizer.peek_token(false).is(TokenKind::OpenSquareBracket) {
        return infix_macro_call(tokenizer);
    }
    let name = tokenizer.peek_token(false).get_identifier().unwrap(); // todo
    tokenizer.skip_token();
    tokenizer.skip_space(false);
    let mut args = Vec::new();
    match tokenizer.peek_token(false).kind {
        TokenKind::OpenBrace | TokenKind::OpenParenthesis => {
            read_macro_call_args(tokenizer, &mut args)
        }
        _ => emit_error!(tokenizer.location(), "unexpected token"),
    };
    let new = (name, args.clone());
    new
}

fn read_macro_call_args<'a>(tokenizer: &Tokenizer2<'a>, args: &mut Vec<Stream<'a>>) {
    match tokenizer.peek_token(false).kind {
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
    if !tokenizer.peek_token(false).is(TokenKind::CloseParenthesis) {
        let mut counter = 1;
        while counter > 0 {
            tokenizer.skip_token();
            match tokenizer.peek_token(false).kind {
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
    let m_end = tokenizer.peek_token(false).location;
    tokenizer.skip_token();
    args.push(Stream::new(m_begin, m_end));
    tokenizer.skip_space(false);
    read_macro_call_args(tokenizer, args);
}

fn read_macro_call_args_b<'a>(tokenizer: &Tokenizer2<'a>, args: &mut Vec<Stream<'a>>) {
    let current_token = tokenizer.peek_token(false);

    let m_begin = current_token.location;
    tokenizer.next_token();
    let mut counter = 1;
    while counter > 0 {
        match tokenizer.peek_token(false).kind {
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
    let m_end = tokenizer.peek_token(false).location;
    args.push(Stream::new(m_begin, m_end));
    tokenizer.skip_space(false);
    read_macro_call_args(tokenizer, args);
}

// === infix macro ===
static INFIX_STREAM: &str = "`ins(`lhs, `rhs)";
static INFIX_ARGS: &[&str] = &["ins", "lhs", "rhs"];
pub fn init_infix_macro() -> Macro<'static> {
    let location = Location::new_builtin(INFIX_STREAM);
    Macro {
        name: INFIX_STREAM,
        args: INFIX_ARGS.to_vec(),
        stream: Stream::new(location, location.end()),
    }
}

fn match_infix(token: TokenKind) -> Option<&str> {
    match token {
        TokenKind::Add => Some("add"),
        TokenKind::And => Some("and"),
        TokenKind::Mov => Some("mov"),
        TokenKind::Sub => Some("sub"),
        _ => None,
    }
}

fn infix_macro_call<'a>(tokenizer: &Tokenizer2<'a>) -> (&'a str, Vec<Stream<'a>>) {
    tokenizer.consume_token(TokenKind::OpenSquareBracket);

    let lhs_begin: Location<'a> = tokenizer.location();
    let mut lhs_end: Option<Location<'a>> = None;
    let mut rhs_begin: Option<Location<'a>> = None;
    let rhs_end: Location<'a>;
    let mut ins: Option<Stream<'a>> = None;
    let mut current_token = tokenizer.peek_token(false);
    while !current_token.is(TokenKind::CloseSquareBracket) {
        if let Some(i) = match_infix(current_token.kind) {
            if ins.is_some() {
                emit_error!(lhs_begin, "unexpected expression")
            }
            lhs_end = Some(current_token.location);
            let loc = Location::new_builtin(i);
            ins = Some(Stream::new(loc, loc.end()));
            tokenizer.skip_token();
            tokenizer.skip_space(false);
            current_token = tokenizer.peek_token(false);
            rhs_begin = Some(tokenizer.location());
            continue;
        }
        tokenizer.skip_token();
        current_token = tokenizer.peek_token(false);
    }
    rhs_end = tokenizer.location();
    // tokenizer.consume_token(TokenKind::CloseSquareBracket);
    tokenizer.skip_token();
    (
        "infix",
        vec![
            ins.unwrap_or_else(|| emit_error!(lhs_begin, "unexpected expression1")),
            Stream::new(
                lhs_begin,
                lhs_end.unwrap_or_else(|| emit_error!(lhs_begin, "unexpected expression2")),
            ),
            Stream::new(
                rhs_begin.unwrap_or_else(|| emit_error!(lhs_begin, "unexpected expression3")),
                rhs_end,
            ),
        ],
    )
}
