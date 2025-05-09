use std::rc::Rc;

use data::{Ast, Path, Scope};
use util::AsmError;

pub fn construct_scope<'code>(
    ast: &Ast<'code>,
    scope: Rc<Scope<'code>>,
) -> Result<(), AsmError<'code>> {
    match ast {
        Ast::Ins(label, asts) => {
            if label.data().get_str() == "extern" {
                for label in asts {
                    assert!(matches!(label, Ast::Label(_)));
                    let ident = if let Ast::Label(l) = label {
                        l.data()
                    } else {
                        return Err(AsmError::ParseError(
                            label.location(),
                            String::new(),
                            String::new(),
                        ));
                    };
                    Scope::new_global(scope.clone(), ident.current(), true, ident);
                }
                return Ok(())
            }
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
            let location = path.location();
            let path = path.data();
            let is = scope.has_path_of(&path);
            eprintln!("wwwa: {}", is);
            if !is {
                Err(AsmError::ParseError(location, String::new(), String::new()))
            } else {
                Ok(())
            }
        }
        Ast::LabelBlock(labelblock) => {
            let mut path = scope.path().path().to_vec();
            let labelblock = labelblock.data();
            path.push(labelblock.name());
            let path = Path::new(Rc::new(path), scope.path().is_relative());
            let new = Scope::new_local(scope.clone(), labelblock.name(), true, path);
            scope.add_to_in_scope(new.clone());
            for ast in labelblock.block().iter() {
                construct_scope(ast, new.clone())?;
            }
            Ok(())
        }
        Ast::Macro(label, stream) => todo!(),
        Ast::Register(register) => Ok(()),
        Ast::Memory(memory) => Ok(()),
        Ast::Immediate(immediate) => Ok(()),
        Ast::String(_) => Ok(()),
    }
}
