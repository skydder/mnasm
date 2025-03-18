use util::{Location, Source, Source2, Tokenizer};

#[derive(Debug, Clone, Copy)]
pub(crate) struct TKNZR4ASM<'a> {
    location: Location<'a>,
}

impl<'a> TKNZR4ASM<'a> {
    pub(crate) fn new(input: String, original_sources: Source2<'a>) -> Self {
        Self {
            location: Location::new_source(original_sources, Source::new(input, "macro")),
        }
    }
}

#[allow(unused_variables)]
impl<'a> Tokenizer<'a> for TKNZR4ASM<'a> {
    fn location(&self) -> util::Location<'a> {
        self.location
    }

    fn peek_token(&self, macro_expand: bool) -> util::Token<'a> {
        todo!()
    }

    fn next_token(&self) -> util::Token<'a> {
        todo!()
    }

    fn skip_space(&self, macro_expand: bool) {
        todo!()
    }

    fn skip_token(&self) {
        todo!()
    }

    fn consume_token(&self, consumeing_token: util::TokenKind<'a>) {
        todo!()
    }

    fn consume_newline(&self) {
        todo!()
    }

    fn consume_indent(&self) {
        todo!()
    }

    fn add_to_code(&self, tokenkind: util::TokenKind<'a>) {
        todo!()
    }

    fn code(&self) -> String {
        todo!()
    }
}
