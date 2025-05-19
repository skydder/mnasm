use std::{fmt::Debug, io::Read, rc::Rc};

use crate::{open_safely, AsmError, AsmResult};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Source<'code> {
    pub code: Rc<String>,
    pub file: &'code str,
}

impl<'code> Source<'code> {
    pub fn new_with_file(file: &'code str) -> AsmResult<'code, Source<'code>> {
        let mut code = String::new();
        open_safely(file)
            .read_to_string(&mut code)
            .map_err(|e| AsmError::IOError(e.to_string()))?;
        Ok(Self::new(code, file))
    }

    pub fn new(code: String, file: &'code str) -> Self {
        Self {
            code: Rc::new(code),
            file,
        }
    }

    pub fn nth(&self, n: usize) -> &str {
        assert!(n < self.end());
        &self.code[n..]
    }

    pub fn end(&self) -> usize {
        self.code.len()
    }

    pub fn file(&self) -> &'code str {
        self.file
    }
}
#[derive(Clone, Default)]
pub struct Location<'code> {
    source: Source<'code>,
    line: usize,
    column: usize,
    nth: usize,
}

impl std::fmt::Display for Location<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.source.file(), self.line, self.column)
    }
}

impl std::fmt::Debug for Location<'_> {
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

impl<'code> Location<'code> {
    pub fn new(source: Source<'code>) -> Self {
        Self {
            source,
            line: 1,
            column: 1,
            nth: 0,
        }
    }

    pub fn end(&self) -> Self {
        Self {
            source: self.source.clone(),
            line: 0,
            column: 0,
            nth: self.source.end(),
        }
    }

    pub fn create_location(source: Source<'code>, line: usize, column: usize, nth: usize) -> Self {
        Self {
            source,
            line,
            column,
            nth,
        }
    }

    pub fn advance_line(&self, dl: usize) -> Location<'code> {
        Self::create_location(self.source.clone(), self.line + dl, 1, self.nth)
    }

    pub fn advance_column(&self, dc: usize) -> Location<'code> {
        Self::create_location(self.source.clone(), self.line, self.column + dc, self.nth)
    }

    pub fn advance_nth(&self, dn: usize) -> Location<'code> {
        Self::create_location(self.source.clone(), self.line, self.column, self.nth + dn)
    }

    pub fn is_eos(&self) -> bool {
        self.nth >= self.source.end()
    }

    pub fn current_slice(&self) -> &str {
        self.source.nth(self.nth)
    }
}

impl std::cmp::PartialEq for Location<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && self.nth == other.nth
    }
}

impl std::cmp::PartialOrd for Location<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.nth.partial_cmp(&other.nth)
    }
}

#[derive(Clone)]
pub struct Stream<'code> {
    begin: Location<'code>,
    end: Location<'code>,
}

impl<'code> Stream<'code> {
    pub fn new(begin: Location<'code>, end: Location<'code>) -> Self {
        Self { begin, end }
    }

    pub fn begin(&self) -> Location<'code> {
        self.begin.clone()
    }

    pub fn end(&self) -> Location<'code> {
        self.end.clone()
    }

    pub fn source(&self) -> Source<'code> {
        self.begin.source.clone()
    }
}

impl Debug for Stream<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.begin, self.end)
    }
}
