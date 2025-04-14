use std::rc::Rc;

use util::{AsmResult, TokenKind, Tokenizer};

use crate::Parser;

pub fn parse_list<'code, T, R>(
    tokenizer: Rc<T>,
    sep: TokenKind,
    end: TokenKind,
    parser: impl Parser<'code, T, R>,
) -> AsmResult<'code, Vec<R>>
where
    T: Tokenizer<'code>,
{
    let mut list = Vec::new();
    parse_list_inside(tokenizer, sep, end, parser, &mut list)?;
    Ok(list)
}

fn parse_list_inside<'code, T, R>(
    tokenizer: Rc<T>,
    sep: TokenKind,
    end: TokenKind,
    parser: impl Parser<'code, T, R>,
    list: &mut Vec<R>,
) -> AsmResult<'code, ()>
where
    T: Tokenizer<'code>,
{
    tokenizer.skip_space();
    if tokenizer.peek_token().is(&end) {
        Ok(())
    } else if tokenizer.peek_token().is(&sep) {
        tokenizer.next_token();
        parse_list_inside(tokenizer, sep, end, parser, list)
    } else {
        let item = parser(tokenizer.clone())?;
        list.push(item);
        parse_list_inside(tokenizer, sep, end, parser, list)
    }
}
