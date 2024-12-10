use std::collections::HashMap;

use data::{Label, Stmt};

use crate::{analyze_block, LabelState};



pub(crate) fn analyze_stmt<'a>(
    stmt: &Box<dyn Stmt + 'a>,
    labels: &'a mut HashMap<Label<'a>, LabelState>,
) -> &'a mut HashMap<Label<'a>, LabelState> {
    // should be implemented in Stmt...
    labels = match stmt.kind() {
        data::StmtKind::Ins => todo!(),
        data::StmtKind::Block => analyze_block(stmt, labels),
        data::StmtKind::LabelDef => todo!(),
    };

    labels
}