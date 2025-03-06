use std::rc::Rc;

use crate::{tokenizer::consume_token, DSLResult, Data, KeyWord, Operator, Token, AST};

pub fn parse<'a>(token_seq: &Vec<Token<'a>>) -> DSLResult<AST> {
    let mut counter: usize = 0;
    let new = parse_assign(token_seq, &mut counter);
    // eprintln!("{:?}", new);
    new
}

fn parse_assign<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST> {
    let lhs = parse_data(token_seq, counter)?;
    let op = match token_seq[*counter] {
        Token::KeyWord(KeyWord::AddAssign) => Operator::AddAssign,
        _ => todo!(),
    };
    *counter += 1;
    let rhs = parse_add(token_seq, counter)?;
    Ok(AST::Expr(op, Rc::new(lhs), Some(Rc::new(rhs))))
}

fn parse_add<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST> {
    let lhs = parse_postfix(token_seq, counter)?;
    let op = match token_seq.get(*counter) {
        Some(Token::KeyWord(KeyWord::Add)) => Operator::Add,
        _ => return Ok(lhs),
    };
    *counter += 1;
    let rhs = parse_add(token_seq, counter)?;
    Ok(AST::Expr(op, Rc::new(lhs), Some(Rc::new(rhs))))
}

fn parse_postfix<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST> {
    let lhs = parse_data(token_seq, counter)?;
    match token_seq.get(*counter) {
        Some(Token::KeyWord(KeyWord::OpenParenthesis)) => {
            // fn-call
            *counter += 1;
            let rhs = parse_data(token_seq, counter)?;
            Ok(AST::Expr(
                Operator::FnCall,
                Rc::new(lhs),
                Some(Rc::new(AST::List(vec![rhs]))),
            ))
        }
        Some(Token::KeyWord(KeyWord::OpenSquareBracket)) => {
            // indexing
            *counter += 1;
            let rhs = parse_data(token_seq, counter)?;
            consume_token(Token::KeyWord(KeyWord::CloseSquareBracket), token_seq, counter)?;
            Ok(AST::Expr(
                Operator::FnCall,
                Rc::new(AST::Data(Rc::new(Data::Symbol("index".to_string())))),
                Some(Rc::new(AST::List(vec![lhs, rhs]))),
            ))
        }
        _ => Ok(lhs),
    }
}

fn parse_data<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST> {
    match token_seq.get(*counter) {
        Some(Token::Identifier(ident)) => {
            *counter += 1;
            Ok(AST::Data(Rc::new(Data::Symbol(ident.to_string()))))
        }
        Some(Token::String(str)) => {
            *counter += 1;
            let new = AST::Data(Rc::new(Data::String(str.to_string())));
            Ok(new)
        }
        Some(Token::Number(i)) => {
            *counter += 1;
            let new = AST::Data(Rc::new(Data::Integer(*i)));
            Ok(new)
        }
        _ => todo!(),
    }
}
