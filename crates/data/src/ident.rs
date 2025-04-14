use std::{rc::Rc, sync::atomic::AtomicUsize};

use util::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Ident<'code> {
    location: Location<'code>,
    label: Rc<String>,
}

static COUNT: AtomicUsize = AtomicUsize::new(0);

impl<'code> Ident<'code> {
    pub fn new(label: Rc<String>, location: Location<'code>) -> Self {
        Self { location, label }
    }
    pub fn location(&self) -> Location<'code> {
        self.location.clone()
    }

    pub fn get_str(&self) -> &str {
        self.label.as_str()
    }

    pub fn anonymous_ident(location: Location<'code>) -> Self {
        let new = Self {
            location,
            label: Rc::new(format!(
                "anonymous{}",
                COUNT.load(std::sync::atomic::Ordering::Relaxed)
            )),
        };
        COUNT.fetch_add(1, std::sync::atomic::Ordering::Release);
        new
    }
}
