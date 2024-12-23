// use std::collections::HashMap;

use data::{Analyze, Code};
// use util::emit_error;
#[allow(unused_variables)]
pub fn analyze<'a>(code: &Code<'a>) {
    for ld in &code.labels {
        ld.analyze();
    }
}
