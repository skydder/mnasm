use std::{
    cell::RefCell, fs::{self, File}, io::{self, Write}, path::Path, process::Command, result::Result
};
use tempfile::NamedTempFile;

use analyzer::analyze;
use codegen::codegen_code;
use parser::parse_code;
use tokenizer::Tokenizer2;
use util::{emit_msg_and_exit, set_iw, Location, Source};

fn main() {
    unsafe { backtrace_on_stack_overflow::enable() };
    run().unwrap_or_else(|e| eprintln!("{}", e));
}

fn assemble(file: &str, flag: &RunFlags) -> String {
    let source = Source::new_with_file(file);
    let source = vec![source];
    let source= RefCell::new(source);
    let loc = Location::new(&source);
    let t = Tokenizer2::new_tokenizer(loc);
    let ast = parse_code(&t).unwrap_or_else(|err| emit_msg_and_exit(format!("{}", err)));
    if flag.is_e {
        println!("{}", t.code());
    }
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
        if flags.input.len() != 0 {
            help()
        }
        flags.input = arg_s.clone();
    }
    if flags.input.len() == 0 {
        help()
    }
    // eprintln!("{:#?}", flags);
    flags
}

fn run() -> Result<(), io::Error> {
    let flag = parse_run_flags(std::env::args().skip(1).collect());
    let nasm_code = NamedTempFile::new()?;

    write!(
        &mut File::create(nasm_code.path())?,
        "{}",
        assemble(&flag.input, &flag)
    )
    .expect("failed to write file");
    if flag.is_cs {
        fs::copy(
            nasm_code.path(),
            if flag.output.len() != 0 {
                flag.output
            } else {
                "out.s".to_string()
            },
        )?;
        return Ok(());
    }

    let obj_file = NamedTempFile::new()?;
    assemble_by_nasm(nasm_code.path(), obj_file.path())?;

    if flag.is_c {
        fs::copy(
            obj_file.path(),
            if flag.output.len() != 0 {
                flag.output
            } else {
                "out.o".to_string()
            },
        )?;
        return Ok(());
    }

    let exc = NamedTempFile::new()?;
    link(obj_file.path(), exc.path())?;

    fs::copy(
        exc.path(),
        if flag.output.len() != 0 {
            flag.output
        } else {
            "a.out".to_string()
        },
    )?;
    Ok(())
}
