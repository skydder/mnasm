use data::{CompoundIns, Ins, Stmt};
use tokenizer::{TokenKind, Tokenizer};
use util::emit_error;

pub fn parse_ins<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<Ins<'a>> {
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
    Some(Ins {
        instruction: ins,
        operand: (),
        location: currrent_token.location,
    })
}

pub fn parse_compound_ins<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<CompoundIns<'a>> {
    let mut compound = Vec::new();
    parse_compound_ins_inside(tokenizer, &mut compound);
    Some(CompoundIns { compound: compound })
}

fn parse_compound_ins_inside<'a>(tokenizer: &'a Tokenizer<'a>, compound: &mut Vec<Ins<'a>>) {
    compound.push(parse_ins(tokenizer).unwrap_or_else(|| emit_error!(tokenizer.location(), "expected ins, but found others")));
    match tokenizer.peek_token().kind {
        TokenKind::NewLine => {
            return;
        },
        TokenKind::Comma => {
            tokenizer.next_token();
            tokenizer.skip_space();
            parse_compound_ins_inside(tokenizer, compound);
        },
        _ => {
            emit_error!(tokenizer.location(), "invalid expression");
        }
    }
}