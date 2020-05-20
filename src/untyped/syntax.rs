use self::Term::*;
use super::parser::ParseError;
use super::support::Info;
use std::convert::TryFrom;
use std::fmt;

#[derive(PartialEq, Clone, Hash, Eq, Debug)]
pub enum Command {
    Eval(Info, Term),
    Bind(Info, String, Binding),
}

#[derive(PartialEq, Clone, Hash, Eq, Debug)]
pub enum Term {
    Var(Info, usize, usize),
    Abs(Info, String, Box<Term>),
    App(Info, Box<Term>, Box<Term>),
}

#[derive(PartialEq, Clone, Hash, Eq, Debug)]
pub enum Binding {
    NameBind,
}

pub fn get_info(term: &Term) -> Info {
    match term {
        Var(ref i, _, _) => i.clone(),
        Abs(ref i, _, _) => i.clone(),
        App(ref i, _, _) => i.clone(),
    }
}

pub fn name2index(i: Info, ctx: &[(String, Binding)], x: String) -> Result<usize, ParseError> {
    match ctx {
        [] => {
            eprintln!("Identifier {} is unbound", x);
            Err(ParseError::InvalidToken(Info::Unknown))
        }
        [(y, _), rest @ ..] => {
            if *y == x {
                Ok(0)
            } else {
                Ok(1 + name2index(i, rest, x)?)
            }
        }
    }
}

pub fn apply(s: Term, t: Term) -> Term {
    shift(-1, 0, substitute(0, shift(1, 0, s), t))
}

pub fn substitute(j: usize, s: Term, term: Term) -> Term {
    match term {
        Var(i, k, l) => {
            if k == j {
                s
            } else {
                Var(i, k, l)
            }
        }
        Abs(_, _, bt) => substitute(j + 1, shift(1, 0, s), *bt),
        App(i, t1, t2) => App(
            i,
            Box::new(substitute(j, s.clone(), *t1)),
            Box::new(substitute(j, s.clone(), *t2)),
        ),
    }
}

pub fn shift(d: isize, c: usize, term: Term) -> Term {
    match term {
        Var(i, k, l) => {
            if k < c {
                Var(i, k, l)
            } else {
                let n = isize::try_from(k).unwrap() + d;
                Var(i, usize::try_from(n).unwrap(), l)
            }
        }
        Abs(i, x, bt) => Abs(i, x, Box::new(shift(d, c + 1, *bt))),
        App(i, t1, t2) => App(i, Box::new(shift(d, c, *t1)), Box::new(shift(d, c, *t2))),
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Var(_, n, _) => write!(f, "{}", n),
            Abs(_, _, ref t) => write!(f, "(λ.{})", *t),
            App(_, ref t1, ref t2) => write!(f, "({} {})", *t1, *t2),
        }
    }
}
