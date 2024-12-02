mod block;
mod label_def;
mod stmt;
mod ins;
mod code;

use std::fmt::Debug;

pub use block::Block;
pub use label_def::LabelDef;
pub use stmt::Stmt;
pub use ins::{Ins, CompoundIns};
pub use code::Code;

trait IsStmt : Debug {}