use util::Location;

use crate::{CompoundIns, IsStmt};

#[derive(Debug)]
pub struct Stmt {
    // pub instruction: &'a str,
    // pub operand: (),
    // pub location: Location<'a>,
    pub line: Box<dyn IsStmt>
}

#[derive(Debug)]
pub struct NullStmt<'a> {
    location: Location<'a>
}

impl<'a> IsStmt for NullStmt<'a> {
}
