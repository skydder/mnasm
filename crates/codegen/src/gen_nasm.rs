use std::rc::Rc;

use data::{Ast, Ident, PathState, Scope, Section, WithLocation, REG16, REG32, REG64, REG8};
use util::AsmResult;
pub fn codegen<'code>(ast: &Ast<'code>, scope: Rc<Scope<'code>>) -> String {
    match ast {
        Ast::Ins(ident, asts) => {
            if ident.data().get_str() == "extern" {
                return String::new();
            }
            let mut code = format!("\t{}", ident.data().get_str());
            for (i, ast) in asts.iter().enumerate() {
                code.push(' ');
                code.push_str(&codegen(ast, scope.clone()));
                if i < asts.len() - 1 {
                    code.push(',');
                }
            }
            code.push('\n');
            code
        }
        Ast::Label(path) => {
            let path = path.data();
            match path.state() {
                PathState::Absolute | PathState::Relative => {
                    let mut code = if !path.is_relative() {
                        String::new()
                    } else {
                        scope.get_label()
                    };
                    for ident in path.path().iter() {
                        code.push('_');
                        code.push_str(&ident.get_str());
                    }
                    code
                }
                PathState::Global => {
                    path.current().get_str()
                }
            }
        }

        Ast::LabelBlock(labelblock) => {
            let mut code = String::new();
            let labelblock = labelblock.data();
            let own_scope = scope.get_child(&labelblock.name()).unwrap();
            if !labelblock.name().is_anonymous() {
                let name = if labelblock.is_global() {
                    labelblock.name().get_str()
                } else {
                    own_scope.get_label()
                };
                if labelblock.section() != Section::None {
                    code.push_str(&format!("section {}\n", labelblock.section().to_string()));
                }
                if labelblock.is_global() {
                    code.push_str(&format!("global {}\n", name));
                }
                code.push_str(&format!("{}:\n", name));
            }

            for ast in labelblock.block().iter() {
                code.push_str(&codegen(ast, own_scope.clone()));
            }
            code
        }
        Ast::Macro(_ident, _ast) => todo!(),
        Ast::Register(register) => {
            let register = register.data();
            let reg = match register.size {
                8 => REG8,
                16 => REG16,
                32 => REG32,
                64 => REG64,
                _ => unimplemented!(),
            };
            reg[register.value as usize].to_string()
        }
        Ast::Memory(memory) => {
            let loc = memory.location();
            let memory = memory.data();
            match (&memory.base, &memory.index, &memory.scale, &memory.disp) {
                (None, None, None, Some(d)) => format!("[{}]", codegen(d, scope)),
                (None, Some(i), Some(s), None) => format!(
                    "[{} * {}]",
                    codegen(&Ast::Register(WithLocation::new(loc, *i)), scope),
                    *s as u64
                ),
                (None, Some(i), Some(s), Some(d)) => format!(
                    "[{} + {} * {}]",
                    codegen(d, scope.clone()),
                    codegen(&Ast::Register(WithLocation::new(loc, *i)), scope),
                    *s as u64
                ),
                (Some(b), None, None, None) => {
                    format!(
                        "[{}]",
                        codegen(&Ast::Register(WithLocation::new(loc, *b)), scope)
                    )
                }
                (Some(b), None, None, Some(d)) => format!(
                    "[{} + {}]",
                    codegen(d, scope.clone()),
                    codegen(&Ast::Register(WithLocation::new(loc, *b)), scope)
                ),
                (Some(b), Some(i), None, None) => format!(
                    "[{} + {}]",
                    codegen(
                        &Ast::Register(WithLocation::new(loc.clone(), *b)),
                        scope.clone()
                    ),
                    codegen(&Ast::Register(WithLocation::new(loc, *i)), scope)
                ),
                (Some(b), Some(i), None, Some(d)) => format!(
                    "[{} + {} + {} ]",
                    codegen(d, scope.clone()),
                    codegen(
                        &Ast::Register(WithLocation::new(loc.clone(), *b)),
                        scope.clone()
                    ),
                    codegen(&Ast::Register(WithLocation::new(loc, *i)), scope)
                ),
                (Some(b), Some(i), Some(s), None) => format!(
                    "[{} + {} * {} ]",
                    codegen(
                        &Ast::Register(WithLocation::new(loc.clone(), *b)),
                        scope.clone()
                    ),
                    codegen(&Ast::Register(WithLocation::new(loc, *i)), scope),
                    *s as u64
                ),
                (Some(b), Some(i), Some(s), Some(d)) => format!(
                    "[{} + {} + {} * {}]",
                    codegen(d, scope.clone()),
                    codegen(
                        &Ast::Register(WithLocation::new(loc.clone(), *b)),
                        scope.clone()
                    ),
                    codegen(&Ast::Register(WithLocation::new(loc, *i)), scope),
                    *s as u64
                ),
                _ => unimplemented!(),
            }
        }
        Ast::Immediate(immediate) => {
            let immediate = immediate.data();
            if immediate.signed {
                format!("-{}", immediate.data)
            } else {
                format!("{}", immediate.data)
            }
        }
        Ast::String(strings) => {
            format!("\"{}\"", strings.data().get_str())
        }
        Ast::EOS => String::new(),
    }
}

pub fn codegen_code<'code>(
    code: &Vec<Ast<'code>>,
    root: Rc<Scope<'code>>,
) -> AsmResult<'code, String> {
    let mut output = String::new();
    for ast in code {
        output.push_str(&codegen(
            ast,
            root.get_child(&Ident::new("_local".to_owned()))
                .clone()
                .unwrap(),
        ));
    }
    Ok(output)
}
