use std::{cell::RefCell, io::Read};

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
        Self {
            code: code,
            file: file,
        }
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
pub struct Source2<'a> {
    source: &'a RefCell<Vec<Source<'a>>>,
    nth: usize,
}

impl<'a> Source2<'a> {
    fn new(source: &'a RefCell<Vec<Source<'a>>>, nth: usize) -> Self {
        Self {
            source: source,
            nth: nth,
        }
    }

    fn file(&self) -> &str {
        self.source.borrow()[self.nth].file
    }

    fn end(&self) -> usize {
        self.source.borrow()[self.nth].end()
    }

    fn nth(&self, nth: usize) -> &'a str {
        let l = (self.source).as_ptr();
        unsafe { &(*l.wrapping_add(0))[self.nth].code[nth..] }
    }

    fn silce(&self, begin: usize, end: usize) -> &'a str {
        let l = (self.source).as_ptr();
        unsafe { &(*l.wrapping_add(0))[self.nth].code[begin..end] }
    }

    pub fn add_source(&self, source: Source<'a>) {
        self.source.borrow_mut().push(source);
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
        write!(
            f,
            "{}:{}:{}({})",
            self.source.file(),
            self.line,
            self.column,
            self.nth
        )
    }
}

impl<'a> Location<'a> {
    pub fn new(source: &'a RefCell<Vec<Source<'a>>>) -> Self {
        Self {
            source: Source2::new(source, 0),
            line: 1,
            column: 1,
            nth: 0,
        }
    }

    pub fn new_source(original_sources: Source2<'a>, new_source: Source<'a>) -> Self {
        original_sources.add_source(new_source);
        Self {
            source: Source2::new(
                original_sources.source,
                original_sources.source.borrow().len() - 1,
            ),
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

#[derive(Debug, Clone, Copy)]
pub struct Stream<'a> {
    begin: Location<'a>,
    end: Location<'a>,
}

impl<'a> Stream<'a> {
    pub fn new(begin: Location<'a>, end: Location<'a>) -> Self {
        Self {
            begin: begin,
            end: end,
        }
    }

    pub fn begin(&self) -> Location<'a> {
        self.begin
    }

    pub fn end(&self) -> Location<'a> {
        self.end
    }

    pub fn stringfiy(&self) -> &'a str {
        self.begin.source.silce(self.begin.nth, self.end.nth)
    }

    pub fn source(&self) -> Source2<'a> {
        self.begin.source
    }
}
