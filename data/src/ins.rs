use util::Location;

#[derive(Debug)]
pub struct Ins<'a> {
    pub instruction: &'a str,
    pub operand: (),
    pub location: Location<'a>,
}

#[derive(Debug)]
pub struct CompoundIns<'a> {
    pub compound: Vec<Ins<'a>>
}