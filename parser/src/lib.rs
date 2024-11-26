mod block;
mod indent;
mod label_def;
mod stmt;

pub use block::{parse_block, Block};
pub use indent::read_indent_by_depth;
pub use stmt::{parse_stmt, Stmt};
pub use label_def::{LabelDef, parse_label_def};
use util::emit_msg_and_exit;

#[test]
fn test() {
    use util::{Source, Location};
    use tokenizer::Tokenizer;
    let source = Source {
        file: "test",
        code: "< test > {\n    test()}".to_string(),
    };
    let loc = Location::new(&source);
    let t = Tokenizer::new(loc);
    eprintln!("{:#?}", parse_label_def(&t));
    None.unwrap()
}
