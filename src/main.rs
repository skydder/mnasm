fn main() {
    use util::{Source, Location};
    use tokenizer::Tokenizer;
    use parser::parse_label_def;
    let source = Source {
        file: "test",
        code: "< test > {\n    test()\n}".to_string(),
    };

    let loc = Location::new(&source);
    let t = Tokenizer::new(loc);
    eprintln!("{:#?}", parse_label_def(&t));
}
