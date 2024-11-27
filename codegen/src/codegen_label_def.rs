use std::fmt::Write;

use parser::LabelDef;

pub fn codegen_label_def(ld: &LabelDef) -> String {
    let mut code = String::new();
    writeln!(code, "{}:", ld.label);
    
    code
}