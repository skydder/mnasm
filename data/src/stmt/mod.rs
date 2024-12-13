use std::{collections::HashMap, fmt::Debug};

mod block;
mod compound_ins;
mod ins;
mod label_def;

pub use block::{Block, Scope};
pub use compound_ins::CompoundIns;
pub use ins::Ins;
pub use label_def::LabelDef;

use crate::{Label, LabelState, Object};

pub enum StmtKind {
    Ins,
    Block,
    LabelDef,
}

pub trait Stmt<'a>: Debug + Object {
    fn kind(&self) -> StmtKind;
}

pub type LabelInfo<'a> = HashMap<Label<'a>, LabelState>;
