use std::{cell::RefCell, rc::Rc};

use data::{Ident, Immediate, Label, Memory, Operand, Register, Scale, Scope};
use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

// <operand> = <memory> | <register> | <immediate> | <label>
pub fn parse_operands<'a>(
    tokenizer: &'a Tokenizer<'a>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> Box<dyn Operand + 'a> {
    let loc = tokenizer.location();
    match tokenizer.peek_token().kind {
        TokenKind::Identifier(s) => {
            // <memory>
            if s == "ptr" {
                return Box::new(parse_memory(tokenizer));

            // <register>
            } else if let Some(reg) = parse_register(tokenizer, s) {
                return Box::new(reg);

            // <label>
            } else {
                tokenizer.next_token();
                return Box::new(Label::new(
                    Ident::new(s),
                    scope
                        .borrow()
                        .find_label(Ident::new(s))
                        .unwrap_or_else(|| emit_error!(loc, "undefined label")),
                    loc,
                ));
            }
        }

        // <immediate>
        TokenKind::Number(_) | TokenKind::Minus => {
            return Box::new(parse_immediate(tokenizer));
        }

        _ => {
            emit_error!(tokenizer.location(), "unexpected token")
        }
    }
}

// <immediate> = ("-")? <number>
pub fn parse_immediate<'a>(tokenizer: &'a Tokenizer<'a>) -> Immediate<'a> {
    let current_token = tokenizer.peek_token();
    match current_token.kind {
        // <number>
        TokenKind::Number(imm) => {
            tokenizer.next_token();
            return Immediate::new(imm, false, 32, current_token.location);
        }
        // "-" <number>
        TokenKind::Minus => {
            tokenizer.next_token();

            // <number>
            match tokenizer.peek_token().kind {
                TokenKind::Number(imm) => {
                    tokenizer.next_token();
                    return Immediate::new(imm, true, 32, current_token.location);
                }
                _ => {
                    emit_error!(
                        current_token.location,
                        "unexpected token, only number can come right after a minus"
                    );
                }
            }
        }
        _ => {
            // never happends
            assert!(false);
            eprintln!("test");
            std::process::exit(1);
        }
    }
}

// <register>
fn parse_register<'a>(tokenizer: &'a Tokenizer<'a>, s: &str) -> Option<Register<'a>> {
    let loc = tokenizer.location();
    if let Some((kind, value, size)) = Register::is_reg(s) {
        tokenizer.next_token();
        return Some(Register::new(kind, value, size, loc));
    } else {
        None
    }
}

// <memory> = "ptr" "(" <base> ","  <index> "," <scale> "," <disp> ")"
fn parse_memory<'a>(tokenizer: &'a Tokenizer<'a>) -> Memory<'a> {
    let loc = tokenizer.location();
    // "ptr"
    tokenizer.expect_token(TokenKind::Identifier("ptr"));

    // size process
    // ============

    // "("
    tokenizer.expect_symbol(TokenKind::OpenParenthesis);
    tokenizer.skip_space();
    // <base> = "_" | <register>
    let base = match tokenizer.peek_symbol().kind {
        // "_"
        TokenKind::Identifier("_") => {
            tokenizer.next_token();
            None
        }
        // <register>
        TokenKind::Identifier(s) => {
            if let Some(b) = parse_register(tokenizer, s) {
                Some(b)
            } else {
                todo!()
            }
        }
        _ => {
            todo!()
        }
    };

    // ","
    tokenizer.expect_symbol(TokenKind::Comma);

    // <index> = "_" | <register>
    let index = match tokenizer.peek_symbol().kind {
        // "_"
        TokenKind::Identifier("_") => {
            tokenizer.next_token();
            None
        }

        // <register>
        TokenKind::Identifier(s) => {
            if let Some(b) = parse_register(tokenizer, s) {
                Some(b)
            } else {
                todo!()
            }
        }
        _ => {
            todo!()
        }
    };

    // ","
    tokenizer.expect_symbol(TokenKind::Comma);

    // <scale> = "1" | "2" | "4" | "8" | "_"
    let scale = match tokenizer.peek_symbol().kind {
        // "_"
        TokenKind::Identifier("_") => {
            tokenizer.next_token();
            None
        }

        // "1"
        TokenKind::Number(1) => {
            tokenizer.next_token();
            Some(Scale::S1)
        }

        // "2"
        TokenKind::Number(2) => {
            tokenizer.next_token();
            Some(Scale::S2)
        }

        // "4"
        TokenKind::Number(4) => {
            tokenizer.next_token();
            Some(Scale::S4)
        }

        // "8"
        TokenKind::Number(8) => {
            tokenizer.next_token();
            Some(Scale::S8)
        }

        _ => {
            todo!()
        }
    };
    // ","
    tokenizer.expect_symbol(TokenKind::Comma);

    // <disp> = "_" | <immediate>
    let disp = match tokenizer.peek_symbol().kind {
        // "_"
        TokenKind::Identifier("_") => {
            tokenizer.next_token();
            None
        }

        // <immediate>
        TokenKind::Number(_) | TokenKind::Minus => Some(parse_immediate(tokenizer)),
        _ => {
            todo!()
        }
    };

    // ")"
    tokenizer.expect_symbol(TokenKind::CloseParenthesis);

    Memory::new((base, index, scale, disp), 0, loc)
}
