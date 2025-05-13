use data::{Ast, Section, WithLocation, REG16, REG32, REG64, REG8};

pub fn pretty_print(ast: &Ast) -> String {
    match ast {
        Ast::Ins(ident, asts) => {
            let mut code = ident.data().get_str().to_string();
            code.push('(');
            for (i, ast) in asts.iter().enumerate() {
                code.push_str(&pretty_print(ast));
                if i < asts.len() - 1 {
                    code.push_str(", ");
                }
            }
            code.push_str(")\n");
            code
        }
        Ast::Label(path) => {
            let path = path.data();
            if path.is_relative() {
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
                let mut code = String::new();
                for ident in path.path().iter() {
                    code.push_str("::");
                    code.push_str(&ident.get_str());
                }
                code
            }
        }
        Ast::LabelBlock(labelblock) => {
            let labelblock = labelblock.data();
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
                let mut block = "{\n".to_string();
                for ast in labelblock.block().iter() {
                    block.push_str(&pretty_print(ast))
                }
                code.push_str(&block.trim_end_matches('\n').replace("\n", "\n    "));
                code.push_str("\n}");
            }
            code.push('\n');
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
                (None, None, None, Some(d)) => format!("[{}]", pretty_print(d)),
                (None, Some(i), Some(s), None) => format!(
                    "[{} * {}]",
                    pretty_print(&Ast::Register(WithLocation::new(loc.clone(), *i))),
                    *s as u64
                ),
                (None, Some(i), Some(s), Some(d)) => format!(
                    "[{} + {} * {}]",
                    pretty_print(d),
                    pretty_print(&Ast::Register(WithLocation::new(loc.clone(), *i))),
                    *s as u64
                ),
                (Some(b), None, None, None) => {
                    format!(
                        "[{}]",
                        pretty_print(&Ast::Register(WithLocation::new(loc.clone(), *b)))
                    )
                }
                (Some(b), None, None, Some(d)) => format!(
                    "[{} + {}]",
                    pretty_print(d),
                    pretty_print(&Ast::Register(WithLocation::new(loc.clone(), *b)))
                ),
                (Some(b), Some(i), None, None) => format!(
                    "[{} + {}]",
                    pretty_print(&Ast::Register(WithLocation::new(loc.clone(), *b))),
                    pretty_print(&Ast::Register(WithLocation::new(loc.clone(), *i)))
                ),
                (Some(b), Some(i), None, Some(d)) => format!(
                    "[{} + {} + {} ]",
                    pretty_print(d),
                    pretty_print(&Ast::Register(WithLocation::new(loc.clone(), *b))),
                    pretty_print(&Ast::Register(WithLocation::new(loc.clone(), *i)))
                ),
                (Some(b), Some(i), Some(s), None) => format!(
                    "[{} + {} * {} ]",
                    pretty_print(&Ast::Register(WithLocation::new(loc.clone(), *b))),
                    pretty_print(&Ast::Register(WithLocation::new(loc.clone(), *i))),
                    *s as u64
                ),
                (Some(b), Some(i), Some(s), Some(d)) => format!(
                    "[{} + {} + {} * {}]",
                    pretty_print(d),
                    pretty_print(&Ast::Register(WithLocation::new(loc.clone(), *b))),
                    pretty_print(&Ast::Register(WithLocation::new(loc.clone(), *i))),
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
        Ast::EOS => String::new()
    }
}
