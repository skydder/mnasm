mod code;
mod name;
mod operands;
mod stmt;

pub use code::Code;
pub use name::Name;
pub use operands::{
    Immediate, Label, LabelState, Memory, Operand, OperandKind, Register, RegisterKind, Scale,
};
pub use stmt::{Block, CompoundIns, Ins, LabelDef, LabelInfo, Scope, Stmt, StmtKind};

pub trait Analyze {
    fn analyze(&self);
}

pub trait Codegen {
    fn codegen(&self) -> String;
}
pub trait Object: Analyze + Codegen {}
