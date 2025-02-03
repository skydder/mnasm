use crate::Ident;

#[derive(Debug, Clone)]
pub struct Path<'a> {
    is_relative: bool,
    path: Vec<Ident<'a>>,
}

impl<'a> Path<'a> {
    pub fn new(is_relative: bool, path: Vec<Ident<'a>>) -> Self {
        Self { is_relative: is_relative, path: path }
    }

    pub fn is_relative(&self) -> bool {
        self.is_relative
    }

    pub fn path_name(&self) -> String {
        let mut name = String::new();
        self._path_name(&mut name, 0);
        name
    }

    pub fn name(&self) -> Ident<'a> {
        *self.path.last().unwrap() // todo
    }

    fn _path_name(&self, name: &mut String, nth: usize) {
        if nth == self.path.len() - 1 {
            name.push_str(&self.path[nth].get());
            return;
        } else {
            name.push_str(&self.path[nth].get());
            name.push_str("__");
            self._path_name(name, nth + 1);
        }
    }
}