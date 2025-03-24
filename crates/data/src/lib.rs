mod code;
mod ident;
mod operands;
mod path;
mod stmt;

use std::fmt::Debug;

pub use code::Code;
pub use ident::Ident;
pub use operands::{
    Immediate, Label, LabelState, Memory, Operand, OperandKind, Register, RegisterKind, Scale,
    UnimplementedOperand,
};
pub use path::Path;
pub use stmt::{Block, CompoundIns, Ins, LabelDef, LabelInfo, PseudoIns, Scope, Stmt, StmtKind};
pub trait Analyze {
    fn analyze(&self);
}

pub trait Codegen {
    fn codegen(&self) -> String;
    fn to_code(&self) -> String;
}
pub trait Object: Analyze + Codegen + Debug {}
