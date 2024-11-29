use crate::Block;

#[derive(Debug)]
pub struct LabelDef<'a> {
    pub label: &'a str,
    pub is_global: bool, // visibility
    pub section: &'a str,
    pub block: Option<Block<'a>>,
}

impl<'a> LabelDef<'a> {
    pub fn new(
        label: &'a str,
        is_global: bool,
        section: &'a str,
        block: Option<Block<'a>>,
    ) -> Self {
        Self {
            label: label,
            is_global: is_global,
            section: section,
            block: block,
        }
    }
}
