use util::Location;

use crate::Stmt;

#[derive(Debug)]
pub struct Block<'a> {
    pub indent_depth: usize,
    pub stmts: Vec<Box<dyn Stmt + 'a>>,
    pub location: Location<'a>
}

impl<'a> Block<'a> {
    pub fn new(indent_depth: usize, stmts: Vec<Box<dyn Stmt + 'a>>, location: Location<'a>) -> Self {
        Self { indent_depth: indent_depth, stmts: stmts, location: location }
    }
}
