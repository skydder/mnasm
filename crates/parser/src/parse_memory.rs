use std::rc::Rc;

use data::{Ast, Memory, Scale, WithLocation};
use util::{AsmError, AsmResult, TokenKind, Tokenizer};

use crate::{parse_ident, parse_immediate, parse_register::parse_register_from_str};

pub fn parse_memory<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    let location = tokenizer.location();
    if tokenizer
        .peek_token()
        .get_identifier()
        .is_some_and(|s| s.as_str() == "ptr")
    {
        tokenizer.next_token();
    } else {
        return Err(util::AsmError::ParseError(
            location,
            "expected Memory, but there isn't".to_string(),
            String::new(),
        ));
    }
    tokenizer.skip_space();

    tokenizer.consume_token(TokenKind::LessThan).map_err(|_| {
        AsmError::ParseError(
            tokenizer.location(),
            "memory size is requiered to be explicit".to_string(),
            String::new(),
        )
    })?;
    tokenizer.skip_space();
    let size = match tokenizer.peek_token().get_identifier() {
        Some(s) if s.as_str() == "byte" => 1,
        Some(s) if s.as_str() == "word" => 2,
        Some(s) if s.as_str() == "dword" => 4,
        Some(s) if s.as_str() == "qword" => 8,
        _ => {
            return Err(util::AsmError::ParseError(
                tokenizer.location(),
                "expected Memory size, but there isn't".to_string(),
                String::new(),
            ))
        }
    };
    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::GreaterThan)?;
    tokenizer.skip_space();

    tokenizer.consume_token(TokenKind::OpenParenthesis)?;
    tokenizer.skip_space();

    let reg_loc = tokenizer.location();
    let base = match parse_ident(tokenizer.clone())?.data().get_str().as_str() {
        "_" => None,
        i => Some(parse_register_from_str(i, reg_loc)?),
    };
    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::Comma)?;
    tokenizer.skip_space();

    let reg_loc = tokenizer.location();
    let index = match parse_ident(tokenizer.clone())?.data().get_str().as_str() {
        "_" => None,
        i => Some(parse_register_from_str(i, reg_loc)?),
    };
    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::Comma)?;
    tokenizer.skip_space();

    let scale = match tokenizer.peek_token().kind {
        TokenKind::Number(1) => Some(Scale::S1),
        TokenKind::Number(2) => Some(Scale::S2),
        TokenKind::Number(4) => Some(Scale::S4),
        TokenKind::Number(8) => Some(Scale::S8),
        TokenKind::Identifier(s) if s.as_str() == "_" => None,
        _ => {
            return Err(util::AsmError::ParseError(
                tokenizer.location(),
                "expected Memory size, but there isn't".to_string(),
                String::new(),
            ))
        }
    };

    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::Comma)?;
    tokenizer.skip_space();

    let disp = match tokenizer.peek_token().kind {
        TokenKind::Number(_) => Some(Rc::new(parse_immediate(tokenizer.clone())?)),
        TokenKind::Identifier(s) if s.as_str() == "-" => {
            Some(Rc::new(parse_immediate(tokenizer.clone())?))
        }
        // TokenKind::Identifier(_) => parse_label(tokenizer)?
        _ => None,
    };
    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::CloseParenthesis)?;
    Ok(Ast::Memory(WithLocation::new(
        location,
        Memory::new(size, base, index, scale, disp),
    )))
}
