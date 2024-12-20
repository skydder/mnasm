use std::{
    cell::RefCell,
    rc::Rc,
};

use util::{emit_error, Location};

use crate::{Ident, Scope};

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
    scope: Rc<RefCell<Scope<'a>>>,
    pub location: Location<'a>,
}

impl<'a> Label<'a> {
    pub fn new(name: Ident<'a>, scope: Rc<RefCell<Scope<'a>>>, location: Location<'a>) -> Self {
        Self {
            name: name,
            scope: scope,
            location: location,
        }
    }
}

impl<'a> Operand for Label<'a> {
    fn codegen(&self) -> String {
        format!("{}", self.scope.borrow().find_label(self.name).unwrap_or_else(|| emit_error!(self.location, "undefined label")),)
    }

    fn size(&self) -> usize {
        64
    }

    fn kind(&self) -> super::OperandKind {
        OperandKind::Label
    }

    fn analyze(&self) {
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
