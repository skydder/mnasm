use crate::tokenizer::Tokenizer2;

pub fn read_indent_by_depth<'a>(tokenizer: &'a Tokenizer2<'a>, depth: usize) {
    for _ in 0..depth {
        tokenizer.consume_indent();
    }
}
