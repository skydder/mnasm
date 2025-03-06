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
        },
        // _ => todo!(),
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
        Operator::FnCall => {
            match lhs
                .get_data()
                .ok_or(DSLError::Parse("invalid fn name".to_string()))?
                .as_ref()
            {
                Data::Symbol(index) if index == "index" => {
                    let evaled_rhs = rhs.unwrap().eval(env)?;
                    let evaled = evaled_rhs._index(0)
                        .ok_or(DSLError::Parse("invalid for indexing".to_string()))?
                        .index(evaled_rhs._index(1).ok_or(DSLError::Parse("invalid for indexing".to_string()))?)
                        .ok_or(DSLError::Parse("invalid for indexing".to_string()))?;
                    Ok(evaled)
                }
                _ => todo!(),
            }
        } // _ => todo!(),
    }
}
