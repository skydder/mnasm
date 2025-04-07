use std::rc::Rc;

use util::Tokenizer;

pub fn read_indent_by_depth<'a, T>(tokenizer: Rc<T>, depth: usize)
where
    T: Tokenizer<'a>,
{
    for _ in 0..depth {
        tokenizer.consume_indent();
    }
}
