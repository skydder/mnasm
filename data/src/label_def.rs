use crate::Block;

#[derive(Debug)]
pub struct LabelDef<'a> {
    pub label: &'a str,
    pub block: Option<Block<'a>>,
}
