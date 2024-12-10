use std::collections::HashMap;

use data::{Label, LabelDef};
use util::emit_error;

use crate::analyze_block;

#[derive(PartialEq)]
pub(crate) enum LabelState {
    Used,
    Defined,
    UsedAndDefined,
}

pub(crate) fn analyze_label_def<'a>(
    label_def: &'a LabelDef<'a>,
    mut labels: &'a mut HashMap<Label<'a>, LabelState>,
) -> &'a mut HashMap<Label<'a>, LabelState> {
    if let Some(data) = labels.get_mut(&label_def.label()) {
        *data = match data {
            LabelState::Used => LabelState::UsedAndDefined,
            _ => {
                emit_error!(label_def.location, "multiple definition!!");
            }
        };
    } else {
        labels.insert(label_def.label(), LabelState::Defined);
    }
    if let Some(block) = &label_def.block  {
        labels = analyze_block(block, labels);
    }
    labels
}
