use std::{cell::RefCell, collections::HashMap, rc::Rc};

use data::Ident;
use util::{pair_end, AsmResult, Location, TokenKind, Tokenizer};

use crate::macro_tokenizer::MacroTokenizer;

#[derive(Debug, Clone)]
pub enum Expander {
    Replace((Rc<Vec<TokenKind>>, Rc<Vec<TokenKind>>)), // args, stream
    Definition,
}

impl Expander {
    pub fn expand<'code>(self, macro_data: &MacroData, stream: Rc<Vec<TokenKind>>) -> AsmResult<Vec<TokenKind>> {
        match self {
            Expander::Definition => {
                // name shoulb be "macro_def"
                // stream should be following cases
                // '(' <args>* ')' "=>" { <newline> <stream> <newline> }
                let tokenizer = Rc::new(MacroTokenizer::new(Location::default(), stream));
                tokenizer.skip_space();
                tokenizer.consume_token(TokenKind::OpenParenthesis)?;
                tokenizer.skip_space();
                let mut args = Vec::new();
                parse_args(tokenizer.clone(), &mut args)?;
                tokenizer.skip_space();
                tokenizer.consume_token(TokenKind::Arcane('='))?;
                tokenizer.consume_token(TokenKind::GreaterThan)?;

                let mut def_stream = Vec::new();
                if tokenizer.peek_token().is(&TokenKind::OpenBrace) {
                    parse_stream(tokenizer.clone(), &mut def_stream)?;
                } else {
                    while !matches!(tokenizer.peek_token().kind, TokenKind::EOS | TokenKind::NewLine) {
                        def_stream.push(tokenizer.next_token().kind);
                    }
                    if !tokenizer.peek_token().is(&TokenKind::EOS) {
                        return Err(util::AsmError::ParseError(tokenizer.location(), "use multiple line stream, use {}".to_string(), String::new()));
                    }
                }
                macro_data.register_macro(todo!(), Expander::Replace((Rc::new(args), Rc::new(def_stream))));

                todo!()
            }
            Expander::Replace((args, def_stream)) => {
                // 1. anlyze the stream
                // 2. connect the anlyzed stream and args
                // 3. load the def_stream and relpace them

                todo!()
            }
        }
    }
}

pub struct MacroData {
    definition: RefCell<HashMap<Ident, Expander>>,
}

impl MacroData {
    pub fn new() -> Self {
        Self {
            definition: RefCell::new(HashMap::new()),
        }
    }

    pub fn get(&self, name: Ident) -> Option<Expander> {
        self.definition.borrow().get(&name).cloned()
    }

    pub fn register_macro(&self, name: Ident, expander: Expander) {
        self.definition.borrow_mut().insert(name, expander);
    }
}

impl Default for MacroData {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_args<'code, T>(
    tokenizer: Rc<T>,
    list: &mut Vec<TokenKind>,
) -> AsmResult<'code, ()>
where
    T: Tokenizer<'code>,
{
    tokenizer.skip_space();
    if tokenizer.peek_token().is(&TokenKind::CloseParenthesis) {
        Ok(())
    } else if tokenizer.peek_token().is(&TokenKind::Comma) {
        tokenizer.next_token();
        parse_args(tokenizer, list)
    } else {
        let item = tokenizer.peek_token().kind;
        if matches!(item, TokenKind::Identifier(_)) {
            tokenizer.next_token();
        } else {
            return Err(util::AsmError::ParseError(tokenizer.location(), String::new(), String::new()));
        }
        list.push(item);
        parse_args(tokenizer, list)
    }
}


fn parse_stream<'code, T>(tokenizer: Rc<T>, list: &mut Vec<TokenKind>) -> AsmResult<'code, ()>
where
    T: Tokenizer<'code>,
{
    let open = match tokenizer.peek_token().kind {
        TokenKind::OpenBrace => {
            let open = tokenizer.next_token().kind;
            list.push(open.clone());
            open
        }
        _ => {
            return Err(util::AsmError::ParseError(
                tokenizer.location(),
                String::new(),
                String::new(),
            ))
        }
    };
    let close = pair_end(&open);
    while !tokenizer.peek_token().is(&close) {
        match tokenizer.peek_token().kind {
            TokenKind::OpenBrace => {
                parse_stream(tokenizer.clone(), list)?
            }
            _ => list.push(tokenizer.next_token().kind),
        }
    }
    tokenizer.consume_token(close.clone())?;
    list.push(close);
    Ok(())
}
