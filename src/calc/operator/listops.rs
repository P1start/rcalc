//!List operations.

extern crate types;

use self::types::{Atom, SExpr};
use super::super::{Expression, Evaluate};
use super::{ArgType, CalcResult, Environment, BigRational, Ratio};
use super::bigint::*;
use super::super::{LiteralType, BigNum, List, Proc, Symbol, Boolean};
use super::special::range_getter;
use super::super::{BadArgType, BadNumberOfArgs};
use std::iter::range_step;

pub fn proc_getter(args: &Vec<ArgType>, 
                   env: &mut Environment) -> CalcResult<(Vec<String>, Expression)> {
        
    match args[0].clone() {
        Atom(Proc(x, y)) => Ok((x.clone(), y.clone())),
        Atom(Symbol(x)) => proc_getter(&vec!(Atom(try!(env.lookup(&x)))), env),
        SExpr(x) => proc_getter(&vec!(try!(x.eval(env))), env),
        _ =>  Err(BadArgType(format!("Expected function but found {}", args[0])))
    }
}

pub fn create_bigrat(x: int) -> BigRational {
    Ratio::from_integer(x.to_bigint().unwrap())
}


/// Map can handle mapping a function to each element of one or more lists.
pub fn map(args: &Vec<ArgType>, env: &mut Environment) -> CalcResult {
    if args.len() < 2 {
        return Err(BadNumberOfArgs("map".to_string(), "at least".to_string(), 2))
    }

    let (names, func) = try!(proc_getter(args, env));

    if names.len() == 0 || names.len() != args.tail().len() {
        return Err(BadArgType("Wrong number of arguments for lists supplied".to_string()))
    }

    let mut list_vec: Vec<Vec<LiteralType>> = Vec::new();

    for maybe_list in args.tail().iter() {
        let list = try!(maybe_list.arg_to_literal(env));
        match list {
            List(x) => list_vec.push(x),
            _ => return Err(BadArgType(format!("{} is not a list!", list)))
        }
    }

    let mut result: Vec<LiteralType> = Vec::new();
    let len = list_vec[0].len();
    
    for x in range(0u, len) {
        let mut temp: Vec<LiteralType> = Vec::new();
        for y in range(0u, names.len()) {
            if list_vec[y].len() != len {
                return Err(BadArgType("Mismatched lengths!".to_string()))
            }
            temp.push(list_vec.as_slice()[y][x].clone());
        }

        let mut child_env = Environment::new_frame(env);
        for (name_key, list_val) in names.iter().zip(temp.iter()) {
            child_env.symbols.insert(name_key.clone(), list_val.clone());
        }
        result.push(try!(try!(func.eval(&mut child_env)).arg_to_literal(env)));
    }

    Ok(Atom(List(result)))
}

pub fn reduce(args: &Vec<ArgType>, env: &mut Environment) -> CalcResult {
    if args.len() < 3 {
        return Err(BadNumberOfArgs("reduce".to_string(), "at least".to_string(), 3))
    }

    let (names, fun) = try!(proc_getter(args, env));

    let (x, y) = if names.len() != 2 {
        return Err(BadArgType("Expected 2 names".to_string()))
    } else {
        (names[0].clone(), names[1].clone())
    };

    let initval = try!(args[1].desymbolize(env));

    let list = match try!(args[2].desymbolize(env)) {
        List(x) => x.clone(),
        _ => return Err(BadArgType("Invalid type for reduce".to_string()))
    };

    Ok(Atom(try!(reduce_helper(x, y, &initval, list.as_slice(), env, &fun))))
}

pub type LitTy<T = LiteralType> = T;
pub type Env<T = Environment> = T;

pub fn reduce_helper(x: String, y: String, initval: &LitTy, list: &[LitTy], 
                     env: &mut Env, fun: &Expression) -> CalcResult<LitTy> {

    if list.len() == 0 {
        return Err(BadArgType("Cannot fold empty lists!".to_string()))
    }
    
    let mut child_env = Environment::new_frame(env);

    child_env.symbols.insert(x.clone(), list[0].clone());
    child_env.symbols.insert(y.clone(), initval.clone());
    let mut result = try!(try!(fun.eval(&mut child_env)).arg_to_literal(env));

    for val in list.tail().iter() {
        child_env.symbols.insert(x.clone(), val.clone());
        child_env.symbols.insert(y.clone(), result.clone());

        result = try!(try!(fun.eval(&mut child_env)).arg_to_literal(env));
    }

    Ok(result)
}


pub fn filter(args: &Vec<ArgType>, env: &mut Environment) -> CalcResult {
    if args.len() < 2 {
        return Err(BadNumberOfArgs("filter".to_string(), "at least".to_string(), 3))
    }

    let (names, func) = try!(proc_getter(args, env));

    if names.len() != 1 {
        return Err(BadArgType("Expected 1 name for predicate".to_string()))
    }

    let list = match try!(args[1].desymbolize(env)) {
        List(x) => x.clone(),
        _ => return Err(BadArgType("Invalid type for filter".to_string()))
    };

    let mut child_env = Environment::new_frame(env);

    let mut new_list: Vec<LiteralType> = Vec::new();

    for item in list.iter() {
        child_env.symbols.insert(names[0].clone(), item.clone());

        match try!(func.eval(&mut child_env)) {
            Atom(Boolean(true)) => new_list.push(item.clone()),
            Atom(Boolean(false)) => { },
            _ => return Err(BadArgType("Invalid predicate type!".to_string()))
        }
    }

    Ok(Atom(List(new_list)))
}

pub fn rangelist(args: &Vec<ArgType>, env: &mut Environment) -> CalcResult {
    if args.len() < 2 || args.len() > 3 {
        return Err(BadNumberOfArgs("rangelist".to_string(),"at least".to_string(), 2))
    }

    let (a, b) = (try!(range_getter(try!(args[0].desymbolize(env)))),
                  try!(range_getter(try!(args[1].desymbolize(env)))));

    let step = if args.len() == 3 {
        try!(range_getter(try!(args[2].desymbolize(env))))
    } else {
        1
    };

    Ok(Atom(List(range_step(a, b, step).map(|x| BigNum(create_bigrat(x))).collect())))
}


