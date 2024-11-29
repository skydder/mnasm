fn main() {
    use codegen::codegen_code;
    use parser::parse_code;
    use tokenizer::Tokenizer;
    use util::{Location, Source};
    // let source = Source {
    // file: "test",
    // code: "< test > {\n    test()\n}".to_string(),
    // };
    let file = parse_args();
    let source = Source::new(&file);
    let loc = Location::new(&source);
    let t = Tokenizer::new(loc);
    let ast = parse_code(&t);
    eprintln!("{}", codegen_code(&ast.unwrap()));
}

fn parse_args<'a>() -> String {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 1 {
        todo!()
    }
    args[0].clone()
}
