mod code;
mod ident;
mod operands;
mod stmt;

pub use code::Code;
pub use ident::Ident;
pub use operands::{
    Immediate, Label, LabelState, Memory, Operand, OperandKind, Register, RegisterKind, Scale,
};
pub use stmt::{Block, CompoundIns, Ins, LabelDef, LabelInfo, PseudoIns, Scope, Stmt, StmtKind};

pub trait Analyze {
    fn analyze(&self);
}

pub trait Codegen {
    fn codegen(&self) -> String;
}
pub trait Object: Analyze + Codegen {}
