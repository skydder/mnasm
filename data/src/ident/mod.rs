use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct _Ident<'a> {
    name: &'a str,
}

#[allow(unused)]
static C: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Ident<'a> {
    Named(&'a str),
    Unnamed(u64)
}

impl<'a> Ident<'a> {
    pub fn new(name: &'a str) -> Self {
        Self::Named(name)
    }

    pub fn new_unnamed() -> Self {
        let new = Self::Unnamed(C.load(Ordering::Relaxed));
        C.fetch_add(1, Ordering::Relaxed);
        new
    }

    pub fn get(&self) -> String {
        match self {
            Ident::Named(name) => format!("{}", name),
            Ident::Unnamed(c) => format!("N_L_L_{}", c),
        }
    }
}

impl<'a> _Ident<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name: name,
        }
    }

    pub fn get(&self) -> String {
        format!("{}", &self.name[0..])
    }
}

impl<'a> std::fmt::Display for Ident<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}
