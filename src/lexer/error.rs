use thiserror::Error;

use super::Span;

/// The Error type returned by `Lexer`
#[derive(Clone, Error, Debug)]
pub enum Error {
    #[error("{kind} Occurred at {span} {message}")]
    SpannedError {
        kind: ErrorKind,
        message: String,
        span: Span,
    },
    // #[error("{kind} {message}")]
    // UnspannedError { kind: ErrorKind, message: String },
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    UnterminatedString(String),
    InvalidEscape(char),
    InvalidSyntax,
    UnexpectedEOF,
    InternalInterpreterError,
    Fatal,
    Other,
}

impl Error {
    pub fn spanned(msg: impl Into<String>, span: Span, kind: ErrorKind) -> Self {
        Error::SpannedError {
            kind,
            message: msg.into(),
            span,
        }
    }

    pub fn internal_interpreter_error(msg: impl Into<String>, span: Span) -> Self {
        Error::SpannedError {
            kind: ErrorKind::InternalInterpreterError,
            message: msg.into(),
            span,
        }
    }

    pub fn fatal(msg: impl Into<String>, span: Span) -> Self {
        Error::SpannedError {
            kind: ErrorKind::Fatal,
            message: msg.into(),
            span,
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::UnterminatedString(s) => write!(f, "Unterminated String: `{}`.", s),
            ErrorKind::InvalidEscape(c) => write!(f, "Invalid Escape character: `{}`.", c),
            ErrorKind::InvalidSyntax => write!(f, "Syntax Error."),
            ErrorKind::UnexpectedEOF => write!(f, "Unexpected End of Source Code."),
            ErrorKind::InternalInterpreterError => write!(f, "Internal Interpreter Error."),
            ErrorKind::Fatal => write!(f, "Fatal Error!"),
            ErrorKind::Other => write!(f, "Unknown Error."),
        }
    }
}
