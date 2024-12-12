use std::{collections::HashMap, fmt::Debug};

mod block;
mod compound_ins;
mod label_def;
mod ins;

pub use block::Block;
pub use compound_ins::CompoundIns;
pub use ins::Ins;
pub use label_def::{LabelDef, Scope};

use crate::{Label, LabelState};

pub enum StmtKind {
    Ins,
    Block,
    LabelDef,
}

pub trait Stmt<'a>: Debug {
    fn codegen(&self) -> String;
    fn analyze(
        &self,
        labels: &'a mut LabelInfo<'a>,
    ) -> &mut LabelInfo<'a>;
    fn kind(&self) -> StmtKind;
}

pub type LabelInfo<'a> = HashMap<Label<'a>, LabelState>;
