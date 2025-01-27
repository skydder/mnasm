use std::{cell::RefCell, rc::Rc};

use data::{Ident, Scope, Stmt};
use tokenizer::{TokenKind, Tokenizer2};
use util::emit_error;

use crate::{
    parse_block, parse_compound_ins, parse_fn_like_macro, parse_fn_like_macro_def, parse_label, parse_label_def, parse_let_macro, parse_pseudo_ins
};

// <stmt> = <compound_ins> | <block> | <label_def>
pub fn parse_stmt<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    indent_depth: usize,
    scope: Rc<RefCell<Scope<'a>>>,
) -> Box<dyn Stmt<'a> + 'a> {
    let currrent_token = tokenizer.peek_token();
    match currrent_token.kind {
        TokenKind::Identifier("db") => Box::new(parse_pseudo_ins(tokenizer, scope)),
        TokenKind::Identifier("resb") => Box::new(parse_pseudo_ins(tokenizer, scope)),
        TokenKind::Identifier("extern") | TokenKind::Identifier("include") => {
            Box::new(parse_pseudo_ins(tokenizer, scope))
        }
        TokenKind::Identifier("let") => Box::new(parse_let_macro(tokenizer, scope)),
        TokenKind::Identifier("macro") => Box::new(parse_fn_like_macro_def(tokenizer, scope)),
        // <compound_stmt>
        TokenKind::Identifier(ident) => Box::new(parse_compound_ins(tokenizer, scope)),
        TokenKind::At => {
            parse_fn_like_macro(tokenizer, indent_depth, scope)
        }

        // <block>
        TokenKind::OpenBrace => Box::new(parse_block(
            tokenizer,
            indent_depth,
            Rc::new(RefCell::new(Scope::new(None, Some(scope)))),
        )),

        // <label_def>
        TokenKind::LessThan => Box::new(parse_label_def(tokenizer, indent_depth, scope)),
        _ => {
            emit_error!(currrent_token.location, "expected stmt, but found other!:{:?}\n{}",tokenizer.peek_token(), tokenizer.code())
        }
    }
}
