use data::LabelDef;

use crate::codegen_block;

pub fn codegen_label_def(ld: &LabelDef) -> String {
    let mut code = String::new();

    if ld.section != "" {
        code.push_str(&format!("section {}\n", ld.section));
    }

    if ld.is_global {
        code.push_str(&format!("global {}\n", ld.label));
    }

    code.push_str(&format!("{}:\n", ld.label));
    if let Some(bl) = &ld.block {
        code.push_str(&codegen_block(&bl));
    }
    code.push('\n');
    code
}
