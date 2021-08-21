use thiserror::Error;

use crate::{ast, lexer, parser};

#[derive(Clone, Error, Debug)]
pub enum LoxError {
    #[error(transparent)]
    LexerError(#[from] lexer::Error),
    #[error(transparent)]
    ParserError(#[from] parser::Error),
    #[error(transparent)]
    EvaluationError(#[from] ast::evaluator::EvaluationError),
}
