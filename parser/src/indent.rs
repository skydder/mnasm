use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

pub fn read_indent_by_depth(tokenizer: &Tokenizer, depth: usize) {
    for _ in 0..depth {
        tokenizer.expect_indent();
    }
}
