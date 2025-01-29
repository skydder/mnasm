// requirement
// - macro def reader => macro data
// - macro data holder
// - macro expander => (macro data + args(stream)) => stream

// macro data
// - name
// - args(name)
// - stream 

// peek get messy!!
// macro marker: @<label> ("(" (<stream>"@,")*")")?

use util::Location;

use crate::Stream;

struct Macro<'a> {
    name: &'a str,
    args: Vec<&'a str>,
    stream: Stream<'a>,
}

