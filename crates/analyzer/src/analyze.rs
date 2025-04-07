// use std::collections::HashMap;

use data::Code;
// use util::emit_error;
#[allow(unused_variables)]
pub fn analyze(code: &Code<'_>) {
    for ld in &code.codes {
        ld.analyze();
    }
}
