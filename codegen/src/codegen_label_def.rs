use std::fmt::Write;

use parser::LabelDef;

use crate::codegen_block;

pub fn codegen_label_def(ld: &LabelDef) -> String {
    let mut code = String::new();
    writeln!(code, "{}:", ld.label);
    if let Some(bl) = &ld.block { 
        write!(code, "{}", codegen_block(&bl));
    }
    code
}