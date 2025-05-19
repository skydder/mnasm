// AST with MetaVariable
// MetaVariable can only appear in macro definition,
// eventually, implement pattern match macro
// however, "for now," implement only replace macro

use std::rc::Rc;

use data::Ast;
use util::{pair_end, AsmResult, TokenKind, Tokenizer};

use crate::parse_ident;

pub fn parse_macro<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    tokenizer.consume_token(TokenKind::At)?;
    let macro_name = parse_ident(tokenizer.clone())?;
    tokenizer.skip_space();
    let mut list = Vec::new();
    parse_stream(tokenizer, &mut list)?;

    Ok(Ast::Macro(macro_name, Rc::new(list)))
}

fn parse_stream<'code, T>(tokenizer: Rc<T>, list: &mut Vec<TokenKind>) -> AsmResult<'code, ()>
where
    T: Tokenizer<'code>,
{
    let open = match tokenizer.peek_token().kind {
        TokenKind::OpenBrace | TokenKind::OpenParenthesis | TokenKind::OpenSquareBracket => {
            let open = tokenizer.next_token().kind;
            list.push(open.clone());
            open
        }
        _ => {
            return Err(util::AsmError::ParseError(
                tokenizer.location(),
                String::new(),
                String::new(),
            ))
        }
    };
    let close = pair_end(&open);
    while !tokenizer.peek_token().is(&close) {
        match tokenizer.peek_token().kind {
            TokenKind::OpenBrace | TokenKind::OpenParenthesis | TokenKind::OpenSquareBracket => {
                parse_stream(tokenizer.clone(), list)?
            }
            _ => list.push(tokenizer.next_token().kind),
        }
    }
    tokenizer.consume_token(close.clone())?;
    list.push(close);
    Ok(())
}
