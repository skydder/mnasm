use std::{cell::RefCell, rc::Rc};

use data::{Immediate, Memory, Operand, Register, Scale, Scope};
use tokenizer::{TokenKind, Tokenizer2};
use util::emit_error;

use crate::parse_label;

// <operand> = <memory> | <register> | <immediate> | <label>
pub fn parse_operands<'a>(
    tokenizer: &'a Tokenizer2<'a>,
    scope: Rc<RefCell<Scope<'a>>>,
) -> Box<dyn Operand + 'a> {
    let loc = tokenizer.location();
    match tokenizer.peek_token(true).kind {
        TokenKind::Identifier(s) => {
            // <memory>
            if s == "ptr" {
                return Box::new(parse_memory(tokenizer));

            // <register>
            } else if let Some(reg) = parse_register(tokenizer, s) {
                return Box::new(reg);

            // <label>
            } else {
                // tokenizer.next_token();
                let label = parse_label(tokenizer, scope.clone());
                // eprintln!("{:#?}", label);
                if let Some(m) = scope.borrow().find_macro(label.ident()) {
                    tokenizer.turn_off_the_record();
                    tokenizer.enter_macro(m.ingredients_of_tokenizer(), Vec::new());
                    let op = parse_operands(tokenizer, scope.clone());
                    tokenizer.skip_space(true);
                    tokenizer.turn_on_the_record();
                    return op;
                }
                Box::new(label)
            }
        }

        // <immediate>
        TokenKind::Number(_) | TokenKind::Minus => {
            return Box::new(parse_immediate(tokenizer));
        }
        TokenKind::Dot => Box::new(parse_label(tokenizer, scope)),

        _ => {
            emit_error!(loc, "unexpected token1, {:#?}", tokenizer.peek_token(true))
        }
    }
}

// <immediate> = ("-")? <number>
pub fn parse_immediate<'a>(tokenizer: &'a Tokenizer2<'a>) -> Immediate<'a> {
    let current_token = tokenizer.peek_token(true);
    match current_token.kind {
        // <number>
        TokenKind::Number(imm) => {
            tokenizer.next_token();
            return Immediate::new(imm, false, 32, current_token.location.clone());
        }
        // "-" <number>
        TokenKind::Minus => {
            tokenizer.next_token();

            // <number>
            match tokenizer.peek_token(true).kind {
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
            std::process::exit(1);
        }
    }
}

// <register>
fn parse_register<'a>(tokenizer: &'a Tokenizer2<'a>, s: &str) -> Option<Register<'a>> {
    let loc = tokenizer.location();
    if let Some((kind, value, size)) = Register::is_reg(s) {
        tokenizer.next_token();
        return Some(Register::new(kind, value, size, loc));
    } else {
        None
    }
}

// <memory> = "ptr" "(" <base> ","  <index> "," <scale> "," <disp> ")"
fn parse_memory<'a>(tokenizer: &'a Tokenizer2<'a>) -> Memory<'a> {
    let loc = tokenizer.location();
    // "ptr"
    tokenizer.consume_token(TokenKind::Identifier("ptr"));
    tokenizer.skip_space(true);

    // size process
    // ============
    let size = if tokenizer.peek_token(true).is(TokenKind::LessThan) {
        tokenizer.next_token();
        tokenizer.skip_space(true);
        let v = match tokenizer.peek_token(true).kind {
            TokenKind::Identifier("byte") => {
                tokenizer.next_token();
                8
            }
            TokenKind::Identifier("word") => {
                tokenizer.next_token();
                16
            }
            TokenKind::Identifier("dword") => {
                tokenizer.next_token();
                32
            }
            TokenKind::Identifier("qword") => {
                tokenizer.next_token();
                64
            }
            _ => {
                emit_error!(
                    tokenizer.location(),
                    "expected size expression here, but there is not."
                );
            }
        };
        tokenizer.skip_space(true);
        tokenizer.consume_token(TokenKind::GreaterThan);
        tokenizer.skip_space(true);
        v
    } else {
        0
    };

    // "("
    tokenizer.consume_token(TokenKind::OpenParenthesis);
    tokenizer.skip_space(true);
    // <base> = "_" | <register>
    let base = match tokenizer.peek_token(true).kind {
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
    tokenizer.skip_space(true);
    tokenizer.consume_token(TokenKind::Comma);
    tokenizer.skip_space(true);

    // <index> = "_" | <register>
    let index = match tokenizer.peek_token(true).kind {
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
    tokenizer.skip_space(true);
    tokenizer.consume_token(TokenKind::Comma);
    tokenizer.skip_space(true);

    // <scale> = "1" | "2" | "4" | "8" | "_"
    let scale = match tokenizer.peek_token(true).kind {
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
    tokenizer.skip_space(true);
    tokenizer.consume_token(TokenKind::Comma);
    tokenizer.skip_space(true);

    // <disp> = "_" | <immediate>
    let disp = match tokenizer.peek_token(true).kind {
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
    tokenizer.skip_space(true);
    tokenizer.consume_token(TokenKind::CloseParenthesis);

    Memory::new((base, index, scale, disp), size, loc)
}
