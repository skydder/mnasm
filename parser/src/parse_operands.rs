use data::{Operand, Register};
use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

pub fn parse_operands<'a>(tokenizer: &'a Tokenizer<'a>) -> Box<dyn Operand + 'a> {
    let loc = tokenizer.location();
    match tokenizer.peek_token().kind {
        TokenKind::Identifier(s) => {
            if let Some((kind, value, size)) = Register::is_reg(s) {
                tokenizer.next_token();
                return Box::new(Register::new(kind, value, size, loc));
            } else {
                emit_error!(tokenizer.location(), "unexpected token")
            }
        }
        _ => {
            emit_error!(tokenizer.location(), "unexpected token")
        }
    }
}
