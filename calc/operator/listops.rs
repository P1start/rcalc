//!List operations.

use super::{CalcResult, Environment};
use super::super::literal::{LiteralType, BigNum, List, Proc, Boolean};
use super::super::{Evaluate};
use super::special::{range_getter, create_bigrat};
use super::{ArgType, Atom, arg_to_literal};
use super::super::expression::Expression;

pub fn proc_getter(args: &Vec<ArgType>) -> CalcResult<(Vec<String>, Expression)> {
    
    match args.get(0).clone() {
        Atom(Proc(x, y)) => Ok((x.clone(), y.clone())),
        _ =>  Err(format!("Expected function but found {}", args.get(0))),
    }
}
    

pub fn map(args: &Vec<ArgType>, env: &mut Environment) -> CalcResult {
    if args.len() < 2 {
        return Err("`map' takes at least two arguments".to_str())
    }

    let (names, func) = try!(proc_getter(args));

    if names.len() == 0 || names.len() != args.tail().len() {
        return Err("Wrong number of arguments for lists supplied".to_str())
    }

    let mut list_vec: Vec<Vec<LiteralType>> = Vec::new();

    for maybe_list in args.tail().iter() {
        let list = try!(arg_to_literal(maybe_list, env));
        match list {
            List(x) => list_vec.push(x),
            _ => return Err(format!("{} is not a list!", list))
        }
    }

    let mut result: Vec<LiteralType> = Vec::new();
    let len = list_vec.get(0).len();
    
    for x in range(0u, len) {
        let mut temp: Vec<LiteralType> = Vec::new();
        for y in range(0u, names.len()) {
            if list_vec.get(y).len() != len {
                return Err("Mismatched lengths!".to_str())
            }
            temp.push(list_vec.as_slice()[y].get(x).clone());
        }

        let mut child_env = Environment::new_frame(env);
        for (name_key, list_val) in names.iter().zip(temp.iter()) {
            child_env.symbols.insert(name_key.clone(), list_val.clone());
        }
        result.push(try!(arg_to_literal(&try!(func.eval(&mut child_env)), env)));
    }

    Ok(Atom(List(result)))
}

pub fn reduce(args: &Vec<ArgType>, env: &mut Environment) -> CalcResult {
    if args.len() < 3 {
        return Err("`reduce' takes at least three arguments".to_str())
    }

    let (names, fun) = try!(proc_getter(args));

    if names.len() != 2 {
        return Err("Expected 2 names".to_str())
    }

    let initval = try!(arg_to_literal(args.get(1), env));

    let list = match try!(arg_to_literal(args.get(2), env)) {
        List(x) => x.clone(),
        _ => return Err("Invalid type for reduce".to_str())
    };

    Ok(Atom(try!(reduce_helper(names.as_slice(), &initval,
                               list.as_slice(), env, &fun))))
}

type LitTy<T = LiteralType> = T;
type Env<T = Environment> = T;

pub fn reduce_helper(names: &[String], initval: &LitTy, list: &[LitTy], 
                     env: &mut Env, fun: &Expression) -> CalcResult<LitTy> {
    
    let mut child_env = Environment::new_frame(env);

    child_env.symbols.insert(names[0].clone(), initval.clone());
    child_env.symbols.insert(names[1].clone(), list[0].clone());
    
    let result = try!(arg_to_literal(&try!(fun.eval(&mut child_env)), env));

    if list.len() == 1 {
        Ok(result)
    } else {
        reduce_helper(names, &result, list.tail(), env, fun)
    }
}


pub fn filter(args: &Vec<ArgType>, env: &mut Environment) -> CalcResult {
    if args.len() < 2 {
        return Err("`filter' takes at least three arguments".to_str())
    }

    let (names, func) = try!(proc_getter(args));

    if names.len() != 1 {
        return Err("Expected 1 name for predicate".to_str())
    }

    let list = match try!(arg_to_literal(args.get(1),env)) {
        List(x) => x.clone(),
        _ => return Err("Invalid type for filter".to_str())
    };

    let mut child_env = Environment::new_frame(env);

    let mut new_list: Vec<LiteralType> = Vec::new();

    for item in list.iter() {
        child_env.symbols.insert(names.get(0).clone(), item.clone());

        match try!(func.eval(&mut child_env)) {
            Atom(Boolean(true)) => new_list.push(item.clone()),
            Atom(Boolean(false)) => { },
            _ => return Err("Invalid predicate type!".to_str())
        }
    }

    Ok(Atom(List(new_list)))
}

pub fn rangelist(args: &Vec<ArgType>, env: &mut Environment) -> CalcResult {
    if args.len() != 2 {
        return Err("`rangelist' requires a beginning and end.".to_str())
    }

    let (a, b) = (try!(range_getter(try!(arg_to_literal(args.get(0), env)))),
                  try!(range_getter(try!(arg_to_literal(args.get(1), env)))));

    if a > b {
        return Err("Bad range!".to_str())
    }

    Ok(Atom(List(range(a, b).map(|x| BigNum(create_bigrat(x))).collect())))
}
