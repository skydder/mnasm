use util::Location;

use crate::Stmt;
#[derive(Debug)]
pub struct NullStmt<'a> {
    pub location: Location<'a>
}

impl<'a> NullStmt<'a> {
    pub fn new(location: Location<'a>) -> Self {
        Self { location: location }
    }
}

impl<'a> Stmt for NullStmt<'a> {
    fn codegen(&self) -> String {
        String::new()
    }
}
