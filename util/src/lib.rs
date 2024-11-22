use std::fs::File;

pub fn open_safely(file: &str) -> File {
    File::open(file).unwrap_or_else(|_| {
        eprintln!("failed to open '{}'", file);
        ::std::process::exit(1);
    })
}