use util::Location;

use crate::Ins;

mod compound_ins;

#[derive(Debug)]
pub struct CompoundIns<'a> {
    pub compound: Vec<Ins<'a>>,
    pub location: Location<'a>,
}