use std::{cell::RefCell, rc::Rc};

use data::{CompoundIns, Ins, Operand, Scope};
use tokenizer::{TokenKind, Tokenizer2};
use util::{AsmError, AsmResult};

use crate::parse_operands;

// <ins> = <instruction(identifier)> "(" <operands>? ")"
pub fn parse_ins<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> AsmResult<'a, Ins<'a>> {
    let currrent_token = tokenizer.peek_token(true);
    assert!(currrent_token.is_identifier());

    // <instruction>
    let ins = currrent_token.get_identifier().ok_or(AsmError::ParseError(
        tokenizer.location(),
        "Identifier is needed for label".to_string(),
        "look at the bnf".to_string(),
    ))?;
    tokenizer.next_token();
    tokenizer.skip_space(true);
    let check = if tokenizer.peek_token(true).is(TokenKind::Not) {
        tokenizer.next_token();
        false
    } else {
        true
    };
    // "("
    tokenizer.consume_token(TokenKind::OpenParenthesis);
    tokenizer.skip_space(true);
    // <operands>?
    let mut operands: Vec<Box<dyn Operand + 'a>> = Vec::new();
    if !tokenizer.peek_token(true).is(TokenKind::CloseParenthesis) {
        parse_ins_operands_inside(tokenizer, &mut operands, scope)?;
    }

    // ")"
    tokenizer.consume_token(TokenKind::CloseParenthesis);

    Ok(Ins::new(ins, operands, currrent_token.location, check))
}

// <operands> = <operand> ("," <operand>)*
fn parse_ins_operands_inside<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    operands: &mut Vec<Box<dyn Operand + 'a>>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> AsmResult<'a, ()> {
    // <operand>
    operands.push(parse_operands(tokenizer, scope.clone())?);
    tokenizer.skip_space(true);
    let current = tokenizer.peek_token(true);
    match current.kind {
        TokenKind::CloseParenthesis => Ok(()),
        // ("," <operand>)*
        TokenKind::Comma => {
            // ","
            tokenizer.next_token();
            tokenizer.skip_space(true);

            // <operand>)*
            parse_ins_operands_inside(tokenizer, operands, scope)
        }
        _ => Err(AsmError::ParseError(
            tokenizer.location(),
            format!(
                "unexpected token, not for operands: {:?}",
                tokenizer.peek_token(true),
            ),
            "look at the bnf".to_string(),
        )),
    }
}

// <compound_ins> = <ins> ("," <ins>)*
pub fn parse_compound_ins<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> AsmResult<'a, CompoundIns<'a>> {
    // <compound_ins>
    let mut compound = Vec::new();
    let loc = tokenizer.location();
    parse_compound_ins_inside(tokenizer, &mut compound, scope)?;
    // tokenizer.add_to_code(TokenKind::NewLine);
    Ok(CompoundIns::new(compound, loc))
}

// <compound_ins> = <ins> ("," <ins>)*
fn parse_compound_ins_inside<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    compound: &mut Vec<Ins<'a>>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> AsmResult<'a, ()> {
    // <ins>
    compound.push(parse_ins(tokenizer, scope.clone())?);
    tokenizer.skip_space(true);

    match tokenizer.peek_token(true).kind {
        TokenKind::NewLine | TokenKind::Semicolon => Ok(()),
        // ("," <ins>)*
        TokenKind::Comma => {
            // ","
            tokenizer.next_token();
            tokenizer.skip_space(true);

            // <ins>)*
            parse_compound_ins_inside(tokenizer, compound, scope)
        }
        _ => Err(AsmError::ParseError(
            tokenizer.location(),
            format!("invalid expression: {:?}", tokenizer.peek_token(true),),
            "look at the bnf".to_string(),
        )),
    }
}
