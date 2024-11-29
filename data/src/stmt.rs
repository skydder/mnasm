use util::Location;

#[derive(Debug)]
pub struct Stmt<'a> {
    pub instruction: &'a str,
    pub operand: (),
    pub location: Location<'a>,
}
