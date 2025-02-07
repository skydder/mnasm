use std::{cell::RefCell, rc::Rc};

use util::{emit_error, Location};

use crate::{Ident, Path, Scope};

use super::{Operand, OperandKind};

#[derive(PartialEq, Debug)]
pub enum LabelState {
    Used,
    Defined,
    UsedAndDefined,
}

#[derive(Clone)]
pub struct Label<'a> {
    name: Ident<'a>,
    pub path: Path<'a>,
    scope: Rc<RefCell<Scope<'a>>>,
    pub location: Location<'a>,
}

impl<'a> Label<'a> {
    pub fn new(
        name: Ident<'a>,
        scope: Rc<RefCell<Scope<'a>>>,
        location: Location<'a>,
        path: Path<'a>,
    ) -> Self {
        Self {
            name: name,
            scope: scope,
            location: location,
            path: path,
        }
    }
    pub fn ident(&self) -> Ident<'a> {
        self.name
    }
}

impl<'a> Operand for Label<'a> {
    fn codegen(&self) -> String {
        // should be run after analyzed
        format!("{}", self.scope.borrow().find_label(&self.path).unwrap())
    }

    fn size(&self) -> usize {
        64
    }

    fn kind(&self) -> super::OperandKind {
        OperandKind::Label
    }

    fn analyze(&self) {
        self.scope
            .borrow()
            .find_label(&self.path)
            .unwrap_or_else(|| {
                emit_error!(
                    self.location,
                    "undefined label: {} {:?}",
                    self.name,
                    self.path
                )
            });
    }

    fn op(&self) -> (OperandKind, usize) {
        (OperandKind::Immediate(false), 64)
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

impl<'a> std::fmt::Debug for Label<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Label").field("name", &self.name).field("path", &self.path).field("location", &self.location).finish()
    }
}
