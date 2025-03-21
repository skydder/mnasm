use std::{cell::RefCell, rc::Rc};

use data::{Ident, Label, Path, Scope};
use util::{AsmError, AsmResult, TokenKind, Tokenizer};

// "."? <ident> ("." <ident>)*
pub fn parse_label<'a, T>(
    tokenizer: Rc<T>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> AsmResult<'a, Label<'a>>
where
    T: Tokenizer<'a>,
{
    let location = tokenizer.location();
    let is_relative = if tokenizer.peek_token(true).is(TokenKind::Dot) {
        tokenizer.next_token();
        true
    } else {
        false
    };
    let mut path: Vec<Ident<'a>> = Vec::new();
    path.push(Ident::new(
        tokenizer
            .peek_token(true)
            .get_identifier()
            .ok_or(AsmError::ParseError(
                tokenizer.location(),
                "Identifier is needed for label".to_string(),
                "look at the bnf".to_string(),
            ))?,
    ));
    tokenizer.next_token();

    while tokenizer.peek_token(true).is(TokenKind::Dot) {
        tokenizer.next_token();
        path.push(Ident::new(
            tokenizer
                .peek_token(true)
                .get_identifier()
                .ok_or(AsmError::ParseError(
                    tokenizer.location(),
                    "Identifier should come after Dots".to_string(),
                    "look at the bnf".to_string(),
                ))?,
        ));
        tokenizer.next_token();
    }

    Ok(Label::new(
        *path.last().unwrap(),
        scope,
        location,
        Path::new(is_relative, path),
    ))
}
