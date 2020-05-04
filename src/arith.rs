mod core;
mod lexer;
mod parser;
mod support;
mod syntax;

pub use self::core::eval;
pub use self::parser::parse;
pub use self::syntax::Command;
pub use self::syntax::Term::*;
