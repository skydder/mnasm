use tokenizer::TokenGenerator;

pub fn read_indent_by_depth<'a>(tokenizer: &'a Box<dyn TokenGenerator + 'a>, depth: usize) {
    for _ in 0..depth {
        tokenizer.consume_indent();
    }
}
