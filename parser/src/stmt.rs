use data::Stmt;
use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

pub fn parse_stmt<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<Stmt<'a>> {
    let currrent_token = tokenizer.peek_token();
    if !currrent_token.is_identifier() {
        return None;
    } else if currrent_token.is(TokenKind::Space) {
        emit_error!(currrent_token.location, "Indent error")
    }

    let ins = currrent_token.get_identifier().unwrap();
    tokenizer.next_symbol();
    tokenizer.expect_symbol(TokenKind::OpenParenthesis);
    tokenizer.expect_symbol(TokenKind::CloseParenthesis);
    tokenizer.skip_space();
    Some(Stmt {
        instruction: ins,
        operand: (),
        location: currrent_token.location,
    })
}
