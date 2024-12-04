mod block;
mod code;
mod indent;
mod ins;
mod label_def;
mod null_stmt;
mod stmt;

pub use block::parse_block;
pub use code::parse_code;
pub use indent::read_indent_by_depth;
pub use ins::parse_compound_ins;
pub use label_def::parse_label_def;
pub use null_stmt::parse_null_stmt;
pub use stmt::parse_stmt;

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
