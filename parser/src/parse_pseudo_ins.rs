use std::{cell::RefCell, rc::Rc};

use data::{Ident, Operand, PseudoIns, Scope};
use tokenizer::{TokenKind, Tokenizer2};
use util::emit_error;

use crate::parse_operands;

pub fn parse_pseudo_ins<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> PseudoIns<'a> {
    let currrent_token = tokenizer.peek_token(true);

    // if currrent_token.is(TokenKind::Not) {
    //     tokenizer.next_token();
    //     let ins = currrent_token.get_identifier().unwrap();
    //     tokenizer.next_token();
    //     tokenizer.skip_space(true);

    //     // "("
    //     tokenizer.consume_token(TokenKind::OpenParenthesis);
    //     tokenizer.skip_space(true);
    //     let mut operands: Vec<String> = Vec::new();


    //     tokenizer.skip_space(true);
    //     tokenizer.consume_token(TokenKind::CloseParenthesis);
    //     // tokenizer.add_to_code(TokenKind::NewLine);
    //     return PseudoIns::new(ins, operands, currrent_token.location)
    // }

    assert!(currrent_token.is_identifier());

    // <instruction>
    let ins = currrent_token.get_identifier().unwrap();
    tokenizer.next_token();
    tokenizer.skip_space(true);

    // "("
    tokenizer.consume_token(TokenKind::OpenParenthesis);
    tokenizer.skip_space(true);
    let mut operands: Vec<String> = Vec::new();
    if ins == "extern" || ins == "include" {
        if tokenizer.peek_token(true).is(TokenKind::CloseParenthesis) {
            emit_error!(tokenizer.location(), "expected label");
        }
        parse_extern_operands_inside(tokenizer, &mut operands, scope);
    } else if ins == "db" || ins == "resb" {
        // <operands>?
        if !tokenizer.peek_token(true).is(TokenKind::CloseParenthesis) {
            parse_ins_operands_inside(tokenizer, &mut operands);
        }
    } else if ins == "nasm"  {
        match tokenizer.peek_token(true).kind {
            TokenKind::String(s) => {
                operands.push(s.to_string());
                tokenizer.next_token();
            },
            TokenKind::CloseParenthesis => {
                operands.push("".to_string())
            },
            _ => emit_error!(tokenizer.location(), "unexpected token, string or ')' are expected")
        }
    }
    // ")"
    tokenizer.skip_space(true);
    tokenizer.consume_token(TokenKind::CloseParenthesis);
    // tokenizer.add_to_code(TokenKind::NewLine);
    PseudoIns::new(ins, operands, currrent_token.location)
}

fn parse_ins_operands_inside<'a>(tokenizer: &'a Tokenizer2<'a>, operands: &mut Vec<String>) {
    // <operand>
    let op = match tokenizer.peek_token(true).kind {
        TokenKind::Minus | TokenKind::Number(_) => {
            parse_operands::parse_immediate(tokenizer).codegen().clone()
        }

        TokenKind::String(i) => {
            tokenizer.next_token();
            tokenizer.skip_space(true);
            format!("\"{}\"", i)
        }
        _ => {
            emit_error!(
                tokenizer.location(),
                "invalid expression, {:#?}",
                tokenizer.peek_token(true)
            );
        }
    };
    operands.push(op);
    match tokenizer.peek_token(true).kind {
        TokenKind::CloseParenthesis => {
            return;
        }
        // ("," <operand>)*
        TokenKind::Comma => {
            // ","
            tokenizer.next_token();
            tokenizer.skip_space(true);

            // <operand>)*
            parse_ins_operands_inside(tokenizer, operands);
        }
        _ => {
            emit_error!(
                tokenizer.location(),
                "invalid expression, is end?, {:#?}",
                tokenizer.peek_token(true)
            );
        }
    }
}

fn parse_extern_operands_inside<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    operands: &mut Vec<String>,
    scope: Rc<RefCell<Scope<'a>>>,
) {
    // <operand>
    let op = match tokenizer.peek_token(true).kind {
        TokenKind::Identifier(ident) => {
            scope.borrow().add_label_to_root(Ident::new(ident));
            tokenizer.next_token();
            ident.to_string()
        }
        _ => {
            emit_error!(
                tokenizer.location(),
                "invalid expression, {:#?}",
                tokenizer.peek_token(true)
            );
        }
    };
    operands.push(op);
    match tokenizer.peek_token(true).kind {
        TokenKind::CloseParenthesis => {
            return;
        }
        // ("," <operand>)*
        TokenKind::Comma => {
            // ","
            tokenizer.next_token();
            tokenizer.skip_space(true);

            // <operand>)*
            parse_extern_operands_inside(tokenizer, operands, scope);
        }
        _ => {
            emit_error!(
                tokenizer.location(),
                "invalid expression, is end?, {:#?}",
                tokenizer.peek_token(true)
            );
        }
    }
}

// fn parse_nasm_operands_inside<'a>(
//     tokenizer: &'a Tokenizer2<'a>,
//     operands: &mut Vec<String>,
//     scope: Rc<RefCell<Scope<'a>>>,
// ) {
//     // <operand>
//     let op = match tokenizer.peek_token(true).kind {
//         TokenKind::Identifier(ident) => {
//             scope.borrow().add_label_to_root(Ident::new(ident));
//             tokenizer.next_token();
//             ident.to_string()
//         }
//         _ => {
//             emit_error!(
//                 tokenizer.location(),
//                 "invalid expression, {:#?}",
//                 tokenizer.peek_token(true)
//             );
//         }
//     };
//     operands.push(op);
//     match tokenizer.peek_token(true).kind {
//         TokenKind::CloseParenthesis => {
//             return;
//         }
//         // ("," <operand>)*
//         TokenKind::Comma => {
//             // ","
//             tokenizer.next_token();
//             tokenizer.skip_space(true);

//             // <operand>)*
//             parse_extern_operands_inside(tokenizer, operands, scope);
//         }
//         _ => {
//             emit_error!(
//                 tokenizer.location(),
//                 "invalid expression, is end?, {:#?}",
//                 tokenizer.peek_token(true)
//             );
//         }
//     }
// }
