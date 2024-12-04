use data::{CompoundIns, Ins};
use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

pub fn parse_ins<'a>(tokenizer: &'a Tokenizer<'a>) -> Ins<'a> {
    let currrent_token = tokenizer.peek_token();
    assert!(currrent_token.is_identifier());

    let ins = currrent_token.get_identifier().unwrap();
    tokenizer.next_symbol();
    tokenizer.expect_symbol(TokenKind::OpenParenthesis);
    tokenizer.expect_symbol(TokenKind::CloseParenthesis);
    // tokenizer.skip_space();
    Ins::new(ins, currrent_token.location)
}

pub fn parse_compound_ins<'a>(tokenizer: &'a Tokenizer<'a>) -> CompoundIns<'a> {
    let mut compound = Vec::new();
    let loc = tokenizer.location();
    parse_compound_ins_inside(tokenizer, &mut compound);
    CompoundIns::new(compound, loc)
}

fn parse_compound_ins_inside<'a>(tokenizer: &'a Tokenizer<'a>, compound: &mut Vec<Ins<'a>>) {
    compound.push(parse_ins(tokenizer));
    tokenizer.skip_space();
    match tokenizer.peek_token().kind {
        TokenKind::NewLine => {
            return;
        }
        TokenKind::Comma => {
            tokenizer.next_token();
            tokenizer.skip_space();
            parse_compound_ins_inside(tokenizer, compound);
        }
        _ => {
            emit_error!(tokenizer.location(), "invalid expression");
        }
    }
}
