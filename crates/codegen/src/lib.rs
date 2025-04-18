use std::rc::Rc;

use data::{Ast, Scope};

fn codegen(ast: &Ast, scope: Rc<Scope>) -> String {
    match ast {
        Ast::Ins(ident, asts) => {
            let mut code = ident.get_str().to_string();
            for (i, ast) in asts.iter().enumerate() {
                code.push(' ');
                code.push_str(&codegen(ast, scope.clone()));
                if i < asts.len() - 1 {
                    code.push(',');
                }
            }
            code.push('\n');
            code
        },
        Ast::Label(path) => {
            if !path.is_relative() {
                let mut code = String::new();
                let len = path.len() - 1;
                for (i, ident) in path.path().iter().enumerate() {
                    code.push_str(ident.get_str());
                    if i < len {
                        code.push('.');
                    }
                }
                code
            } else {
                String::new()
            }
        },
        Ast::LabelDef(ident, section, is_global, ast) => {
            let mut code = if *is_global {
                ident.get_str().to_string()
            } else {
                scope.get_label()
            };
            code.push_str(":\n");
            if let Some(ast) = ast {
                code.push_str(&codegen(ast, scope));
            }
            code
        },
        Ast::Block(asts, location, _) => todo!(),
        Ast::Macro(ident, ast, asts) => todo!(),
        Ast::Register(register) => todo!(),
        Ast::Memory(memory) => todo!(),
        Ast::Immediate(immediate) => todo!(),
        Ast::String(strings) => todo!(),
    }
}