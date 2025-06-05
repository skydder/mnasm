use std::{collections::HashMap, rc::Rc};

use util::{pair_end, AsmResult, Location, TokenKind, Tokenizer};

use crate::MacroTokenizer;



pub fn replaced_stream<'code>(stream: Rc<Vec<TokenKind>>, replacement_table: HashMap<TokenKind, Vec<TokenKind>>) -> AsmResult<'code, Vec<TokenKind>> {
    let def_stream_tokenizer =
        Rc::new(MacroTokenizer::new(Location::default(), stream.clone()));

    let mut replaced_stream = Vec::new();
    
    while !def_stream_tokenizer.peek_token().is(&TokenKind::EOS) {
        match def_stream_tokenizer.peek_token().kind {
            TokenKind::BackQuote => {
                def_stream_tokenizer.next_token();
                let param = def_stream_tokenizer.peek_token().kind;
                replaced_stream
                    .append(&mut replacement_table.get(&param).unwrap().to_vec()); //todo: handle error properly
                def_stream_tokenizer.next_token();
            }
            _ => replaced_stream.push(def_stream_tokenizer.next_token().kind),
        }
    }
    
    Ok(replaced_stream)
}

pub fn parse_def_stream<'code, T>(tokenizer: Rc<T>) -> AsmResult<'code, Vec<TokenKind>>
where
    T: Tokenizer<'code>,
{
    let mut def_stream = Vec::new();
    if tokenizer.peek_token().is(&TokenKind::OpenBrace) {
        // tokenizer.next_token();
        parse_stream(tokenizer.clone(), &mut def_stream)?;
        tokenizer.consume_token(TokenKind::CloseParenthesis)?;
    } else {
        // eprintln!("woe");
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
    // def_stream.push(TokenKind::NewLine);
    Ok(def_stream)
}


pub fn parse_args<'code, T>(tokenizer: Rc<T>, list: &mut Vec<TokenKind>) -> AsmResult<'code, ()>
where
    T: Tokenizer<'code>,
{
    tokenizer.skip_space();
    if tokenizer.peek_token().is(&TokenKind::CloseParenthesis) {
        tokenizer.next_token();
        Ok(())
    } else if tokenizer.peek_token().is(&TokenKind::Comma) {
        tokenizer.next_token();
        parse_args(tokenizer, list)
    } else {
        let item = tokenizer.peek_token().kind;
        // eprintln!("item: {}", item);
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

pub fn parse_stream<'code, T>(tokenizer: Rc<T>, list: &mut Vec<TokenKind>) -> AsmResult<'code, ()>
where
    T: Tokenizer<'code>,
{
    // eprintln!("ps_start");
    let open = match tokenizer.peek_token().kind {
        TokenKind::OpenBrace | TokenKind::OpenParenthesis | TokenKind::OpenSquareBracket => {
            let open = tokenizer.next_token().kind;
            list.push(open.clone());
            open
        }
        _ => {
            // eprintln!("st:item:{:?}", tokenizer.peek_token().kind);
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
    // eprintln!("ps_end");
    Ok(())
}

pub fn parse_params<'code, T>(tokenizer: Rc<T>, list: &mut Vec<Vec<TokenKind>>) -> AsmResult<'code, ()>
where
    T: Tokenizer<'code>,
{
    // eprintln!("pp_start");
    tokenizer.skip_space();
    if tokenizer.peek_token().is(&TokenKind::CloseParenthesis) {
        // eprintln!("pp_end");
        tokenizer.next_token();
        Ok(())
    } else if tokenizer.peek_token().is(&TokenKind::Comma) {
        tokenizer.next_token();
        parse_params(tokenizer, list)
    } else {
        let mut stream = Vec::new();
        // let item = tokenizer.peek_token().kind;
        while !matches!(
            tokenizer.peek_token().kind,
            TokenKind::Comma | TokenKind::CloseParenthesis
        ) {
            match tokenizer.peek_token().kind {
                TokenKind::OpenBrace
                | TokenKind::OpenParenthesis
                | TokenKind::OpenSquareBracket => {
                    parse_stream(tokenizer.clone(), &mut stream)?;
                }
                _ => stream.push(tokenizer.next_token().kind),
            }
        }
        // tokenizer.consume_token(TokenKind::Comma)?;
        list.push(stream);
        parse_params(tokenizer, list)
    }
}
