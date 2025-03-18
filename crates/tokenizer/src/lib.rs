mod macro_related;
// mod token;
mod tokenizer;
mod tokenizer2;

// pub(crate) use tokenizer;
pub(crate) use macro_related::{read_macro_call, read_macro_call_dsl, read_macro_def, Macro};
// pub use token::{Token, TokenKind};
pub use tokenizer2::{MacroStatus, Tokenizer2};
