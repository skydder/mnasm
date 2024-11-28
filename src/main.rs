
fn main() {
    use util::{Source, Location};
    use tokenizer::Tokenizer;
    use parser::parse_label_def;
    use codegen::codegen_label_def;
    // let source = Source {
        // file: "test",
        // code: "< test > {\n    test()\n}".to_string(),
    // };
    let file = parse_args();
    let source = Source::new(&file);
    let loc = Location::new(&source);
    let t = Tokenizer::new(loc);
    let ast =  parse_label_def(&t);
    eprintln!("{}", codegen_label_def(&ast.unwrap()));
}


fn parse_args<'a>() -> String {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 1 {
        todo!()
    }
    args[0].clone()
}