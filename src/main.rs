use std::{
    fs::{self, File},
    io::Write,
    process::Command,
};

use analyzer::analyze;
use codegen::codegen_code;
use parser::parse_code;
use tokenizer::Tokenizer;
use util::{Location, Source};


// todo: use better way to create binary: not using "out.s", which might exsists in dir
fn main() {
    let file = parse_args();
    let mut out = File::create("out.s").expect("failed to create file");
    write!(&mut out, "{}", assemble(&file)).expect("failed to write file");
    // println!("{}", assemble(&file));
    nasm_and_link("out.s");
    fs::remove_file("out.s").expect("failed to clean up");
    fs::remove_file("out.o").expect("failed to clean up");
}

fn parse_args<'a>() -> String {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 1 {
        todo!()
    }
    args[0].clone()
}

fn assemble(file: &str) -> String {
    let source = Source::new(file);
    let loc = Location::new(&source);
    let t = Tokenizer::new(loc);
    let ast = parse_code(&t);
    analyze(&ast);
    codegen_code(&ast)
}

fn nasm_and_link(file: &str) {
    let nasm = Command::new("nasm")
        .arg(file)
        .arg("-f")
        .arg("elf64")
        .arg("-o")
        .arg("out.o")
        .spawn()
        .expect("do you have nasm?")
        .wait();

    let ld = Command::new("ld")
        .arg("out.o")
        .arg("-o")
        .arg("out")
        .arg("-m")
        .arg("elf_x86_64")
        .spawn()
        .expect("do you have ld?")
        .wait();
}
