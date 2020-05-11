use super::syntax::Term;
use super::syntax::Term::*;

#[derive(Debug, PartialEq)]
pub enum EvaluateError {
    NoRuleApplies,
    InvalidApplies,
}

pub fn eval(term: Term) -> Result<Term, EvaluateError> {
    match _eval(term.clone()) {
        Ok(t) => eval(t),
        Err(EvaluateError::NoRuleApplies) => Ok(term),
        e => e,
    }
}

fn _eval(term: Term) -> Result<Term, EvaluateError> {
    match term {
        If(i, bt1, bt2, bt3) => match *bt1 {
            True(_) => Ok(*bt2),
            False(_) => Ok(*bt3),
            _ => Ok(If(i, Box::new(_eval(*bt1)?), bt2, bt3)),
        },
        Succ(i, bt) => Ok(Succ(i, Box::new(_eval(*bt)?))),
        Pred(i, bt) => match *bt {
            Zero(i) => Ok(Zero(i)),
            Succ(_, bt) if is_numericval(&(*bt)) => Ok(*bt),
            t => Ok(Pred(i, Box::new(_eval(t)?))),
        },
        IsZero(i, bt) => match *bt {
            Zero(i) => Ok(True(i)),
            Succ(i, bt) if is_numericval(&(*bt)) => Ok(False(i)),
            t => Ok(IsZero(i, Box::new(_eval(t)?))),
        },
        _ => Err(EvaluateError::NoRuleApplies),
    }
}

fn is_numericval(term: &Term) -> bool {
    match term {
        Zero(_) => true,
        Succ(_, t) => is_numericval(t),
        _ => false,
    }
}
