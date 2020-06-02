mod core;
mod lexer;
mod parser;
mod support;
mod syntax;

pub use self::core::{big_step_eval, eval};
pub use self::parser::parse;
pub use self::syntax::Term::*;
pub use self::syntax::{Binding, Command};
