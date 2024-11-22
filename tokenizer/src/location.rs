use std::io::Read;

use util::open_safely;

pub struct Source<'a> {
    pub code: String,
    pub file: &'a str,
}

impl<'a> Source<'a> {
    pub fn new(file: &'a str) -> Self {
        let mut code = String::new();
        open_safely(file).read_to_string(&mut code).unwrap_or_else(|_| {
            eprintln!("failed to load '{}' into 'String'", file);
            ::std::process::exit(1);
        });
        Self { code: code, file: file }
    }

    pub fn nth(&self, n: usize) -> &str {
        &self.code[n..]
    }
}

#[derive(Clone, Copy)]
pub struct Location<'a> {
    source: &'a Source<'a>,
    line: usize,
    column: usize,
    nth: usize
}

impl<'a> std::fmt::Display for Location<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.source.file, self.line, self.column)
    }
}

impl<'a> std::fmt::Debug for Location<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.source.file, self.line, self.column)
    }
}


impl<'a> Location<'a> {
    pub fn new(source: &'a Source<'a>) -> Self {
        Self { source: source, line: 1, column: 1, nth: 0 }
    }

    pub(crate) fn create_location(source: &'a Source<'a>, line: usize, column: usize, nth: usize) -> Self {
        Self { source: source, line: line, column: column, nth: nth }
    }

    pub(crate) fn advance_line(&self, dl: usize) -> Location<'a> {
        Self::create_location(self.source, self.line + dl, self.column, self.nth)
    }

    pub(crate) fn advance_column(&self, dc: usize) -> Location<'a> {
        Self::create_location(self.source, self.line , self.column + dc, self.nth)
    }

    pub(crate) fn advance_nth(&self, dn: usize) -> Location<'a>  {
        Self::create_location(self.source, self.line, self.column, self.nth + dn)
    }
    pub(crate) fn current_slice(&self) -> &'a str {
        self.source.nth(self.nth)
    }
}