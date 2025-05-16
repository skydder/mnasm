// 1. see the part of macro: (name, stream)
// 2. find the way of expansion: (name, stream) + expander
// 3. expand: (name, stream(Vec<TokenKind>)) | expander -> stream (AST)

mod macro_data;
use crate::macro_data::MacroData;
use data::Ast;

pub fn expand_macro(ast: Vec<Ast<'_>>) -> Vec<Ast<'_>> {
    let data = MacroData::new();
    let mut expanded = Vec::new();
    for a in ast {
        expanded.push(expand(a, &data));
    }
    expanded
}

pub fn expand<'code>(ast: Ast<'code>, macro_data: &MacroData) -> Ast<'code> {
    match ast {
        Ast::Macro(name, stream) => {
            let expander = macro_data.get(name.data()).unwrap();
            expander.expand(stream)
        }
        _ => ast
    }
}