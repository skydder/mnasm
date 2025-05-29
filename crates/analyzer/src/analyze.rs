use std::{cell::RefCell, collections::HashMap, rc::Rc};

use data::{Ast, DefinedStatus, Ident, Path, PathState, Scope};
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
            let path = path.data().absolutify(scope.absolute_path());
            let res = scope.manager().find_label(path);

            match res {
                Ok(s) => {
                    let status = s.get_defined_status();
                    match status {
                        DefinedStatus::Undefined(lox) => {
                            lox.borrow_mut().push(location);
                            Ok(())
                        }
                        DefinedStatus::Defined => Ok(()),
                    }
                }
                Err(Ok(s)) => {
                    let missing = path.diff(s.absolute_path());
                    let mut new_scope = s;
                    for i in 0..missing.len() {
                        let name = missing.get(i).unwrap();
                        let n = Scope::new(
                            s.manager(),
                            name.clone(),
                            RefCell::new(DefinedStatus::Undefined(Rc::new(RefCell::new(vec![
                                location,
                            ])))),
                            RefCell::new(HashMap::new()),
                            RefCell::new(new_scope.absolute_path().append(name)),
                        );
                        new_scope.add_new_scope(n);
                        new_scope = n;
                    }
                    Ok(())
                }
                Err(Err(())) => todo!(),
            }
        }
        Ast::LabelBlock(labelblock) => {
            let mut path = scope.path().path().to_vec();
            let labelblock = labelblock.data();
            path.push(labelblock.name());
            let path = Path::new(Rc::new(path), scope.path().state());
            let new = Scope::new_label(
                scope.clone(),
                labelblock.name(),
                true,
                path,
                labelblock.is_global(),
            );
            scope.add_to_in_scope(new.clone());
            for ast in labelblock.block().iter() {
                construct_scope(ast, new.clone())?;
            }
            Ok(())
        }
        Ast::Macro(_label, _streamm) => {
            todo!()
        }
        Ast::Register(_register) => Ok(()),
        Ast::Memory(_memory) => Ok(()),
        Ast::Immediate(_immediate) => Ok(()),
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

fn analyze_scope<'code>(scope: Rc<Scope<'code>>) -> AsmResult<'code, AsmError<'code>> {
    todo!()
}
