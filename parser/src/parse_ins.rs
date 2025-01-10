use std::{cell::RefCell, rc::Rc};

use data::{CompoundIns, Ins, Operand, Scope};
use tokenizer::{TokenGenerator, TokenKind, Tokenizer};
use util::emit_error;

use crate::parse_operands;

// <ins> = <instruction(identifier)> "(" <operands>? ")"
pub fn parse_ins<'a>(tokenizer: &'a Tokenizer<'a>, scope: Rc<RefCell<Scope<'a>>>) -> Ins<'a> {
    let currrent_token = tokenizer.peek_token();
    assert!(currrent_token.is_identifier());

    // <instruction>
    let ins = currrent_token.get_identifier().unwrap();
    tokenizer.next_token();
    tokenizer.skip_space();

    // "("
    tokenizer.consume_token(TokenKind::OpenParenthesis);
    tokenizer.skip_space();

    // <operands>?
    let mut operands: Vec<Box<dyn Operand + 'a>> = Vec::new();
    if !tokenizer.peek_token().is(TokenKind::CloseParenthesis) {
        parse_ins_operands_inside(tokenizer, &mut operands, scope);
    }

    // ")"
    tokenizer.consume_token(TokenKind::CloseParenthesis);

    Ins::new(ins, operands, currrent_token.location)
}

// <operands> = <operand> ("," <operand>)*
fn parse_ins_operands_inside<'a>(
    tokenizer: &'a Tokenizer<'a>,
    operands: &mut Vec<Box<dyn Operand + 'a>>,
    scope: Rc<RefCell<Scope<'a>>>,
) {
    // <operand>
    operands.push(parse_operands(tokenizer, scope.clone()));
    tokenizer.skip_space();

    match tokenizer.peek_token().kind {
        TokenKind::CloseParenthesis => {
            return;
        }
        // ("," <operand>)*
        TokenKind::Comma => {
            // ","
            tokenizer.next_token();

            tokenizer.skip_space();

            // <operand>)*
            parse_ins_operands_inside(tokenizer, operands, scope);
        }
        _ => {
            emit_error!(tokenizer.location(), "invalid expression");
        }
    }
}

// <compound_ins> = <ins> ("," <ins>)*
pub fn parse_compound_ins<'a>(
    tokenizer: &'a Tokenizer<'a>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> CompoundIns<'a> {
    // <compound_ins>
    let mut compound = Vec::new();
    let loc = tokenizer.location();
    parse_compound_ins_inside(tokenizer, &mut compound, scope);

    CompoundIns::new(compound, loc)
}

// <compound_ins> = <ins> ("," <ins>)*
fn parse_compound_ins_inside<'a>(
    tokenizer: &'a Tokenizer<'a>,
    compound: &mut Vec<Ins<'a>>,
    scope: Rc<RefCell<Scope<'a>>>,
) {
    // <ins>
    compound.push(parse_ins(tokenizer, scope.clone()));
    tokenizer.skip_space();

    match tokenizer.peek_token().kind {
        TokenKind::NewLine => {
            return;
        }
        // ("," <ins>)*
        TokenKind::Comma => {
            // ","
            tokenizer.next_token();
            tokenizer.skip_space();

            // <ins>)*
            parse_compound_ins_inside(tokenizer, compound, scope);
        }
        _ => {
            emit_error!(tokenizer.location(), "invalid expression");
        }
    }
}
