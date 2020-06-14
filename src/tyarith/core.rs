use super::syntax::Term::{self, *};
use super::syntax::Type::{self, *};

#[derive(Debug, PartialEq)]
pub enum EvaluateError {
    NoRuleApplies,
    InvalidApplies,
}

#[derive(Debug, PartialEq)]
pub enum TypeError {
    InvalidType,
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

// This function is big-step operational semantics for evaluation.
pub fn big_step_eval(term: Term) -> Result<Term, EvaluateError> {
    match term {
        term if is_val(&term) => Ok(term),
        If(_, bt1, bt2, bt3) => match big_step_eval(*bt1)? {
            True(_) => big_step_eval(*bt2),
            False(_) => big_step_eval(*bt3),
            _ => Err(EvaluateError::NoRuleApplies),
        },
        Succ(i, bt) => match big_step_eval(*bt)? {
            t if is_numericval(&t) => Ok(Succ(i, Box::new(t))),
            _ => Err(EvaluateError::NoRuleApplies),
        },
        Pred(_, bt) => match big_step_eval(*bt)? {
            Zero(i) => Ok(Zero(i)),
            Succ(_, bt) if is_numericval(&(*bt)) => Ok(*bt),
            _ => Err(EvaluateError::NoRuleApplies),
        },
        IsZero(_, bt) => match big_step_eval(*bt)? {
            Zero(i) => Ok(True(i)),
            Succ(i, bt) if is_numericval(&(*bt)) => Ok(False(i)),
            _ => Err(EvaluateError::NoRuleApplies),
        },
        _ => Err(EvaluateError::NoRuleApplies),
    }
}

fn is_val(term: &Term) -> bool {
    match term {
        True(_) => true,
        False(_) => true,
        t => is_numericval(t),
    }
}

fn is_numericval(term: &Term) -> bool {
    match term {
        Zero(_) => true,
        Succ(_, t) => is_numericval(t),
        _ => false,
    }
}

pub fn type_of(term: Term) -> Result<Type, TypeError> {
    match term {
        True(_) | False(_) => Ok(Bool),
        If(_, bt1, bt2, bt3) => {
            if let Bool = type_of(*bt1)? {
                let t2 = type_of(*bt2);
                let t3 = type_of(*bt3);
                if t2 != t3 {
                    return Err(TypeError::InvalidType);
                }
                return t2;
            } else {
                Err(TypeError::InvalidType)
            }
        }
        Zero(_) => Ok(Nat),
        Succ(_, bt) => {
            if let Nat = type_of(*bt)? {
                Ok(Nat)
            } else {
                Err(TypeError::InvalidType)
            }
        }
        Pred(_, bt) => {
            if let Nat = type_of(*bt)? {
                Ok(Nat)
            } else {
                Err(TypeError::InvalidType)
            }
        }
        IsZero(_, bt) => {
            if let Nat = type_of(*bt)? {
                Ok(Bool)
            } else {
                Err(TypeError::InvalidType)
            }
        }
    }
}
