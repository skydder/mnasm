#[allow(clippy::module_inception)]
mod label_def;

use crate::{Block, Ident};
use util::Location;

#[derive(Debug)]
pub struct LabelDef<'a> {
    pub label: Ident<'a>,
    gen_label: String,
    pub is_global: bool, // visibility
    pub section: Option<Ident<'a>>,
    pub block: Option<Block<'a>>,
    pub location: Location<'a>,
}
