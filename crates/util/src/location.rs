use std::io::Read;

use crate::{emit_msg_and_exit, open_safely};

#[derive(Debug, PartialEq)]
pub struct Source<'a> {
    pub code: String,
    pub file: &'a str,
}

impl<'a> Source<'a> {
    pub fn new_with_file(file: &'a str) -> Self {
        let mut code = String::new();
        open_safely(file)
            .read_to_string(&mut code)
            .unwrap_or_else(|_| {
                emit_msg_and_exit!("failed to load '{}' into 'String'\n", file);
            });
        Self {
            code: code,
            file: file,
        }
    }
    
    pub fn new(code: String, file: &'a str) -> Self {
        Self { code: code, file: file }
    }

    pub fn nth(&self, n: usize) -> &str {
        assert!(n < self.end());
        &self.code[n..]
    }

    pub fn end(&self) -> usize {
        self.code.len()
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Source2<'a> {
    Source(&'a Source<'a>),
    BuiltIn(&'a str),
}

impl<'a> Source2<'a> {
    fn file(&self) -> &str {
        match self {
            Source2::Source(source) => source.file,
            Source2::BuiltIn(_) => "builtin",
        }
    }

    fn end(&self) -> usize {
        match self {
            Source2::Source(source) => source.end(),
            Source2::BuiltIn(s) => s.len(),
        }
    }

    fn nth(&self, n: usize) -> &'a str {
        match self {
            Source2::Source(source) => source.nth(n),
            Source2::BuiltIn(s) => {
                assert!(n < self.end());
                &s[n..]
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Location<'a> {
    source: Source2<'a>,
    line: usize,
    column: usize,
    nth: usize,
}

impl<'a> std::fmt::Display for Location<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.source.file(), self.line, self.column)
    }
}

impl<'a> std::fmt::Debug for Location<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.source.file(), self.line, self.column)
    }
}

impl<'a> Location<'a> {
    pub fn new(source: &'a Source<'a>) -> Self {
        Self {
            source: Source2::Source(source),
            line: 1,
            column: 1,
            nth: 0,
        }
    }

    pub fn new_builtin(source: &'a str) -> Self {
        Self {
            source: Source2::BuiltIn(source),
            line: 1,
            column: 1,
            nth: 0,
        }
    }

    pub fn end(&self) -> Self {
        Self {
            source: self.source,
            line: 0,
            column: 0,
            nth: self.source.end(),
        }
    }

    pub fn create_location(source: Source2<'a>, line: usize, column: usize, nth: usize) -> Self {
        Self {
            source: source,
            line: line,
            column: column,
            nth: nth,
        }
    }

    pub fn advance_line(&self, dl: usize) -> Location<'a> {
        Self::create_location(self.source, self.line + dl, 1, self.nth)
    }

    pub fn advance_column(&self, dc: usize) -> Location<'a> {
        Self::create_location(self.source, self.line, self.column + dc, self.nth)
    }

    pub fn advance_nth(&self, dn: usize) -> Location<'a> {
        Self::create_location(self.source, self.line, self.column, self.nth + dn)
    }
    pub fn current_slice(&self) -> &'a str {
        if self.is_eos() {
            eprintln!("eror: {:#?}", self);
        }
        self.source.nth(self.nth)
    }

    pub fn is_eos(&self) -> bool {
        self.nth >= self.source.end()
    }
}

impl<'a> std::cmp::PartialEq for Location<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && self.nth == other.nth
    }
}

impl<'a> std::cmp::PartialOrd for Location<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.nth.partial_cmp(&other.nth)
    }
}
