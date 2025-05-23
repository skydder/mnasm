use std::rc::Rc;

use data::{Ast, Path, PathState, WithLocation};
use util::{AsmResult, TokenKind, Tokenizer};

use crate::parse_ident;

pub fn parse_label<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    let location = tokenizer.location();
    let state = if tokenizer.peek_token().is(&TokenKind::Colon) {
        tokenizer.consume_token(TokenKind::Colon)?;
        tokenizer.consume_token(TokenKind::Colon)?;
        tokenizer.skip_space();
        PathState::Absolute
    } else if tokenizer.peek_token().is(&TokenKind::Dot) {
        tokenizer.next_token();
        tokenizer.skip_space();
        PathState::Relative
    }else {
        PathState::Global
    };
    let mut path = vec![parse_ident(tokenizer.clone())?.data()];
    tokenizer.skip_space();

    while tokenizer.peek_token().is(&TokenKind::Colon) {
        tokenizer.consume_token(TokenKind::Colon)?;
        tokenizer.consume_token(TokenKind::Colon)?;
        tokenizer.skip_space();
        path.push(parse_ident(tokenizer.clone())?.data());
        tokenizer.skip_space();
    }

    Ok(Ast::Label(WithLocation::new(
        location,
        Path::new(Rc::new(path), state),
    )))
}
