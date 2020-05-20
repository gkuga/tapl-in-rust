use super::syntax::apply;
use super::syntax::Binding;
use super::syntax::Term::{self, *};

#[derive(Debug, PartialEq)]
pub enum EvaluateError {
    NoRuleApplies,
    InvalidApplies,
}

pub fn eval(ctx: Vec<(String, Binding)>, term: Term) -> Result<Term, EvaluateError> {
    match _eval(ctx, term.clone()) {
        Ok((ctx, t)) => eval(ctx, t),
        Err(EvaluateError::NoRuleApplies) => Ok(term),
        Err(e) => Err(e),
    }
}

fn _eval(
    ctx: Vec<(String, Binding)>,
    term: Term,
) -> Result<(Vec<(String, Binding)>, Term), EvaluateError> {
    if let App(i, t1, t2) = term {
        if is_val(&(*t2)) {
            if let Abs(_, _, t12) = *t1 {
                return Ok((ctx, apply(*t2, *t12)));
            }
        }
        if is_val(&(*t1)) {
            let (ctx, r) = _eval(ctx, *t2)?;
            return Ok((ctx, App(i, t1, Box::new(r))));
        }
        let (ctx, t) = _eval(ctx, *t1)?;
        return Ok((ctx, App(i, Box::new(t), t2)));
    }
    Err(EvaluateError::NoRuleApplies)
}

fn is_val(term: &Term) -> bool {
    match term {
        Abs(_, _, _) => true,
        _ => false,
    }
}
