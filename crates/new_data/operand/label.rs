use util::Location;

#[derive(Clone, PartialEq)]
pub struct Label<'code> {
    location: Location<'code>,
    label: &'code str,
}
