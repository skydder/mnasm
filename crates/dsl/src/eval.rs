use std::rc::Rc;

use crate::{DSLError, DSLResult, Data, Environment, Operator, AST};

pub fn eval(ast: &AST, env: &Environment) -> DSLResult<Rc<Data>> {
    match ast {
        AST::Data(data) => match data.as_ref() {
            Data::Symbol(name) => Ok(Rc::new(
                env.get_variable(&name, env).map(|v| v.to_data()).unwrap(),
            )),
            _ => Ok(data.clone()),
        },
        AST::Expr(op, lhs, rhs) => apply_op(op, lhs.clone(), rhs.clone(), env),
        AST::List(ast_list) => {
            let mut data_list = Vec::new();
            for ast in ast_list {
                data_list.push(ast.eval(env)?);
            }
            Ok(Rc::new(Data::List(data_list)))
        } // _ => todo!(),
    }
}

fn apply_op(
    op: &Operator,
    lhs: Rc<AST>,
    rhs: Option<Rc<AST>>,
    env: &Environment,
) -> DSLResult<Rc<Data>> {
    match op {
        Operator::AddAssign => {
            let evaled_lhs = lhs.get_data().unwrap().get(env).unwrap();
            let evaled_rhs = rhs.unwrap().eval(env)?; //todo
            evaled_lhs.add(&evaled_rhs);
            Ok(Rc::new(Data::None))
        }
        Operator::Add => {
            let evaled_lhs = lhs.eval(env)?;
            let evaled_rhs = rhs.unwrap().eval(env)?;
            let evaled = evaled_lhs.add(evaled_rhs);
            Ok(evaled)
        }
        Operator::CmpEqual => {
            let evaled_lhs = lhs.eval(env)?;
            let evaled_rhs = rhs.unwrap().eval(env)?;
            let evaled = evaled_lhs.cmp_equal(evaled_rhs);
            Ok(evaled)
        }
        Operator::FnCall => apply_fn(lhs, rhs.unwrap(), env), // _ => todo!(),
    }
}

fn apply_fn(fn_name: Rc<AST>, args: Rc<AST>, env: &Environment) -> DSLResult<Rc<Data>> {
    let fn_name = fn_name
        .get_data()
        .and_then(|f| f.get_symbol())
        .ok_or(DSLError::Parse("invalid fn name".to_string()))?;
    // let args = args.eval(env)?;
    match fn_name.as_str() {
        "index" => {
            let list = args.eval_list_nth(env, 0)?;
            let nth = args.eval_list_nth(env, 1)?;
            let evaled = list
                .index(nth)
                .ok_or(DSLError::Parse("invalid for indexing".to_string()))?;
            Ok(evaled)
        }
        "slice" => {
            let list = args.eval_list_nth(env, 0)?;
            let begin = args.eval_list_nth(env, 1)?;
            let end = args.eval_list_nth(env, 2)?;
            let evaled = list
                .slice(begin, end)
                .ok_or(DSLError::Parse("invalid for slicing".to_string()))?;
            eprintln!("slice: {:#?}", evaled);
            Ok(evaled)
        }
        "if" => {
            let cond = args.eval_list_nth(env, 0)?;
            let evaled = if cond.is_zero() {
                args.eval_list_nth(env, 2)?
            } else {
                args.eval_list_nth(env, 1)?
            };
            Ok(evaled)
        
        }
        _ => todo!(),
    }
}
