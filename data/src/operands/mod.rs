use std::fmt::Debug;

mod register;
mod label;
mod immediate;
mod memory;
pub use register::{Register, RegisterKind};
pub use label::Label;
pub use immediate::Immediate; 

pub trait Operand: Debug {
    fn codegen(&self) -> String;
    // fn kind
    fn size(&self) -> usize;
}
