use data::{CompoundIns, Ins, Operand};
use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

use crate::parse_operands;

pub fn parse_ins<'a>(tokenizer: &'a Tokenizer<'a>) -> Ins<'a> {
    let currrent_token = tokenizer.peek_token();
    assert!(currrent_token.is_identifier());

    let ins = currrent_token.get_identifier().unwrap();
    tokenizer.next_symbol();
    tokenizer.expect_symbol(TokenKind::OpenParenthesis);
    let mut operands: Vec<Box<dyn Operand + 'a>> = Vec::new();
    if !tokenizer.peek_symbol().is(TokenKind::CloseParenthesis) {
        parse_ins_operands_inside(tokenizer, &mut operands);
    }
    tokenizer.expect_symbol(TokenKind::CloseParenthesis);
    // tokenizer.skip_space();
    Ins::new(ins, operands, currrent_token.location)
}

fn parse_ins_operands_inside<'a>(
    tokenizer: &'a Tokenizer<'a>,
    operands: &mut Vec<Box<dyn Operand + 'a>>,
) {
    operands.push(parse_operands(tokenizer));
    tokenizer.skip_space();
    match tokenizer.peek_token().kind {
        TokenKind::CloseParenthesis => {
            return;
        }
        TokenKind::Comma => {
            tokenizer.next_token();
            tokenizer.skip_space();
            parse_ins_operands_inside(tokenizer, operands);
        }
        _ => {
            emit_error!(tokenizer.location(), "invalid expression");
        }
    }
}

pub fn parse_compound_ins<'a>(tokenizer: &'a Tokenizer<'a>) -> CompoundIns<'a> {
    let mut compound = Vec::new();
    let loc = tokenizer.location();
    parse_compound_ins_inside(tokenizer, &mut compound);
    CompoundIns::new(compound, loc)
}

fn parse_compound_ins_inside<'a>(tokenizer: &'a Tokenizer<'a>, compound: &mut Vec<Ins<'a>>) {
    compound.push(parse_ins(tokenizer));
    tokenizer.skip_space();
    match tokenizer.peek_token().kind {
        TokenKind::NewLine => {
            return;
        }
        TokenKind::Comma => {
            tokenizer.next_token();
            tokenizer.skip_space();
            parse_compound_ins_inside(tokenizer, compound);
        }
        _ => {
            emit_error!(tokenizer.location(), "invalid expression");
        }
    }
}
