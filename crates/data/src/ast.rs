use std::{cell::RefCell, rc::Rc};

use util::{AsmError, Location};

use super::{
    operand::{Immediate, Path, Memory, Register},
    ident::Ident,
};

pub enum Ast<'code> {
    Ins(Ident<'code>, Vec<Ast<'code>>),
    Label(Path<'code>),
    LabelDef(Ident<'code>, Box<Ast<'code>>),
    Block(Vec<Ast<'code>>, Location<'code>),
    BlockLessBlock(Vec<Ast<'code>>, Location<'code>),
    Macro(Ident<'code>, Box<Ast<'code>>, Vec<Ast<'code>>), // 1 -> argument
    Register(Register<'code>),
    Memory(Memory<'code>),
    Immediate(Immediate<'code>),
}

#[allow(clippy::needless_lifetimes)]
impl<'code> Ast<'code> {
    pub fn is_operand(&self) -> bool {
        matches!(
            self,
            Ast::Label(..) | Ast::Immediate(..) | Ast::Memory(..) | Ast::Register(..)
        )
    }

    pub fn is_block(&self) -> bool {
        matches!(self, Ast::Block(..))
    }

    pub fn location(&self) -> Location<'code> {
        match self {
            Ast::Ins(label, _) => label.location(),
            Ast::Label(path) => path.location(),
            Ast::LabelDef(label, _) => label.location(),
            Ast::Block(_, loc) => loc.clone(),
            Ast::BlockLessBlock(_, loc) => loc.clone(),
            Ast::Macro(label, ..) => label.location(),
            Ast::Register(register) => register.location(),
            Ast::Memory(memory) => memory.location(),
            Ast::Immediate(immediate) => immediate.location(),
        }
    }

    pub fn print_ast(&self) -> String {
        match self {
            Ast::Ins(label, asts) => {
                // format!("{}(", label, )
            }
            Ast::Label(path) => todo!(),
            Ast::Block(asts, _) => todo!(),
            Ast::BlockLessBlock(asts, _) => todo!(),
            Ast::Macro(label, ast, labels) => todo!(),
            Ast::Register(register) => todo!(),
            Ast::Memory(memory) => todo!(),
            Ast::Immediate(immediate) => todo!(),
            Ast::LabelDef(..) => todo!(),
        }
        todo!()
    }
}

pub struct Scope<'code> {
    global: Option<Rc<Scope<'code>>>,
    name: Ident<'code>,
    in_scope: RefCell<Vec<Rc<Scope<'code>>>>,
}

impl<'code> Scope<'code> {
    pub fn new(global: Rc<Scope<'code>>, name: Ident<'code>) -> Rc<Self> {
        Rc::new(Self {
            global: Some(global),
            name,
            in_scope: RefCell::new(Vec::new()),
        })
    }

    fn new_global(name: Ident<'code>) -> Rc<Self> {
        Rc::new(Self {
            global: None,
            name,
            in_scope: RefCell::new(Vec::new()),
        })
    }

    fn has(&self, name: Ident<'code>) -> bool {
        for scope in self.in_scope.borrow().iter() {
            if scope.name == name {
                return true;
            }
        }
        false
    }

    pub fn has_path_of(self: Rc<Self>, path: &Path<'code>) -> bool {
        // recursive
        todo!()
    }
    pub fn add_new_scope(self: Rc<Self>, name: Ident<'code>) -> Rc<Scope<'code>> {
        let new = Scope::new(
            if self.global.is_none() {
                self.clone()
            } else {
                self.global.clone().unwrap()
            },
            name,
        );
        self.in_scope.borrow_mut().push(new.clone());
        new
    }
}

pub fn analyze<'code>(ast: &Ast<'code>, scope: Rc<Scope<'code>>) -> Result<(), AsmError<'code>> {
    match ast {
        Ast::Ins(label, asts) => {
            for op in asts {
                if !op.is_operand() {
                    return Err(AsmError::ParseError(
                        op.location(),
                        String::new(),
                        String::new(),
                    ));
                }
                analyze(op, scope.clone())?;
            }
            Ok(())
        }
        Ast::Label(path) => {
            if !scope.has_path_of(path) {
                Err(AsmError::ParseError(
                    ast.location(),
                    String::new(),
                    String::new(),
                ))
            } else {
                Ok(())
            }
        }
        Ast::LabelDef(label, labeled_ast) => {
            let new = scope.clone().add_new_scope(label.clone()); // nl
            if labeled_ast.is_block() {
                analyze(labeled_ast, new)?;
            } else {
                analyze(labeled_ast, scope)?;
            }
            Ok(())
        }
        Ast::Block(asts, _) => {
            for a in asts {
                analyze(a, scope.clone())?;
            }
            Ok(())
        }
        Ast::BlockLessBlock(asts, _) => {
            for a in asts {
                analyze(a, scope.clone())?;
            }
            Ok(())
        }
        Ast::Macro(label, ast, labels) => todo!(),
        Ast::Register(register) => Ok(()),
        Ast::Memory(memory) => Ok(()),
        Ast::Immediate(immediate) => Ok(()),
    }
}
