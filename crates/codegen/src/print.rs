use std::rc::Rc;

use data::{Ast, Scope, REG16, REG32, REG64, REG8};

pub fn codegen(ast: &Ast, scope: Rc<Scope>) -> String {
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
                        code.push_str("::");
                    }
                }
                code
            } else {
                String::new()
            }
        },
        // Ast::LabelDef(ident, section, is_global, ast) => {
        //     let mut code = String::new();
        //     code.push_str(&format!("<{}", ident.get_str()));
        //     if section.is_some() {
        //         code.push_str(&format!(":{}", section.clone().unwrap().to_string()));
        //     }
        //     if *is_global {
        //         code.push_str(":global");
        //     }
        //     code.push('>');
        //     if let Some(ast) = ast {
        //         code.push_str(&codegen(ast, scope));
        //     }
        //     code
        // },
        // Ast::Block(asts, location, _) => {
        //     let mut code = String::new();
        //     for ast in asts {
        //         code.push_str(&codegen(ast, scope.clone()));
        //     }
        //     code
        // },
        Ast::LabelBlock(labelblock) => {
            todo!()
        }
        Ast::Macro(ident, ast, asts) => todo!(),
        Ast::Register(register) => {
            let reg = match register.size {
                8 => REG8,
                16 => REG16,
                32 => REG32,
                64 => REG64,
                _ => unimplemented!()
            };
            reg[register.value as usize].to_string()
        },
        Ast::Memory(memory) => {
            match (&memory.base, &memory.index, &memory.scale, &memory.disp) {
                (None, None, None, Some(d)) => format!("[{}]", codegen(d, scope)),
                (None, Some(i), Some(s), None) => format!("[{} * {}]", codegen(&Ast::Register(i.clone()), scope), *s as u64),
                (None, Some(i), Some(s), Some(d)) => format!("[{} + {} * {}]", codegen(d, scope.clone()),codegen(&Ast::Register(i.clone()), scope), *s as u64),
                (Some(b), None, None, None) => format!("[{}]", codegen(&Ast::Register(b.clone()), scope)),
                (Some(b), None, None, Some(d)) => format!("[{} + {}]", codegen(d, scope.clone()), codegen(&Ast::Register(b.clone()), scope)),
                (Some(b), Some(i), None, None) => format!("[{} + {}]", codegen(&Ast::Register(b.clone()), scope.clone()), codegen(&Ast::Register(i.clone()), scope)),
                (Some(b), Some(i), None, Some(d)) => format!("[{} + {} + {} ]", codegen(d, scope.clone()), codegen(&Ast::Register(b.clone()), scope.clone()), codegen(&Ast::Register(i.clone()), scope)),
                (Some(b), Some(i), Some(s), None) => format!("[{} + {} * {} ]", codegen(&Ast::Register(b.clone()), scope.clone()), codegen(&Ast::Register(i.clone()), scope), *s as u64),
                (Some(b), Some(i), Some(s), Some(d)) => format!("[{} + {} + {} * {}]", codegen(d, scope.clone()), codegen(&Ast::Register(b.clone()), scope.clone()), codegen(&Ast::Register(i.clone()), scope), *s as u64),
                _ => unimplemented!()
            }
        },
        Ast::Immediate(immediate) => {
            if immediate.signed {
                format!("-{}", immediate.data)
            } else {
                format!("{}", immediate.data)
            }
        },
        Ast::String(strings) => {
            format!("\"{}\"", strings.get_str())
        },
    }
}