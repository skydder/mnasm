use std::fmt::Debug;

pub trait Stmt: Debug {
    fn codegen(&self) -> String;
}
