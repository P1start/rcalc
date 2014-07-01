//! Logic and odering.

use super::super::{CalcResult, Environment, NonBoolean, BadNumberOfArgs, BadArgType};
use super::super::literal::{Boolean, BigNum, LiteralType};
use super::{ArgType, Atom, SExpr, Mpq, Mpz, If};

pub type Args<T = Vec<ArgType>> = T;
pub type Env<T = Environment> = T;

/// Loop through nested conditional statements until a non-conditional expression
/// is reached.
pub fn cond(args: &Args, env: &mut Env)  -> CalcResult {
    let mut arguments = args.clone();

    loop {
        if args.len() != 3 {
            return Err(BadNumberOfArgs(format!(
                "`if` requires three arguments, {} provided", args.len())))
        }

        let condition = match try!(arguments.get(0).desymbolize(env)) {
            Boolean(x)  => x,
            _ => return Err(NonBoolean)
        };

        let result = if condition {
            arguments.get(1).clone()
        } else {
            arguments.get(2).clone()
        };

        match result {
            Atom(_) => return Ok(result),
            SExpr(ref x) => {
                if x.expr_type == super::super::expression::Operator(If) {
                    arguments = x.args.clone();
                } else {
                    return x.eval(env)
                }
            }
        }
    }
}

pub type LitTy = LiteralType;

pub fn ordering(args: &Vec<ArgType>, env: &mut Env, comp: |LitTy, LitTy| -> bool) -> CalcResult {
    if args.len() != 2 {
        return Err(BadNumberOfArgs("Ordering requires two arguments".to_str()))
    }


    Ok(Atom(Boolean(comp(try!(args.get(0).desymbolize(env)),
                         try!(args.get(1).desymbolize(env))))))
}

pub fn and_or(args: &Args, env: &mut Env, short: bool) -> CalcResult {
    
    for val in args.iter() {
        if try!(val.desymbolize(env)) == Boolean(short) {
            return Ok(Atom(Boolean(short)))
        }
    }

    Ok(Atom(Boolean(!short)))
}

pub fn xor(args: &Args, env: &mut Env) -> CalcResult {
    if args.len() < 2 {
        return Err(BadNumberOfArgs("`xor' requires at least two arguments".to_str()))
    }

    let mut result = match try!(args.get(0).desymbolize(env)) {
        Boolean(x) => x,
        _ => return Err(NonBoolean)
    };

    for val in args.tail().iter() {
        result = match try!(val.desymbolize(env)) {
            Boolean(x) => result ^ x,
            _ => return Err(NonBoolean)
        };
    }

    Ok(Atom(Boolean(result)))
}

pub fn not(args: &Args, env: &mut Env) -> CalcResult {
    if args.len() != 1 {
        return Err(BadNumberOfArgs("Not only takes one argument".to_str()))
    }

    let val = match try!(args.get(0).desymbolize(env)) {
        Boolean(x)  => x,
        _   => return Err(NonBoolean)
    };

    Ok(Atom(Boolean(!val)))
}

#[deriving(PartialEq)]
pub enum NumOps {
    Round,
    Floor,
    Ceiling,
    Even,
    Odd,
    Zero,
}

impl NumOps {
    pub fn name(self) -> String {
        let s = match self {
            Round => "`round'",
            Floor => "`floor'",
            Ceiling => "`ceiling'",
            Even => "`even?'",
            Odd => "`odd?'",
            Zero => "`zero?",
        };
        s.to_str()
    }

    pub fn idea(self) -> String {
        let s = match self {
            Round => "be rounded",
            Floor => "have their floor returned",
            Ceiling => "have their ceiling returned",
            Even => "be even",
            Odd => "be odd",
            Zero => "be zero",
        };

        s.to_str()
    }
}

pub fn num_op(args: &Args, env: &mut Env, op: NumOps) -> CalcResult {
    use std::num;

    let one: Mpq = num::one();
    let two = one + one;
    let half = one / two;

    if args.len() != 1 {
        return Err(BadNumberOfArgs(format!("{} only takes one argument", op.name())))
    }

    let num = match try!(args.get(0).desymbolize(env)) {
        BigNum(x) => x,
        _ => return Err(BadArgType(format!("Only numbers can {}", op.idea())))
    };

    match op { 
        //round() doesn't work right for BigRationals.
/*
        Round => Ok(Atom(BigNum(if num - num.floor() >= half { 
            num.ceil() 
        } else { num.floor()
        }))),
        Floor => Ok(Atom(BigNum(num.floor()))),
        Ceiling => Ok(Atom(BigNum(num.ceil()))),
*/
        Floor | Ceiling | Round => Ok(Atom(BigNum(num))), //sorry 
        Zero => Ok(Atom(Boolean(num.is_zero()))),
        _ => Ok(Atom(Boolean((op != Even) ^ (num.get_num() % two == num::zero()
                                             && *num.get_den() == num::one()))))
    }
}