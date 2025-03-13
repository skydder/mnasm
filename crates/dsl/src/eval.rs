use std::{cell::RefCell, rc::Rc};

use crate::{DSLError, DSLResult, Data, Environment, Operator, Types, AST};

pub fn eval(ast: &AST, env: Rc<Environment>) -> DSLResult<Rc<Data>> {
    match ast {
        AST::Data(data) => match data.as_ref() {
            Data::Symbol(name) => Ok(Rc::new(
                env.get_variable(&name).map(|v| v.to_data()).unwrap(),
            )),
            _ => Ok(data.clone()),
        },
        AST::Expr(op, lhs, rhs) => apply_op(op, lhs.clone(), rhs.clone(), env),
        AST::List(ast_list) => {
            let mut data_list = Vec::new();
            for ast in ast_list {
                data_list.push(ast.eval(env.clone())?);
            }
            Ok(Rc::new(Data::List(data_list)))
        }
    }
}

fn apply_op(
    op: &Operator,
    lhs: Rc<AST>,
    rhs: Option<Rc<AST>>,
    env: Rc<Environment>,
) -> DSLResult<Rc<Data>> {
    match op {
        Operator::AddAssign => {
            let evaled_lhs = lhs.get_data().unwrap().get(&env).unwrap();
            let evaled_rhs = rhs.unwrap().eval(env)?; //todo
            evaled_lhs.add(&evaled_rhs);
            Ok(Rc::new(Data::None))
        }
        Operator::Add => {
            let evaled_lhs = lhs.eval(env.clone())?;
            let evaled_rhs = rhs.unwrap().eval(env)?;
            let evaled = evaled_lhs.add(evaled_rhs);
            Ok(evaled)
        }
        Operator::CmpEqual => {
            let evaled_lhs = lhs.eval(env.clone())?;
            let evaled_rhs = rhs.unwrap().eval(env)?;
            let evaled = evaled_lhs.cmp_equal(evaled_rhs);
            Ok(evaled)
        }
        Operator::FnCall => apply_fn(lhs, rhs.unwrap(), env), // _ => todo!(),
    }
}

fn apply_fn(fn_name: Rc<AST>, args: Rc<AST>, env: Rc<Environment>) -> DSLResult<Rc<Data>> {
    let fn_name = fn_name
        .get_data()
        .and_then(|f| f.get_symbol())
        .ok_or(DSLError::Parse("invalid fn name".to_string()))?;
    match fn_name.as_str() {
        "index" => {
            let list = args.eval_list_nth(env.clone(), 0)?;
            let nth = args.eval_list_nth(env, 1)?;
            let evaled = list
                .index(nth)
                .ok_or(DSLError::Parse("invalid for indexing".to_string()))?;
            Ok(evaled)
        }
        "slice" => {
            let list = args.eval_list_nth(env.clone(), 0)?;
            let begin = args.eval_list_nth(env.clone(), 1)?;
            let end = args.eval_list_nth(env, 2)?;
            let evaled = list
                .slice(begin, end)
                .ok_or(DSLError::Parse("invalid for slicing".to_string()))?;
            eprintln!("slice: {:#?}", evaled);
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
            Ok(Rc::new(Data::None))
        }
        "let" => {
            let name = args.get_list_nth(0)?.get_data().unwrap();
            let data = args.eval_list_nth(env.clone(), 1)?;
            env.push_var(
                name.get_symbol().unwrap(),
                Rc::new(data.to_type(&env).unwrap()),
            );
            Ok(Rc::new(Data::None))
        }
        "len" => {
            let item = args.eval_list_nth(env, 0)?;
            Ok(item.len().unwrap())
        }
        name if env.global.borrow().contains_key(name) => {
            let binding = env.clone().get_variable(name)?;
            let (fn_args, body) = if let Types::Fn(a, b) = binding.as_ref() {
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
            body.eval(fn_env)
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
                let f = Rc::new(Types::Fn(args, body));
                env.push_global(name.get_data().unwrap().get_symbol().unwrap(), f);
            }
        }
        _ => todo!(),
    }
    // cbb
    let binding = env.get_variable("main")?;
    let (args, body) = if let Types::Fn(a, b) = binding.as_ref() {
        (a, b)
    } else {
        todo!()
    };
    let fn_env = Rc::new(env.enter_fn());
    fn_env.push_var(
        "input".to_string(),
        Rc::new(Types::String(RefCell::new(input))),
    );
    fn_env.push_var(
        "output".to_string(),
        Rc::new(Types::String(RefCell::new(String::new()))),
    );
    assert!(args.len() == 0);
    body.eval(fn_env.clone())?;
    Ok(fn_env.get_variable("output").unwrap().get_string().unwrap())
}
