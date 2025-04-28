use std::rc::Rc;

use data::{Ast, Immediate, WithLocation};
use util::{AsmError, AsmResult, TokenKind, Tokenizer};

pub fn parse_immediate<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    let location = tokenizer.location();
    let signed = if matches!(tokenizer.peek_token().kind, TokenKind::Minus) {
        tokenizer.next_token();
        true
    } else {
        false
    };
    match tokenizer.peek_token().kind {
        TokenKind::Number(data) => {
            tokenizer.next_token();
            Ok(Ast::Immediate(WithLocation::new(
                location,
                Immediate::new(data, signed),
            )))
        }
        _ => Err(AsmError::ParseError(
            location,
            "expected Immediate, but there isn't".to_string(),
            String::new(),
        )),
    }
}
