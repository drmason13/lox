use thiserror::Error;

use crate::escape_string;
use crate::lex::{Span, Token};

#[derive(Clone, Error, Debug)]
pub enum LoxError {
    // general error types
    #[error("{0}")]
    Spanned(SpannedMessage),
    #[error("{0}")]
    Bare(String),
    // specific error types
    #[error("{0}\n{1}")]
    UnterminatedString(SpannedMessage, String),
    #[error("{1:?}\n{0}")]
    SyntaxError(Token, String),
}

impl LoxError {
    pub fn spanned(msg: impl Into<String>, span: Span) -> Self {
        LoxError::Spanned(SpannedMessage::new(msg, span))
    }

    pub fn bare(msg: impl Into<String>) -> Self {
        LoxError::Bare(msg.into())
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

    pub fn syntax_error(msg: impl Into<String>, token: Token) -> Self {
        LoxError::SyntaxError(token, msg.into())
    }
}

#[derive(Clone, Debug)]
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
