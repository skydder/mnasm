use std::rc::Rc;

use data::{Ast, Ident, Path, Scope};
use util::{AsmError, AsmResult};

pub fn construct_scope<'code>(
    ast: &Ast<'code>,
    scope: Rc<Scope<'code>>,
) -> Result<(), AsmError<'code>> {
    match ast {
        Ast::Ins(label, asts) => {
            if label.data().get_str() == "extern" {
                for label in asts.iter() {
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
                return Ok(());
            }
            for op in asts.iter() {
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
                Err(AsmError::ParseError(
                    location,
                    "undefined label".to_string(),
                    String::new(),
                ))
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
        Ast::Macro(label, stream) => {
            todo!()
        }
        Ast::Register(register) => Ok(()),
        Ast::Memory(memory) => Ok(()),
        Ast::Immediate(immediate) => Ok(()),
        Ast::String(_) => Ok(()),
        Ast::EOS => Ok(()),
    }
}

pub fn analyze_code<'code>(code: &Vec<Ast<'code>>) -> AsmResult<'code, Rc<Scope<'code>>> {
    let root = Scope::init_root();
    for ast in code {
        construct_scope(
            ast,
            root.get_child(&Ident::new("_local".to_owned()))
                .clone()
                .unwrap(),
        )?;
    }
    Ok(root)
}
