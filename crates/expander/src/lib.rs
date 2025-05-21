// 1. see the part of macro: (name, stream)
// 2. find the way of expansion: (name, stream) + expander
// 3. expand: (name, stream(Vec<TokenKind>)) | expander -> stream (AST)

mod macro_data;
mod macro_tokenizer;
use std::rc::Rc;

pub use crate::macro_data::MacroData;
pub use crate::macro_tokenizer::MacroTokenizer;
use data::Ast;
use util::AsmResult;

// this module is a sub-module of parser and real_expander would be implemented in parser module

// pub fn expand_macro(ast: Vec<Ast<'_>>) -> Vec<Ast<'_>> {
//     let data = MacroData::new();
//     let mut expanded = Vec::new();
//     for a in ast {
//         expanded.push(expand(a, &data));
//     }
//     expanded
// }

pub fn expand_macro<'code>(
    ast: &Ast<'code>,
    macro_data: MacroData,
) -> AsmResult<'code, Rc<Vec<util::TokenKind>>> {
    match ast {
        Ast::Macro(name, stream) => {
            eprintln!("{:?}", name);
            let expander = macro_data.get(name.data()).unwrap();
            let res = expander.expand(macro_data, stream.clone())?;
            Ok(res)
        }
        _ => unimplemented!(),
    }
}
