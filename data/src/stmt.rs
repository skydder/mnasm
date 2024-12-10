use std::fmt::Debug;

pub enum StmtKind {
    Ins,
    Block,
    LabelDef
}

pub trait Stmt: Debug {
    fn codegen(&self) -> String;
    fn kind(&self) -> StmtKind;
}
