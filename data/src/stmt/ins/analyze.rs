use util::emit_error;

use crate::{Label, LabelState, OperandKind};

use super::Ins;

pub(crate) fn analyze<'a, 'b>(
    ins: &'a Ins<'a>,
    labels: &'b mut std::collections::HashMap<Label<'a>, crate::LabelState>,
) -> &'b mut std::collections::HashMap<Label<'a>, crate::LabelState> 
where 'a: 'b{
    if ins.operands.len() >= 4 {
        emit_error!(ins.location, "unexpected number of operands")
    }
    for oprand in &ins.operands {
        if let OperandKind::Label = oprand.kind() {
            let lbl = oprand.get_label().unwrap();
            if let Some(data) = labels.get_mut(&lbl) {
                match data {
                    LabelState::Defined => {
                        *data = LabelState::UsedAndDefined;
                    }
                    _ => (),
                };
            } else {
                labels.insert(lbl, LabelState::Used);
            }
        }
    }
    labels
}
