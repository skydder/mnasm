use std::cell::RefCell;

use util::{Location, Source, Source2, Stream};

// data ()
// - dsl code
// - raw stream
// - source info

pub struct DSLData<'a> {
    source: Source2<'a>,
    raw_stream: String,
    output: RefCell<String>
}

impl<'a> DSLData<'a> {
    fn new(source: Source2<'a>, raw_stream: String) -> Self {
        Self { source: source, raw_stream: raw_stream, output: RefCell::new(String::new()) }
    }
}

pub fn read_stream<'a>(stream: Stream<'a>) -> DSLData<'a> {
    let new = DSLData::new(stream.source(), stream.stringfiy());
    new.output.borrow_mut().push_str(&new.raw_stream);
    new
}

// todo: remove used stream in Source2
pub fn eval_macro<'a>(data: DSLData<'a>) -> Stream<'a> {
    let begin = Location::new_source(data.source, Source::new(data.output.borrow().to_string(), "macro"));
    let end = begin.end();
    Stream::new(begin, end)
}
