use super::operand::Label;

pub struct Path<'code> {
    is_relative: bool,
    path: Vec<Label<'code>>,
}

#[allow(clippy::needless_lifetimes)]
impl<'code> Path<'code> {
    pub fn is_relative(&self) -> bool {
        self.is_relative
    }

    pub fn path(&self) -> Vec<Label<'code>> {
        self.path.clone()
    }
}