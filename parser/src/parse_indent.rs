use tokenizer::{TokenGenerator, Tokenizer};

pub fn read_indent_by_depth(tokenizer: &Tokenizer, depth: usize) {
    for _ in 0..depth {
        tokenizer.consume_indent();
    }
}
