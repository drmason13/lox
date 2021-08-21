use thiserror::Error;

use crate::{evaluate, lex, parse};

#[derive(Clone, Error, Debug)]
pub enum LoxError {
    #[error(transparent)]
    LexerError(#[from] lex::Error),
    #[error(transparent)]
    ParserError(#[from] parse::Error),
    #[error(transparent)]
    RuntimeError(#[from] RuntimeError),
}

#[derive(Clone, Error, Debug)]
pub enum RuntimeError {
    #[error(transparent)]
    EvaluationError(#[from] evaluate::Error),
}

impl From<evaluate::Error> for LoxError {
    fn from(error: evaluate::Error) -> Self {
        LoxError::RuntimeError(RuntimeError::EvaluationError(error))
    }
}
