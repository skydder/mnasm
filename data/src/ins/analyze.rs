use util::emit_error;

use crate::Label;

use super::Ins;


fn analyze<'a, 'b>(ins: &Ins<'a>, labels: &'b mut std::collections::HashMap<Label<'a>, crate::LabelState>) -> &'b mut std::collections::HashMap<Label<'a>, crate::LabelState> {
    if ins.operands.len() >= 4 {
        emit_error!(ins.location, "unexpected number of operands")
    }
    
    labels
}