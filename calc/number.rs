//! implementation of the eval() method for Number (BigRational)
//! used by translate for boxing individual numbers

extern crate num;

use super::{CalcResult, Evaluate};
use self::num::rational::BigRational;

#[deriving(Clone)]
pub struct Number(pub BigRational);

impl Evaluate for Number {
    fn eval(&self) -> CalcResult {
        let Number(x) = self.clone();
        Ok(x)
    }
}