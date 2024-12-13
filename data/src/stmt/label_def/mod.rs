mod label_def;

use crate::{Block, Name};
use util::Location;

#[derive(Debug)]
pub struct LabelDef<'a> {
    pub label: Name<'a>,
    pub is_global: bool, // visibility
    pub section: Option<Name<'a>>,
    pub block: Option<Block<'a>>,
    pub location: Location<'a>,
}
