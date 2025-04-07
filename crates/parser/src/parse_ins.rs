use std::{cell::RefCell, rc::Rc};

use data::{CompoundIns, Ins, Operand, Scope};
use util::{AsmError, AsmResult, TokenKind, Tokenizer};

use crate::parse_operands;

// <ins> = <instruction(identifier)> "(" <operands>? ")"
pub fn parse_ins<'a, T>(tokenizer: Rc<T>, scope: Rc<RefCell<Scope<'a>>>) -> AsmResult<'a, Ins<'a>>
where
    T: Tokenizer<'a>,
{
    let currrent_token = tokenizer.peek_token();
    assert!(currrent_token.is_identifier());

    // <instruction>
    let ins = currrent_token.get_identifier().ok_or(AsmError::ParseError(
        tokenizer.location(),
        "Identifier is needed for label".to_string(),
        "look at the bnf".to_string(),
    ))?;
    tokenizer.next_token();
    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::OpenParenthesis);
    tokenizer.skip_space();
    // <operands>?
    let mut operands: Vec<Box<dyn Operand + 'a>> = Vec::new();
    if !tokenizer.peek_token().is(TokenKind::CloseParenthesis) {
        parse_ins_operands_inside(tokenizer.clone(), &mut operands, scope)?;
    }

    // ")"
    tokenizer.consume_token(TokenKind::CloseParenthesis);

    Ok(Ins::new(ins, operands, currrent_token.location))
}

// <operands> = <operand> ("," <operand>)*
fn parse_ins_operands_inside<'a, T>(
    tokenizer: Rc<T>,
    operands: &mut Vec<Box<dyn Operand + 'a>>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> AsmResult<'a, ()>
where
    T: Tokenizer<'a>,
{
    // <operand>
    operands.push(parse_operands(tokenizer.clone(), scope.clone())?);
    tokenizer.skip_space();
    let current = tokenizer.peek_token();
    match current.kind {
        TokenKind::CloseParenthesis => Ok(()),
        // ("," <operand>)*
        TokenKind::Comma => {
            // ","
            tokenizer.next_token();
            tokenizer.skip_space();

            // <operand>)*
            parse_ins_operands_inside(tokenizer, operands, scope)
        }
        _ => Err(AsmError::ParseError(
            tokenizer.location(),
            format!(
                "unexpected token, not for operands: {:?}",
                tokenizer.peek_token(),
            ),
            "look at the bnf".to_string(),
        )),
    }
}

// <compound_ins> = <ins> ("," <ins>)*
pub fn parse_compound_ins<'a, T>(
    tokenizer: Rc<T>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> AsmResult<'a, CompoundIns<'a>>
where
    T: Tokenizer<'a>,
{
    // <compound_ins>
    let mut compound = Vec::new();
    let loc = tokenizer.location();
    parse_compound_ins_inside(tokenizer, &mut compound, scope)?;
    // tokenizer.add_to_code(TokenKind::NewLine);
    Ok(CompoundIns::new(compound, loc))
}

// <compound_ins> = <ins> ("," <ins>)*
fn parse_compound_ins_inside<'a, T>(
    tokenizer: Rc<T>,
    compound: &mut Vec<Ins<'a>>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> AsmResult<'a, ()>
where
    T: Tokenizer<'a>,
{
    // <ins>
    compound.push(parse_ins(tokenizer.clone(), scope.clone())?);
    tokenizer.skip_space();

    match tokenizer.peek_token().kind {
        TokenKind::NewLine | TokenKind::Semicolon => Ok(()),
        // ("," <ins>)*
        TokenKind::Comma => {
            // ","
            tokenizer.next_token();
            tokenizer.skip_space();

            // <ins>)*
            parse_compound_ins_inside(tokenizer, compound, scope)
        }
        _ => Err(AsmError::ParseError(
            tokenizer.location(),
            format!("invalid expression: {:?}", tokenizer.peek_token(),),
            "look at the bnf".to_string(),
        )),
    }
}
