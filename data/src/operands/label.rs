use util::Location;

use crate::Name;

use super::{Operand, OperandKind};

#[derive(PartialEq, Debug)]
pub enum LabelState {
    Used,
    Defined,
    UsedAndDefined,
}

#[derive(Debug, Clone, Copy)]
pub struct Label<'a> {
    name: Name<'a>,
    pub location: Location<'a>,
}

impl<'a> Label<'a> {
    pub fn new(name: Name<'a>, location: Location<'a>) -> Self {
        Self {
            name: name,
            location: location,
        }
    }
}

impl<'a> Operand for Label<'a> {
    fn codegen(&self) -> String {
        format!("{}", self.name.get())
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
