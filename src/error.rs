use thiserror::Error;

use crate::escape_string;
use crate::lex::Span;

#[derive(Error, Debug)]
pub enum LoxError {
    #[error("{0}")]
    Generic(SpannedMessage),
    #[error("{0}\n{1}")]
    UnterminatedString(SpannedMessage, String),
}

impl LoxError {
    pub fn new(msg: impl Into<String>, span: Span) -> Self {
        LoxError::Generic(SpannedMessage::new(msg, span))
    }

    pub fn unterminated_string(
        msg: impl Into<String>,
        span: Span,
        unterminated_string: String,
    ) -> Self {
        LoxError::UnterminatedString(
            SpannedMessage::new(msg, span),
            escape_string(&unterminated_string),
        )
    }
}

#[derive(Debug)]
pub struct SpannedMessage {
    message: String,
    span: Span,
}

impl std::fmt::Display for SpannedMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\n{}", self.message, self.span)
    }
}

impl SpannedMessage {
    pub fn new(msg: impl Into<String>, span: Span) -> Self {
        SpannedMessage {
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
