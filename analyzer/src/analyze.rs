use std::collections::HashMap;

use crate::{analyze_label_def, LabelState};
use data::Code;
use util::emit_error;

pub fn analyze<'a>(code: &'a Code<'a>) {
    let mut labels = &mut HashMap::new();
    for ld in &code.labels {
        labels = analyze_label_def(ld, labels);
    }

    for (lb, state) in labels {
        if *state == LabelState::Used {
            emit_error!(lb.location, "undefined label")
        }
    }
}
