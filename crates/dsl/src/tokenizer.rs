use std::rc::Rc;

use crate::{Data, Operator, AST};

#[derive(Debug)]
pub enum Token<'a> {
    Identifier(&'a str),
    KeyWord(KeyWord),
    Number(i64),
    Char(char),
    String(&'a str)
}

#[derive(Debug)]
pub enum KeyWord {
    AddAssign
}

fn is_letter_for_identifier(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => true,
        _ => false,
    }
}

pub fn tokenize<'a>(code: &'a str) -> Vec<Token<'a>> {
    let mut counter: usize = 0;
    let mut token_seq: Vec<Token<'a>> = Vec::new();
    while counter < code.len() {
        if code.chars().nth(counter).unwrap().is_ascii_whitespace() {
            counter += 1;
            continue;
        }
        if matches!(code.chars().nth(counter).unwrap(), 'a'..='z' | 'A'..='Z' | '_') {
            let begin = counter;
            while counter < code.len() && is_letter_for_identifier(code.chars().nth(counter).unwrap()) {
                counter += 1;
            }
            token_seq.push(Token::Identifier(&code[begin..counter]));
            continue;
        }
        if matches!(code.chars().nth(counter).unwrap(), '+') {
            counter += 1;
            match code.chars().nth(counter) {
                Some('=') => token_seq.push(Token::KeyWord(KeyWord::AddAssign)),
                _ => todo!(),
            }
            counter += 1;
            continue;
        }
        if matches!(code.chars().nth(counter).unwrap(), '\"') {
            counter += 1;
            let begin = counter;
            while !matches!(code.chars().nth(counter).unwrap(), '\"') {
                counter += 1;
            }
            // eprintln!("{}", &code[begin..counter]);
            token_seq.push(Token::String(&code[begin..counter]));
            // eprintln!("{:?}", token_seq);
            counter += 1; 
            continue;
        }
        // eprintln!("{}", code.chars().nth(counter).unwrap());
        todo!()

    }
    token_seq
}

pub fn parse<'a>(token_seq: &Vec<Token<'a>>) -> AST {
    let mut counter: usize = 0;
    let new = parse_assign(token_seq, &mut counter);
    // eprintln!("{:?}", new);
    new
}

fn parse_assign<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> AST {
    let lhs = parse_data(token_seq, counter);
    let op = match token_seq[*counter] {
        Token::KeyWord(KeyWord::AddAssign) => Operator::AddAssign,
        _ => todo!()
    };
    *counter += 1;
    let rhs = parse_data(token_seq, counter);
    AST::Expr(op, Box::new(lhs), Some(Box::new(rhs)))
}

fn parse_data<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> AST {
    match token_seq[*counter] {
        Token::Identifier(ident) => {
            *counter += 1;
            // eprintln!("{}", ident);
            AST::Data(Rc::new(Data::Symbol(ident.to_string())))
        },
        Token::String(str) => {
            // eprintln!("{}", str);
            *counter += 1;
            let new = AST::Data(Rc::new(Data::String(str.to_string())));
            // eprintln!("parse: {:?}", new);
            new
        }
        _ => todo!()
    }
}