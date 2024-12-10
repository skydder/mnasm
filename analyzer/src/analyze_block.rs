use std::collections::HashMap;

use data::{Block, Label};

use crate::LabelState;

pub(crate) fn analyze_block<'a>(
    block: &Block<'a>,
    labels: &'a mut HashMap<Label<'a>, LabelState>,
) -> &'a mut HashMap<Label<'a>, LabelState> {
    for stmt in &block.stmts {
        todo!()
    }
    labels
}
