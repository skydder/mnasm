use std::{cell::RefCell, rc::Rc};

use data::{Codegen, Ident, Operand, PseudoIns, Scope, UnimplementedOperand};
use util::{AsmError, AsmResult, TokenKind, Tokenizer};

use crate::parse_operands;

pub fn parse_pseudo_ins<'a, T>(
    tokenizer: Rc<T>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> AsmResult<'a, PseudoIns<'a>>
where
    T: Tokenizer<'a>,
{
    let currrent_token = tokenizer.peek_token();

    if currrent_token.is(TokenKind::Not) {
        tokenizer.next_token();
        let ins = tokenizer.peek_token().get_identifier().unwrap();
        tokenizer.next_token();
        tokenizer.skip_space();

        // "("
        tokenizer.consume_token(TokenKind::OpenParenthesis);
        tokenizer.skip_space();
        let mut operands: Vec<Box<dyn Operand + 'a>> = Vec::new();
        if !tokenizer.peek_token().is(TokenKind::CloseParenthesis) {
            parse_nasm_operands_inside(tokenizer.clone(), &mut operands, scope)?;
        }
        tokenizer.skip_space();
        tokenizer.consume_token(TokenKind::CloseParenthesis);
        // tokenizer.add_to_code(TokenKind::NewLine);
        return Ok(PseudoIns::new_nasm(ins, operands, currrent_token.location));
    }

    assert!(currrent_token.is_identifier());

    // <instruction>
    let ins = currrent_token.get_identifier().unwrap();
    tokenizer.next_token();
    tokenizer.skip_space();

    // "("
    tokenizer.consume_token(TokenKind::OpenParenthesis);
    tokenizer.skip_space();
    let mut operands: Vec<String> = Vec::new();
    if ins == "extern" || ins == "include" {
        if tokenizer.peek_token().is(TokenKind::CloseParenthesis) {
            return Err(AsmError::ParseError(
                tokenizer.location(),
                "expected label, but found other".to_string(),
                "look at the bnf".to_string(),
            ));
        }
        parse_extern_operands_inside(tokenizer.clone(), &mut operands, scope)?;
    } else if ins == "db" || ins == "resb" {
        // <operands>?
        if !tokenizer.peek_token().is(TokenKind::CloseParenthesis) {
            parse_ins_operands_inside(tokenizer.clone(), &mut operands)?;
        }
    } else if ins == "nasm" {
        match tokenizer.peek_token().kind {
            TokenKind::String(s) => {
                operands.push(s.to_string());
                tokenizer.next_token();
            }
            TokenKind::CloseParenthesis => operands.push("".to_string()),
            _ => {
                return Err(AsmError::ParseError(
                    tokenizer.location(),
                    "unexpected token, string or ')' are expected".to_string(),
                    "look at the bnf".to_string(),
                ));
            }
        }
    }
    // ")"
    tokenizer.skip_space();
    tokenizer.consume_token(TokenKind::CloseParenthesis);
    // tokenizer.add_to_code(TokenKind::NewLine);
    Ok(PseudoIns::new(ins, operands, currrent_token.location))
}

fn parse_ins_operands_inside<'a, T>(
    tokenizer: Rc<T>,
    operands: &mut Vec<String>,
) -> AsmResult<'a, ()>
where
    T: Tokenizer<'a>,
{
    // <operand>
    let op = match tokenizer.peek_token().kind {
        TokenKind::Minus | TokenKind::Number(_) => {
            parse_operands::parse_immediate(tokenizer.clone())?
                .codegen()
                .clone()
        }

        TokenKind::String(i) => {
            tokenizer.next_token();
            tokenizer.skip_space();
            format!("\"{}\"", i)
        }
        _ => {
            return Err(AsmError::ParseError(
                tokenizer.location(),
                format!(
                    "invalid expression for db and resb: {:#?}",
                    tokenizer.peek_token()
                ),
                "look at the bnf".to_string(),
            ));
        }
    };
    operands.push(op);
    match tokenizer.peek_token().kind {
        TokenKind::CloseParenthesis => Ok(()),
        // ("," <operand>)*
        TokenKind::Comma => {
            // ","
            tokenizer.next_token();
            tokenizer.skip_space();

            // <operand>)*
            parse_ins_operands_inside(tokenizer, operands)
        }
        _ => Err(AsmError::ParseError(
            tokenizer.location(),
            format!(
                "invalid expression for db and resb: {:#?}",
                tokenizer.peek_token()
            ),
            "look at the bnf".to_string(),
        )),
    }
}

fn parse_extern_operands_inside<'a, T>(
    tokenizer: Rc<T>,
    operands: &mut Vec<String>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> AsmResult<'a, ()>
where
    T: Tokenizer<'a>,
{
    // <operand>
    let op = match tokenizer.peek_token().kind {
        TokenKind::Identifier(ident) => {
            scope.borrow().add_label_to_root(Ident::new(ident));
            tokenizer.next_token();
            ident.to_string()
        }
        _ => {
            return Err(AsmError::ParseError(
                tokenizer.location(),
                "invalid operands".to_string(),
                "look at the bnf".to_string(),
            ));
        }
    };
    operands.push(op);
    match tokenizer.peek_token().kind {
        TokenKind::CloseParenthesis => Ok(()),
        // ("," <operand>)*
        TokenKind::Comma => {
            // ","
            tokenizer.next_token();
            tokenizer.skip_space();

            // <operand>)*
            parse_extern_operands_inside(tokenizer, operands, scope)
        }
        _ => Err(AsmError::ParseError(
            tokenizer.location(),
            "invalid expression for extern:".to_string(),
            "look at the bnf".to_string(),
        )),
    }
}

fn parse_nasm_operands_inside<'a, T>(
    tokenizer: Rc<T>,
    operands: &mut Vec<Box<dyn Operand + 'a>>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> AsmResult<'a, ()>
where
    T: Tokenizer<'a>,
{
    // <operand>
    let op = match tokenizer.peek_token().kind {
        TokenKind::String(ident) => {
            // scope.borrow().add_label_to_root(Ident::new(ident));
            tokenizer.next_token();
            Box::new(UnimplementedOperand::new(ident))
        }
        _ => parse_operands(tokenizer.clone(), scope.clone())?,
    };
    operands.push(op);
    match tokenizer.peek_token().kind {
        TokenKind::CloseParenthesis => Ok(()),
        // ("," <operand>)*
        TokenKind::Comma => {
            // ","
            tokenizer.next_token();
            tokenizer.skip_space();

            // <operand>)*
            parse_nasm_operands_inside(tokenizer, operands, scope)
        }
        _ => Err(AsmError::ParseError(
            tokenizer.location(),
            "invalid token for raw nasm instruction".to_string(),
            "look at the bnf".to_string(),
        )),
    }
}
