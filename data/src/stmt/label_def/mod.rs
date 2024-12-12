mod label_def;

use util::Location;
use crate::{Block, Name};

pub struct Scope<'a> {
    parents: Option<Box<LabelDef<'a>>>,
    labels: Vec<Name<'a>>,
}


#[derive(Debug)]
pub struct LabelDef<'a> {
    pub label: Name<'a>,
    pub is_global: bool, // visibility
    pub section: Option<Name<'a>>,
    pub block: Option<Block<'a>>,
    pub location: Location<'a>,


}