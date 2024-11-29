mod block;
mod label_def;
mod stmt;
mod ins;

pub use block::Block;
pub use label_def::LabelDef;
pub use stmt::Stmt;
pub use ins::{Ins, CompoundIns};