//! Expressions

use super::{LiteralType, function, operator, CalcResult, Environment};
use super::tokenize;
use super::tokenize::Token;
use super::operator::OperatorType;

#[deriving(Show, Clone, PartialEq)]
pub enum ExprType {
    Operator(OperatorType),
    Function(String)
}

pub fn token_to_expr(token: Token) -> Result<ExprType, String> {
    match token {
        tokenize::Variable(x) => Ok(Function(x)),
        tokenize::Operator(op_ty) => Ok(Operator(op_ty)),
        _ => Err("Not a valid token!".to_str())
    }
}

#[deriving(Clone, Show, PartialEq)]
pub struct Expression {
    pub expr_type: ExprType,
    pub args: Vec<ArgType>,
}

impl Expression {
    pub fn new(e: ExprType, a: Vec<ArgType>) -> Expression {
        Expression { expr_type: e, args: a }
    }
    pub fn eval(&self, env: &mut Environment) -> CalcResult {
        match self.expr_type {
            Operator(op_type)   => {
                operator::eval(op_type, &self.args, env)
            }
            Function(ref fn_name)    => {
                function::eval(fn_name, &self.args, env)
            }
        }
    }
}

#[deriving(Clone, Show, PartialEq)]
pub enum ArgType {
    Atom(LiteralType),
    SExpr(Expression),
}

pub fn arg_to_literal(arg: &ArgType, env: &mut Environment) -> CalcResult<LiteralType> {
    match arg {
        &SExpr(ref x) => arg_to_literal(&try!(x.eval(env)), env),
        &Atom(ref x) => Ok(x.clone())
    }
}

 
