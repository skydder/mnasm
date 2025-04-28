use data::{Ast, Section, REG16, REG32, REG64, REG8};

pub fn pretty_print(ast: &Ast) -> String {
    match ast {
        Ast::Ins(ident, asts) => {
            let mut code = ident.get_str().to_string();
            for (i, ast) in asts.iter().enumerate() {
                code.push(' ');
                code.push_str(&pretty_print(ast));
                if i < asts.len() - 1 {
                    code.push(',');
                }
            }
            code.push('\n');
            code
        }
        Ast::Label(path) => {
            if !path.is_relative() {
                let mut code = String::new();
                let len = path.len() - 1;
                for (i, ident) in path.path().iter().enumerate() {
                    code.push_str(&ident.get_str());
                    if i < len {
                        code.push_str("::");
                    }
                }
                code
            } else {
                String::new()
            }
        }
        Ast::LabelBlock(labelblock) => {
            let mut code = String::new();
            if !labelblock.name().is_anonymous() {
                code.push_str(&format!("<{}", labelblock.name().get_str()));
                if labelblock.section() != Section::None {
                    code.push_str(&format!(":{}", labelblock.section().to_string()));
                }
                if labelblock.is_global() {
                    code.push_str(":global");
                }
                code.push('>');
            }

            if !labelblock.block().is_empty() {
                code.push('{');
                for ast in labelblock.block().iter() {
                    code.push_str(&pretty_print(ast))
                }
            }
            code
        }
        Ast::Macro(ident, ast, asts) => todo!(),
        Ast::Register(register) => {
            let reg = match register.size {
                8 => REG8,
                16 => REG16,
                32 => REG32,
                64 => REG64,
                _ => unimplemented!(),
            };
            reg[register.value as usize].to_string()
        }
        Ast::Memory(memory) => match (&memory.base, &memory.index, &memory.scale, &memory.disp) {
            (None, None, None, Some(d)) => format!("[{}]", pretty_print(d)),
            (None, Some(i), Some(s), None) => format!(
                "[{} * {}]",
                pretty_print(&Ast::Register(i.clone())),
                *s as u64
            ),
            (None, Some(i), Some(s), Some(d)) => format!(
                "[{} + {} * {}]",
                pretty_print(d),
                pretty_print(&Ast::Register(i.clone())),
                *s as u64
            ),
            (Some(b), None, None, None) => format!("[{}]", pretty_print(&Ast::Register(b.clone()))),
            (Some(b), None, None, Some(d)) => format!(
                "[{} + {}]",
                pretty_print(d),
                pretty_print(&Ast::Register(b.clone()))
            ),
            (Some(b), Some(i), None, None) => format!(
                "[{} + {}]",
                pretty_print(&Ast::Register(b.clone())),
                pretty_print(&Ast::Register(i.clone()))
            ),
            (Some(b), Some(i), None, Some(d)) => format!(
                "[{} + {} + {} ]",
                pretty_print(d),
                pretty_print(&Ast::Register(b.clone())),
                pretty_print(&Ast::Register(i.clone()))
            ),
            (Some(b), Some(i), Some(s), None) => format!(
                "[{} + {} * {} ]",
                pretty_print(&Ast::Register(b.clone())),
                pretty_print(&Ast::Register(i.clone())),
                *s as u64
            ),
            (Some(b), Some(i), Some(s), Some(d)) => format!(
                "[{} + {} + {} * {}]",
                pretty_print(d),
                pretty_print(&Ast::Register(b.clone())),
                pretty_print(&Ast::Register(i.clone())),
                *s as u64
            ),
            _ => unimplemented!(),
        },
        Ast::Immediate(immediate) => {
            if immediate.signed {
                format!("-{}", immediate.data)
            } else {
                format!("{}", immediate.data)
            }
        }
        Ast::String(strings) => {
            format!("\"{}\"", strings.get_str())
        }
    }
}
