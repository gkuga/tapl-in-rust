use self::Term::*;
use std::fmt;

#[derive(PartialEq, Clone, Hash, Eq, Debug)]
pub enum Command {
    Eval(Term),
}

#[derive(PartialEq, Clone, Hash, Eq, Debug)]
pub enum Term {
    True,
    False,
    If(Box<Term>, Box<Term>, Box<Term>),
    Zero,
    Succ(Box<Term>),
    Pred(Box<Term>),
    IsZero(Box<Term>),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            True => write!(f, "true"),
            False => write!(f, "false"),
            If(ref t1, ref t2, ref t3) => write!(f, "(If {} {} {})", t1, t2, t3),
            Zero => write!(f, "0"),
            Succ(ref t) => write!(f, "{}", (*t).to_string().parse::<i32>().unwrap() + 1),
            Pred(ref t) => write!(f, "(pred {})", t),
            IsZero(ref t) => write!(f, "(iszero {})", t),
        }
    }
}
