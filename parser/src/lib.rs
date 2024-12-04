mod parse_block;
mod parse_code;
mod parse_indent;
mod parse_ins;
mod parse_label_def;
mod parse_null_stmt;
mod parse_stmt;

pub use parse_block::parse_block;
pub use parse_code::parse_code;
pub use parse_indent::read_indent_by_depth;
pub use parse_ins::parse_compound_ins;
pub use parse_label_def::parse_label_def;
pub use parse_null_stmt::parse_null_stmt;
pub use parse_stmt::parse_stmt;

#[test]
fn test() {
    use tokenizer::Tokenizer;
    use util::{Location, Source};
    let source = Source {
        file: "test",
        code: "< test > {\n    test()}".to_string(),
    };
    let loc = Location::new(&source);
    let t = Tokenizer::new(loc);
    eprintln!("{:#?}", parse_label_def(&t, 0));
    None.unwrap()
}
