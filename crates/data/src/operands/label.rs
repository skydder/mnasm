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
            name,
            scope,
            location,
            path,
        }
    }
    pub fn ident(&self) -> Ident<'a> {
        self.name
    }
}

impl Operand for Label<'_> {
    fn codegen(&self) -> String {
        // should be run after analyzed
        self.scope.borrow().find_label(&self.path).unwrap().to_string()
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

impl std::cmp::PartialEq for Label<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl std::cmp::Eq for Label<'_> {}

impl std::hash::Hash for Label<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.get().hash(state);
    }
}

impl std::fmt::Debug for Label<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Label")
            .field("name", &self.name)
            .field("path", &self.path)
            .field("location", &self.location)
            .finish()
    }
}
