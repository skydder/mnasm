use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use crate::{data::DSLFn, DSLError, DSLResult};

use super::{Data, Environment, Operator, AST};

pub fn eval(env: &Environment, ast: &AST) -> DSLResult<Data> {
    match ast {
        AST::Expr(operator, lhs, rhs) => eval_expr(env, *operator, lhs.clone(), rhs.clone()),
        AST::Data(data) => match data.as_ref() {
            Data::Symbol(sym) => env.get_variable(sym.as_str()),
            _ => Ok(data.as_ref().clone()),
        },
        AST::List(asts) => {
            // // this part is only for making Data::List
            // // use other function if you want to eval block or function body
            let mut constant_list = Vec::new();
            for ast in asts.as_ref() {
                constant_list.push(eval(env, ast)?);
            }
            Ok(Data::List(Rc::new(RefCell::new(constant_list))))
        }
    }
}

fn eval_expr(
    env: &Environment,
    operator: Operator,
    lhs: Rc<AST>,
    rhs: Option<Rc<AST>>,
) -> DSLResult<Data> {
    match operator {
        Operator::FnCall => apply_fn(env, lhs, rhs.unwrap()),
        _ => eval_built_in(env, operator, lhs, rhs),
    }
}

fn apply_fn(env: &Environment, fn_name: Rc<AST>, fn_args: Rc<AST>) -> DSLResult<Data> {
    // prep for applying function
    let fn_name = fn_name
        .get_data()
        .ok_or(todo!())?
        .get_symbol()
        .ok_or(todo!())?.as_str();
    let fn_info = env
        .get_variable(fn_name)?
        .get_fn()
        .ok_or(todo!())?
        .as_ref();
    let fn_args = fn_args.get_list().ok_or(todo!())?.iter();
    let fn_env = env.clone().enter_fn();

    // error handling
    if fn_args.len() != fn_info.params.len() {
        return Err(todo!());
    }

    // prep env
    for (param, arg) in fn_info.params.iter().zip(fn_args) {
        let param_name = param
            .get_data()
            .ok_or(todo!())?
            .get_symbol()
            .ok_or(todo!())?.to_string(); // todo: cbb
        let evaled_arg = eval(env, arg)?; //todo
        fn_env.push_var(param_name, evaled_arg);
    }

    eval(&fn_env, &fn_info.body)?
        .get_list()
        .unwrap()
        .borrow()
        .last()
        .map(|d| d.clone())
        .ok_or(DSLError::Eval(format!("something is wrong")))
}

fn eval_built_in(
    env: &Environment,
    operator: Operator,
    lhs: Rc<AST>,
    rhs: Option<Rc<AST>>,
) -> DSLResult<Data> {
    match operator {
        Operator::AddAssign => todo!(),
        Operator::MulAssign => todo!(),

        Operator::Add => {
            let evaled_lhs = eval(env, lhs.as_ref())?;
            let evaled_rhs = eval(env, rhs.unwrap().as_ref())?;
            match (evaled_lhs, evaled_rhs) {
                (Data::Integer(l), Data::Integer(r)) => {
                    Ok(Data::Integer(Rc::new(Cell::new(l.get() + r.get()))))
                }
                (Data::String(l), Data::String(r)) => Ok(Data::String(Rc::new(RefCell::new(
                    format!("{}{}", l.borrow(), r.borrow()),
                )))),
                (Data::List(l), r) => {
                    let mut list = l.borrow().clone();
                    list.push(r);
                    Ok(Data::List(Rc::new(RefCell::new(list))))
                }
                _ => todo!(),
            }
        }
        Operator::CmpEqual => {
            let evaled_lhs = eval(env, lhs.as_ref())?;
            let evaled_rhs = eval(env, rhs.unwrap().as_ref())?;
            Ok(Data::Integer(Rc::new(Cell::new(
                (evaled_lhs == evaled_rhs) as i64,
            ))))
        }
        Operator::CmpLessThan => {
            let evaled_lhs = eval(env, lhs.as_ref())?;
            let evaled_rhs = eval(env, rhs.unwrap().as_ref())?;
            match (evaled_lhs, evaled_rhs) {
                (Data::Integer(l), Data::Integer(r)) => Ok(Data::Integer(Rc::new(Cell::new(
                    (l.get() < r.get()) as i64,
                )))),
                _ => Ok(Data::Integer(Rc::new(Cell::new(0)))),
            }
        }
        Operator::CmpNoMoreThan => {
            let evaled_lhs = eval(env, lhs.as_ref())?;
            let evaled_rhs = eval(env, rhs.unwrap().as_ref())?;
            match (evaled_lhs, evaled_rhs) {
                (Data::Integer(l), Data::Integer(r)) => Ok(Data::Integer(Rc::new(Cell::new(
                    (l.get() <= r.get()) as i64,
                )))),
                _ => Ok(Data::Integer(Rc::new(Cell::new(0)))),
            }
        }
        Operator::Break => Ok(Data::None),
        Operator::List => {
            if let AST::List(ast_list) = lhs.clone().as_ref() {
                let mut evaled = Vec::new();
                for ast in ast_list.as_ref() {
                    evaled.push(eval(env, ast)?);
                }
                Ok(Data::List(Rc::new(RefCell::new(evaled))))
            } else {
                Err(DSLError::Eval(format!("expected list")))
            }
        }
        Operator::LOr => {
            let evaled_lhs = eval(env, lhs.as_ref())?;
            let evaled_rhs = eval(env, rhs.unwrap().as_ref())?;
            match (evaled_lhs, evaled_rhs) {
                (Data::Integer(l), Data::Integer(r)) => match (l.get(), r.get()) {
                    (0, 0) => Ok(Data::Integer(Rc::new(Cell::new(0)))),
                    _ => Ok(Data::Integer(Rc::new(Cell::new(0)))),
                },
                _ => Ok(Data::Integer(Rc::new(Cell::new(0)))),
            }
        }
        Operator::LAnd => {
            let evaled_lhs = eval(env, lhs.as_ref())?;
            match evaled_lhs.get_integer() {
                Some(i) if i.get() != 0 => (),
                _ => return Ok(Data::Integer(Rc::new(Cell::new(0)))),
            };
            let evaled_rhs = eval(env, rhs.unwrap().as_ref())?;
            match evaled_rhs.get_integer() {
                Some(i) if i.get() != 0 => Ok(Data::Integer(Rc::new(Cell::new(1)))),
                _ => Ok(Data::Integer(Rc::new(Cell::new(0)))),
            }
        }
        Operator::Mul => {
            let evaled_lhs = eval(env, lhs.as_ref())?;
            let evaled_rhs = eval(env, rhs.unwrap().as_ref())?;
            match (evaled_lhs, evaled_rhs) {
                (Data::Integer(l), Data::Integer(r)) => {
                    Ok(Data::Integer(Rc::new(Cell::new(l.get() * r.get()))))
                }
                _ => Err(DSLError::Eval(format!("cant mul"))),
            }
        }
        Operator::Let => todo!(),
        Operator::Index => {
            let list = lhs.eval_list_nth(env, 0)?;
            let nth = lhs.eval_list_nth(env, 1)?.get_integer().unwrap().get() as usize;
            let evaled = list
                .get_list()
                .ok_or(todo!())?
                .borrow()
                .get(nth)
                .ok_or(DSLError::Parse("invalid for indexing".to_string()))?;

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
            let begin = lhs.eval_list_nth(env, 1)?.get_integer().unwrap().get() as usize;
            let end = lhs.eval_list_nth(env, 2)?.get_integer().unwrap().get() as usize;
            let evaled = list
                .get_list()
                .ok_or(todo!())?
                .borrow()
                .get(begin..end)
                .ok_or(DSLError::Parse("invalid for slicing".to_string()))?
                .into_iter()
                .map(|d| d.clone())
                .collect::<Vec<Data>>();
            // eprintln!("slice: {:#?}", evaled);
            Ok(Data::List(Rc::new(RefCell::new(evaled))))
        }

        Operator::Len => {
            let item = lhs.eval_list_nth(env, 0)?;
            match item {
                Data::List(list) => Ok(Data::Integer(Rc::new(Cell::new(
                    list.borrow().len() as i64
                )))),
                Data::String(s) => Ok(Data::Integer(Rc::new(Cell::new(s.borrow().len() as i64)))),
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
        _ => unreachable!(),
    }
}

pub fn run(ast: &AST, env: Rc<Environment>, input: String) -> DSLResult<String> {
    match ast {
        AST::List(list) => {
            for func in list.as_ref() {
                let name = func.get_list().ok_or(todo!())?.get(0).ok_or(todo!())?;
                let params = if let AST::List(list) = func.get_list().ok_or(todo!())?.get(1).ok_or(todo!())? {
                    list
                } else {
                    todo!()
                };
                let body = func.get_list().ok_or(todo!())?.get(2).ok_or(todo!())?;
                let f = Data::Fn(Rc::new(DSLFn { body: body.clone(), params: params.to_vec() }));
                env.push_global(name.get_data().unwrap().get_symbol().unwrap().to_string(), f);
            }
        }
        _ => todo!(),
    }

    // let fn_name = fn_name
    //     .get_data()
    //     .ok_or(todo!())?
    //     .get_symbol()
    //     .ok_or(todo!())?;
    // let fn_info = env
    //     .get_variable(fn_name.as_str())?
    //     .get_fn()
    //     .ok_or(todo!())?
    //     .as_ref();
    // let (fn_body, fn_params) = (fn_info.body, fn_info.params);
    // let fn_args = fn_args.get_list().ok_or(todo!())?;
    // let fn_env = env.clone().enter_fn();

    // // error handling
    // if fn_args.len() != fn_params.len() {
    //     return Err(todo!());
    // }

    // // prep env
    // for (param, arg) in fn_params.iter().zip(fn_args.iter()) {
    //     let param_name = param
    //         .get_data()
    //         .ok_or(todo!())?
    //         .get_symbol()
    //         .ok_or(todo!())?; // todo: cbb
    //     let evaled_arg = eval(env, arg)?; //todo
    //     fn_env.push_var(param_name.to_string(), evaled_arg);
    // }

    // eval(&fn_env, &fn_body)?
    //     .get_list()
    //     .unwrap()
    //     .borrow()
    //     .last()
    //     .map(|d| d.clone())
    //     .ok_or(DSLError::Eval(format!("something is wrong")))
    // cbb
    let fn_info = env
        .get_variable("main")?
        .get_fn()
        .ok_or(todo!())?
        .as_ref();
    let fn_body = fn_info.body;
    let fn_env = env.enter_fn();
    fn_env.push_var(
        "input".to_string(),
        Data::String(Rc::new(RefCell::new(input))),
    );
    fn_env.push_var(
        "output".to_string(),
        Data::String(Rc::new(RefCell::new(String::new()))),
    );
    
    eval(&fn_env, &fn_body)?;
    Ok(fn_env.get_variable("output").unwrap().get_string().unwrap().borrow().to_string())
}
