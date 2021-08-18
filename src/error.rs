use thiserror::Error;

use crate::{lexer, parser};

#[derive(Clone, Error, Debug)]
pub enum LoxError {
    #[error(transparent)]
    LexerError(#[from] lexer::Error),
    #[error(transparent)]
    ParserError(#[from] parser::Error),
}
