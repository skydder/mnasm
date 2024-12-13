use util::Location;

use crate::{Analyze, Codegen, Object};


pub struct PseudoIns<'a> {
    pub instruction: &'a str,
    pub operands: &'a str,
    pub location: Location<'a>,
}

impl<'a> Object for PseudoIns<'a> {}
impl<'a> Analyze for PseudoIns<'a> {
    fn analyze(&self) {
        todo!()
    }
}
impl<'a> Codegen for PseudoIns<'a>  {
    fn codegen(&self) -> String {
        todo!()
    }
}