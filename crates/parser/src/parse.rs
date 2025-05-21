use std::rc::Rc;

use crate::{parse_label_block, parse_line, parse_macro};
use data::{Ast, LabelBlock, WithLocation};
use expander::{expand_macro, MacroData, MacroTokenizer};
use util::{AsmResult, TokenKind, Tokenizer};

pub fn parse<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Ast<'code>>
where
    T: Tokenizer<'code>,
{
    match tokenizer.peek_token().kind {
        TokenKind::LessThan | TokenKind::OpenBrace => parse_label_block(tokenizer),
        TokenKind::Identifier(_) => parse_line(tokenizer),
        TokenKind::At => parse_macro(tokenizer),
        TokenKind::NewLine => {
            tokenizer.next_token();
            parse(tokenizer)
        }
        TokenKind::EOS => Ok(Ast::EOS),
        _ => Err(util::AsmError::ParseError(
            tokenizer.location(),
            "unexpected token".to_string(),
            String::new(),
        )),
    }
}

pub fn parse_code<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Vec<Ast<'code>>>
where
    T: Tokenizer<'code>,
{
    let mut code = Vec::new();
    loop {
        let ast = parse(tokenizer.clone())?;
        match ast {
            Ast::EOS => return Ok(code),
            _ => code.push(ast),
        }
    }
}

pub fn expand_code(code: Vec<Ast<'_>>) -> AsmResult<'_, Vec<Ast<'_>>> {
    let macro_data = MacroData::new();
    let mut expanded_code = Vec::new();
    for ast in code {
        expanded_code.push(expand_macro_ast(&ast, macro_data.clone())?);
    }
    Ok(expanded_code)
}

pub fn expand_macro_ast<'code>(
    ast: &Ast<'code>,
    pacro_data: MacroData,
) -> AsmResult<'code, Ast<'code>> {
    match ast {
        Ast::Macro(name, ..) => {
            let expanded = expand_macro(ast, pacro_data.clone())?;
            expand_macro_ast(
                &parse(Rc::new(MacroTokenizer::new(name.location(), expanded)))?,
                pacro_data,
            )
        }
        Ast::LabelBlock(labelblock) => {
            let mut expanded_labelblock = Vec::new();
            for inner_block_ast in labelblock.data().block().iter() {
                expanded_labelblock.push(expand_macro_ast(inner_block_ast, pacro_data.clone())?);
            }
            Ok(Ast::LabelBlock(WithLocation::new(
                labelblock.location(),
                LabelBlock::new(
                    labelblock.data().name(),
                    labelblock.data().section(),
                    labelblock.data().is_global(),
                    expanded_labelblock,
                ),
            )))
        }
        _ => Ok(ast.clone()),
    }
}
