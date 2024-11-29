use util::Location;

use crate::CompoundIns;

#[derive(Debug)]
pub struct Stmt<'a> {
    // pub instruction: &'a str,
    // pub operand: (),
    // pub location: Location<'a>,
    pub line: CompoundIns<'a>
}
