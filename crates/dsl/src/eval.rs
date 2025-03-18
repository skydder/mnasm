use std::{cell::RefCell, rc::Rc};

use crate::{Constant, DSLError, DSLResult, Environment, Operator, Variable, AST};

pub fn eval(ast: &AST, env: Rc<Environment>) -> DSLResult<Rc<Constant>> {
    match ast {
        AST::Data(constant) => match constant.as_ref() {
            Constant::Symbol(name) => Ok(Rc::new(
                env.get_variable(&name).map(|v| v.to_constant()).unwrap(),
            )),
            _ => Ok(constant.clone()),
        },
        AST::Expr(op, lhs, rhs) => apply_op(op, lhs.clone(), rhs.clone(), env),
        AST::List(ast_list) => {
            let mut constant_list = Vec::new();
            for ast in ast_list {
                if let AST::Expr(Operator::Break, _, _) = ast {
                    break;
                }
                constant_list.push(ast.eval(env.clone())?);
            }
            Ok(Rc::new(Constant::List(constant_list)))
        }
    }
}

fn apply_op(
    op: &Operator,
    lhs: Rc<AST>,
    rhs: Option<Rc<AST>>,
    env: Rc<Environment>,
) -> DSLResult<Rc<Constant>> {
    match op {
        Operator::AddAssign => {
            let evaled_lhs = lhs.get_data().unwrap().get(&env).unwrap();
            let evaled_rhs = rhs.unwrap().eval(env)?; //todo
            evaled_lhs.add(&evaled_rhs);
            Ok(Rc::new(Constant::None))
        }
        Operator::Add => {
            let evaled_lhs = lhs.eval(env.clone())?;
            let evaled_rhs = rhs.unwrap().eval(env)?;
            match (evaled_lhs.as_ref(), evaled_rhs.as_ref()) {
                (Constant::Integer(lhs), Constant::Integer(rhs)) => {
                    Ok(Rc::new(Constant::Integer(lhs + rhs)))
                }
                (Constant::String(lhs), Constant::String(rhs)) => {
                    Ok(Rc::new(Constant::String(format!("{}{}", lhs, rhs))))
                }
                _ => Err(DSLError::Eval(format!(
                    "cannot evaluate {:#?} + {:#?}",
                    evaled_lhs, evaled_rhs
                ))),
            }
        }
        Operator::CmpEqual => {
            let evaled_lhs = lhs.eval(env.clone())?;
            let evaled_rhs = rhs.unwrap().eval(env)?;
            match (evaled_lhs.as_ref(), evaled_rhs.as_ref()) {
                (Constant::Integer(lhs), Constant::Integer(rhs)) => {
                    Ok(Rc::new(Constant::Integer((*lhs == *rhs) as i64)))
                }
                (Constant::String(lhs), Constant::String(rhs)) => {
                    Ok(Rc::new(Constant::Integer((*lhs == *rhs) as i64)))
                }
                _ => Ok(Rc::new(Constant::Integer(0))),
            }
        }
        Operator::CmpLessThan => {
            let evaled_lhs = lhs.eval(env.clone())?;
            let evaled_rhs = rhs.unwrap().eval(env)?;
            match (evaled_lhs.as_ref(), evaled_rhs.as_ref()) {
                (Constant::Integer(lhs), Constant::Integer(rhs)) => {
                    Ok(Rc::new(Constant::Integer((*lhs < *rhs) as i64)))
                }
                _ => Ok(Rc::new(Constant::Integer(0))),
            }
        }
        Operator::CmpNoMoreThan => {
            let evaled_lhs = lhs.eval(env.clone())?;
            let evaled_rhs = rhs.unwrap().eval(env)?;
            match (evaled_lhs.as_ref(), evaled_rhs.as_ref()) {
                (Constant::Integer(lhs), Constant::Integer(rhs)) => {
                    Ok(Rc::new(Constant::Integer((*lhs <= *rhs) as i64)))
                }
                _ => Ok(Rc::new(Constant::Integer(0))),
            }
        }
        Operator::LOr => {
            let evaled_lhs = lhs.eval(env.clone())?;
            let evaled_rhs = rhs.unwrap().eval(env)?;
            match (evaled_lhs.as_ref(), evaled_rhs.as_ref()) {
                (Constant::Integer(lhs), Constant::Integer(rhs)) => match (*lhs, *rhs) {
                    (0, 0) => Ok(Rc::new(Constant::Integer(0))),
                    _ => Ok(Rc::new(Constant::Integer(1))),
                },
                _ => Ok(Rc::new(Constant::Integer(0))),
            }
        }
        Operator::LAnd => {
            let evaled_lhs = lhs.eval(env.clone())?;
            match evaled_lhs.as_ref() {
                Constant::Integer(i) if *i != 0 => (),
                _ => return Ok(Rc::new(Constant::Integer(0))),
            };
            let evaled_rhs = rhs.unwrap().eval(env)?;
            match evaled_rhs.as_ref() {
                Constant::Integer(i) if *i != 0 => Ok(Rc::new(Constant::Integer(1))),
                _ => Ok(Rc::new(Constant::Integer(0))),
            }
        }
        Operator::Mul => {
            let evaled_lhs = lhs.eval(env.clone())?;
            let evaled_rhs = rhs.unwrap().eval(env)?;
            match (evaled_lhs.as_ref(), evaled_rhs.as_ref()) {
                (Constant::Integer(l), Constant::Integer(r)) => {
                    Ok(Rc::new(Constant::Integer(*l * *r)))
                }
                _ => Err(DSLError::Eval(format!("cant mul"))),
            }
        }
        Operator::MulAssign => {
            let evaled_lhs = lhs.get_data().unwrap().get(&env).unwrap();
            let evaled_rhs = rhs.unwrap().eval(env)?; //todo
            evaled_lhs.mul(&evaled_rhs);
            Ok(Rc::new(Constant::None))
        }
        Operator::List => {
            if let AST::List(ast_list) = lhs.clone().as_ref() {
                let mut evaled = Vec::new();
                for ast in ast_list {
                    evaled.push(ast.eval(env.clone())?);
                }
                Ok(Rc::new(Constant::List(evaled)))
            } else {
                Err(DSLError::Eval(format!("expected list")))
            }
        }
        Operator::Break => Ok(Rc::new(Constant::None)),
        Operator::FnCall => apply_fn(lhs, rhs.unwrap(), env),
    }
}

fn apply_fn(fn_name: Rc<AST>, args: Rc<AST>, env: Rc<Environment>) -> DSLResult<Rc<Constant>> {
    let fn_name = fn_name
        .get_data()
        .and_then(|f| f.get_symbol())
        .ok_or(DSLError::Parse("invalid fn name".to_string()))?;
    match fn_name.as_str() {
        "index" => {
            let list = args.eval_list_nth(env.clone(), 0)?;
            let nth = args.eval_list_nth(env, 1)?.get_integer().unwrap();
            let evaled = list
                ._index(nth as usize)
                .ok_or(DSLError::Parse("invalid for indexing".to_string()))?;

            Ok(evaled)
        }
        "slice" => {
            let list = args.eval_list_nth(env.clone(), 0)?;
            let begin = args.eval_list_nth(env.clone(), 1)?.get_integer().unwrap();
            let end = args.eval_list_nth(env, 2)?.get_integer().unwrap();
            let evaled = list
                ._slice(begin as usize, end as usize)
                .ok_or(DSLError::Parse("invalid for slicing".to_string()))?;
            // eprintln!("slice: {:#?}", evaled);
            Ok(evaled)
        }
        "if" => {
            let cond = args.eval_list_nth(env.clone(), 0)?;
            let evaled = if cond.is_zero() {
                args.eval_list_nth(env, 2)?
            } else {
                args.eval_list_nth(env, 1)?
            };
            Ok(evaled)
        }
        "print" => {
            let evaled = args.eval(env)?;
            eprintln!("dsl: {:?}", evaled);
            Ok(Rc::new(Constant::None))
        }
        "let" => {
            let name = args.get_list_nth(0)?.get_data().unwrap();
            let constant = args.eval_list_nth(env.clone(), 1)?;
            env.push_var(
                name.get_symbol().unwrap(),
                Rc::new(constant.to_type(&env).unwrap()),
            );
            Ok(Rc::new(Constant::None))
        }
        "len" => {
            let item = args.eval_list_nth(env, 0)?;
            match item.as_ref() {
                Constant::List(list) => Ok(Rc::new(Constant::Integer(list.len() as i64))),
                Constant::String(s) => Ok(Rc::new(Constant::Integer(s.len() as i64))),
                _ => Err(todo!()),
            }
        }

        "is_digit" => {
            let item = args.eval_list_nth(env, 0)?;
            if let Constant::String(s) = item.as_ref() {
                Ok(Rc::new(Constant::Integer(
                    s.chars()
                        .nth(0)
                        .and_then(|c| Some(c.is_digit(10)))
                        .unwrap_or(false) as i64,
                )))
            } else {
                Ok(Rc::new(Constant::Integer(0)))
            }
        }

        "get_digit" => {
            let item = args.eval_list_nth(env, 0)?;
            if let Constant::String(s) = item.as_ref() {
                Ok(Rc::new(Constant::Integer(
                    s.chars().nth(0).and_then(|c| c.to_digit(10)).unwrap_or(0) as i64,
                )))
            } else {
                Ok(Rc::new(Constant::Integer(0)))
            }
        }

        "eval" => args.eval(env),
        "while" => {
            while !args.eval_list_nth(env.clone(), 0)?.is_zero() {
                args.eval_list_nth(env.clone(), 1)?;
            }
            Ok(Rc::new(Constant::None))
        }
        name if env.global.borrow().contains_key(name) => {
            let binding = env.clone().get_variable(name)?;
            let (fn_args, body) = if let Variable::Fn(a, b) = binding.as_ref() {
                (a, b)
            } else {
                todo!()
            };
            let fn_env = Rc::new(env.clone().enter_fn());
            for (i, arg) in fn_args.iter().enumerate() {
                let arg_name = arg.get_data().unwrap().get_symbol().unwrap(); // todo: cbb
                let real_arg = Rc::new(
                    args.eval_list_nth(env.clone(), i)?
                        .to_type(&fn_env)
                        .unwrap(),
                );
                fn_env.push_var(arg_name, real_arg);
            }
            body.eval(fn_env).and_then(|c| Ok(c.tail_of_list()))
        }
        _ => Err(DSLError::Eval(format!("undefined function"))),
    }
}

pub fn run(ast: &AST, env: Rc<Environment>, input: String) -> DSLResult<String> {
    match ast {
        AST::List(list) => {
            for func in list {
                let name = func.get_list_nth(0)?;
                let args = if let AST::List(list) = func.get_list_nth(1)? {
                    list
                } else {
                    todo!()
                };
                let body = func.get_list_nth(2)?;
                let f = Rc::new(Variable::Fn(args, body));
                env.push_global(name.get_data().unwrap().get_symbol().unwrap(), f);
            }
        }
        _ => todo!(),
    }
    // cbb
    let binding = env.get_variable("main")?;
    let (args, body) = if let Variable::Fn(a, b) = binding.as_ref() {
        (a, b)
    } else {
        todo!()
    };
    let fn_env = Rc::new(env.enter_fn());
    fn_env.push_var(
        "input".to_string(),
        Rc::new(Variable::String(RefCell::new(input))),
    );
    fn_env.push_var(
        "output".to_string(),
        Rc::new(Variable::String(RefCell::new(String::new()))),
    );
    assert!(args.len() == 0);
    body.eval(fn_env.clone())?;
    Ok(fn_env.get_variable("output").unwrap().get_string().unwrap())
}
