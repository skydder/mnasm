use std::rc::Rc;

use crate::{Ast, Ident};

#[derive(Debug, Clone, PartialEq)]
pub enum Section {
    None,
    Text,
    Data,
    Bss,
    Custom(Rc<String>),
}

impl Section {
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        match self {
            Section::Text => ".text".to_string(),
            Section::Data => ".data".to_string(),
            Section::Bss => ".bss".to_string(),
            Section::Custom(c) => c.to_string(),
            Section::None => String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LabelBlock<'code> {
    name: Ident,
    section: Section,
    is_global: bool,
    block: Rc<Vec<Ast<'code>>>,
}

impl<'code> LabelBlock<'code> {
    pub fn new(name: Ident, section: Section, is_global: bool, block: Vec<Ast<'code>>) -> Self {
        Self {
            name,
            section,
            is_global,
            block: Rc::new(block),
        }
    }
    pub fn name(&self) -> Ident {
        self.name.clone()
    }
    pub fn section(&self) -> Section {
        self.section.clone()
    }
    pub fn is_global(&self) -> bool {
        self.is_global
    }
    pub fn block(&self) -> Rc<Vec<Ast<'code>>> {
        self.block.clone()
    }
}
