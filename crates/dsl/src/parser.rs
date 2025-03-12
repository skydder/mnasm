use std::rc::Rc;

use crate::{consume_token, peek_token, DSLResult, Data, KeyWord, Operator, Token, AST};

pub fn parse<'a>(token_seq: &Vec<Token<'a>>) -> DSLResult<AST> {
    let mut counter: usize = 0;
    let mut list = Vec::new();
    while peek_token(token_seq, &mut counter) != None {
        list.push(parse_stmt(token_seq, &mut counter)?);
    }
    Ok(AST::List(list))
}

fn parse_stmt<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST> {
    match peek_token(token_seq, counter) {
        Some(Token::KeyWord(KeyWord::OpenBrace)) => parse_block(token_seq, counter),
        Some(Token::KeyWord(KeyWord::If)) => parse_if(token_seq, counter),
        Some(Token::KeyWord(KeyWord::Let)) => parse_let(token_seq, counter),
        _ => parse_expr_stmt(token_seq, counter),
    }
}

fn parse_block<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST> {
    consume_token(Token::KeyWord(KeyWord::OpenBrace), token_seq, counter)?;
    let mut list = Vec::new();
    while peek_token(token_seq, counter) != Some(Token::KeyWord(KeyWord::CloseBrace)) {
        list.push(parse_stmt(token_seq, counter)?);
    }
    consume_token(Token::KeyWord(KeyWord::CloseBrace), token_seq, counter)?;
    Ok(AST::List(list))
}

fn parse_if<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST> {
    consume_token(Token::KeyWord(KeyWord::If), token_seq, counter)?;
    let cond = parse_expr(token_seq, counter)?;
    let then = parse_block(token_seq, counter)?;
    let _else = if peek_token(token_seq, counter) == Some(Token::KeyWord(KeyWord::Else)) {
        consume_token(Token::KeyWord(KeyWord::Else), token_seq, counter)?;
        if peek_token(token_seq, counter) == Some(Token::KeyWord(KeyWord::If)) {
            parse_if(token_seq, counter)?
        } else {
            parse_block(token_seq, counter)?
        }
    } else {
        AST::Data(Rc::new(Data::None))
    };
    Ok(AST::Expr(
        Operator::FnCall,
        Rc::new(AST::Data(Rc::new(Data::Symbol("if".to_string())))),
        Some(Rc::new(AST::List(vec![cond, then, _else]))),
    ))
}

fn parse_let<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST> {
    consume_token(Token::KeyWord(KeyWord::Let), token_seq, counter)?;
    let lhs = parse_data(token_seq, counter)?;
    consume_token(Token::KeyWord(KeyWord::Assign), token_seq, counter)?;
    let rhs = parse_expr(token_seq, counter)?;
    consume_token(Token::KeyWord(KeyWord::SemiColon), token_seq, counter)?;
    Ok(AST::Expr(
        Operator::FnCall,
        Rc::new(AST::Data(Rc::new(Data::Symbol("let".to_string())))),
        Some(Rc::new(AST::List(vec![lhs, rhs]))),
    ))
}

fn parse_expr_stmt<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST> {
    let expr = parse_expr(token_seq, counter)?;
    consume_token(Token::KeyWord(KeyWord::SemiColon), token_seq, counter)?;
    Ok(expr)
}

fn parse_expr<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST> {
    if token_seq.get(*counter + 1).copied() == Some(Token::KeyWord(KeyWord::AddAssign)) {
        parse_assign(token_seq, counter)
    } else {
        parse_add(token_seq, counter)
    }
}

fn parse_assign<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST> {
    let lhs = parse_data(token_seq, counter)?;
    let op = match peek_token(token_seq, counter) {
        Some(Token::KeyWord(KeyWord::AddAssign)) => Operator::AddAssign,
        _ => todo!(),
    };
    *counter += 1;
    let rhs = parse_cmp(token_seq, counter)?;
    Ok(AST::Expr(op, Rc::new(lhs), Some(Rc::new(rhs))))
}


fn parse_add<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST> {
    let lhs = parse_cmp(token_seq, counter)?;
    let op = match peek_token(token_seq, counter) {
        Some(Token::KeyWord(KeyWord::Add)) => Operator::Add,
        _ => return Ok(lhs),
    };
    *counter += 1;
    let rhs = parse_add(token_seq, counter)?;
    Ok(AST::Expr(op, Rc::new(lhs), Some(Rc::new(rhs))))
}

fn parse_cmp<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST> {
    let lhs = parse_postfix(token_seq, counter)?;
    let op = match peek_token(token_seq, counter) {
        Some(Token::KeyWord(KeyWord::Equal)) => Operator::CmpEqual,
        _ => return Ok(lhs),
    };
    *counter += 1;
    let rhs = parse_cmp(token_seq, counter)?;
    Ok(AST::Expr(op, Rc::new(lhs), Some(Rc::new(rhs))))
}

fn parse_postfix<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST> {
    let lhs = parse_data(token_seq, counter)?;
    match peek_token(token_seq, counter) {
        Some(Token::KeyWord(KeyWord::OpenParenthesis)) => {
            // fn-call
            *counter += 1;
            let mut list = Vec::new();
            parse_fn_arg(token_seq, counter, &mut list)?;
            consume_token(Token::KeyWord(KeyWord::CloseParenthesis), token_seq, counter)?;
            Ok(AST::Expr(
                Operator::FnCall,
                Rc::new(lhs),
                Some(Rc::new(AST::List(list))),
            ))
        }
        Some(Token::KeyWord(KeyWord::OpenSquareBracket)) => {
            // indexing et slicing
            *counter += 1;
            let rhs = parse_data(token_seq, counter)?;
            match peek_token(token_seq, counter) {
                Some(Token::KeyWord(KeyWord::CloseSquareBracket)) => {
                    *counter += 1;
                    Ok(AST::Expr(
                        Operator::FnCall,
                        Rc::new(AST::Data(Rc::new(Data::Symbol("index".to_string())))),
                        Some(Rc::new(AST::List(vec![lhs, rhs]))),
                    ))
                }
                Some(Token::KeyWord(KeyWord::Slice)) => {
                    *counter += 1;
                    let rhs2 = parse_data(token_seq, counter)?;
                    consume_token(Token::KeyWord(KeyWord::CloseSquareBracket), token_seq, counter)?;
                    Ok(AST::Expr(
                        Operator::FnCall,
                        Rc::new(AST::Data(Rc::new(Data::Symbol("slice".to_string())))),
                        Some(Rc::new(AST::List(vec![lhs, rhs, rhs2]))),
                    ))
                }
                _ => {
                    eprintln!("{:?}", peek_token(token_seq, counter));
                    todo!()
                },
            }
        }
        _ => Ok(lhs),
    }
}

fn parse_fn_arg<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize, list: &mut Vec<AST>) -> DSLResult<()> {
    match peek_token(token_seq, counter) {
        Some(Token::KeyWord(KeyWord::CloseParenthesis)) => Ok(()),
        Some(Token::KeyWord(KeyWord::Comma)) => {
            consume_token(Token::KeyWord(KeyWord::Comma), token_seq, counter)?;
            parse_fn_arg(token_seq, counter, list)
        }
        _ => {
            list.push(parse_expr(token_seq, counter)?);
            parse_fn_arg(token_seq, counter, list)
        }
    }
}

fn parse_data<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST> {
    match peek_token(token_seq, counter) {
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
            let new = AST::Data(Rc::new(Data::Integer(i)));
            Ok(new)
        }
        _ => {
            eprintln!("{:?}", peek_token(token_seq, counter));
            todo!()
        },
    }
}
