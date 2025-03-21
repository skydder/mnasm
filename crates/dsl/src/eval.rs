use std::rc::Rc;

use util::Tokenizer;

use crate::{asm_tokenizer::TKNZR4ASM, data::DSLFn, DSLError, DSLResult};

use super::{Data, Environment, Operator, AST};

pub fn eval<'a>(env: &Environment<'a>, ast: &AST<'a>) -> DSLResult<Data<'a>> {
    match ast {
        AST::Expr(operator, lhs, rhs) => eval_expr(env, *operator, lhs.clone(), rhs.clone()),
        AST::Data(data) => match data.as_ref() {
            Data::Symbol(sym) => env.get_variable(sym.clone()),
            _ => Ok(data.as_ref().clone()),
        },
        AST::List(asts) => {
            let mut constant_list = Vec::new();
            for ast in asts.as_ref() {
                constant_list.push(eval(env, ast)?);
            }
            Ok(Data::List(Rc::new(constant_list)))
        }
    }
}

fn eval_expr<'a>(
    env: &Environment<'a>,
    operator: Operator,
    lhs: Rc<AST<'a>>,
    rhs: Option<Rc<AST<'a>>>,
) -> DSLResult<Data<'a>> {
    match operator {
        Operator::FnCall => apply_fn(env, lhs, rhs.unwrap()),
        _ => eval_built_in(env, operator, lhs, rhs),
    }
}

fn apply_fn<'a>(
    env: &Environment<'a>,
    fn_name: Rc<AST<'a>>,
    fn_args: Rc<AST<'a>>,
) -> DSLResult<Data<'a>> {
    let fn_info = eval(env, &fn_name)?
        .get_fn()
        .ok_or(DSLError::Eval(format!("")))?;
    let fn_args = fn_args.get_list().ok_or(DSLError::Eval(format!("")))?;
    let fn_env = env.clone().enter_fn();

    // error handling
    if fn_args.len() != fn_info.params.len() {
        return Err(DSLError::Eval(format!("")));
    }

    // prep env
    for (param, arg) in fn_info.params.iter().zip(fn_args.iter()) {
        let param_name = param
            .get_data()
            .ok_or(DSLError::Eval(format!("")))?
            .get_symbol()
            .ok_or(DSLError::Eval(format!("")))?; // todo: cbb
        let evaled_arg = eval(env, arg)?; //todo
        fn_env.push_var(param_name, evaled_arg);
    }

    eval(&fn_env, &fn_info.body)?
        .get_list()
        .unwrap()
        .last()
        .map(|d| d.clone())
        .ok_or(DSLError::Eval(format!("something is wrong")))
}

fn eval_built_in<'a>(
    env: &Environment<'a>,
    operator: Operator,
    lhs: Rc<AST<'a>>,
    rhs: Option<Rc<AST<'a>>>,
) -> DSLResult<Data<'a>> {
    match operator {
        Operator::Assign => {
            let name = lhs
                .get_data()
                .and_then(|d| d.get_symbol())
                .ok_or(DSLError::Eval(format!("")))?;
            let evaled_rhs = eval(env, rhs.unwrap().as_ref())?;
            if env
                .local
                .borrow_mut()
                .insert(name.clone(), evaled_rhs)
                .is_some()
            {
                Ok(Data::None)
            } else {
                Err(DSLError::Eval(format!("undefined variable: {}", name)))
            }
        }
        Operator::Add => {
            let evaled_lhs = eval(env, lhs.as_ref())?;
            let evaled_rhs = eval(env, rhs.unwrap().as_ref())?;
            match (evaled_lhs, evaled_rhs) {
                (Data::Integer(l), Data::Integer(r)) => Ok(Data::Integer(l + r)),
                (Data::String(l), Data::String(r)) => {
                    Ok(Data::String(Rc::new(format!("{}{}", l, r))))
                }
                (Data::List(l), r) => {
                    let mut list = l.as_ref().clone();
                    list.push(r);
                    Ok(Data::List(Rc::new(list)))
                }
                _ => todo!(),
            }
        }
        Operator::CmpEqual => {
            let evaled_lhs = eval(env, lhs.as_ref())?;
            let evaled_rhs = eval(env, rhs.unwrap().as_ref())?;
            Ok(Data::Integer((evaled_lhs == evaled_rhs) as i64))
        }
        Operator::CmpLessThan => {
            let evaled_lhs = eval(env, lhs.as_ref())?;
            let evaled_rhs = eval(env, rhs.unwrap().as_ref())?;
            match (evaled_lhs, evaled_rhs) {
                (Data::Integer(l), Data::Integer(r)) => Ok(Data::Integer((l < r) as i64)),
                _ => Ok(Data::Integer(0)),
            }
        }
        Operator::CmpNoMoreThan => {
            let evaled_lhs = eval(env, lhs.as_ref())?;
            let evaled_rhs = eval(env, rhs.unwrap().as_ref())?;
            match (evaled_lhs, evaled_rhs) {
                (Data::Integer(l), Data::Integer(r)) => Ok(Data::Integer((l <= r) as i64)),
                _ => Ok(Data::Integer(0)),
            }
        }
        Operator::Break => Ok(Data::None),
        Operator::List => {
            if let AST::List(ast_list) = lhs.clone().as_ref() {
                let mut evaled = Vec::new();
                for ast in ast_list.as_ref() {
                    evaled.push(eval(env, ast)?);
                }
                Ok(Data::List(Rc::new(evaled)))
            } else {
                Err(DSLError::Eval(format!("expected list")))
            }
        }
        Operator::LOr => {
            let evaled_lhs = eval(env, lhs.as_ref())?;
            let evaled_rhs = eval(env, rhs.unwrap().as_ref())?;
            match (evaled_lhs, evaled_rhs) {
                (Data::Integer(l), Data::Integer(r)) => match (l, r) {
                    (0, 0) => Ok(Data::Integer(0)),
                    _ => Ok(Data::Integer(0)),
                },
                _ => Ok(Data::Integer(0)),
            }
        }
        Operator::LAnd => {
            let evaled_lhs = eval(env, lhs.as_ref())?;
            match evaled_lhs.get_integer() {
                Some(i) if i != 0 => (),
                _ => return Ok(Data::Integer(0)),
            };
            let evaled_rhs = eval(env, rhs.unwrap().as_ref())?;
            match evaled_rhs.get_integer() {
                Some(i) if i != 0 => Ok(Data::Integer(1)),
                _ => Ok(Data::Integer(0)),
            }
        }
        Operator::Mul => {
            let evaled_lhs = eval(env, lhs.as_ref())?;
            let evaled_rhs = eval(env, rhs.unwrap().as_ref())?;
            match (evaled_lhs, evaled_rhs) {
                (Data::Integer(l), Data::Integer(r)) => Ok(Data::Integer(l * r)),
                _ => Err(DSLError::Eval(format!("cant mul"))),
            }
        }
        Operator::Let => {
            let name = lhs
                .get_data()
                .and_then(|d| d.get_symbol())
                .ok_or(DSLError::Eval(format!("")))?;
            let evaled_rhs = eval(env, rhs.unwrap().as_ref())?;
            env.local.borrow_mut().insert(name.clone(), evaled_rhs);
            Ok(Data::None)
        }

        Operator::Index => {
            let list = lhs.eval_list_nth(env, 0)?;
            let nth = lhs.eval_list_nth(env, 1)?.get_integer().unwrap() as usize;
            let evaled = list
                .get_list_nth(nth)
                .ok_or(DSLError::Parse(format!("invalid for indexing: {:?}", list)))?;

            Ok(evaled.clone())
        }
        Operator::If => {
            let cond = lhs.eval_list_nth(env, 0)?;
            let evaled = if cond.is_zero() {
                lhs.eval_list_nth(env, 2)?
            } else {
                lhs.eval_list_nth(env, 1)?
            };
            Ok(evaled)
        }
        Operator::While => {
            while !lhs.eval_list_nth(env, 0)?.is_zero() {
                lhs.eval_list_nth(env, 1)?;
            }
            Ok(Data::None)
        }
        Operator::Slice => {
            let list = lhs.eval_list_nth(env, 0)?;
            let begin = lhs.eval_list_nth(env, 1)?.get_integer().unwrap() as usize;
            let end = lhs.eval_list_nth(env, 2)?.get_integer().unwrap() as usize;
            let evaled = list
                .get_list()
                .ok_or(DSLError::Eval(format!("")))?
                .get(begin..end)
                .ok_or(DSLError::Parse("invalid for slicing".to_string()))?
                .into_iter()
                .map(|d| d.clone())
                .collect::<Vec<Data>>();
            // eprintln!("slice: {:#?}", evaled);
            Ok(Data::List(Rc::new(evaled)))
        }

        Operator::Len => {
            let item = lhs.eval_list_nth(env, 0)?;
            match item {
                Data::List(list) => Ok(Data::Integer(list.len() as i64)),
                Data::String(s) => Ok(Data::Integer(s.len() as i64)),
                _ => Err(DSLError::Eval(format!(
                    "expected list or string but found other"
                ))),
            }
        }
        Operator::Print => {
            let evaled = eval(env, &lhs)?;
            eprintln!("dsl: {:?}", evaled);
            Ok(Data::None)
        }

        Operator::IsDigit => {
            let evaled = lhs.eval_list_nth(env, 0)?;
            Ok(Data::Integer(
                evaled
                    .get_string()
                    .and_then(|s| s.chars().nth(0).map(|c| c.is_digit(10) as i64))
                    .unwrap_or(0),
            ))
        }
        Operator::GetDigit => {
            let evaled = lhs.eval_list_nth(env, 0)?;
            Ok(Data::Integer(
                evaled
                    .get_string()
                    .and_then(|s| s.chars().nth(0).map(|c| c.to_digit(10).unwrap_or(0) as i64))
                    .unwrap_or(0),
            ))
        }
        Operator::Eval => {
            let evaled = eval(env, &lhs)?;
            Ok(evaled.get_list_last().unwrap_or(Data::None))
        }
        Operator::TokenizerNew => {
            let evaled = lhs.eval_list_nth(env, 0)?;
            let tokenizer = TKNZR4ASM::new(evaled.get_string().unwrap().to_string(), *env.source); //todo
            Ok(Data::AsmTokenizer(Rc::new(tokenizer)))
        }
        Operator::TokenizerNext => {
            let tokenizer = lhs.eval_list_nth(env, 0)?.get_tokenizer().unwrap(); //todo

            Ok(Data::AsmToken(tokenizer.next_token()))
        }
        Operator::TokenizerPeek => {
            let tokenizer = lhs.eval_list_nth(env, 0)?.get_tokenizer().unwrap(); //todo

            Ok(Data::AsmToken(tokenizer.peek_token(false)))
        }
        Operator::AsmParse => {
            let object = lhs.eval_list_nth(env, 0)?.get_symbol().unwrap(); //todo
            let tokenizer = lhs.eval_list_nth(env, 1)?.get_tokenizer().unwrap(); //todo
            match object.as_str() {
                "Ins" => Ok(Data::AsmData(Rc::new(parser::parse_compound_ins(tokenizer, scope).unwrap()))),
                _ => return Err(DSLError::Eval(format!("expected spesific symbol")))
            }
            
        }
        _ => unreachable!(),
    }
}

pub fn run<'a>(ast: &AST<'a>, env: Rc<Environment<'a>>, input: String) -> DSLResult<String> {
    match ast {
        AST::List(list) => {
            for func in list.as_ref() {
                let name = func.get_list_nth(0).ok_or(DSLError::Eval(format!("")))?;
                let params = if let AST::List(list) =
                    func.get_list_nth(1).ok_or(DSLError::Eval(format!("")))?
                {
                    list
                } else {
                    todo!()
                };
                let body = func.get_list_nth(2).ok_or(DSLError::Eval(format!("")))?;
                let f = Data::Fn(Rc::new(DSLFn {
                    body: body.clone(),
                    params: params.to_vec(),
                }));
                env.push_global(name.get_data().unwrap().get_symbol().unwrap(), f);
            }
        }
        _ => todo!(),
    }

    let fn_info = env
        .get_variable(Rc::new("main".to_string()))?
        .get_fn()
        .ok_or(DSLError::Eval(format!("")))?;

    let fn_body = &fn_info.body;
    let fn_env = env.enter_fn();
    fn_env.push_var(Rc::new("input".to_string()), Data::String(Rc::new(input)));
    fn_env.push_var(
        Rc::new("output".to_string()),
        Data::String(Rc::new(String::new())),
    );

    eval(&fn_env, &fn_body)?;
    Ok(fn_env
        .get_variable(Rc::new("output".to_string()))
        .unwrap()
        .get_string()
        .unwrap()
        .to_string())
}
