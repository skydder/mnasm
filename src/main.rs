fn main() {
    use codegen::codegen_code;
    use parser::parse_code;
    use tokenizer::Tokenizer;
    use util::{Location, Source};
    use analyzer::analyze;
    // let source = Source {
    // file: "test",
    // code: "< test > {\n    test()\n}".to_string(),
    // };
    let file = parse_args();
    let source = Source::new(&file);
    let loc = Location::new(&source);
    let t = Tokenizer::new(loc);
    let ast = parse_code(&t).unwrap();
    analyze(&ast);
    eprintln!("{}", codegen_code(&ast));
}

fn parse_args<'a>() -> String {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 1 {
        todo!()
    }
    args[0].clone()
}
