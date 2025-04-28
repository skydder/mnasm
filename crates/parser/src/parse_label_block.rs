use std::rc::Rc;

use data::{Ast, Ident, LabelBlock, Section, WithLocation};
use util::{AsmResult, TokenKind, Tokenizer};

use crate::{parse, parse_ident, util::parse_list};

pub fn parse_label_block<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    let location = tokenizer.location();
    match tokenizer.peek_token().kind {
        TokenKind::OpenBrace => {
            let name = Ident::anonymous_ident();
            let block = parse_block(tokenizer.clone())?;
            Ok(Ast::LabelBlock(WithLocation::new(
                location,
                LabelBlock::new(name, Section::None, false, block),
            )))
        }
        TokenKind::LessThan => {
            let (name, section, is_global) = parse_label(tokenizer.clone())?;
            let block = if tokenizer.peek_token().is(&TokenKind::OpenBrace) {
                parse_block(tokenizer)?
            } else {
                Vec::new()
            };
            Ok(Ast::LabelBlock(WithLocation::new(
                location,
                LabelBlock::new(name.data(), section, is_global, block),
            )))
        }
        _ => {
            todo!()
        }
    }
}

fn parse_block<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Vec<Ast<'code>>>
where
    T: Tokenizer<'code>,
{
    tokenizer.consume_token(TokenKind::OpenBrace)?;
    if tokenizer.peek_token().is(&TokenKind::NewLine) {
        tokenizer.next_token();
    }
    let list = parse_list(
        tokenizer.clone(),
        TokenKind::NewLine,
        TokenKind::CloseBrace,
        parse,
    )?;

    tokenizer.consume_token(TokenKind::CloseBrace)?;
    Ok(list)
}

fn parse_label<'code, T>(
    tokenizer: Rc<T>,
) -> AsmResult<'code, (WithLocation<'code, Ident>, Section, bool)>
where
    T: Tokenizer<'code>,
{
    // let location = tokenizer.location();
    tokenizer.consume_token(TokenKind::LessThan)?;
    tokenizer.skip_space();

    let label = parse_ident(tokenizer.clone())?;
    tokenizer.skip_space();

    let mut is_global = false;
    let mut section = Section::None;

    if tokenizer.peek_token().is(&TokenKind::Colon) {
        tokenizer.next_token();
        tokenizer.skip_space();

        is_global = if parse_global(tokenizer.clone()) {
            if tokenizer.peek_token().is(&TokenKind::Colon) {
                tokenizer.next_token();
                section = parse_section(tokenizer.clone())?;
            }
            true
        } else {
            section = parse_section(tokenizer.clone())?;
            false
        };
    }

    tokenizer.consume_token(TokenKind::GreaterThan)?;
    tokenizer.skip_space();

    Ok((label, section, is_global))
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

fn parse_section<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Section>
where
    T: Tokenizer<'code>,
{
    match tokenizer.peek_token().kind {
        TokenKind::Dot => {
            tokenizer.next_token();
            match tokenizer.peek_token().kind {
                TokenKind::Identifier(s) if s.as_str() == "text" => {
                    tokenizer.next_token();
                    Ok(Section::Text)
                }
                TokenKind::Identifier(s) if s.as_str() == "data" => {
                    tokenizer.next_token();
                    Ok(Section::Data)
                }
                TokenKind::Identifier(s) if s.as_str() == "bss" => {
                    tokenizer.next_token();
                    Ok(Section::Bss)
                }
                _ => todo!(),
            }
        }
        TokenKind::Identifier(s) => {
            tokenizer.next_token();
            Ok(Section::Custom(s.clone()))
        }
        _ => Err(util::AsmError::ParseError(
            tokenizer.location(),
            "expected token for representing section!".to_string(),
            String::new(),
        )),
    }
}
