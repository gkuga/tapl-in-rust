use self::Term::*;
use super::support::Info;
use std::fmt;

#[derive(PartialEq, Clone, Hash, Eq, Debug)]
pub enum Command {
    Eval(Term),
}

#[derive(PartialEq, Clone, Hash, Eq, Debug)]
pub enum Term {
    True(Info),
    False(Info),
    If(Info, Box<Term>, Box<Term>, Box<Term>),
    Zero(Info),
    Succ(Info, Box<Term>),
    Pred(Info, Box<Term>),
    IsZero(Info, Box<Term>),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            True(_) => write!(f, "true"),
            False(_) => write!(f, "false"),
            If(_, ref t1, ref t2, ref t3) => write!(f, "(If {} {} {})", t1, t2, t3),
            Zero(_) => write!(f, "0"),
            Succ(_, ref t) => write!(f, "{}", (*t).to_string().parse::<i32>().unwrap() + 1),
            Pred(_, ref t) => write!(f, "(pred {})", t),
            IsZero(_, ref t) => write!(f, "(iszero {})", t),
        }
    }
}
