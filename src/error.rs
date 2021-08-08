use thiserror::Error;

use crate::lexing::Span;

#[derive(Error, Debug)]
#[error("Syntax Error: {message}\n{span}")]
pub struct LoxError {
    message: String,
    span: Span,
}

impl LoxError {
    pub(crate) fn new(msg: impl Into<String>, span: Span) -> Self {
        LoxError {
            message: msg.into(),
            span,
        }
    }
}

// pub enum LoxError {
//     #[error("Syntax Error: {message}\n{span}")]
//     InvalidSyntax { message: String, span: Span },
//     #[error("Runtime Error: {message}\n{span}")]
//     RuntimeError { message: String, span: Span },
//     #[error("unknown error")]
//     UnknownError,
// }
