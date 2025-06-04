use std::{cell::RefCell, collections::HashMap, rc::Rc};

use data::{Ast, DefinedStatus, Scope, ScopeManager};
use util::{AsmError, AsmResult, Location};

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
                    Scope::new(
                        Rc::downgrade(&scope.manager()),
                        ident.get(0).unwrap(),
                        DefinedStatus::Defined,
                        HashMap::new(),
                        ident,
                    );
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
            let res = scope.manager().find_label(&path);

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
                    let mut new_scope = s.clone();
                    for i in 0..missing.len() {
                        let name = missing.get(i).unwrap();
                        let n = Scope::new(
                            Rc::downgrade(&s.manager()),
                            name.clone(),
                            DefinedStatus::Undefined(Rc::new(RefCell::new(vec![location.clone()]))),
                            HashMap::new(),
                            new_scope.absolute_path().append(name),
                        );
                        new_scope.add_new_scope(n.clone());
                        new_scope = n;
                    }
                    Ok(())
                }
                Err(Err(())) => todo!(),
            }
        }
        Ast::LabelBlock(labelblock) => {
            let path = scope.absolute_path().append(labelblock.data().name());

            let new = Scope::new(
                Rc::downgrade(&scope.manager()),
                labelblock.data().name(),
                DefinedStatus::Defined,
                HashMap::new(),
                path,
            );
            scope.add_new_scope(new.clone());
            if labelblock.data().is_global() {
                scope.manager().add_global_label(new.clone());
            }
            for ast in labelblock.data().block().iter() {
                construct_scope(ast, new.clone())?;
            }
            Ok(())
        }
        Ast::Macro(_label, _streamm) => {
            eprintln!("should have expanded");
            todo!()
        }
       _ => Ok(()),
    }
}

pub fn analyze_code<'code>(code: &Vec<Ast<'code>>) -> AsmResult<'code, Rc<ScopeManager<'code>>> {
    let manager = ScopeManager::new();

    for ast in code {
        construct_scope(ast, manager.local())?;
    }

    analyze_scope(manager.global())?;
    analyze_scope(manager.local())?;
    Ok(manager)
}

fn analyze_scope<'code>(scope: Rc<Scope<'code>>) -> AsmResult<'code, ()> {
    if matches!(scope.get_defined_status(), DefinedStatus::Undefined(_)) {
        eprintln!("{:?}", scope.absolute_path());
        return Err(AsmError::ParseError(Location::default(), "undefined label:: analyzer".to_string(), String::new()));
    }
    for child in scope.get_children().borrow().iter() {
        analyze_scope(child.1.clone())?;
    }
    Ok(())
}
