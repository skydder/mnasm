mod block;
mod code;
mod ins;
mod label_def;
mod stmt;

use std::fmt::Debug;

pub use block::Block;
pub use code::Code;
pub use ins::{CompoundIns, Ins};
pub use label_def::LabelDef;
pub use stmt::NullStmt;

pub trait Stmt: Debug {
    fn codegen(&self) -> String;
}
