use tokenizer::{TokenKind, Tokenizer};
use util::Location;
pub struct Stmt<'a> {
    instruction: &'a str,
    operand: (),
    location: Location<'a>,
}

pub fn parse_stmt<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<Stmt<'a>> {
    let currrent_token = tokenizer.peek_token();
    if !currrent_token.is_identifier() {
        return None;
    }

    let ins = currrent_token.get_identifier().unwrap();
    tokenizer.expect_token(TokenKind::OpenParenthesis);
    tokenizer.expect_token(TokenKind::CloseParenthesis);
    Some(Stmt { instruction: ins, operand: (), location: currrent_token.location })    
}

