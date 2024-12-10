mod alayze_ins;
mod analyze;
mod analyze_block;
mod analyze_label;
mod read_ins;
mod analyze_stmt;

pub use analyze::analyze;
pub(crate) use analyze_label::{analyze_label_def, LabelState};
pub(crate) use analyze_block::analyze_block;
pub(crate) use analyze_stmt::analyze_stmt;
// use data::OperandKind;
// pub(crate) type OperandType = (OperandKind, usize);
