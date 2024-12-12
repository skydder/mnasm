
mod code;
mod operands;
mod stmt;
mod name;

pub use code::Code;
pub use operands::{
    Immediate, Label, LabelState, Memory, Operand, OperandKind, Register, RegisterKind, Scale,
};
pub use stmt::{Stmt, StmtKind, LabelInfo, LabelDef, Block, Ins, CompoundIns, Scope};
pub use name::Name;

use tokenizer::Tokenizer;


pub trait Object{}

pub trait Parse: Object {
    fn parse<'a>(tokenizer: &'a Tokenizer<'a>) -> Self;
}

pub trait Analyze: Object {
    fn analyze(&self);
}

pub trait Codegen: Object {
    fn codegen(&self) -> String;
}
