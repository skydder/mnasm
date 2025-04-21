use std::rc::Rc;

use data::{Ast, Ident, Scope};
use util::AsmError;



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
            let global = scope.get_global().unwrap();
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
        Ast::LabelDef(label, _, is_global, Some(labeled_ast)) => {
            if !*is_global || scope.get_global().is_some() {
                let new = scope.clone().add_new_scope(label.clone(), *is_global, true);
                construct_scope(labeled_ast, new)
            } else {
                unimplemented!()
            }
        }
        Ast::LabelDef(label, _, is_global, None) => {
            if !*is_global || scope.get_global().is_some() {
                scope.clone().add_new_scope(label.clone(), *is_global, true);
                Ok(())
            } else {
                unimplemented!()
            }
        }
        Ast::Block(asts, loc, ..) => {
            let new = scope
                .clone()
                .add_new_scope(Ident::anonymous_ident(loc.clone()), false, true); // nl
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

