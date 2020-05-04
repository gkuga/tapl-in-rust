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
        If(bt1, bt2, _) if *bt1 == True => Ok(*bt2),
        If(bt1, _, bt3) if *bt1 == False => Ok(*bt3),
        If(bt1, bt2, bt3) => Ok(If(Box::new(_eval(*bt1)?), bt2, bt3)),
        Succ(bt) => Ok(Succ(Box::new(_eval(*bt)?))),
        Pred(bt1) => match *bt1 {
            Zero => Ok(Zero),
            Succ(bt2) if is_numericval(&(*bt2)) => Ok(*bt2),
            t => Ok(Pred(Box::new(_eval(t)?))),
        },
        IsZero(bt1) => match *bt1 {
            Zero => Ok(True),
            Succ(bt2) if is_numericval(&(*bt2)) => Ok(False),
            t => Ok(IsZero(Box::new(_eval(t)?))),
        },
        _ => Err(EvaluateError::NoRuleApplies),
    }
}

fn is_numericval(term: &Term) -> bool {
    match term {
        Zero => true,
        Succ(t) => is_numericval(t),
        _ => false,
    }
}
