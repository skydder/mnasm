use std::{cell::RefCell, rc::Rc};

use data::{Ast, Ident, Path};
use util::AsmError;

pub struct Scope<'code> {
    global: Option<Rc<Scope<'code>>>,
    name: Ident<'code>,
    in_scope: RefCell<Vec<Rc<Scope<'code>>>>,
    is_defined: bool,
}

impl<'code> Scope<'code> {
    pub fn new(global: Rc<Scope<'code>>, name: Ident<'code>, is_defined: bool) -> Rc<Self> {
        Rc::new(Self {
            global: Some(global),
            name,
            in_scope: RefCell::new(Vec::new()),
            is_defined,
        })
    }

    fn new_global(name: Ident<'code>) -> Rc<Self> {
        Rc::new(Self {
            global: None,
            name,
            in_scope: RefCell::new(Vec::new()),
            is_defined: true,
        })
    }

    pub fn has_path_of(self: Rc<Self>, path: &Path<'code>) -> bool {
        for label in self.in_scope.borrow().iter() {
            if label.name == path.current() {
                if path.is_last() {
                    return true;
                } else {
                    return label.clone().has_path_of(&path.next_path().unwrap());
                }
            }
        }
        let new = self.add_new_scope(path.current(), false);
        new.has_path_of(&path.next_path().unwrap());
        false
    }

    pub fn add_new_scope(self: Rc<Self>, name: Ident<'code>, is_defined: bool) -> Rc<Scope<'code>> {
        let new = Scope::new(
            if self.global.is_none() {
                self.clone()
            } else {
                self.global.clone().unwrap()
            },
            name,
            is_defined,
        );
        self.in_scope.borrow_mut().push(new.clone());
        new
    }
}

pub fn construct_scope<'code>(
    ast: &Ast<'code>,
    scope: Rc<Scope<'code>>,
) -> Result<(), AsmError<'code>> {
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
                construct_scope(op, scope.clone())?;
            }
            Ok(())
        }
        Ast::Label(path) => {
            let global = scope.global.clone().unwrap();
            if if path.is_relative() {
                !scope.has_path_of(path)
            } else {
                !global.has_path_of(path)
            } {
                Err(AsmError::ParseError(
                    ast.location(),
                    String::new(),
                    String::new(),
                ))
            } else {
                Ok(())
            }
        }
        Ast::LabelDef(label, _, _, Some(labeled_ast)) => {
            let new = scope.clone().add_new_scope(label.clone(), true); // nl
            if labeled_ast.is_block() {
                construct_scope(labeled_ast, new)?;
            } else {
                construct_scope(labeled_ast, scope)?;
            }
            Ok(())
        }
        Ast::LabelDef(label, _, _, None) => {
            let new = scope.clone().add_new_scope(label.clone(), true);
            Ok(())
        }
        Ast::Block(asts, loc, ..) => {
            let new = scope
                .clone()
                .add_new_scope(Ident::anonymous_ident(loc.clone()), true); // nl
            for a in asts {
                construct_scope(a, new.clone())?;
            }
            Ok(())
        }
        Ast::Macro(label, ast, labels) => todo!(),
        Ast::Register(register) => Ok(()),
        Ast::Memory(memory) => Ok(()),
        Ast::Immediate(immediate) => Ok(()),
        Ast::String(_) => Ok(()),
    }
}
