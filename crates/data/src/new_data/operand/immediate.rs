use util::Location;

pub struct Immediate<'code> {
    location: Location<'code>,
    data: u64,
}
