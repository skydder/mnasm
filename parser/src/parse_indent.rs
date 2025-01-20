use tokenizer::TokenGenerator;

use crate::tokenizer::Tokenizer2;

pub fn read_indent_by_depth<'a>(tokenizer: &'a mut Tokenizer2, depth: usize) {
    for _ in 0..depth {
        tokenizer.consume_indent();
    }
}
