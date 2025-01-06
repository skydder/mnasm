use std::{collections::HashMap, fmt::Debug};

mod block;
mod compound_ins;
mod ins;
mod label_def;
mod pseudo_ins;
mod r#macro;

pub use block::{Block, Scope};
pub use compound_ins::CompoundIns;
pub use ins::Ins;
pub use label_def::LabelDef;
pub use pseudo_ins::PseudoIns;
pub use r#macro::Macro;

use crate::{Label, LabelState, Object};

pub enum StmtKind {
    Ins,
    Block,
    LabelDef,
    Macro,
}

pub trait Stmt<'a>: Debug + Object {
    fn kind(&self) -> StmtKind;
}

pub type LabelInfo<'a> = HashMap<Label<'a>, LabelState>;
