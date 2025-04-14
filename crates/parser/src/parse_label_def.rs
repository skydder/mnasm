use std::rc::Rc;

use data::{Ast, Section};
use util::{AsmResult, TokenKind, Tokenizer};

use crate::{parse, parse_ident};

pub fn parse_label_def<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    // let location = tokenizer.location();
    tokenizer.consume_token(TokenKind::LessThan)?;
    tokenizer.skip_space();

    let label = parse_ident(tokenizer.clone())?;
    tokenizer.skip_space();

    let mut is_global = false;
    let mut section = None;

    if tokenizer.peek_token().is(&TokenKind::Colon) {
        tokenizer.next_token();
        tokenizer.skip_space();

        is_global = if parse_global(tokenizer.clone()) {
            section = if tokenizer.peek_token().is(&TokenKind::Colon) {
                tokenizer.next_token();
                parse_section(tokenizer.clone())
            } else {
                None
            };
            true
        } else {
            section = parse_section(tokenizer.clone());
            false
        };
    }
    tokenizer.consume_token(TokenKind::GreaterThan)?;
    tokenizer.skip_space();

    let next = if tokenizer.peek_token().is(&TokenKind::NewLine) {
        None
    } else {
        Some(Box::new(parse(tokenizer)?))
    };

    Ok(Ast::LabelDef(label, section, is_global, next))
}

fn parse_global<'code, T>(tokenizer: Rc<T>) -> bool
where
    T: Tokenizer<'code>,
{
    if matches!(tokenizer.peek_token().kind, TokenKind::Identifier(s) if s.as_str() == "global") {
        tokenizer.next_token();
        true
    } else {
        false
    }
}

fn parse_section<'code, T>(tokenizer: Rc<T>) -> Option<Section>
where
    T: Tokenizer<'code>,
{
    match tokenizer.peek_token().kind {
        TokenKind::Dot => {
            tokenizer.next_token();
            match tokenizer.peek_token().kind {
                TokenKind::Identifier(s) if s.as_str() == "text" => {
                    tokenizer.next_token();
                    Some(Section::Text)
                }
                TokenKind::Identifier(s) if s.as_str() == "data" => {
                    tokenizer.next_token();
                    Some(Section::Data)
                }
                TokenKind::Identifier(s) if s.as_str() == "bss" => {
                    tokenizer.next_token();
                    Some(Section::Bss)
                }
                _ => todo!(),
            }
        }
        TokenKind::Identifier(s) => {
            tokenizer.next_token();
            Some(Section::Custom(s.clone()))
        }
        _ => None,
    }
}
