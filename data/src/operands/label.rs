use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use util::Location;

use crate::Ident;

use super::{Operand, OperandKind};

#[derive(PartialEq, Debug)]
pub enum LabelState {
    Used,
    Defined,
    UsedAndDefined,
}

#[derive(Debug, Clone)]
pub struct Label<'a> {
    name: Ident<'a>,
    label: String,
    pub location: Location<'a>,
}

impl<'a> Label<'a> {
    pub fn new(name: Ident<'a>, label: String, location: Location<'a>) -> Self {
        Self {
            name: name,
            label: label,
            location: location,
        }
    }
}

impl<'a> Operand for Label<'a> {
    fn codegen(&self) -> String {
        format!("{}", self.label)
    }

    fn size(&self) -> usize {
        64
    }

    fn kind(&self) -> super::OperandKind {
        OperandKind::Label
    }

    fn get_label(&self) -> Option<Label> {
        Some(self.clone())
    }

    fn analyze(&self, scope: &crate::Scope) {
        // self.gen_label =
    }
}

impl<'a> std::cmp::PartialEq for Label<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'a> std::cmp::Eq for Label<'a> {}

impl<'a> std::hash::Hash for Label<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.get().hash(state);
    }
}
