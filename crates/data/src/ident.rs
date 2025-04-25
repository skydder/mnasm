use std::{rc::Rc, sync::atomic::AtomicUsize};

use util::Location;

#[derive(Debug, Clone, PartialEq)]
enum Label {
    Named(String),
    Nameless(usize)
}

impl Label {
    fn as_string(&self) -> String {
        match self {
            Label::Named(name) => name.clone(),
            Label::Nameless(c) => format!("anonymous{}", c),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ident<'code> {
    location: Location<'code>,
    label: Rc<Label>,
}

static COUNT: AtomicUsize = AtomicUsize::new(0);

impl<'code> Ident<'code> {
    pub fn new(label: String, location: Location<'code>) -> Self {
        Self { location, label: Rc::new(Label::Named(label)) }
    }
    pub fn location(&self) -> Location<'code> {
        self.location.clone()
    }

    pub fn get_str(&self) -> String {
        self.label.as_string()
    }

    pub fn anonymous_ident(location: Location<'code>) -> Self {
        let new = Self {
            location,
            label: Rc::new(Label::Nameless(COUNT.load(std::sync::atomic::Ordering::Relaxed))),
        };
        COUNT.fetch_add(1, std::sync::atomic::Ordering::Release);
        new
    }

    pub fn is_anonymous(&self) -> bool {
        matches!(*self.label, Label::Nameless(_))
    }
}
