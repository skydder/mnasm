use std::{cell::RefCell, collections::HashMap, rc::Rc};

use data::Ident;
use util::{AsmResult, Location, TokenKind, Tokenizer, pair_end};

use crate::macro_tokenizer::MacroTokenizer;

#[derive(Debug, Clone)]
pub enum Expander {
    Replace((Rc<Vec<TokenKind>>, Rc<Vec<TokenKind>>)), // args, stream
    Definition,
}

impl Expander {
    pub fn expand<'code>(
        &self,
        macro_data: MacroData,
        stream: Rc<Vec<TokenKind>>,
    ) -> AsmResult<'code, Rc<Vec<TokenKind>>> {
        match self {
            Expander::Definition => {
                // name shoulb be "macro_def"
                // stream should be following cases
                // <macro_name> '(' <args>* ')' "=>" { <newline> <stream> <newline> }
                eprintln!("read macro-def");
                let tokenizer = Rc::new(MacroTokenizer::new(Location::default(), stream));
                tokenizer.consume_token(TokenKind::OpenParenthesis)?; 
                let macro_name = match tokenizer.peek_token().kind {
                    TokenKind::Identifier(ident) => Ident::new(ident.to_string()),
                    _ => {
                        eprintln!("e: {:?}", tokenizer.peek_token());
                        return Err(util::AsmError::ParseError(
                            tokenizer.location(),
                            "expected Ident, but found others".to_string(),
                            String::new(),
                        ));
                    }
                };
                tokenizer.next_token();
                tokenizer.skip_space();
                tokenizer.consume_token(TokenKind::OpenParenthesis)?;
                tokenizer.skip_space();
                let mut params = Vec::new();
                parse_args(tokenizer.clone(), &mut params)?;
                tokenizer.consume_token(TokenKind::CloseParenthesis)?;
                tokenizer.skip_space();
                tokenizer.consume_token(TokenKind::Arcane('='))?;
                tokenizer.consume_token(TokenKind::GreaterThan)?;
                tokenizer.skip_space();
                let mut def_stream = Vec::new();
                if tokenizer.peek_token().is(&TokenKind::OpenBrace) {
                    // tokenizer.next_token();
                    parse_stream(tokenizer.clone(), &mut def_stream)?;
                    tokenizer.consume_token(TokenKind::CloseParenthesis)?;
                } else {
                    eprintln!("woe");
                    while !matches!(
                        tokenizer.peek_token().kind,
                        TokenKind::EOS | TokenKind::NewLine
                    ) {
                        def_stream.push(tokenizer.next_token().kind);
                    }
                    if !tokenizer.peek_token().is(&TokenKind::CloseParenthesis) {
                        return Err(util::AsmError::ParseError(
                            tokenizer.location(),
                            "use multiple line stream, use {}".to_string(),
                            String::new(),
                        ));
                    }
                }
                eprintln!("wow");
                macro_data.register_macro(
                    macro_name,
                    Expander::Replace((Rc::new(params), Rc::new(def_stream))),
                );

                Ok(Rc::new(Vec::new()))
            }
            Expander::Replace((params, def_stream)) => {
                // 1. anlyze the stream
                // 2. connect the anlyzed stream and args
                // 3. load the def_stream and relpace them
                eprintln!("relpase");
                let stream_tokenizer = Rc::new(MacroTokenizer::new(Location::default(), stream));
                let mut args = Vec::new();
                stream_tokenizer.consume_token(TokenKind::OpenParenthesis)?;
                parse_params(stream_tokenizer, &mut args)?;
                eprintln!("paa");
                let replacement_table = if params.len() == args.len() {
                    params
                        .iter()
                        .cloned()
                        .zip(args)
                        .collect::<HashMap<TokenKind, Vec<TokenKind>>>()
                } else {
                    unimplemented!("should be error")
                };

                let def_stream_tokenizer =
                    Rc::new(MacroTokenizer::new(Location::default(), def_stream.clone()));
                let mut output_stream = Vec::new();
                while !def_stream_tokenizer.peek_token().is(&TokenKind::EOS) {
                    match def_stream_tokenizer.peek_token().kind {
                        TokenKind::BackQuote => {
                            def_stream_tokenizer.next_token();
                            let param = def_stream_tokenizer.peek_token().kind;
                            output_stream
                                .append(&mut replacement_table.get(&param).unwrap().to_vec()); //todo: handle error properly
                            def_stream_tokenizer.next_token();
                        }
                        _ => output_stream.push(def_stream_tokenizer.next_token().kind),
                    }
                }

                Ok(Rc::new(output_stream))
            }
        }
    }
}

#[derive(Clone)]
pub struct MacroData {
    definition: Rc<RefCell<HashMap<Ident, Expander>>>,
}

impl MacroData {
    pub fn new() -> Self {
        Self {
            definition: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn get(&self, name: Ident) -> Option<Expander> {
        if name.get_str() == "macro_def" {
            return Some(Expander::Definition);
        }
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

fn parse_args<'code, T>(tokenizer: Rc<T>, list: &mut Vec<TokenKind>) -> AsmResult<'code, ()>
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
        eprintln!("item: {}", item);
        if matches!(item, TokenKind::Identifier(_)) {
            tokenizer.next_token();
        } else {
            return Err(util::AsmError::ParseError(
                tokenizer.location(),
                String::new(),
                String::new(),
            ));
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
        TokenKind::OpenBrace
        | TokenKind::OpenParenthesis
        | TokenKind::OpenSquareBracket => {
            let open = tokenizer.next_token().kind;
            list.push(open.clone());
            open
        }
        _ => {
            eprintln!("st:item:{:?}", tokenizer.peek_token().kind);
            return Err(util::AsmError::ParseError(
                tokenizer.location(),
                String::new(),
                String::new(),
            ));
        }
    };
    let close = pair_end(&open);
    while !tokenizer.peek_token().is(&close) {
        match tokenizer.peek_token().kind {
            TokenKind::OpenBrace => parse_stream(tokenizer.clone(), list)?,
            _ => list.push(tokenizer.next_token().kind),
        }
    }
    tokenizer.consume_token(close.clone())?;
    list.push(close);
    Ok(())
}

fn parse_params<'code, T>(tokenizer: Rc<T>, list: &mut Vec<Vec<TokenKind>>) -> AsmResult<'code, ()>
where
    T: Tokenizer<'code>,
{
    tokenizer.skip_space();
    if tokenizer.peek_token().is(&TokenKind::CloseParenthesis) {
        Ok(())
    } else if tokenizer.peek_token().is(&TokenKind::Comma) {
        tokenizer.next_token();
        parse_params(tokenizer, list)
    } else {
        let mut stream = Vec::new();
        // let item = tokenizer.peek_token().kind;
        while !tokenizer.peek_token().is(&TokenKind::Comma) {
            match tokenizer.peek_token().kind {
                TokenKind::OpenBrace
                | TokenKind::OpenParenthesis
                | TokenKind::OpenSquareBracket => {
                    parse_stream(tokenizer.clone(), &mut stream)?;
                }
                _ => stream.push(tokenizer.next_token().kind),
            }
        }
        tokenizer.consume_token(TokenKind::Comma)?;
        list.push(stream);
        parse_params(tokenizer, list)
    }
}
