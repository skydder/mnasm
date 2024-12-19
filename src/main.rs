use std::{
    fs::{self, File}, io::{self, Write}, path::Path, process::Command, result::Result
};
use tempfile::NamedTempFile;

use analyzer::analyze;
use codegen::codegen_code;
use parser::parse_code;
use tokenizer::Tokenizer;
use util::{emit_msg_and_exit, Location, Source};


fn main() {
    let file = parse_args();
    nasm_and_link(assemble(&file), "out").unwrap_or_else(|e| eprintln!("{}", e));
    // run().unwrap_or_else(|e| eprintln!("{}", e));
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

fn assemble_by_nasm(nasm_file: &Path, out_file: &Path) -> Result<(), io::Error> {
    Command::new("nasm")
        .arg(format!("{}", nasm_file.display()))
        .arg("-f")
        .arg("elf64")
        .arg("-o")
        .arg(format!("{}", out_file.display()))
        .spawn()
        .expect("do you have nasm?")
        .wait()?;
    Ok(())
}

fn link(obj_file: &Path, out_file: &Path) -> Result<(), io::Error> {
    Command::new("ld")
        .arg(format!("{}", obj_file.display()))
        .arg("-o")
        .arg(format!("{}", out_file.display()))
        .arg("-m")
        .arg("elf_x86_64")
        .spawn()
        .expect("do you have ld?")
        .wait()?;
    
    Ok(())
}

fn nasm_and_link(code: String, file: &str) -> Result<(), io::Error> {
    let nasm_code = NamedTempFile::new()?;

    let mut out = File::create(nasm_code.path())?;
    write!(&mut out, "{}", code).expect("failed to write file");

    let obj_file = NamedTempFile::new()?;
    assemble_by_nasm(nasm_code.path(), obj_file.path())?;
    link(obj_file.path(), Path::new(file))?;
    
    Ok(())
}

#[derive(Debug)]
struct RunFlags {
    input: String,
    output: String,
    is_c: bool,     // -c flag
    is_cs: bool,    // -S flag
}

impl RunFlags {
    fn default() -> Self {
        Self { input: String::new(), output: String::new(), is_c: false, is_cs: false }
    }
}

fn help() -> ! {
    emit_msg_and_exit!("invalid flag usage\n")
}

fn parse_run_flags<'a>(args: Vec<String>) -> RunFlags {
    let mut arg_iter = args.iter();
    let mut arg: Option<&String>; 
    let mut arg_s: &String; 
    let mut flags = RunFlags::default();
    loop {
        arg = arg_iter.next();
        if arg.is_none() {
            break;
        }
        arg_s = arg.unwrap();
        eprintln!("{}", arg_s);
        if arg_s == "-S" {
            flags.is_cs = true;
            continue;
        }
        if arg_s == "-c" {
            flags.is_c = true;
            continue;
        }
        if arg_s == "-o" {
            // arg_iter.next();
            flags.output = arg_iter.next().unwrap_or_else(|| help()).clone();
            continue;
        }
        flags.input = arg_s.clone();
    }
    if flags.input.len() == 0 {
        help()
    }
    eprintln!("{:#?}", flags);
    flags
}

fn run() -> Result<(), io::Error> {
    let flag = parse_run_flags(std::env::args().skip(1).collect());
    let nasm_code = NamedTempFile::new()?;

    let mut nasm_out = File::create(nasm_code.path())?;
    write!(&mut nasm_out, "{}", assemble(&flag.input)).expect("failed to write file");
    if flag.is_c {
        fs::copy(nasm_code.path(), if flag.output.len() != 0 {
            flag.output
        } else {
            "out.s".to_string()
        })?;
        return Ok(());
    }
    let obj_file = NamedTempFile::new()?;
    assemble_by_nasm(nasm_code.path(), obj_file.path())?;
    if flag.is_c {
        fs::copy(obj_file.path(), if flag.output.len() != 0 {
            flag.output
        } else {
            "out.o".to_string()
        })?;
        return Ok(());
    }
    let exc = NamedTempFile::new()?;
    link(obj_file.path(), exc.path())?;
    fs::copy(obj_file.path(), if flag.output.len() != 0 {
        flag.output
    } else {
        "out".to_string()
    })?;
    Ok(())
}
