use crate::{DSLError, DSLResult};

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Identifier(&'a str),
    KeyWord(KeyWord),
    Number(i64),
    Char(char),
    String(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum KeyWord {
    AddAssign,
    Add,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenParenthesis,
    CloseParenthesis,
}

pub fn tokenize<'a>(code: &'a str) -> DSLResult<Vec<Token<'a>>> {
    let mut counter: usize = 0;
    let mut token_seq: Vec<Token<'a>> = Vec::new();
    while counter < code.len() {
        if code.chars().nth(counter).unwrap().is_ascii_whitespace() {
            counter += 1;
            continue;
        }
        if matches!(code.chars().nth(counter).unwrap(), 'a'..='z' | 'A'..='Z' | '_') {
            let begin = counter;
            while counter < code.len()
                && matches!(code.chars().nth(counter).unwrap(), 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
            {
                counter += 1;
            }
            token_seq.push(Token::Identifier(&code[begin..counter]));
            continue;
        }
        if matches!(code.chars().nth(counter).unwrap(), '0'..='9') {
            let begin = counter;
            while counter < code.len() && matches!(code.chars().nth(counter).unwrap(), '0'..='9') {
                counter += 1;
            }
            token_seq.push(Token::Number(code[begin..counter].parse().unwrap()));
            continue;
        }
        if matches!(code.chars().nth(counter).unwrap(), '+') {
            counter += 1;
            match code.chars().nth(counter) {
                Some('=') => {
                    counter += 1;
                    token_seq.push(Token::KeyWord(KeyWord::AddAssign))
                }
                _ => token_seq.push(Token::KeyWord(KeyWord::Add)),
            }

            continue;
        }
        if matches!(code.chars().nth(counter).unwrap(), '[') {
            counter += 1;
            token_seq.push(Token::KeyWord(KeyWord::OpenSquareBracket));
            continue;
        }
        if matches!(code.chars().nth(counter).unwrap(), ']') {
            counter += 1;
            token_seq.push(Token::KeyWord(KeyWord::CloseSquareBracket));
            continue;
        }
        if matches!(code.chars().nth(counter).unwrap(), '(') {
            counter += 1;
            token_seq.push(Token::KeyWord(KeyWord::OpenParenthesis));
            continue;
        }
        if matches!(code.chars().nth(counter).unwrap(), ')') {
            counter += 1;
            token_seq.push(Token::KeyWord(KeyWord::CloseParenthesis));
            continue;
        }
        if matches!(code.chars().nth(counter).unwrap(), '\"') {
            counter += 1;
            let begin = counter;
            while !matches!(code.chars().nth(counter).unwrap(), '\"') {
                counter += 1;
            }
            token_seq.push(Token::String(&code[begin..counter]));
            counter += 1;
            continue;
        }
        return Err(DSLError::Tokenize(format!(
            "unexpected letter: {}",
            code.chars().nth(counter).unwrap()
        )));
    }
    Ok(token_seq)
}


pub fn consume_token<'a>(expected_token: Token<'a>, token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<()> {
    if *token_seq.get(*counter).ok_or(DSLError::Parse("unexpected token".to_string()))? == expected_token {
        Ok(())
    } else {
        Err(DSLError::Parse("unexpected token".to_string()))
    }
}