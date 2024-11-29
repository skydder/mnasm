use data::Code;

use crate::codegen_label_def;

pub fn codegen_code(code: &Code) -> String {
    let mut res =  String::new();
    for l in &code.labels {
        res.push_str(&codegen_label_def(l));
    }
    res
}