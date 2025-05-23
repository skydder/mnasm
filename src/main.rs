use analyzer::analyze_code;
use codegen::{codegen_code, pretty_print_code};
// use data::{Ident, Scope};
use std::{
    fs::{self, File}, io::{Write}, path::Path, process::Command, rc::Rc, result::Result
};
use tempfile::NamedTempFile;

use parser::{expand_code, parse_code};
use tokenizer::Tokenizer;
use util::{convert_to_asmerror, emit_msg_and_exit, set_iw, AsmError, AsmResult, Location, Source};

fn main() {
    unsafe { backtrace_on_stack_overflow::enable() };
    let flag = parse_run_flags(std::env::args().skip(1).collect());
    run(&flag).unwrap_or_else(|e| eprintln!("{}", e));
}

fn assemble(file: &str, is_only_macroexpantion: bool) -> AsmResult<'_, String> {
    let source = Source::new_with_file(file)?;
    let loc = Location::new(source);
    let tokenizer = Rc::new(Tokenizer::new(loc.clone(), loc.end()));
    let asts = parse_code(tokenizer)?;
    let expanded = expand_code(asts)?;
    if is_only_macroexpantion {
        println!("{}", pretty_print_code(&expanded));
    }
    let root = analyze_code(&expanded)?;
    codegen_code(&expanded, root)
}

fn assemble_by_nasm<'code>(nasm_file: &Path, out_file: &Path) ->  AsmResult<'code, ()> {
    convert_to_asmerror(Command::new("nasm")
        .arg(format!("{}", nasm_file.display()))
        .arg("-f")
        .arg("elf64")
        .arg("-o")
        .arg(format!("{}", out_file.display()))
        .spawn()
        .expect("do you have nasm?")
        .wait())?;
    Ok(())
}

fn link<'code>(obj_file: &Path, out_file: &Path) -> AsmResult<'code, ()> {
    convert_to_asmerror(Command::new("ld")
        .arg(format!("{}", obj_file.display()))
        .arg("-o")
        .arg(format!("{}", out_file.display()))
        .arg("-m")
        .arg("elf_x86_64")
        .spawn()
        .expect("do you have ld?")
        .wait())?;

    Ok(())
}

#[derive(Debug)]
struct RunFlags {
    input: String,
    output: String,
    is_c: bool,  // -c flag
    is_cs: bool, // -S flag
    is_e: bool,
}

impl RunFlags {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            is_c: false,
            is_cs: false,
            is_e: false,
        }
    }
}

fn help() -> ! {
    emit_msg_and_exit!("mnasm [ -o <path> || -c || -S ] <file>\n")
}

fn parse_run_flags(args: Vec<String>) -> RunFlags {
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

        if arg_s == "-S" {
            flags.is_cs = true;
            continue;
        }
        if arg_s == "-c" {
            flags.is_c = true;
            continue;
        }
        if arg_s == "-e" {
            flags.is_e = true;
            continue;
        }
        if arg_s == "-o" {
            flags.output = arg_iter
                .next()
                .unwrap_or_else(|| {
                    eprintln!("what?");
                    help()
                })
                .clone();
            continue;
        }
        if arg_s == "-iw" {
            set_iw();
            continue;
        }
        if !flags.input.is_empty() {
            help()
        }
        flags.input = arg_s.clone();
    }
    if flags.input.is_empty() {
        help()
    }
    // eprintln!("{:#?}", flags);
    flags
}

fn run(flag: &RunFlags) -> Result<(), AsmError<'_>> {
    let nasm_code = convert_to_asmerror(NamedTempFile::new())?;
    let obj_file = convert_to_asmerror(NamedTempFile::new())?;
    let exc = convert_to_asmerror(NamedTempFile::new())?;


    convert_to_asmerror(write!(
        &mut convert_to_asmerror(File::create(nasm_code.path()))?,
        "{}",
        assemble(&flag.input, flag.is_e)?
    ))?;

    if flag.is_cs {
        convert_to_asmerror(fs::copy(
            nasm_code.path(),
            if !flag.output.is_empty() {
                &flag.output
            } else {
                "out.s"
            },
        ))?;
        Ok(())
    } else {

        assemble_by_nasm(nasm_code.path(), obj_file.path())?;

        if flag.is_c {
            convert_to_asmerror(fs::copy(
                obj_file.path(),
                if !flag.output.is_empty() {
                    &flag.output
                } else {
                    "out.o"
                },
            ))?;
            Ok(())
        } else {
            link(obj_file.path(), exc.path())?;

            convert_to_asmerror(fs::copy(
                exc.path(),
                if !flag.output.is_empty() {
                    &flag.output
                } else {
                    "a.out"
                },
            ))?;
            Ok(())
        }
    }
}
