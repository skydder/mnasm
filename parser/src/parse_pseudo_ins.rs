use data::{Operand, PseudoIns};
use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

use crate::parse_operands;

pub fn parse_pseudo_ins<'a>(tokenizer: &'a Tokenizer<'a>) -> PseudoIns<'a> {
    let currrent_token = tokenizer.peek_token();
    assert!(currrent_token.is_identifier());

    // <instruction>
    let ins = currrent_token.get_identifier().unwrap();
    tokenizer.next_symbol();

    // "("
    tokenizer.expect_symbol(TokenKind::OpenParenthesis);

    // <operands>?
    let mut operands: Vec<String> = Vec::new();
    if !tokenizer.peek_symbol().is(TokenKind::CloseParenthesis) {
        parse_ins_operands_inside(tokenizer, &mut operands);
    }

    // ")"
    tokenizer.expect_symbol(TokenKind::CloseParenthesis);

    PseudoIns::new(ins, operands, currrent_token.location)
}

fn parse_ins_operands_inside<'a>(tokenizer: &'a Tokenizer<'a>, operands: &mut Vec<String>) {
    // <operand>
    let op = match tokenizer.peek_token().kind {
        TokenKind::Minus | TokenKind::Number(_) => {
            parse_operands::parse_immediate(tokenizer).codegen().clone()
        },

        TokenKind::String(i) => {
            tokenizer.next_token();
            tokenizer.skip_space();
            format!("\"{}\"", i)
        },
        _ => {
            emit_error!(
                tokenizer.location(),
                "invalid expression, {:#?}",
                tokenizer.peek_token()
            );
        }
    };
    operands.push(op);
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
            parse_ins_operands_inside(tokenizer, operands);
        }
        _ => {
            emit_error!(
                tokenizer.location(),
                "invalid expression, is end?, {:#?}",
                tokenizer.peek_token()
            );
        }
    }
}
