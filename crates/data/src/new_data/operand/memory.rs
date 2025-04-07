use util::Location;

use crate::new_data::ast::Ast;

pub struct Memory<'code> {
    location: Location<'code>,
    base: Option<Box<Ast<'code>>>,
    index: Option<Box<Ast<'code>>>,
    scale: Option<Box<Ast<'code>>>,
    disp: Option<Box<Ast<'code>>>,
}
