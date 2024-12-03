mod block;
mod label_def;
mod stmt;
mod ins;
mod code;

use std::fmt::Debug;

pub use block::Block;
pub use label_def::LabelDef;
pub use stmt::NullStmt;
pub use ins::{Ins, CompoundIns};
pub use code::Code;

pub trait Stmt : Debug {
    fn codegen(&self) -> String;
}