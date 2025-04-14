use std::rc::Rc;

use data::Ast;
use util::{AsmResult, TokenKind, Tokenizer};

use crate::{parse_immediate, parse_label, parse_memory, parse_register, parse_strings};

pub fn parse_operand<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    match tokenizer.peek_token().kind {
        TokenKind::Number(_) => parse_immediate(tokenizer),
        TokenKind::Colon => parse_label(tokenizer),
        TokenKind::Identifier(s) if s.as_str() == "ptr" => parse_memory(tokenizer),
        TokenKind::Identifier(_) => {
            if let Ok(reg) = parse_register(tokenizer.clone()) {
                Ok(reg)
            } else {
                parse_label(tokenizer)
            }
        }
        TokenKind::String(_) => Ok(Ast::String(parse_strings(tokenizer)?)),
        _ => todo!(),
    }
}
