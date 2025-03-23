use std::{cell::RefCell, rc::Rc};

use data::{Ident, LabelDef, Scope};
use util::{AsmError, AsmResult, TokenKind, Tokenizer};

use crate::{parse_block, parse_label};

// <label_def> = "<" <label> (":" "global")? (":" <section> )? ">" <block>?
//&'a T,
pub fn parse_label_def<'a, T>(
    tokenizer: Rc<T>,
    indent_depth: usize,
    scope: Rc<RefCell<Scope<'a>>>,
) -> AsmResult<'a, LabelDef<'a>>
where
    T: Tokenizer<'a>,
{
    let loc = tokenizer.location();

    // "<"
    assert!(tokenizer.peek_token(true).is(TokenKind::LessThan));
    tokenizer.next_token();
    tokenizer.skip_space(true);

    // <label>
    let label_data = parse_label(tokenizer.clone(), scope.clone())?;
    let label = label_data.ident();
    if scope.borrow().find_label_local(&label_data.path).is_some() {
        return Err(AsmError::ParseError(
            loc,
            "this label is defined more than once!".to_string(),
            "label should be defined only once".to_string(),
        ));
    }

    let gen_label = scope.borrow().gen_label(label);
    // kimokimo-nest :<

    // (":" "global")? (":" <section> )?
    tokenizer.skip_space(true);
    let (is_global, section) = if tokenizer.peek_token(true).is(TokenKind::Colon) {
        tokenizer.next_token();
        tokenizer.skip_space(true);
        // "global" (":" <section> )?
        if tokenizer
            .peek_token(true)
            .is(TokenKind::Identifier("global"))
        {
            tokenizer.next_token();
            tokenizer.skip_space(true);

            // (":" <section> )?
            let sec = if tokenizer.peek_token(true).is(TokenKind::Colon) {
                tokenizer.next_token();
                tokenizer.skip_space(true);
                // <section>
                Some(parse_section(tokenizer.clone())?)
            } else {
                None
            };
            (true, sec)

        // <section>
        } else {
            let sec = Some(parse_section(tokenizer.clone())?);
            (false, sec)
        }
    } else {
        (false, None)
    };

    // ">"
    tokenizer.skip_space(true);
    tokenizer.consume_token(TokenKind::GreaterThan);
    tokenizer.skip_space(true);

    // <block>?
    let block = match tokenizer.peek_token(true).kind {
        TokenKind::OpenBrace => {
            let s = Rc::new(RefCell::new(Scope::new(Some(label), Some(scope.clone()))));
            scope.borrow_mut().add_label(label, Some(s.clone()));
            Some(parse_block(tokenizer, indent_depth, s)?)
        }
        TokenKind::NewLine | TokenKind::EOS => {
            scope.borrow_mut().add_label(label, None);
            // tokenizer.add_to_code(TokenKind::NewLine);
            None
        }
        _ => {
            todo!()
        }
    };
    Ok(LabelDef::new(
        label, gen_label, is_global, section, block, loc,
    ))
}

fn parse_section<'a, T>(tokenizer: Rc<T>) -> AsmResult<'a, Ident<'a>>
where
    T: Tokenizer<'a>,
{
    let s = if tokenizer.peek_token(true).is(TokenKind::Dot) {
        tokenizer.next_token();
        // todo: add all reserved section
        match tokenizer.peek_token(true).kind {
            TokenKind::Identifier("text") => Ok(Ident::new(".text")),
            TokenKind::Identifier("data") => Ok(Ident::new(".data")),
            TokenKind::Identifier("bss") => Ok(Ident::new(".bss")),
            _ => Err(AsmError::ParseError(
                tokenizer.location(),
                "only special label can come here".to_string(),
                "look at the bnf".to_string(),
            )),
        }
    } else {
        tokenizer.skip_space(true);
        Ok(Ident::new(
            tokenizer
                .peek_token(true)
                .get_identifier()
                .ok_or(AsmError::ParseError(
                    tokenizer.location(),
                    "consumed label here but found other".to_string(),
                    "look at the bnf".to_string(),
                ))?,
        ))
    };
    tokenizer.next_token();
    s
}
