use tokenizer::{TokenKind, Tokenizer};

pub fn parse_null_stmt<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<()> {
    if !tokenizer.peek_token().is(TokenKind::Space) && !tokenizer.peek_token().is(TokenKind::NewLine) {
        return None;
    }
    tokenizer.skip_space();
    if tokenizer.peek_token().is(TokenKind::NewLine) {
        Some(())
    } else {
        None
    }
}