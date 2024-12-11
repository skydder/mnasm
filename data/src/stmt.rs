use std::{collections::HashMap, fmt::Debug};

use crate::{Label, LabelState};

pub enum StmtKind {
    Ins,
    Block,
    LabelDef
}

pub trait Stmt<'a>: Debug {
    fn codegen(&self) -> String;
    fn analyze<'b>(&self, labels: &'b mut HashMap<Label<'a>, LabelState>) -> &'b mut HashMap<Label<'a>, LabelState>;
    fn kind(&self) -> StmtKind;
}
