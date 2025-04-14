use std::rc::Rc;

use data::{Ast, Path};
use util::{AsmResult, TokenKind, Tokenizer};

use crate::parse_ident;

pub fn parse_label<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    let location = tokenizer.location();
    let is_relative = if tokenizer.peek_token().is(&TokenKind::Colon) {
        tokenizer.consume_token(TokenKind::Colon)?;
        tokenizer.consume_token(TokenKind::Colon)?;
        tokenizer.skip_space();
        false
    } else {
        true
    };
    let mut path = vec![parse_ident(tokenizer.clone())?];
    tokenizer.skip_space();

    while tokenizer.peek_token().is(&TokenKind::Colon) {
        tokenizer.consume_token(TokenKind::Colon)?;
        tokenizer.consume_token(TokenKind::Colon)?;
        tokenizer.skip_space();
        path.push(parse_ident(tokenizer.clone())?);
        tokenizer.skip_space();
    }

    Ok(Ast::Label(Path::new(location, path, is_relative)))
}
