use data::{Immediate, Label, Memory, Operand, Register, Scale};
use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

pub fn parse_operands<'a>(tokenizer: &'a Tokenizer<'a>) -> Box<dyn Operand + 'a> {
    let loc = tokenizer.location();
    match tokenizer.peek_token().kind {
        TokenKind::Identifier(s) => {
            if s == "ptr" {
                return Box::new(parse_memory(tokenizer));
            } else if let Some(reg) = parse_register(tokenizer, s) {
                return Box::new(reg);
            } else {
                tokenizer.next_token();
                return Box::new(Label::new(s, loc));
                // emit_error!(tokenizer.location(), "unexpected token")
            }
        }
        TokenKind::Number(_) | TokenKind::Minus => {
            return Box::new(parse_immediate(tokenizer));
        }
        _ => {
            emit_error!(tokenizer.location(), "unexpected token")
        }
    }
}

fn parse_immediate<'a>(tokenizer: &'a Tokenizer<'a>) -> Immediate<'a> {
    let current_token = tokenizer.peek_token();
    match current_token.kind {
        TokenKind::Number(imm) => {
            tokenizer.next_token();
            return Immediate::new(imm, false, 32, current_token.location);
        }
        TokenKind::Minus => {
            tokenizer.next_token();
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
            std::process::exit(1);
        }
    }
}

fn parse_register<'a>(tokenizer: &'a Tokenizer<'a>, s: &str) -> Option<Register<'a>> {
    let loc = tokenizer.location();
    if let Some((kind, value, size)) = Register::is_reg(s) {
        tokenizer.next_token();
        return Some(Register::new(kind, value, size, loc));
    } else {
        None
    }
}

fn parse_memory<'a>(tokenizer: &'a Tokenizer<'a>) -> Memory<'a> {
    let loc = tokenizer.location();
    tokenizer.expect_token(TokenKind::Identifier("ptr"));
    // size process
    // ============
    tokenizer.expect_symbol(TokenKind::OpenParenthesis);
    tokenizer.skip_space();
    // base
    let base = match tokenizer.peek_symbol().kind {
        TokenKind::Identifier("_") => {
            tokenizer.next_token();
            None
        },
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
    tokenizer.expect_symbol(TokenKind::Comma);
    // index
    let index = match tokenizer.peek_symbol().kind {
        TokenKind::Identifier("_") => {
            tokenizer.next_token();
            None
        },
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
    tokenizer.expect_symbol(TokenKind::Comma);

    // scale
    let scale = match tokenizer.peek_symbol().kind {
        TokenKind::Identifier("_") => {
            tokenizer.next_token();
            None
        },
        TokenKind::Number(1) => {
            tokenizer.next_token();
            Some(Scale::S1)
        },
        TokenKind::Number(2) => {
            tokenizer.next_token();
            Some(Scale::S2)
        },
        TokenKind::Number(4) => {
            tokenizer.next_token();
            Some(Scale::S4)
        },
        TokenKind::Number(8) => {
            tokenizer.next_token();
            Some(Scale::S8)
        },
        _ => {
            todo!()
        }
    };
    tokenizer.expect_symbol(TokenKind::Comma);
    // disp
    let disp = match tokenizer.peek_symbol().kind {
        TokenKind::Identifier("_") => {
            tokenizer.next_token();
            None
        },
        TokenKind::Number(_) | TokenKind::Minus => Some(parse_immediate(tokenizer)),
        _ => {
            todo!()
        }
    };
    tokenizer.expect_symbol(TokenKind::CloseParenthesis);
    Memory::new((base, index, scale, disp),0, loc)
}
