use std::fmt::Write;

use data::LabelDef;

use crate::codegen_block;

pub fn codegen_label_def(ld: &LabelDef) -> String {
    let mut code = String::new();

    match ld.section {
        "" => (),
        _ => {
            writeln!(code, "section {}", ld.section);
        }
    }

    if ld.is_global {
        write!(code, "global {}\n", ld.label);
    }

    writeln!(code, "{}:", ld.label);
    if let Some(bl) = &ld.block {
        write!(code, "{}", codegen_block(&bl));
    }
    code
}
