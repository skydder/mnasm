mod analyze;
#[allow(clippy::module_inception)]
mod ins;
mod ins_analyzer;

use analyze::analyze_ins;
pub use ins::Ins;
