use std::rc::Rc;

use crate::{consume_token, peek_token, DSLError, DSLResult, Data, KeyWord, Operator, Token, AST};

pub fn parse<'a>(token_seq: &Vec<Token<'a>>) -> DSLResult<AST<'a>> {
    let mut counter: usize = 0;
    let mut list = Vec::new();
    while peek_token(token_seq, &mut counter) != None {
        list.push(parse_fn(token_seq, &mut counter)?);
    }
    Ok(AST::List(Rc::new(list)))
}

fn parse_fn<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST<'a>> {
    consume_token(Token::KeyWord(KeyWord::Fn), token_seq, counter)?;
    let name = match peek_token(token_seq, counter) {
        Some(Token::Identifier(s)) => {
            Some(AST::Data(Rc::new(Data::Symbol(Rc::new(s.to_string())))))
        }
        _ => None,
    }
    .ok_or(DSLError::Parse(format!(
        "expected identifier, but found other"
    )))?;
    *counter += 1;
    consume_token(Token::KeyWord(KeyWord::OpenParenthesis), token_seq, counter)?;
    let mut args = Vec::new();
    parse_list(token_seq, counter, &mut args, KeyWord::CloseParenthesis)?;
    consume_token(
        Token::KeyWord(KeyWord::CloseParenthesis),
        token_seq,
        counter,
    )?;
    let body = parse_block(token_seq, counter)?;
    let list = vec![name, AST::List(Rc::new(args)), body];
    Ok(AST::List(Rc::new(list)))
}

fn parse_stmt<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST<'a>> {
    match peek_token(token_seq, counter) {
        Some(Token::KeyWord(KeyWord::OpenBrace)) => parse_block(token_seq, counter),
        Some(Token::KeyWord(KeyWord::If)) => parse_if(token_seq, counter),
        Some(Token::KeyWord(KeyWord::Let)) => parse_let(token_seq, counter),
        Some(Token::KeyWord(KeyWord::While)) => parse_while(token_seq, counter),
        Some(Token::KeyWord(KeyWord::Break)) => parse_break(token_seq, counter),
        _ => parse_expr_stmt(token_seq, counter),
    }
}

fn parse_while<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST<'a>> {
    consume_token(Token::KeyWord(KeyWord::While), token_seq, counter)?;
    let cond = parse_expr(token_seq, counter)?;
    eprintln!("test: {:?}", cond);
    let then = parse_block(token_seq, counter)?;
    Ok(AST::Expr(
        Operator::While,
        Rc::new(AST::List(Rc::new(vec![cond, then]))),
        None,
    ))
}

fn parse_block<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST<'a>> {
    consume_token(Token::KeyWord(KeyWord::OpenBrace), token_seq, counter)?;
    let mut list = Vec::new();
    while peek_token(token_seq, counter) != Some(Token::KeyWord(KeyWord::CloseBrace)) {
        list.push(parse_stmt(token_seq, counter)?);
    }
    consume_token(Token::KeyWord(KeyWord::CloseBrace), token_seq, counter)?;
    Ok(AST::List(Rc::new(list)))
}

fn parse_if<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST<'a>> {
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
        Operator::If,
        Rc::new(AST::List(Rc::new(vec![cond, then, _else]))),
        None,
    ))
}

fn parse_let<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST<'a>> {
    consume_token(Token::KeyWord(KeyWord::Let), token_seq, counter)?;
    let lhs = parse_data(token_seq, counter)?;
    consume_token(Token::KeyWord(KeyWord::Assign), token_seq, counter)?;
    let rhs = parse_expr(token_seq, counter)?;
    consume_token(Token::KeyWord(KeyWord::SemiColon), token_seq, counter)?;
    Ok(AST::Expr(Operator::Let, Rc::new(lhs), Some(Rc::new(rhs))))
}

fn parse_break<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST<'a>> {
    consume_token(Token::KeyWord(KeyWord::Break), token_seq, counter)?;
    consume_token(Token::KeyWord(KeyWord::SemiColon), token_seq, counter)?;
    Ok(AST::Expr(
        Operator::Break,
        Rc::new(AST::Data(Rc::new(Data::None))),
        None,
    ))
}

fn parse_expr_stmt<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST<'a>> {
    let expr = parse_expr(token_seq, counter)?;
    consume_token(Token::KeyWord(KeyWord::SemiColon), token_seq, counter)?;
    Ok(expr)
}

fn parse_expr<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST<'a>> {
    parse_assign(token_seq, counter)
}

fn parse_assign<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST<'a>> {
    let lhs = parse_logical(token_seq, counter)?;
    let op = match peek_token(token_seq, counter) {
        Some(Token::KeyWord(KeyWord::AddAssign)) => Operator::Add,
        Some(Token::KeyWord(KeyWord::MulAssign)) => Operator::Mul,
        _ => return Ok(lhs),
    };
    *counter += 1;
    let lhs = Rc::new(lhs);
    let rhs = parse_logical(token_seq, counter)?;
    Ok(AST::Expr(
        Operator::Assign,
        lhs.clone(),
        Some(Rc::new(AST::Expr(op, lhs, Some(Rc::new(rhs))))),
    ))
}

fn parse_logical<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST<'a>> {
    let lhs = parse_cmp(token_seq, counter)?;
    let op = match peek_token(token_seq, counter) {
        Some(Token::KeyWord(KeyWord::LOr)) => Operator::LOr,
        Some(Token::KeyWord(KeyWord::LAnd)) => Operator::LAnd,
        _ => return Ok(lhs),
    };
    *counter += 1;
    let rhs = parse_logical(token_seq, counter)?;
    Ok(AST::Expr(op, Rc::new(lhs), Some(Rc::new(rhs))))
}

fn parse_cmp<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST<'a>> {
    let lhs = parse_rel(token_seq, counter)?;
    let op = match peek_token(token_seq, counter) {
        Some(Token::KeyWord(KeyWord::Equal)) => Operator::CmpEqual,
        _ => return Ok(lhs),
    };
    *counter += 1;
    let rhs = parse_cmp(token_seq, counter)?;
    Ok(AST::Expr(op, Rc::new(lhs), Some(Rc::new(rhs))))
}

fn parse_rel<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST<'a>> {
    let lhs = parse_add(token_seq, counter)?;
    match peek_token(token_seq, counter) {
        Some(Token::KeyWord(KeyWord::LessThan)) => {
            let op = Operator::CmpLessThan;
            *counter += 1;
            let rhs = parse_rel(token_seq, counter)?;
            Ok(AST::Expr(op, Rc::new(lhs), Some(Rc::new(rhs))))
        }
        Some(Token::KeyWord(KeyWord::NoMoreThan)) => {
            let op = Operator::CmpNoMoreThan;
            *counter += 1;
            let rhs = parse_rel(token_seq, counter)?;
            Ok(AST::Expr(op, Rc::new(lhs), Some(Rc::new(rhs))))
        }
        Some(Token::KeyWord(KeyWord::MoreThan)) => {
            let op = Operator::CmpNoMoreThan;
            *counter += 1;
            let rhs = parse_rel(token_seq, counter)?;
            Ok(AST::Expr(op, Rc::new(rhs), Some(Rc::new(lhs))))
        }
        Some(Token::KeyWord(KeyWord::NoLessThan)) => {
            let op = Operator::CmpLessThan;
            *counter += 1;
            let rhs = parse_rel(token_seq, counter)?;
            Ok(AST::Expr(op, Rc::new(rhs), Some(Rc::new(lhs))))
        }
        _ => Ok(lhs),
    }
}

fn parse_add<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST<'a>> {
    let lhs = parse_mul(token_seq, counter)?;
    let op = match peek_token(token_seq, counter) {
        Some(Token::KeyWord(KeyWord::Add)) => Operator::Add,
        _ => return Ok(lhs),
    };
    *counter += 1;
    let rhs = parse_add(token_seq, counter)?;
    Ok(AST::Expr(op, Rc::new(lhs), Some(Rc::new(rhs))))
}

fn parse_mul<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST<'a>> {
    let lhs = parse_postfix(token_seq, counter)?;
    let op = match peek_token(token_seq, counter) {
        Some(Token::KeyWord(KeyWord::Astar)) => Operator::Mul,
        _ => return Ok(lhs),
    };
    *counter += 1;
    let rhs = parse_mul(token_seq, counter)?;
    Ok(AST::Expr(op, Rc::new(lhs), Some(Rc::new(rhs))))
}

fn parse_postfix<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST<'a>> {
    let lhs = parse_data(token_seq, counter)?;
    match peek_token(token_seq, counter) {
        Some(Token::KeyWord(KeyWord::OpenParenthesis)) => {
            // fn-call
            *counter += 1;
            let mut list = Vec::new();
            parse_list(token_seq, counter, &mut list, KeyWord::CloseParenthesis)?;
            consume_token(
                Token::KeyWord(KeyWord::CloseParenthesis),
                token_seq,
                counter,
            )?;
            if lhs.get_data().is_some_and(|d| d.get_symbol().is_some()) {
                match lhs.get_data().unwrap().get_symbol().unwrap().as_str() {
                    "print" => {
                        return Ok(AST::Expr(
                            Operator::Print,
                            Rc::new(AST::List(Rc::new(list))),
                            None,
                        ));
                    }
                    "len" => {
                        return Ok(AST::Expr(
                            Operator::Len,
                            Rc::new(AST::List(Rc::new(list))),
                            None,
                        ));
                    }
                    "is_digit" => {
                        return Ok(AST::Expr(
                            Operator::IsDigit,
                            Rc::new(AST::List(Rc::new(list))),
                            None,
                        ));
                    }
                    "get_digit" => {
                        return Ok(AST::Expr(
                            Operator::GetDigit,
                            Rc::new(AST::List(Rc::new(list))),
                            None,
                        ));
                    }
                    "eval" => {
                        return Ok(AST::Expr(
                            Operator::Eval,
                            Rc::new(AST::List(Rc::new(list))),
                            None,
                        ));
                    }

                    "asm_tokenizer" => {
                        return Ok(AST::Expr(
                            Operator::TokenizerNew,
                            Rc::new(AST::List(Rc::new(list))),
                            None,
                        ));
                    }
                    "asm_next_token" => {
                        return Ok(AST::Expr(
                            Operator::TokenizerNext,
                            Rc::new(AST::List(Rc::new(list))),
                            None,
                        ));
                    }
                    "asm_peek_token" => {
                        return Ok(AST::Expr(
                            Operator::TokenizerPeek,
                            Rc::new(AST::List(Rc::new(list))),
                            None,
                        ));
                    }
                    "asm_parse" => {
                        return Ok(AST::Expr(
                            Operator::AsmParse,
                            Rc::new(AST::List(Rc::new(list))),
                            None,
                        ));
                    }
                    _ => (),
                }
            }
            Ok(AST::Expr(
                Operator::FnCall,
                Rc::new(lhs),
                Some(Rc::new(AST::List(Rc::new(list)))),
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
                        Operator::Index,
                        Rc::new(AST::List(Rc::new(vec![lhs, rhs]))),
                        None,
                    ))
                }
                Some(Token::KeyWord(KeyWord::Slice)) => {
                    *counter += 1;
                    let rhs2 = parse_data(token_seq, counter)?;
                    consume_token(
                        Token::KeyWord(KeyWord::CloseSquareBracket),
                        token_seq,
                        counter,
                    )?;
                    Ok(AST::Expr(
                        Operator::Slice,
                        Rc::new(AST::List(Rc::new(vec![lhs, rhs, rhs2]))),
                        None,
                    ))
                }
                _ => {
                    eprintln!("{:?}", peek_token(token_seq, counter));
                    todo!()
                }
            }
        }
        _ => Ok(lhs),
    }
}

fn parse_list<'a>(
    token_seq: &Vec<Token<'a>>,
    counter: &mut usize,
    list: &mut Vec<AST<'a>>,
    stopper: KeyWord,
) -> DSLResult<()> {
    match peek_token(token_seq, counter) {
        Some(Token::KeyWord(s)) if s == stopper => Ok(()),
        Some(Token::KeyWord(KeyWord::Comma)) => {
            consume_token(Token::KeyWord(KeyWord::Comma), token_seq, counter)?;
            parse_list(token_seq, counter, list, stopper)
        }
        _ => {
            list.push(parse_expr(token_seq, counter)?);
            parse_list(token_seq, counter, list, stopper)
        }
    }
}

fn parse_data<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> DSLResult<AST<'a>> {
    match peek_token(token_seq, counter) {
        Some(Token::Identifier(ident)) => {
            *counter += 1;
            Ok(AST::Data(Rc::new(Data::Symbol(Rc::new(ident.to_string())))))
        }
        Some(Token::String(str)) => {
            *counter += 1;
            let new = AST::Data(Rc::new(Data::String(Rc::new(str.to_string()))));
            Ok(new)
        }
        Some(Token::Number(i)) => {
            *counter += 1;
            let new = AST::Data(Rc::new(Data::Integer(i)));
            Ok(new)
        }
        Some(Token::KeyWord(KeyWord::OpenSquareBracket)) => {
            *counter += 1;
            let mut list = Vec::new();
            parse_list(token_seq, counter, &mut list, KeyWord::CloseSquareBracket)?;
            consume_token(
                Token::KeyWord(KeyWord::CloseSquareBracket),
                token_seq,
                counter,
            )?;
            Ok(AST::Expr(
                Operator::List,
                Rc::new(AST::List(Rc::new(list))),
                None,
            ))
        }
        _ => {
            eprintln!("{:?}", peek_token(token_seq, counter));
            todo!()
        }
    }
}
