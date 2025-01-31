mod code;
mod ident;
mod operands;
mod stmt;
mod path;

pub use code::Code;
pub use ident::Ident;
pub use operands::{
    Immediate, Label, LabelState, Memory, Operand, OperandKind, Register, RegisterKind, Scale,
};
pub use stmt::{
    Block, CompoundIns, Ins, LabelDef, LabelInfo, Macro, PseudoIns, Scope, Stmt, StmtKind,
};
pub use path::Path;
pub trait Analyze {
    fn analyze(&self);
}

pub trait Codegen {
    fn codegen(&self) -> String;
}
pub trait Object: Analyze + Codegen {}
