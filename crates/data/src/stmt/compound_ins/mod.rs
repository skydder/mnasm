use util::Location;

use crate::Ins;

#[allow(clippy::module_inception)] //todo
mod compound_ins;

#[derive(Debug)]
pub struct CompoundIns<'a> {
    pub compound: Vec<Ins<'a>>,
    pub location: Location<'a>,
}
