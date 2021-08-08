mod error;
mod interpreter;
pub mod visitor;

pub use visitor::Visitor;

pub mod lex;
pub(crate) use lex::escape_string;

pub mod parse;
pub use parse::{ast, Parser};

pub use error::LoxError;
pub use interpreter::Interpreter;
